use crate::core::draw_command::DrawCommand;
use crate::graphics::transforms::Transform3D;
use crate::runtime::contracts::WindowMetrics;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ClearState {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
    pub clear_color: bool,
}

impl Default for ClearState {
    fn default() -> Self {
        Self { r: 0.0, g: 0.0, b: 0.0, a: 1.0, clear_color: true }
    }
}

pub struct DrawDataView<'a> {
    pub commands: &'a [DrawCommand],
    pub text_arena: &'a [u8],
    pub transform_payloads: &'a [Transform3D],
}

pub trait RendererBackend {
    fn begin_frame(&mut self, metrics: &WindowMetrics, clear_state: &ClearState);
    fn render(&mut self, draw_data: &DrawDataView, metrics: &WindowMetrics);
    fn end_frame(&mut self);
    fn release_resources(&mut self);
}
