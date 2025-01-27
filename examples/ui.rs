#![feature(slice_as_array)]

use std::cmp::min;
use std::f32::consts::PI;
use std::ops::Mul;
use std::sync::Arc;

use fur::pipeline::drawing::{TriangleDrawCommand, PrimitiveTriangle, DrawCommand};
use fur::{
    color::Color,
    display::{Display, DisplayDriver},
};
use minifb::{Key, Window, WindowOptions};
use nalgebra::{Point3, Vector3};
use spin::RwLock;
use fur::pipeline::default::{DefaultColorImpl, DefaultVertexImpl, DirectFragmentPass, DirectVertexPass, Pipeline};
use fur::pipeline::pipeline::{Matrix4f, MatrixProperty, PipelineState, Vector4f, Vertex, VertexPass, Viewport};

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

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
        for dx in 0..min(WIDTH - x, width) {
            for dy in 0..min(HEIGHT - y, height) {
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

    // let res = pipeline.vertex_pass.transform(
    //     &DefaultVertexImpl::new(Vector4f::new(300.0, 180.0, 5.0, 1.0), DefaultColorImpl::new(255, 0, 0, 255)));
    // println!("{:?}", res.position());

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

    let start_time = std::time::Instant::now();
    let mut last_frame_time = start_time.elapsed().as_millis();

    let view = Matrix4f::look_at_rh(
        &Point3::new(3.0, 3.0, 2.0),
        &Point3::new(0.0, 0.0, 0.0),
        &Vector3::new(1.0, 0.0, 0.0)
    );
    let projection = Matrix4f::new_perspective(WIDTH as f32 / HEIGHT as f32, 45.0 * (PI / 180.0), 0.1, 100.0);

    let vertices = [
        Vector4f::new(1.0, 0.0, 0.0, 1.0),
        Vector4f::new(0.0, 1.0, 0.0, 1.0),
        Vector4f::new(0.0, 0.0, 0.0, 1.0),

        Vector4f::new(1.0, 1.0, 0.0, 1.0),
        Vector4f::new(0.0, 1.0, 0.0, 1.0),
        Vector4f::new(1.0, 0.0, 0.0, 1.0),

        Vector4f::new(0.0, 0.0, 0.0, 1.0),
        Vector4f::new(0.0, 0.0, 1.0, 1.0),
        Vector4f::new(1.0, 0.0, 0.0, 1.0),

        Vector4f::new(1.0, 0.0, 0.0, 1.0),
        Vector4f::new(0.0, 0.0, 1.0, 1.0),
        Vector4f::new(1.0, 0.0, 1.0, 1.0),

        Vector4f::new(0.0, 0.0, 0.0, 1.0),
        Vector4f::new(0.0, 1.0, 0.0, 1.0),
        Vector4f::new(0.0, 0.0, 1.0, 1.0),

        Vector4f::new(0.0, 1.0, 0.0, 1.0),
        Vector4f::new(0.0, 0.0, 1.0, 1.0),
        Vector4f::new(0.0, 1.0, 1.0, 1.0),

        Vector4f::new(0.0, 0.0, 1.0, 1.0),
        Vector4f::new(1.0, 0.0, 1.0, 1.0),
        Vector4f::new(0.0, 1.0, 1.0, 1.0),

        Vector4f::new(0.0, 1.0, 1.0, 1.0),
        Vector4f::new(1.0, 0.0, 1.0, 1.0),
        Vector4f::new(1.0, 1.0, 1.0, 1.0),

        Vector4f::new(0.0, 1.0, 0.0, 1.0),
        Vector4f::new(0.0, 1.0, 1.0, 1.0),
        Vector4f::new(1.0, 1.0, 0.0, 1.0),

        Vector4f::new(1.0, 1.0, 0.0, 1.0),
        Vector4f::new(0.0, 1.0, 1.0, 1.0),
        Vector4f::new(1.0, 1.0, 1.0, 1.0),

        Vector4f::new(1.0, 0.0, 0.0, 1.0),
        Vector4f::new(1.0, 1.0, 0.0, 1.0),
        Vector4f::new(1.0, 0.0, 1.0, 1.0),

        Vector4f::new(1.0, 1.0, 0.0, 1.0),
        Vector4f::new(1.0, 0.0, 1.0, 1.0),
        Vector4f::new(1.0, 1.0, 1.0, 1.0),
    ];
    let chunks = vertices.chunks(3);
    let primitives = chunks.map(|chunk| {
        let vertices = chunk.iter().map(|pos| {
            let red = if pos.x == 1.0 { 255 } else { 0 };
            let green = if pos.y == 1.0 { 255 } else { 0 };
            let blue = if pos.z == 1.0 { 255 } else { 0 };

            DefaultVertexImpl::new(*pos, DefaultColorImpl::new(red, green, blue, 255))
        }).collect::<Vec<_>>();
        PrimitiveTriangle::new(vertices.as_array().unwrap().clone())
    }).collect::<Vec<_>>();
    
    while window.is_open() && !window.is_key_down(Key::Escape) {
        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&buffer.read().buffer, WIDTH, HEIGHT)
            .unwrap();

        display.write(0, 0, WIDTH, HEIGHT, &pixels);

        let rotate_speed_degree = 30.0;
        let time_elapsed = start_time.elapsed().as_millis();
        let frame_time = time_elapsed - last_frame_time;
        last_frame_time = time_elapsed;
        let x_delta = rotate_speed_degree * (time_elapsed as f32 / 1000.0) * (PI / 180.0);
        let model = Matrix4f::new_rotation(Vector3::new(x_delta, 0.0, 0.0));
        let pipeline = Pipeline::new(
            Viewport::new(0, 0, WIDTH, HEIGHT, 0.1, 100.0),
            PipelineState::new(true, true),
            DirectVertexPass::new(projection.mul(view).mul(model)),
            DirectFragmentPass::new(DefaultColorImpl::new(255, 255, 255, 255))
        );
        
        let _primitives = primitives.clone();
        println!("Primitives count: {}", _primitives.len());
        let draw_cmd = TriangleDrawCommand::new(pipeline, _primitives);
        let (regional_buffer, region, size) = draw_cmd.execute();
        println!("{:?}, {}, frame time: {}", region, size, frame_time);

        buffer.write().write_data(
            region.x as usize, region.y as usize,
            (region.z - region.x) as usize, (region.w - region.y) as usize,
            &regional_buffer.buffer)
    }
}
