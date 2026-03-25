use crate::color::{rgba, Color};
use crate::core::context::Context;
use crate::graphics::effects::{Brush, BrushKind, GfxColor, Shadow};
use crate::graphics::primitives::*;
use crate::graphics::transforms::{Transform2D, Transform3D};
use crate::math::{apply_rect_transform_2d, apply_rect_transform_3d_fallback, projected_rect_bounds};
use crate::rect::Rect;

pub fn average_corner_radius(r: &CornerRadius) -> f32 {
    ((r.top_left + r.top_right + r.bottom_right + r.bottom_left) * 0.25).max(0.0)
}

pub fn brush_primary_color(brush: &Brush) -> Option<GfxColor> {
    match brush.kind {
        BrushKind::Solid => Some(brush.solid),
        BrushKind::LinearGradient if brush.linear.stop_count > 0 => Some(brush.linear.stops[0].color),
        BrushKind::RadialGradient if brush.radial.stop_count > 0 => Some(brush.radial.stops[0].color),
        _ => None,
    }
}

pub fn combine_rect_transforms(t2d: &Transform2D, t3d: &Transform3D) -> Transform3D {
    let mut combined = *t3d;
    combined.translation_x += t2d.translation_x;
    combined.translation_y += t2d.translation_y;
    combined.scale_x *= t2d.scale_x.max(0.0);
    combined.scale_y *= t2d.scale_y.max(0.0);
    combined.rotation_z_deg += t2d.rotation_deg;
    if combined.origin_x.abs() <= 1e-6 {
        combined.origin_x = t2d.origin_x;
    }
    if combined.origin_y.abs() <= 1e-6 {
        combined.origin_y = t2d.origin_y;
    }
    combined
}

pub fn resolve_rectangle_rect(prim: &RectanglePrimitive) -> Rect {
    let combined = combine_rect_transforms(&prim.transform_2d, &prim.transform_3d);
    projected_rect_bounds(&prim.rect, &combined)
}

pub fn resolve_image_rect(prim: &ImagePrimitive) -> Rect {
    let combined = combine_rect_transforms(&prim.transform_2d, &prim.transform_3d);
    projected_rect_bounds(&prim.rect, &combined)
}

pub fn resolve_icon_rect(prim: &IconPrimitive) -> Rect {
    let rect = apply_rect_transform_2d(&prim.rect, &prim.transform_2d);
    apply_rect_transform_3d_fallback(&rect, &prim.transform_3d)
}

pub fn paint_shadow_approx(ctx: &mut Context, rect: &Rect, radius: f32, shadow: &Shadow, opacity: f32) {
    let blur = shadow.blur_radius.max(0.0);
    if blur <= 0.0 && shadow.spread <= 0.0 {
        return;
    }

    let layers = ((blur / 4.5) as i32 + 6).clamp(6, 16);
    let base = Color::new(shadow.color.r, shadow.color.g, shadow.color.b, shadow.color.a);
    for i in (1..=layers).rev() {
        let t = i as f32 / layers as f32;
        let grow = (shadow.spread + blur * (0.10 + t * 0.82)).max(0.0);
        let layer = Rect {
            x: rect.x + shadow.offset_x - grow,
            y: rect.y + shadow.offset_y - grow,
            w: rect.w + grow * 2.0,
            h: rect.h + grow * 2.0,
        };
        let a = (base.a * opacity * (0.54 / layers as f32) * (1.08 - 0.38 * t)).clamp(0.0, 1.0);
        let layer_color = rgba(base.r, base.g, base.b, a);
        ctx.paint_filled_rect(layer, layer_color, (radius + grow * 0.56).max(0.0));
    }
}

pub fn paint_fill_brush(ctx: &mut Context, rect: &Rect, radius: f32, brush: &Brush, opacity: f32) {
    let mut scaled = *brush;
    match scaled.kind {
        BrushKind::Solid => {
            scaled.solid.a = (scaled.solid.a * opacity).clamp(0.0, 1.0);
        }
        BrushKind::LinearGradient => {
            for i in 0..scaled.linear.stop_count.min(scaled.linear.stops.len()) {
                scaled.linear.stops[i].color.a = (scaled.linear.stops[i].color.a * opacity).clamp(0.0, 1.0);
            }
        }
        BrushKind::RadialGradient => {
            for i in 0..scaled.radial.stop_count.min(scaled.radial.stops.len()) {
                scaled.radial.stops[i].color.a = (scaled.radial.stops[i].color.a * opacity).clamp(0.0, 1.0);
            }
        }
        BrushKind::None => return,
    }
    ctx.paint_filled_rect_with_brush(*rect, scaled, radius);
}

pub fn paint_rectangle(ctx: &mut Context, prim: &RectanglePrimitive) {
    let combined = combine_rect_transforms(&prim.transform_2d, &prim.transform_3d);
    let rect = projected_rect_bounds(&prim.rect, &combined);
    let radius = average_corner_radius(&prim.radius);
    let opacity = prim.opacity.clamp(0.0, 1.0);
    let blur_radius = prim.blur.radius.max(prim.blur.backdrop_radius);

    let has_clip = prim.clip.mode == ClipMode::Bounds;
    if has_clip {
        ctx.push_clip(prim.clip.rect);
    }

    paint_shadow_approx(ctx, &rect, radius, &prim.shadow, opacity);
    if blur_radius > 0.0 {
        ctx.paint_backdrop_blur(prim.rect, blur_radius, radius);
    }

    paint_fill_brush(ctx, &prim.rect, radius, &prim.fill, opacity);
    if !prim.image_source.is_empty() {
        ctx.paint_image_rect(prim.rect, &prim.image_source, prim.image_fit, radius);
    }

    if prim.stroke.width > 0.0 {
        if let Some(sc) = brush_primary_color(&prim.stroke.brush) {
            let color = rgba(sc.r, sc.g, sc.b, (sc.a * opacity).clamp(0.0, 1.0));
            ctx.paint_outline_rect(prim.rect, color, radius, prim.stroke.width);
        }
    }

    if has_clip {
        ctx.pop_clip();
    }
}

pub fn paint_icon(ctx: &mut Context, prim: &IconPrimitive) {
    let rect = resolve_icon_rect(prim);
    let tint = brush_primary_color(&prim.tint).unwrap_or(GfxColor::new(1.0, 1.0, 1.0, 1.0));
    let color = rgba(tint.r, tint.g, tint.b, (tint.a * prim.opacity).clamp(0.0, 1.0));
    let glyph = char::from_u32(prim.glyph).map(|c| c.to_string()).unwrap_or_default();

    let has_clip = prim.clip.mode == ClipMode::Bounds;
    if has_clip {
        ctx.push_clip(prim.clip.rect);
    }
    let font_size = rect.h * 0.72;
    ctx.paint_text(rect, &glyph, font_size.max(8.0), color, crate::core::draw_command::TextAlign::Center);
    if has_clip {
        ctx.pop_clip();
    }
}

pub fn paint_image(ctx: &mut Context, prim: &ImagePrimitive) {
    let radius = average_corner_radius(&prim.radius);

    let has_clip = prim.clip.mode == ClipMode::Bounds;
    if has_clip {
        ctx.push_clip(prim.clip.rect);
    }

    ctx.paint_image_rect(prim.rect, &prim.source, prim.fit, radius);

    if has_clip {
        ctx.pop_clip();
    }
}
