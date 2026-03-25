use crate::graphics::effects::{Blur, Brush, Shadow, Stroke};
use crate::graphics::transforms::{Transform2D, Transform3D};
use crate::rect::Rect;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Size {
    pub w: f32,
    pub h: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct CornerRadius {
    pub top_left: f32,
    pub top_right: f32,
    pub bottom_right: f32,
    pub bottom_left: f32,
}

impl CornerRadius {
    pub fn uniform(r: f32) -> Self {
        Self { top_left: r, top_right: r, bottom_right: r, bottom_left: r }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u8)]
pub enum ClipMode {
    #[default]
    None,
    Bounds,
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct ClipRect {
    pub rect: Rect,
    pub mode: ClipMode,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u8)]
pub enum ImageFit {
    Fill,
    Contain,
    #[default]
    Cover,
    Stretch,
    Center,
}

#[derive(Debug, Clone, Default)]
pub struct RectanglePrimitive {
    pub rect: Rect,
    pub radius: CornerRadius,
    pub fill: Brush,
    pub image_source: String,
    pub image_fit: ImageFit,
    pub stroke: Stroke,
    pub shadow: Shadow,
    pub blur: Blur,
    pub opacity: f32,
    pub clip: ClipRect,
    pub transform_2d: Transform2D,
    pub transform_3d: Transform3D,
}

impl RectanglePrimitive {
    pub fn new() -> Self {
        Self { opacity: 1.0, ..Default::default() }
    }
}

#[derive(Debug, Clone, Default)]
pub struct ImagePrimitive {
    pub rect: Rect,
    pub source: String,
    pub radius: CornerRadius,
    pub opacity: f32,
    pub fit: ImageFit,
    pub clip: ClipRect,
    pub transform_2d: Transform2D,
    pub transform_3d: Transform3D,
}

impl ImagePrimitive {
    pub fn new() -> Self {
        Self { opacity: 1.0, ..Default::default() }
    }
}

#[derive(Debug, Clone, Default)]
pub struct IconPrimitive {
    pub rect: Rect,
    pub glyph: u32,
    pub font_family: String,
    pub tint: Brush,
    pub opacity: f32,
    pub clip: ClipRect,
    pub transform_2d: Transform2D,
    pub transform_3d: Transform3D,
}

impl IconPrimitive {
    pub fn new() -> Self {
        Self { opacity: 1.0, ..Default::default() }
    }
}
