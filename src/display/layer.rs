use super::{ColorBuffer, DisplayDriver};

/// This is not the real layer, but the id of the layer in the display.
/// You can get the references of the real layer by calling function `Display::layer` and `Display::layer_mut`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Layer {
    id: usize,
}

impl Layer {
    pub(crate) fn new(id: usize) -> Self {
        Self { id }
    }
}

/// The real layer. But it does not contain the display driver.
pub struct LayerData {
    width: usize,
    height: usize,
    x: usize,
    y: usize,
    color_buffer: ColorBuffer,
    pub(crate) priority: usize,
}

impl LayerData {
    pub(crate) fn new(width: usize, height: usize, x: usize, y: usize, priority: usize) -> Self {
        Self {
            width,
            height,
            x,
            y,
            color_buffer: ColorBuffer::new(width, height),
            priority,
        }
    }
}

impl LayerData {
    pub fn position(&self) -> (usize, usize) {
        (self.x, self.y)
    }

    pub fn size(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    pub fn move_to(&mut self, x: usize, y: usize) {
        self.x = x;
        self.y = y;
    }

    pub fn resize(&mut self, width: usize, height: usize) {
        self.width = width;
        self.height = height;
    }
}

impl DisplayDriver for LayerData {
    fn read(
        &self,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        pixels: &mut [crate::color::Color],
    ) {
        self.color_buffer.read(x, y, width, height, pixels);
    }

    fn write(
        &mut self,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        pixels: &[crate::color::Color],
    ) {
        self.color_buffer.write(x, y, width, height, pixels);
    }
}
