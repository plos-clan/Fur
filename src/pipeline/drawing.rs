use crate::pipeline::default::{DefaultColorImpl, DefaultVertexImpl, Pipeline};
use crate::pipeline::pipeline::{FragmentPass, MatrixProperty, Vector4i, Vertex, VertexPass};
use alloc::vec;
use alloc::vec::Vec;
use core::ops::Mul;
use nalgebra::{max, min};
use crate::display::{DisplayDriver, DrawBuffer};
use crate::pixel::PixelFormat;

pub struct TriangleDrawCall {
    pipeline: Pipeline,
    primitives: Vec<TrianglePrimitive>
}

impl TriangleDrawCall {
    pub fn new(pipeline: Pipeline, primitives: Vec<TrianglePrimitive>) -> Self {
        Self { pipeline, primitives }
    }

    pub fn draw(&self) -> (DrawBuffer, Vector4i) {
        let mut max_x = 0;
        let mut max_y = 0;
        let mut min_x = 0;
        let mut min_y = 0;

        let mut fragments: Vec<DefaultVertexImpl> = vec!();
        self.primitives.iter().for_each(|primitive| {
            let (fragments0, update_range) = primitive.rasterize(&self.pipeline);
            max_x = max(max_x, update_range[2]);
            max_y = max(max_y, update_range[3]);
            min_x = min(min_x, update_range[0]);
            min_y = min(min_y, update_range[1]);
            fragments0.into_iter().for_each(|fragment| {
                fragments.push(fragment);
            })
        });

        let mut buffer = DrawBuffer::new((max_x - min_x) as usize, (max_y - min_y) as usize, PixelFormat::Argb);
        fragments.iter().for_each(|fragment| {
            let x = fragment.position().x as i32;
            let y = fragment.position().y as i32;
            let color = self.pipeline.fragment_pass.clone().transform(fragment);
            buffer.write_at((x - min_x) as usize, (y - min_y) as usize, color.rgba);
        });
        (buffer, Vector4i::new(min_x, min_y, max_x, max_y))
    }
}

/// A triangle
pub struct TrianglePrimitive {
    vertices: [DefaultVertexImpl; 3]
}

impl TrianglePrimitive {
    pub fn new(vertices: [DefaultVertexImpl; 3]) -> Self {
        Self { vertices }
    }

    pub fn rasterize(&self, pipeline: &Pipeline) -> (Vec<DefaultVertexImpl>, Vector4i) {
        let vertex_pass = &pipeline.vertex_pass;
        let viewport_transform = &pipeline.viewport;
        let mut transformed_vertices =
            self.vertices.clone().map(|vertex| { vertex_pass.clone().transform(&vertex) });
        // transformed_vertices.iter_mut().for_each(|mut vertex| {
        //     vertex.set_position(&viewport_transform.clone().get_matrix().mul(vertex.position()))
        // });

        let a = transformed_vertices[0].clone();
        let b = transformed_vertices[1].clone();
        let c = transformed_vertices[2].clone();

        let mut v1 = a.position().clone();
        let mut v2 = b.position().clone();
        let mut v3 = c.position().clone();

        let max_x = [v1.x, v2.x, v3.x].iter().map(|x| { *x as i32 }).max().unwrap();
        let max_y = [v1.y, v2.y, v3.y].iter().map(|x| { *x as i32 }).max().unwrap();
        let min_x = [v1.x, v2.x, v3.x].iter().map(|x| { *x as i32 }).min().unwrap();
        let min_y = [v1.y, v2.y, v3.y].iter().map(|x| { *x as i32 }).min().unwrap();

        let i1 = v1.y - v2.y;
        let i2 = v2.y - v3.y;
        let i3 = v3.y - v1.y;
        let j1 = v2.x - v1.x;
        let j2 = v3.x - v2.x;
        let j3 = v1.x - v3.x;
        let f1 = v1.x * v2.y - v1.y * v2.x;
        let f2 = v2.x * v3.y - v2.y * v3.x;
        let f3 = v3.x * v1.y - v3.y * v1.x;
        
        let delta = f1 + f2 + f3;
        if delta <= 0.0 { return (vec!(), Vector4i::default()); }
        
        let r_delta = 1.0 / delta;
        
        v2 = (v2 - v1) * r_delta;
        v3 = (v3 - v1) * r_delta;
        let vx = i3 * v2 + i1 * v3;
        let color_x = DefaultColorImpl::new(
            ((b.color.red() as f32 * i3 + c.color.red() as f32 * i1) as u32) as u8,
            ((b.color.green() as f32 * i3 + c.color.green() as f32 * i1) as u32) as u8,
            ((b.color.blue() as f32 * i3 + c.color.blue() as f32 * i1) as u32) as u8,
            ((b.color.alpha() as f32 * i3 + c.color.alpha() as f32 * i1) as u32) as u8,
        );
        let vy = j3 * v2 + j1 * v3;
        let color_y = DefaultColorImpl::new(
            ((b.color.red() as f32 * j3 + c.color.red() as f32 * j1) as u32) as u8,
            ((b.color.green() as f32 * j3 + c.color.green() as f32 * j1) as u32) as u8,
            ((b.color.blue() as f32 * j3 + c.color.blue() as f32 * j1) as u32) as u8,
            ((b.color.alpha() as f32 * j3 + c.color.alpha() as f32 * j1) as u32) as u8,
        );

        let mut cy1 = f1;
        let mut cy2 = f2;
        let mut cy3 = f3;
        let mut v0 = v1 + v2 * cy3 + v3 * cy1;
        let mut color = DefaultColorImpl::new(
            (a.color.red() as u32 + (b.color.red() as f32 * cy1 + c.color.red() as f32 * cy1) as u32) as u8,
            (a.color.green() as u32 + (b.color.green() as f32 * cy1 + c.color.green() as f32 * cy1) as u32) as u8,
            (a.color.blue() as u32 + (b.color.blue() as f32 * cy1 + c.color.blue() as f32 * cy1) as u32) as u8,
            (a.color.alpha() as u32 + (b.color.alpha() as f32 * cy1 + c.color.alpha() as f32 * cy1) as u32) as u8,
        );

        let mut fragments: Vec<DefaultVertexImpl> = vec!();
        for y in min_y..=max_y {
            let mut cx1 = cy1;
            let mut cx2 = cy2;
            let mut cx3 = cy3;
            for x in min_x..=max_x {
                if cx1 >= 0.0 && cx2 >= 0.0 && cx3 >= 0.0 {
                    fragments.push(DefaultVertexImpl::new(v0, color.clone()))
                }
                cx1 += i1;
                cx2 += i2;
                cx3 += i3;
                v0 += vx;
                color = DefaultColorImpl::new(
                    color.red() + color_x.red(),
                    color.green() + color_x.green(),
                    color.blue() + color_x.blue(),
                    color.alpha() + color_x.alpha()
                )
            }
            cy1 += j1;
            cy2 += j2;
            cy3 += j3;
            v0 += vy;
            color = DefaultColorImpl::new(
                color.red() + color_y.red(),
                color.green() + color_y.green(),
                color.blue() + color_y.blue(),
                color.alpha() + color_y.alpha()
            )
        }

        (fragments, Vector4i::new(min_x, min_y, max_x, max_y))
    }
}