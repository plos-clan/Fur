use spin::RwLock;

use crate::display::DisplayDriver;

pub struct Window {
    x: usize,
    y: usize,
    width: usize,
    height: usize,
}

impl Window {
    pub fn new(x: usize, y: usize, width: usize, height: usize) -> Self {
        Self { x, y, width, height }
    }
}

impl Window {
    pub fn draw<T: DisplayDriver>(&self, driver: T) {
        
    }
}
