use alloc::string::String;
use cosmic_text::{Buffer, FontSystem, Metrics};

use crate::{color::Color, display::DisplayDriver};

#[allow(unused)]
pub struct TextBuilder {
    font_system: FontSystem,
    metrics: Metrics,
    text: String,
    position: (usize, usize),
    color: Color,
}

impl Default for TextBuilder {
    fn default() -> Self {
        Self {
            font_system: FontSystem::new(),
            metrics: Metrics::new(14.0, 10.0),
            text: String::new(),
            position: (0, 0),
            color: Color::new_rgb(0xff, 0xff, 0xff),
        }
    }
}

impl TextBuilder {
    pub fn position(&mut self, position: (usize, usize)) -> &mut Self {
        self.position = position;
        self
    }

    pub fn x(&mut self, x: usize) -> &mut Self {
        self.position.0 = x;
        self
    }

    pub fn y(&mut self, y: usize) -> &mut Self {
        self.position.1 = y;
        self
    }

    pub fn font_size(&mut self, font_size: f32) -> &mut Self {
        self.metrics.font_size = font_size;
        self
    }

    pub fn line_height(&mut self, line_height: f32) -> &mut Self {
        self.metrics.line_height = line_height;
        self
    }

    pub fn text<S>(&mut self, text: S) -> &mut Self
    where
        String: From<S>,
    {
        self.text = String::from(text);
        self
    }
}

impl TextBuilder {
    pub fn draw<T: DisplayDriver>(&mut self, _driver: &mut T) {
        let _font_buffer = Buffer::new(&mut self.font_system, self.metrics);
    }
}
