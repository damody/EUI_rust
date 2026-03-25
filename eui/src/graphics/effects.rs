#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct GfxColor {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl GfxColor {
    pub const fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }
}

impl From<crate::color::Color> for GfxColor {
    fn from(c: crate::color::Color) -> Self {
        Self { r: c.r, g: c.g, b: c.b, a: c.a }
    }
}

impl From<GfxColor> for crate::color::Color {
    fn from(c: GfxColor) -> Self {
        Self { r: c.r, g: c.g, b: c.b, a: c.a }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct ColorStop {
    pub position: f32,
    pub color: GfxColor,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u8)]
pub enum BrushKind {
    #[default]
    None,
    Solid,
    LinearGradient,
    RadialGradient,
}

pub const MAX_GRADIENT_STOPS: usize = 4;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct LinearGradient {
    pub start: Point,
    pub end: Point,
    pub stops: [ColorStop; MAX_GRADIENT_STOPS],
    pub stop_count: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct RadialGradient {
    pub center: Point,
    pub radius: f32,
    pub stops: [ColorStop; MAX_GRADIENT_STOPS],
    pub stop_count: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Brush {
    pub kind: BrushKind,
    pub solid: GfxColor,
    pub linear: LinearGradient,
    pub radial: RadialGradient,
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Stroke {
    pub width: f32,
    pub brush: Brush,
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Shadow {
    pub offset_x: f32,
    pub offset_y: f32,
    pub blur_radius: f32,
    pub spread: f32,
    pub color: GfxColor,
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Blur {
    pub radius: f32,
    pub backdrop_radius: f32,
}
