use alloc::vec::Vec;
use core::ops::Mul;
use crate::pipeline::default::{DefaultColorImpl, DefaultVertexImpl, Pipeline};
use crate::pipeline::pipeline::{Fragment, MatrixProperty, Vertex, VertexPass, Viewport};

pub struct DrawCall {
    pipeline: Pipeline,
    primitives: Vec<Primitive>
}

/// A triangle
pub struct Primitive {
    vertices: [DefaultVertexImpl; 3]
}

impl Primitive {
    pub fn rasterize(self, pipeline: &Pipeline) -> Vec<Fragment<DefaultColorImpl>> {
        let vertex_pass = &pipeline.vertex_pass;
        let viewport_transform = &pipeline.viewport;
        let mut transformed_vertices =
            self.vertices.map(|vertex| { vertex_pass.clone().transform(&vertex) });
        transformed_vertices.iter_mut().for_each(|mut vertex| {
            vertex.set_vertex_pos(&viewport_transform.clone().get_matrix().mul(vertex.get_vertex_pos()))
        });

        todo!()
    }
}