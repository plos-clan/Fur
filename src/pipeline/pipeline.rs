use alloc::boxed::Box;
use alloc::vec::Vec;
use nalgebra::{Matrix4, Vector2, Vector4};

pub type Matrix4f = Matrix4<f32>;
pub type Vector4f = Vector4<f32>;
pub type Vector2f = Vector2<f32>;
pub type Vector2i = Vector2<i32>;

pub trait Vertex {
    fn get_vertex_pos(&self) -> Vector4f;
    fn set_vertex_pos(&mut self, value: &Vector4f);
}

pub trait Color {
    fn get_size(&self) -> usize;
    fn has_alpha(&self) -> bool;
}

pub struct Fragment<C: Color> {
    pub position: Vector2i,
    pub color: C,
}

pub struct Viewport {
    x: u32,
    y: u32,
    width: usize,
    height: usize,
    near: f32,
    far: f32,
}

pub trait Texture<C : Color> {
    fn get_color(self, u: f32, v: f32) -> C;
}

pub trait VertexPass<V : Vertex> {
    fn transform(self, vertex: &V) -> V;
}

pub trait FragmentPass<V : Vertex, C : Color> {
    fn transform(self, vertex: &V) -> C;
}

impl Clone for Viewport {
    fn clone(&self) -> Self {
        Self {
            x: self.x, y: self.y, width: self.width, height: self.height, near: self.near, far: self.far
        }
    }
}

pub trait MatrixProperty {
    fn get_matrix(self) -> Matrix4f;
}

impl MatrixProperty for Viewport {
    fn get_matrix(self) -> Matrix4f {
        let half_width = self.width as f32 / 2.0;
        let half_height = self.height as f32 / 2.0;
        let half_near = self.near / 2.0;
        let half_far = self.far / 2.0;

        Matrix4f::new(
            half_width, 0.0, 0.0, self.x as f32 + half_width,
            0.0, half_height, 0.0, self.y as f32 + half_height,
            0.0, 0.0, half_far - half_near, half_far + half_near,
            0.0, 0.0, 0.0, 1.0
        )
    }
}

impl Viewport {
    pub fn new(x: u32, y: u32, width: usize, height: usize, near: f32, far: f32) -> Self {
        Self {
            x, y, width, height, near, far
        }
    }
}
