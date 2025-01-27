use crate::pipeline::pipeline::{Color, FragmentPass, Matrix4f, PipelineState, Vector4f, Vertex, VertexPass, Viewport};
use core::ops::Mul;

pub struct Pipeline {
    pub viewport: Viewport,
    pub pipeline_state: PipelineState,
    pub vertex_pass: DirectVertexPass,
    pub fragment_pass: DirectFragmentPass,
}

impl Pipeline {
    pub const fn new(
        viewport: Viewport,
        pipeline_state: PipelineState,
        vertex_pass: DirectVertexPass,
        fragment_pass: DirectFragmentPass,
    ) -> Self {
        Self {
            viewport, pipeline_state, vertex_pass, fragment_pass
        }
    }

    pub fn set_culling(&mut self, value: bool) {
        self.pipeline_state.culling = value;
    }
}

pub struct DefaultVertexImpl {
    position: Vector4f,
    pub(crate) color: DefaultColorImpl
}

impl Clone for DefaultVertexImpl {
    fn clone(&self) -> Self {
        Self {
            position: self.position, color: DefaultColorImpl::raw(self.color.rgba)
        }
    }
}

impl Vertex for DefaultVertexImpl {
    fn position(&self) -> Vector4f {
        self.position
    }

    fn set_position(&mut self, value: &Vector4f) {
        self.position = *value
    }
}

impl DefaultVertexImpl {
    pub const fn new(position: Vector4f, color: DefaultColorImpl) -> Self {
        Self { position, color }
    }
}

pub struct DefaultColorImpl {
    pub(crate) rgba: u32
}

impl Clone for DefaultColorImpl {
    fn clone(&self) -> Self {
        Self::raw(self.rgba)
    }
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
    pub const RED: Self = Self::new(255, 0, 0, 255);
    pub const GREEN: Self = Self::new(0, 255, 0, 255);
    pub const BLUE: Self = Self::new(0, 0, 255, 255);

    pub const fn raw(rgba: u32) -> Self {
        Self { rgba }
    }

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

    pub const fn get_abgr(&self) -> u32 {
        (((self.alpha() as u32) << 24) | ((self.blue() as u32) << 16) | ((self.green() as u32) << 8) | self.red() as u32)
    }

    pub const fn get_argb(&self) -> u32 {
        (((self.alpha() as u32) << 24) | ((self.red() as u32) << 16) | ((self.green() as u32) << 8) | self.blue() as u32)
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
    
    pub fn translation(&mut self, new: Matrix4f) {
        self.translation = new;
    }
}

pub struct DirectFragmentPass {
    color: DefaultColorImpl
}

impl Clone for DirectFragmentPass {
    fn clone(&self) -> Self {
        Self {
            color: self.color.clone()
        }
    }
}

impl FragmentPass<DefaultVertexImpl, DefaultColorImpl> for DirectFragmentPass {
    fn transform(self, vertex: &DefaultVertexImpl) -> DefaultColorImpl {
        DefaultColorImpl::new(vertex.color.red(), vertex.color.green(), vertex.color.blue(), vertex.color.alpha())
    }
}

impl DirectFragmentPass {
    pub const fn new(color: DefaultColorImpl) -> Self {
        Self { color }
    }
}