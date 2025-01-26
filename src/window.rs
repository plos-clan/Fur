use crate::{color::Color, display::DisplayDriver};

pub struct Window {
    width: usize,
    height: usize,
}

impl Window {
    pub fn new(width: usize, height: usize) -> Self {
        Self { width, height }
    }
}

impl Window {
    pub fn draw<T: DisplayDriver>(&self, driver: &mut T) {
        let titile_color = Color::new_rgb(0x1a, 0x1a, 0x1a);

        driver.write(
            0,
            0,
            self.width,
            self.height,
            &titile_color,
        );
    }
}
