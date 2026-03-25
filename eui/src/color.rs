/// RGBA color with f32 components in [0, 1].
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Default for Color {
    fn default() -> Self {
        Self { r: 1.0, g: 1.0, b: 1.0, a: 1.0 }
    }
}

impl Color {
    pub const fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    pub const WHITE: Self = Self { r: 1.0, g: 1.0, b: 1.0, a: 1.0 };
    pub const BLACK: Self = Self { r: 0.0, g: 0.0, b: 0.0, a: 1.0 };
    pub const TRANSPARENT: Self = Self { r: 0.0, g: 0.0, b: 0.0, a: 0.0 };
}

pub fn rgba(r: f32, g: f32, b: f32, a: f32) -> Color {
    Color { r, g, b, a }
}

pub fn rgb(r: f32, g: f32, b: f32) -> Color {
    Color { r, g, b, a: 1.0 }
}

pub fn mix(lhs: Color, rhs: Color, t: f32) -> Color {
    let t = t.clamp(0.0, 1.0);
    Color {
        r: lhs.r + (rhs.r - lhs.r) * t,
        g: lhs.g + (rhs.g - lhs.g) * t,
        b: lhs.b + (rhs.b - lhs.b) * t,
        a: lhs.a + (rhs.a - lhs.a) * t,
    }
}

pub fn srgb_to_linear(value: f32) -> f32 {
    let value = value.clamp(0.0, 1.0);
    if value <= 0.04045 {
        value / 12.92
    } else {
        ((value + 0.055) / 1.055).powf(2.4)
    }
}

pub fn color_luminance(color: &Color) -> f32 {
    let r = srgb_to_linear(color.r);
    let g = srgb_to_linear(color.g);
    let b = srgb_to_linear(color.b);
    0.2126 * r + 0.7152 * g + 0.0722 * b
}

pub fn brighten_primary_for_dark_mode(primary: &Color) -> Color {
    let mut tuned = Color {
        r: primary.r.clamp(0.0, 1.0),
        g: primary.g.clamp(0.0, 1.0),
        b: primary.b.clamp(0.0, 1.0),
        a: primary.a.clamp(0.0, 1.0),
    };
    let luminance = color_luminance(&tuned);
    let target_luminance = 0.24;
    if luminance >= target_luminance {
        return tuned;
    }

    let denom = (1.0 - luminance).max(1e-6);
    let lift = ((target_luminance - luminance) / denom).clamp(0.0, 0.72);
    let white = rgba(1.0, 1.0, 1.0, tuned.a);
    tuned = mix(tuned, white, lift);
    tuned = mix(tuned, *primary, 0.14);
    tuned.r = tuned.r.clamp(0.0, 1.0);
    tuned.g = tuned.g.clamp(0.0, 1.0);
    tuned.b = tuned.b.clamp(0.0, 1.0);
    tuned.a = primary.a.clamp(0.0, 1.0);
    tuned
}

pub const K_ICON_VISUAL_SCALE: f32 = 0.86;
