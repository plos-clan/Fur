use alloc::string::String;

use crate::{color::Color, display::DisplayDriver};

pub struct WindowBuilder {
    width: usize,
    height: usize,
    title_height: usize,
    title: String,
}

impl WindowBuilder {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            title_height: 20,
            title: String::new(),
        }
    }
}

impl WindowBuilder {
    pub fn width(&mut self, width: usize) -> &mut Self {
        self.width = width;
        self
    }

    pub fn height(&mut self, height: usize) -> &mut Self {
        self.height = height;
        self
    }

    pub fn title_height(&mut self, title_height: usize) -> &mut Self {
        self.title_height = title_height;
        self
    }

    pub fn title<S>(&mut self, title: S) -> &mut Self
    where
        String: From<S>,
    {
        self.title = String::from(title);
        self
    }
}

impl WindowBuilder {
    pub fn draw<T: DisplayDriver>(&self, driver: &mut T) {
        let titile_color = Color::new_rgb(0x1a, 0x1a, 0x1a);

        driver.write(0, 0, self.width, 20, &titile_color);
    }
}
