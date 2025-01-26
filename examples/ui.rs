use std::sync::Arc;

use fur::pipeline::drawing::{TriangleDrawCommand, PrimitiveTriangle, DrawCommand};
use fur::{
    color::Color,
    display::{Display, DisplayDriver},
};
use minifb::{Key, Window, WindowOptions};
use spin::RwLock;
use fur::pipeline::default::{DefaultColorImpl, DefaultVertexImpl, DirectFragmentPass, DirectVertexPass, Pipeline};
use fur::pipeline::pipeline::{Matrix4f, Vector4f, Vertex, VertexPass, Viewport};

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

struct DrawBuffer {
    buffer: Vec<u32>,
}

impl DrawBuffer {
    pub fn new() -> Self {
        Self {
            buffer: vec![0; WIDTH * HEIGHT],
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

    fn write(&mut self, x: usize, y: usize, width: usize, height: usize, pixels: &[Color]) {
        for dx in 0..width {
            for dy in 0..height {
                let t_x = x + dx;
                let t_y = y + dy;
                self.buffer[t_y * WIDTH + t_x] = pixels[dy * width + dx].as_0rgb_u32();
            }
        }
    }

    fn write_data(&mut self, x: usize, y: usize, width: usize, height: usize, pixels: &[u32]) {
        for dx in 0..width {
            for dy in 0..height {
                let t_x = x + dx;
                let t_y = y + dy;
                self.buffer[t_y * WIDTH + t_x] = pixels[dy * width + dx];
            }
        }
    }

    fn write_at(&mut self, x: usize, y: usize, color: u32) {
        todo!()
    }
}

fn main() {
    let buffer = Arc::new(RwLock::new(DrawBuffer::new()));

    let pipeline = Pipeline::new(
        // Viewport::new(0, 0, WIDTH, HEIGHT, 0.0, 0.0),
        // DirectVertexPass::new(Matrix4f::identity()),
        Viewport::new(0, 0, WIDTH, HEIGHT, 100.0, 3000.0),
        DirectVertexPass::new(Matrix4f::new_orthographic(0.0, WIDTH as f32, 0.0, HEIGHT as f32, 100.0, 3000.0)),
        DirectFragmentPass::new(DefaultColorImpl::new(255, 255, 255, 255))
    );
    let draw_cmd = TriangleDrawCommand::new(pipeline, vec![PrimitiveTriangle::new([
        DefaultVertexImpl::new(Vector4f::new(WIDTH as f32 / 2.0, 10.0, 0.0, 1.0), DefaultColorImpl::new(255, 0, 0, 255)),
        DefaultVertexImpl::new(Vector4f::new(WIDTH as f32 - 10.0, HEIGHT as f32 - 10.0, 0.0, 1.0), DefaultColorImpl::new(0, 0, 255, 255)),
        DefaultVertexImpl::new(Vector4f::new(10.0, HEIGHT as f32 - 10.0, 0.0, 1.0), DefaultColorImpl::new(0, 255, 0, 255)),
    ])]);

    let (regional_buffer, region, size) = draw_cmd.execute();
    // let res = pipeline.vertex_pass.transform(
    //     &DefaultVertexImpl::new(Vector4f::new(10.0, 10.0, 0.0, 1.0), DefaultColorImpl::new(255, 0, 0, 255)));
    // println!("{:?}", res.position());
    println!("{:?}, {}", region, size);

    let mut display = Display::new(buffer.clone(), WIDTH, HEIGHT);

    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    window.set_target_fps(60);

    let pixels = (0..100)
        .map(|_| Color::new_argb(0xaa, 0xff, 0x00, 0x00))
        .collect::<Vec<_>>();

    let square_layer = display.create_layer(10, 10, 10, 10);
    display
        .layer_mut(&square_layer)
        .unwrap()
        .write(0, 0, 10, 10, &pixels);
    //display.flush();

    let pixels = (0..WIDTH * HEIGHT)
        .map(|_| Color::new_argb(0, 0, 0, 0))
        .collect::<Vec<_>>();

    let background_layer = display.create_layer(WIDTH, HEIGHT, 0, 0);
    display
        .layer_mut(&background_layer)
        .unwrap()
        .write(0, 0, WIDTH, HEIGHT, &pixels);
    display.flush_all();

    display.put_upper_than(&square_layer, &background_layer);
    display.flush_area((0, 30), (0, 30));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&buffer.read().buffer, WIDTH, HEIGHT)
            .unwrap();

        buffer.write().write_data(
            region.x as usize, region.y as usize,
            (region.z - region.x) as usize, (region.w - region.y) as usize,
            &regional_buffer.buffer)
    }
}
