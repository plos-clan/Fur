use alloc::{vec, vec::Vec};

use crate::{color::Color, display::DisplayDriver, pixel::PixelFormat};

#[allow(dead_code)]
pub struct DrawBuffer {
    buffer: Vec<u32>,
    width: usize,
    height: usize,
    pixel_format: PixelFormat,
}

impl DrawBuffer {
    pub fn new(width: usize, height: usize, pixel_format: PixelFormat) -> Self {
        if pixel_format == PixelFormat::U8 {
            panic!("U8 is not supported for DrawBuffer.");
        }
        Self {
            buffer: vec![0; width * height * 4],
            width,
            height,
            pixel_format,
        }
    }
}

impl DisplayDriver for DrawBuffer {
    fn read(
        &self,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        pixels: &mut [crate::color::Color],
    ) {
        assert!(x < self.width);
        assert!(y < self.height);

        for dx in 0..width {
            for dy in 0..height {
                let t_x = x + dx;
                let t_y = y + dy;
                pixels[dy * width + dx] = self
                    .pixel_format
                    .u32_as_color(self.buffer[t_y * self.width + t_x]);
            }
        }
    }

    fn write(
        &mut self,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        pixels: &[crate::color::Color],
    ) {
        assert!(x < self.width);
        assert!(y < self.height);

        for dx in 0..width {
            for dy in 0..height {
                let t_x = x + dx;
                let t_y = y + dy;
                self.buffer[t_y * self.width + t_x] =
                    self.pixel_format.color_as_u32(&pixels[dy * width + dx]);
            }
        }
    }
}

#[allow(dead_code)]
pub struct ColorBuffer {
    buffer: Vec<Color>,
    width: usize,
    height: usize,
}

impl ColorBuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            buffer: vec![Color::new_rgb(0, 0, 0); width * height],
            width,
            height,
        }
    }
}

impl DisplayDriver for ColorBuffer {
    fn read(&self, x: usize, y: usize, width: usize, height: usize, pixels: &mut [Color]) {
        assert!(x < self.width);
        assert!(y < self.height);

        for dx in 0..width {
            for dy in 0..height {
                let t_x = x + dx;
                let t_y = y + dy;
                pixels[dy * width + dx] = self.buffer[t_y * self.width + t_x].clone();
            }
        }
    }

    fn write(&mut self, x: usize, y: usize, width: usize, height: usize, pixels: &[Color]) {
        assert!(x < self.width);
        assert!(y < self.height);

        for dx in 0..width {
            for dy in 0..height {
                let t_x = x + dx;
                let t_y = y + dy;
                self.buffer[t_y * self.width + t_x] = pixels[dy * width + dx].clone();
            }
        }
    }
}
