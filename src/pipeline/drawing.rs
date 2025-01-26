use crate::pipeline::default::{DefaultColorImpl, DefaultVertexImpl, Pipeline};
use crate::pipeline::pipeline::{FragmentPass, MatrixProperty, Vector4f, Vector4i, Vertex, VertexPass};
use alloc::vec;
use alloc::vec::Vec;
use core::ops::Mul;
use nalgebra::{max, min};
use crate::display::{DisplayDriver, DrawBuffer};
use crate::pixel::PixelFormat;

pub trait DrawCommand {
    fn execute(&self) -> (DrawBuffer, Vector4i, usize);
}

pub struct TriangleDrawCommand {
    pipeline: Pipeline,
    pub primitives: Vec<PrimitiveTriangle>
}

impl DrawCommand for TriangleDrawCommand {
    fn execute(&self) -> (DrawBuffer, Vector4i, usize) {
        let mut max_x: i32 = i32::MIN;
        let mut max_y: i32 = i32::MIN;
        let mut min_x: i32 = i32::MAX;
        let mut min_y: i32 = i32::MAX;

        let mut fragments: Vec<DefaultVertexImpl> = vec!();
        let mut fragments_count = 0;
        self.primitives.iter().for_each(|primitive| {
            let (fragments0, update_range) = primitive.rasterize(&self.pipeline);
            max_x = max(max_x, update_range[2]);
            max_y = max(max_y, update_range[3]);
            min_x = min(min_x, update_range[0]);
            min_y = min(min_y, update_range[1]);
            fragments0.into_iter().for_each(|fragment| {
                fragments.push(fragment);
                fragments_count += 1;
            })
        });

        let mut buffer = DrawBuffer::new((max_x - min_x) as usize, (max_y - min_y) as usize, PixelFormat::Argb);
        fragments.iter().for_each(|fragment| {
            let x = fragment.position().x as i32;
            let y = fragment.position().y as i32;
            let color = self.pipeline.fragment_pass.clone().transform(fragment);
            buffer.write_at((x - min_x) as usize, (y - min_y) as usize, color.get_argb());
        });
        (buffer, Vector4i::new(min_x, min_y, max_x, max_y), fragments_count)
    }
}

impl TriangleDrawCommand {
    pub fn new(pipeline: Pipeline, primitives: Vec<PrimitiveTriangle>) -> Self {
        Self { pipeline, primitives }
    }
}

/// A triangle
pub struct PrimitiveTriangle {
    pub vertices: [DefaultVertexImpl; 3]
}

impl PrimitiveTriangle {
    pub fn new(vertices: [DefaultVertexImpl; 3]) -> Self {
        Self { vertices }
    }

    fn barycentric_coords(p: (f32, f32), v0: Vector4f, v1: Vector4f, v2: Vector4f) -> (f32, f32, f32) {
        let area = (v1.x - v0.x) * (v2.y - v0.y) - (v1.y - v0.y) * (v2.x - v0.x);
        let u = ((v1.y - v2.y) * (p.0 - v2.x) + (v2.x - v1.x) * (p.1 - v2.y)) / area;
        let v = ((v2.y - v0.y) * (p.0 - v2.x) + (v0.x - v2.x) * (p.1 - v2.y)) / area;
        let w = 1.0 - u - v;
        (u, v, w)
    }

    pub fn rasterize(&self, pipeline: &Pipeline) -> (Vec<DefaultVertexImpl>, Vector4i) {
        let vertex_pass = &pipeline.vertex_pass;
        let viewport_transform = &pipeline.viewport;
        let mut transformed_vertices =
            self.vertices.clone().map(|vertex| { vertex_pass.clone().transform(&vertex) });
        transformed_vertices.iter_mut().for_each(|mut vertex| {
            vertex.set_position(&viewport_transform.clone().get_matrix().mul(vertex.position()))
        });

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

        let mut fragments: Vec<DefaultVertexImpl> = vec!();
        for y in min_y..max_y {
            for x in min_x..max_x {
                let p = (x as f32, y as f32);
                let (u, v, w) = PrimitiveTriangle::barycentric_coords(p, v1, v2, v3);
                // 判断点是否在三角形内
                if u >= 0.0 && v >= 0.0 && w >= 0.0 {
                    // 插值计算颜色
                    let red = (u * a.color.red() as f32 + v * b.color.red() as f32 + w * c.color.red() as f32) as u8;
                    let green = (u * a.color.green() as f32 + v * b.color.green() as f32 + w * c.color.green() as f32) as u8;
                    let blue = (u * a.color.blue() as f32 + v * b.color.blue() as f32 + w * c.color.blue() as f32) as u8;
                    let alpha = (u * a.color.alpha() as f32 + v * b.color.alpha() as f32 + w * c.color.alpha() as f32) as u8;
                    fragments.push(DefaultVertexImpl::new(Vector4f::new(p.0, p.1, 0.0, 1.0), DefaultColorImpl::new(
                        red, green, blue, alpha
                    )))
                }
            }
        }
        // let i1 = v1.y - v2.y;
        // let i2 = v2.y - v3.y;
        // let i3 = v3.y - v1.y;
        // let j1 = v2.x - v1.x;
        // let j2 = v3.x - v2.x;
        // let j3 = v1.x - v3.x;
        // let f1 = v1.x * v2.y - v1.y * v2.x;
        // let f2 = v2.x * v3.y - v2.y * v3.x;
        // let f3 = v3.x * v1.y - v3.y * v1.x;
        //
        // let delta = f1 + f2 + f3;
        // if delta <= 0.0 { panic!() }
        //
        // let r_delta = 1.0 / delta;
        //
        // v2 = (v2 - v1) * r_delta;
        // v3 = (v3 - v1) * r_delta;
        // let vx = i3 * v2 + i1 * v3;
        // let color_x = DefaultColorImpl::new(
        //     ((b.color.red() as f32 * i3 + c.color.red() as f32 * i1) as u32) as u8,
        //     ((b.color.green() as f32 * i3 + c.color.green() as f32 * i1) as u32) as u8,
        //     ((b.color.blue() as f32 * i3 + c.color.blue() as f32 * i1) as u32) as u8,
        //     ((b.color.alpha() as f32 * i3 + c.color.alpha() as f32 * i1) as u32) as u8,
        // );
        // let vy = j3 * v2 + j1 * v3;
        // let color_y = DefaultColorImpl::new(
        //     ((b.color.red() as f32 * j3 + c.color.red() as f32 * j1) as u32) as u8,
        //     ((b.color.green() as f32 * j3 + c.color.green() as f32 * j1) as u32) as u8,
        //     ((b.color.blue() as f32 * j3 + c.color.blue() as f32 * j1) as u32) as u8,
        //     ((b.color.alpha() as f32 * j3 + c.color.alpha() as f32 * j1) as u32) as u8,
        // );
        //
        // let mut cy1 = f1;
        // let mut cy2 = f2;
        // let mut cy3 = f3;
        // let mut v0 = v1 + v2 * cy3 + v3 * cy1;
        // let mut color = DefaultColorImpl::new(
        //     (a.color.red() as u32 + (b.color.red() as f32 * cy3 + c.color.red() as f32 * cy1) as u32) as u8,
        //     (a.color.green() as u32 + (b.color.green() as f32 * cy3 + c.color.green() as f32 * cy1) as u32) as u8,
        //     (a.color.blue() as u32 + (b.color.blue() as f32 * cy3 + c.color.blue() as f32 * cy1) as u32) as u8,
        //     (a.color.alpha() as u32 + (b.color.alpha() as f32 * cy3 + c.color.alpha() as f32 * cy1) as u32) as u8,
        // );
        //
        // for y in min_y..=max_y {
        //     let mut cx1 = cy1;
        //     let mut cx2 = cy2;
        //     let mut cx3 = cy3;
        //     for x in min_x..=max_x {
        //         if cx1 >= 0.0 && cx2 >= 0.0 && cx3 >= 0.0 {
        //             fragments.push(DefaultVertexImpl::new(v0, color.clone()))
        //         }
        //         cx1 += i1;
        //         cx2 += i2;
        //         cx3 += i3;
        //         v0 += vx;
        //         color = DefaultColorImpl::new(
        //             (color.red() as u32 + color_x.red() as u32) as u8,
        //             (color.green() as u32 + color_x.green() as u32) as u8,
        //             (color.blue() as u32 + color_x.blue() as u32) as u8,
        //             (color.alpha() as u32 + color_x.alpha() as u32) as u8
        //         )
        //     }
        //     cy1 += j1;
        //     cy2 += j2;
        //     cy3 += j3;
        //     v0 += vy;
        //     color = DefaultColorImpl::new(
        //         (color.red() as u32 + color_y.red() as u32) as u8,
        //         (color.green() as u32 + color_y.green() as u32) as u8,
        //         (color.blue() as u32 + color_y.blue() as u32) as u8,
        //         (color.alpha() as u32 + color_y.alpha() as u32) as u8
        //     )
        // }

        (fragments, Vector4i::new(min_x, min_y, max_x, max_y))
    }
}