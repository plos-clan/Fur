use crate::pipeline::pipeline::{Color, FragmentPass, Matrix4f, Vector4f, Vertex, VertexPass, Viewport};
use core::ops::Mul;

pub struct Pipeline {
    pub viewport: Viewport,
    pub vertex_pass: DirectVertexPass,
    pub fragment_pass: DirectFragmentPass,
}

impl Pipeline {
    pub const fn new(
        viewport: Viewport,
        vertex_pass: DirectVertexPass,
        fragment_pass: DirectFragmentPass,
    ) -> Self {
        Self {
            viewport, vertex_pass, fragment_pass
        }
    }
}

pub struct DefaultVertexImpl {
    position: Vector4f,
    color: DefaultColorImpl
}

impl Vertex for DefaultVertexImpl {
    fn get_vertex_pos(&self) -> Vector4f {
        self.position
    }

    fn set_vertex_pos(&mut self, value: &Vector4f) {
        self.position = *value
    }
}

impl DefaultVertexImpl {
    pub const fn new(position: Vector4f, color: DefaultColorImpl) -> Self {
        Self { position, color }
    }
}

pub struct DefaultColorImpl {
    rgba: u32
}

impl Color for DefaultColorImpl {
    fn get_size(&self) -> usize {
        32
    }

    fn has_alpha(&self) -> bool {
        true
    }
}

impl DefaultColorImpl {
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { rgba: (r as u32) << 24 | (g as u32) << 16 | (b as u32) << 8 | a as u32 }
    }

    pub const fn red(&self) -> u8 {
        (self.rgba >> 24) as u8
    }

    pub const fn green(&self) -> u8 {
        (self.rgba >> 16 & 0b11111111) as u8
    }

    pub const fn blue(&self) -> u8 {
        (self.rgba >> 8 & 0b11111111) as u8
    }

    pub const fn alpha(&self) -> u8 {
        (self.rgba & 0b11111111) as u8
    }
}

pub struct DirectVertexPass {
    translation: Matrix4f
}

impl Clone for DirectVertexPass {
    fn clone(&self) -> Self {
        Self {
            translation: self.translation
        }
    }
}

impl VertexPass<DefaultVertexImpl> for DirectVertexPass {
    fn transform(self, vertex: &DefaultVertexImpl) -> DefaultVertexImpl {
        DefaultVertexImpl::new(
            self.translation.mul(vertex.position),
            DefaultColorImpl::new(vertex.color.red(), vertex.color.green(), vertex.color.blue(), vertex.color.alpha())
        )
    }
}

impl DirectVertexPass {
    pub const fn new(translation: Matrix4f) -> Self {
        DirectVertexPass { translation }
    }
}

pub struct DirectFragmentPass {
    color: DefaultColorImpl
}

impl FragmentPass<DefaultVertexImpl, DefaultColorImpl> for DirectFragmentPass {
    fn transform(self, vertex: &DefaultVertexImpl) -> DefaultColorImpl {
        DefaultColorImpl::new(vertex.color.red(), vertex.color.green(), vertex.color.blue(), vertex.color.alpha())
    }
}