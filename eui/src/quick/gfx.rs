use crate::color::Color;
use crate::graphics::effects::*;
use crate::graphics::primitives::{ClipMode, ClipRect, CornerRadius};
use crate::rect::Rect;

pub fn color_from_hex(hex: u32, alpha: f32) -> GfxColor {
    let inv = 1.0 / 255.0;
    GfxColor {
        r: ((hex >> 16) & 0xff) as f32 * inv,
        g: ((hex >> 8) & 0xff) as f32 * inv,
        b: (hex & 0xff) as f32 * inv,
        a: alpha.clamp(0.0, 1.0),
    }
}

pub fn gfx_color(c: &Color, alpha: f32) -> GfxColor {
    GfxColor {
        r: c.r,
        g: c.g,
        b: c.b,
        a: (c.a * alpha).clamp(0.0, 1.0),
    }
}

pub fn solid(color: GfxColor) -> Brush {
    Brush { kind: BrushKind::Solid, solid: color, ..Default::default() }
}

pub fn solid_color(color: &Color, alpha: f32) -> Brush {
    solid(gfx_color(color, alpha))
}

pub fn solid_hex(hex: u32, alpha: f32) -> Brush {
    solid(color_from_hex(hex, alpha))
}

pub fn vertical_gradient(top: GfxColor, bottom: GfxColor) -> Brush {
    let mut brush = Brush { kind: BrushKind::LinearGradient, ..Default::default() };
    brush.linear.start = Point { x: 0.0, y: 0.0 };
    brush.linear.end = Point { x: 0.0, y: 1.0 };
    brush.linear.stops[0] = ColorStop { position: 0.0, color: top };
    brush.linear.stops[1] = ColorStop { position: 1.0, color: bottom };
    brush.linear.stop_count = 2;
    brush
}

pub fn radial_gradient_brush(inner: GfxColor, outer: GfxColor, radius: f32) -> Brush {
    let mut brush = Brush { kind: BrushKind::RadialGradient, ..Default::default() };
    brush.radial.center = Point { x: 0.5, y: 0.5 };
    brush.radial.radius = radius.max(0.0);
    brush.radial.stops[0] = ColorStop { position: 0.0, color: inner };
    brush.radial.stops[1] = ColorStop { position: 1.0, color: outer };
    brush.radial.stop_count = 2;
    brush
}

pub fn stroke(brush: Brush, width: f32) -> Stroke {
    Stroke { width: width.max(0.0), brush }
}

pub fn stroke_color(color: &Color, width: f32, alpha: f32) -> Stroke {
    stroke(solid_color(color, alpha), width)
}

pub fn stroke_hex(hex: u32, width: f32, alpha: f32) -> Stroke {
    stroke(solid_hex(hex, alpha), width)
}

pub fn radius(uniform: f32) -> CornerRadius {
    CornerRadius::uniform(uniform.max(0.0))
}

pub fn radius_corners(tl: f32, tr: f32, br: f32, bl: f32) -> CornerRadius {
    CornerRadius {
        top_left: tl.max(0.0),
        top_right: tr.max(0.0),
        bottom_right: br.max(0.0),
        bottom_left: bl.max(0.0),
    }
}

pub fn clip(rect: &Rect) -> ClipRect {
    ClipRect {
        rect: *rect,
        mode: ClipMode::Bounds,
    }
}
