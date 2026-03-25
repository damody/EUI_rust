use crate::color::Color;
use crate::core::draw_command::DrawCommand;
use crate::graphics::effects::{Brush, BrushKind, GfxColor};
use crate::graphics::transforms::Transform3D;
use crate::rect::Rect;

const FNV_OFFSET: u64 = 1469598103934665603;
const FNV_PRIME: u64 = 1099511628211;

pub fn context_hash_sv(text: &str) -> u64 {
    let mut value = FNV_OFFSET;
    for &ch in text.as_bytes() {
        value ^= ch as u64;
        value = value.wrapping_mul(FNV_PRIME);
    }
    value
}

pub fn context_hash_mix(hash: u64, value: u64) -> u64 {
    (hash ^ value).wrapping_mul(FNV_PRIME)
}

pub fn context_float_bits(value: f32) -> u32 {
    value.to_bits()
}

pub fn context_hash_rect(rect: &Rect) -> u64 {
    let mut hash = FNV_OFFSET;
    hash = context_hash_mix(hash, context_float_bits(rect.x) as u64);
    hash = context_hash_mix(hash, context_float_bits(rect.y) as u64);
    hash = context_hash_mix(hash, context_float_bits(rect.w) as u64);
    hash = context_hash_mix(hash, context_float_bits(rect.h) as u64);
    hash
}

pub fn context_hash_color(color: &Color) -> u64 {
    let mut hash = FNV_OFFSET;
    hash = context_hash_mix(hash, context_float_bits(color.r) as u64);
    hash = context_hash_mix(hash, context_float_bits(color.g) as u64);
    hash = context_hash_mix(hash, context_float_bits(color.b) as u64);
    hash = context_hash_mix(hash, context_float_bits(color.a) as u64);
    hash
}

pub fn context_hash_graphics_color(color: &GfxColor) -> u64 {
    context_hash_color(&Color { r: color.r, g: color.g, b: color.b, a: color.a })
}

pub fn context_hash_brush(brush: &Brush) -> u64 {
    let mut hash = FNV_OFFSET;
    hash = context_hash_mix(hash, brush.kind as u64);
    match brush.kind {
        BrushKind::Solid => {
            hash = context_hash_mix(hash, context_hash_graphics_color(&brush.solid));
        }
        BrushKind::LinearGradient => {
            hash = context_hash_mix(hash, context_float_bits(brush.linear.start.x) as u64);
            hash = context_hash_mix(hash, context_float_bits(brush.linear.start.y) as u64);
            hash = context_hash_mix(hash, context_float_bits(brush.linear.end.x) as u64);
            hash = context_hash_mix(hash, context_float_bits(brush.linear.end.y) as u64);
            hash = context_hash_mix(hash, brush.linear.stop_count as u64);
            for i in 0..brush.linear.stop_count.min(brush.linear.stops.len()) {
                hash = context_hash_mix(hash, context_float_bits(brush.linear.stops[i].position) as u64);
                hash = context_hash_mix(hash, context_hash_graphics_color(&brush.linear.stops[i].color));
            }
        }
        BrushKind::RadialGradient => {
            hash = context_hash_mix(hash, context_float_bits(brush.radial.center.x) as u64);
            hash = context_hash_mix(hash, context_float_bits(brush.radial.center.y) as u64);
            hash = context_hash_mix(hash, context_float_bits(brush.radial.radius) as u64);
            hash = context_hash_mix(hash, brush.radial.stop_count as u64);
            for i in 0..brush.radial.stop_count.min(brush.radial.stops.len()) {
                hash = context_hash_mix(hash, context_float_bits(brush.radial.stops[i].position) as u64);
                hash = context_hash_mix(hash, context_hash_graphics_color(&brush.radial.stops[i].color));
            }
        }
        BrushKind::None => {}
    }
    hash
}

pub fn context_hash_transform_3d(transform: &Transform3D) -> u64 {
    let mut hash = FNV_OFFSET;
    hash = context_hash_mix(hash, context_float_bits(transform.translation_x) as u64);
    hash = context_hash_mix(hash, context_float_bits(transform.translation_y) as u64);
    hash = context_hash_mix(hash, context_float_bits(transform.translation_z) as u64);
    hash = context_hash_mix(hash, context_float_bits(transform.scale_x) as u64);
    hash = context_hash_mix(hash, context_float_bits(transform.scale_y) as u64);
    hash = context_hash_mix(hash, context_float_bits(transform.scale_z) as u64);
    hash = context_hash_mix(hash, context_float_bits(transform.rotation_x_deg) as u64);
    hash = context_hash_mix(hash, context_float_bits(transform.rotation_y_deg) as u64);
    hash = context_hash_mix(hash, context_float_bits(transform.rotation_z_deg) as u64);
    hash = context_hash_mix(hash, context_float_bits(transform.origin_x) as u64);
    hash = context_hash_mix(hash, context_float_bits(transform.origin_y) as u64);
    hash = context_hash_mix(hash, context_float_bits(transform.origin_z) as u64);
    hash = context_hash_mix(hash, context_float_bits(transform.perspective) as u64);
    hash
}

pub fn context_hash_command_base(cmd: &DrawCommand) -> u64 {
    let mut hash = FNV_OFFSET;
    hash = context_hash_mix(hash, cmd.command_type as u64);
    hash = context_hash_mix(hash, context_hash_rect(&cmd.rect));
    hash = context_hash_mix(hash, context_hash_color(&cmd.color));
    hash = context_hash_mix(hash, cmd.payload_hash);
    hash = context_hash_mix(hash, context_float_bits(cmd.font_size) as u64);
    hash = context_hash_mix(hash, cmd.align as u64);
    hash = context_hash_mix(hash, cmd.image_fit as u64);
    hash = context_hash_mix(hash, context_float_bits(cmd.radius) as u64);
    hash = context_hash_mix(hash, context_float_bits(cmd.thickness) as u64);
    hash = context_hash_mix(hash, context_float_bits(cmd.rotation) as u64);
    hash = context_hash_mix(hash, context_float_bits(cmd.blur_radius) as u64);
    hash = context_hash_mix(hash, context_float_bits(cmd.effect_alpha) as u64);
    hash = context_hash_mix(hash, if cmd.has_clip { 1u64 } else { 0u64 });
    if cmd.has_clip {
        hash = context_hash_mix(hash, context_hash_rect(&cmd.clip_rect));
    }
    hash
}

pub fn context_intersect_rects(lhs: &Rect, rhs: &Rect) -> Option<Rect> {
    let x1 = lhs.x.max(rhs.x);
    let y1 = lhs.y.max(rhs.y);
    let x2 = (lhs.x + lhs.w).min(rhs.x + rhs.w);
    let y2 = (lhs.y + lhs.h).min(rhs.y + rhs.h);
    if x2 <= x1 || y2 <= y1 {
        None
    } else {
        Some(Rect { x: x1, y: y1, w: x2 - x1, h: y2 - y1 })
    }
}

pub fn context_expand_rect(rect: &Rect, dx: f32, dy: f32) -> Rect {
    Rect {
        x: rect.x - dx,
        y: rect.y - dy,
        w: rect.w + dx * 2.0,
        h: rect.h + dy * 2.0,
    }
}

pub fn context_translate_rect(rect: &Rect, dx: f32, dy: f32) -> Rect {
    if dx.abs() < 0.15 && dy.abs() < 0.15 {
        return *rect;
    }
    Rect {
        x: rect.x + dx,
        y: rect.y + dy,
        w: rect.w,
        h: rect.h,
    }
}

pub fn context_scale_rect_from_center(rect: &Rect, scale_x: f32, scale_y: f32) -> Rect {
    let sx = scale_x.max(0.1);
    let sy = scale_y.max(0.1);
    if (sx - 1.0).abs() < 0.0035 && (sy - 1.0).abs() < 0.0035 {
        return *rect;
    }
    let new_w = rect.w * sx;
    let new_h = rect.h * sy;
    Rect {
        x: rect.x + (rect.w - new_w) * 0.5,
        y: rect.y + (rect.h - new_h) * 0.5,
        w: new_w,
        h: new_h,
    }
}

pub fn context_scale_alpha_color(color: &Color, factor: f32) -> Color {
    Color {
        r: color.r,
        g: color.g,
        b: color.b,
        a: (color.a * factor).clamp(0.0, 1.0),
    }
}

pub fn context_scale_alpha_gfx(color: &GfxColor, factor: f32) -> GfxColor {
    GfxColor {
        r: color.r,
        g: color.g,
        b: color.b,
        a: (color.a * factor).clamp(0.0, 1.0),
    }
}

pub fn context_scale_alpha_brush(brush: &Brush, factor: f32) -> Brush {
    let mut scaled = *brush;
    match scaled.kind {
        BrushKind::Solid => {
            scaled.solid = context_scale_alpha_gfx(&scaled.solid, factor);
        }
        BrushKind::LinearGradient => {
            for i in 0..scaled.linear.stop_count.min(scaled.linear.stops.len()) {
                scaled.linear.stops[i].color = context_scale_alpha_gfx(&scaled.linear.stops[i].color, factor);
            }
        }
        BrushKind::RadialGradient => {
            for i in 0..scaled.radial.stop_count.min(scaled.radial.stops.len()) {
                scaled.radial.stops[i].color = context_scale_alpha_gfx(&scaled.radial.stops[i].color, factor);
            }
        }
        BrushKind::None => {}
    }
    scaled
}
