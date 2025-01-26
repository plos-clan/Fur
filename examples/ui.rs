use std::sync::Arc;

use fur::{
    color::Color,
    display::{Display, DisplayDriver},
};
use minifb::{Key, Window, WindowOptions};
use spin::RwLock;

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

struct DrawBuffer {
    buffer: [u32;WIDTH * HEIGHT],
}

impl DrawBuffer {
    pub fn new() -> Self {
        Self {
            buffer: [0; WIDTH * HEIGHT],
        }
    }
}

impl DisplayDriver for DrawBuffer {
    fn read(&self, x: usize, y: usize, width: usize, height: usize, pixels: &mut [Color]) {
        for dx in 0..width {
            for dy in 0..height {
                let t_x = x + dx;
                let t_y = y + dy;
                pixels[dy * width + dx] = Color::from_0rgb_u32(self.buffer[t_y * WIDTH + t_x]);
            }
        }
    }

    fn write(&mut self, x: usize, y: usize, width: usize, height: usize, color: &Color) {
        for dx in 0..width {
            for dy in 0..height {
                let t_x = x + dx;
                let t_y = y + dy;
                self.buffer[t_y * WIDTH + t_x] = color.as_0rgb_u32();
            }
        }
    }

    fn size(&self) -> (usize, usize) {
        (WIDTH, HEIGHT)
    }
}

fn main() {
    let buffer = Arc::new(RwLock::new(DrawBuffer::new()));

    let mut display = Display::new(buffer.clone());

    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    window.set_target_fps(60);

    let window_layer = display.create_layer(100, 50, 10, 10);
    let window_layer_mut = display.layer_mut(&window_layer).unwrap();
    fur::window::WindowBuilder::new(100, 50).draw(window_layer_mut);

    let color = Color::new_argb(0, 0x00, 0x00, 0xff);

    let background_layer = display.create_layer(WIDTH, HEIGHT, 0, 0);
    display
        .layer_mut(&background_layer)
        .unwrap()
        .write(0, 0, WIDTH, HEIGHT, &color);

    display.flush_all();

    display.put_upper_than(&window_layer, &background_layer);
    display.flush_area((0, 110), (0, 60));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window
            .update_with_buffer(&buffer.read().buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}
