use alloc::{collections::BTreeMap, sync::Arc, vec::Vec};
use spin::RwLock;

use crate::color::Color;

mod buffers;
mod layer;

pub use buffers::*;
pub use layer::*;

/// Implement this trait if you need to customize the action of reading and writing displays.
/// The Display structure needs a type implemented this trait.
/// The most easy one is to prepare a buffer.
/// ## Example
/// ``` rust
///
/// use fur::{color::Color, display::DisplayDriver};
///
/// const WIDTH: usize = 320;
/// const HEIGHT: usize = 240;
///
/// pub struct DrawBuffer {
///     data: Vec<u32>,
/// }
///
/// impl DrawBuffer {
///     pub fn new() -> Self {
///         Self {
///             data: (0..WIDTH*HEIGHT).map(|_| 0).collect::<Vec<_>>(),
///         }
///     }
/// }
///
/// impl DisplayDriver for DrawBuffer {
///     fn read(&self, x: usize, y: usize, width: usize, height: usize, pixels: &mut [Color]) {
///         for dx in 0..width {
///             for dy in 0..height {
///                 let t_x = dx + x;
///                 let t_y = dy + y;
///                 pixels[dy * width + dx] = Color::from_argb_u32(self.data[t_y * WIDTH + t_x]);
///             }
///         }
///     }
///
///     fn write(&mut self, x: usize, y: usize, width: usize, height: usize, pixels: &[Color]) {
///         for dx in 0..width {
///             for dy in 0..height {
///                 let t_x = dx + x;
///                 let t_y = dy + y;
///                 self.data[t_y * WIDTH + t_x] = pixels[dy * width + dx].as_argb_u32();
///             }
///         }
///     }    
/// }
/// ```
pub trait DisplayDriver {
    /// Read pixels from (x,y) to `pixels`, and you need to tell the width and the height.
    fn read(&self, x: usize, y: usize, width: usize, height: usize, pixels: &mut [Color]);
    /// The same as `read`, but it writes pixels.
    fn write(&mut self, x: usize, y: usize, width: usize, height: usize, color: &Color);
    /// Get the size of the display.
    fn size(&self) -> (usize, usize);
}

/// The main structure of FUR. \
/// It provides layer management, all the widgets are ploted here. \
/// It needs a display driver, which implements the `DisplayDriver` trait.
pub struct Display {
    driver: Arc<RwLock<dyn DisplayDriver>>,
    width: usize,
    height: usize,
    layers: BTreeMap<Layer, LayerData>,
    layer_sorted: BTreeMap<usize, Vec<Layer>>,
}

impl Display {
    /// Create a new display with a display driver and the following width and height.
    pub fn new(driver: Arc<RwLock<dyn DisplayDriver>>) -> Self {
        let (width, height) = driver.read().size();
        Self {
            driver,
            width,
            height,
            layers: BTreeMap::new(),
            layer_sorted: BTreeMap::new(),
        }
    }
}

impl DisplayDriver for Display {
    fn read(&self, x: usize, y: usize, width: usize, height: usize, pixels: &mut [Color]) {
        debug_assert!(x < self.width);
        debug_assert!(y < self.height);

        self.driver.read().read(x, y, width, height, pixels);
    }

    fn write(&mut self, x: usize, y: usize, width: usize, height: usize, color: &Color) {
        debug_assert!(x < self.width);
        debug_assert!(y < self.height);

        self.driver.write().write(x, y, width, height, color);
    }

    fn size(&self) -> (usize, usize) {
        (self.width, self.height)
    }
}

impl Display {
    /// Create a new layer.
    pub fn create_layer(&mut self, width: usize, height: usize, x: usize, y: usize) -> Layer {
        let id = self.layers.len();
        let layer = Layer::new(id);
        self.layers
            .insert(layer, LayerData::new(width, height, x, y, id));
        if let Some(layers) = self.layer_sorted.get_mut(&id) {
            layers.push(layer);
        } else {
            self.layer_sorted.insert(id, alloc::vec![layer]);
        }

        layer
    }

    /// Get the immutable reference to the layer's data.
    pub fn layer(&self, layer: &Layer) -> Option<&LayerData> {
        self.layers.get(layer)
    }

    /// Get the mutable reference to the layer's data. \
    /// So that you can modify it.
    pub fn layer_mut(&mut self, layer: &Layer) -> Option<&mut LayerData> {
        self.layers.get_mut(layer)
    }
}

impl Display {
    pub fn put_upper_than(&mut self, layer: &Layer, other: &Layer) -> Option<()> {
        let old_priority = self.layer(layer)?.priority;
        let new_priority = self.layer(other)?.priority + 1;
        self.layer_sorted
            .get_mut(&old_priority)
            .unwrap()
            .retain(|l| l != layer);
        if let Some(layers) = self.layer_sorted.get_mut(&new_priority) {
            layers.push(*layer);
        } else {
            self.layer_sorted.insert(new_priority, alloc::vec![*layer]);
        }
        self.layer_mut(layer)?.priority = new_priority;
        Some(())
    }

    pub fn flush_all(&self) {
        for (_, layers) in self.layer_sorted.iter() {
            for layer in layers {
                let layer_data = self.layer(layer).unwrap();
                let (x, y) = layer_data.position();
                let (width, height) = layer_data.size();

                for dx in 0..width {
                    for dy in 0..height {
                        let t_x = dx + x;
                        let t_y = dy + y;

                        let mut color = [Color::new_rgb(0, 0, 0)];
                        layer_data.read(dx, dy, 1, 1, &mut color);
                        let color = color[0].clone();

                        let mut base_color = [Color::new_rgb(0, 0, 0)];
                        self.driver.read().read(t_x, t_y, 1, 1, &mut base_color);
                        let base_color = base_color[0].clone();

                        let color = base_color.mix(&color);
                        self.driver.write().write(t_x, t_y, 1, 1, &color);
                    }
                }
            }
        }
    }

    pub fn flush_area(&self, x_range: (usize, usize), y_range: (usize, usize)) {
        for (_, layers) in self.layer_sorted.iter() {
            for layer in layers {
                let layer_data = self.layer(layer).unwrap();
                let (x, y) = layer_data.position();
                let (width, height) = layer_data.size();
                if x >= x_range.0
                    && !((x + width) < x_range.0)
                    && y >= y_range.0
                    && !((y + height) < y_range.0)
                {
                    for dx in 0..width {
                        for dy in 0..height {
                            let t_x = dx + x;
                            let t_y = dy + y;

                            let mut color = [Color::new_rgb(0, 0, 0)];
                            layer_data.read(dx, dy, 1, 1, &mut color);
                            let color = color[0].clone();

                            let mut base_color = [Color::new_rgb(0, 0, 0)];
                            self.driver.read().read(t_x, t_y, 1, 1, &mut base_color);
                            let base_color = base_color[0].clone();

                            let color = base_color.mix(&color);
                            self.driver.write().write(t_x, t_y, 1, 1, &color);
                        }
                    }
                }
            }
        }
    }
}
