use crate::color::Color;
use crate::core::context::Context;
use crate::graphics::primitives::*;
use crate::quick::anchor::{resolve_anchor_rect, AnchorRect};
use crate::quick::builders::*;
use crate::quick::layouts::*;
use crate::rect::{Rect, SplitRects};

pub fn rgb_hex(hex: u32, alpha: f32) -> Color {
    let inv = 1.0 / 255.0;
    Color::new(
        ((hex >> 16) & 0xff) as f32 * inv,
        ((hex >> 8) & 0xff) as f32 * inv,
        (hex & 0xff) as f32 * inv,
        alpha.clamp(0.0, 1.0),
    )
}

pub fn with_alpha(color: &Color, alpha: f32) -> Color {
    Color::new(color.r, color.g, color.b, (color.a * alpha).clamp(0.0, 1.0))
}

pub fn make_rect(x: f32, y: f32, w: f32, h: f32) -> Rect {
    Rect::new(x, y, w.max(0.0), h.max(0.0))
}

pub fn inset(rect: &Rect, px: f32, py: f32) -> Rect {
    let px = px.max(0.0);
    let py = py.max(0.0);
    make_rect(rect.x + px, rect.y + py, (rect.w - px * 2.0).max(0.0), (rect.h - py * 2.0).max(0.0))
}

pub fn inset_uniform(rect: &Rect, padding: f32) -> Rect {
    inset(rect, padding, padding)
}

pub fn translate(rect: &Rect, dx: f32, dy: f32) -> Rect {
    Rect::new(rect.x + dx, rect.y + dy, rect.w, rect.h)
}

pub fn has_area(rect: &Rect) -> bool {
    rect.w > 0.0 && rect.h > 0.0
}

pub struct UI<'a> {
    ctx: &'a mut Context,
}

impl<'a> UI<'a> {
    pub fn new(ctx: &'a mut Context) -> Self {
        Self { ctx }
    }

    pub fn ctx(&mut self) -> &mut Context {
        self.ctx
    }

    pub fn theme(&self) -> &crate::core::foundation::Theme {
        self.ctx.theme()
    }

    pub fn content(&self) -> Rect {
        self.ctx.layout_rect()
    }

    pub fn content_rect(&self) -> Rect {
        self.ctx.layout_rect()
    }

    pub fn measure_text(&self, text: &str, font_size: f32) -> f32 {
        self.ctx.measure_text(text, font_size)
    }

    // ── Split ──

    pub fn split_h(&self, rect: &Rect, first_width: f32, gap: f32) -> SplitRects {
        let g = gap.max(0.0);
        SplitRects {
            first: Rect::new(rect.x, rect.y, first_width, rect.h),
            second: Rect::new(rect.x + first_width + g, rect.y, (rect.w - first_width - g).max(0.0), rect.h),
        }
    }

    pub fn split_v(&self, rect: &Rect, first_height: f32, gap: f32) -> SplitRects {
        let g = gap.max(0.0);
        SplitRects {
            first: Rect::new(rect.x, rect.y, rect.w, first_height),
            second: Rect::new(rect.x, rect.y + first_height + g, rect.w, (rect.h - first_height - g).max(0.0)),
        }
    }

    pub fn split_h_ratio(&self, rect: &Rect, ratio: f32, gap: f32) -> SplitRects {
        let g = gap.max(0.0);
        let usable = (rect.w - g).max(0.0);
        self.split_h(rect, usable * ratio.clamp(0.0, 1.0), gap)
    }

    pub fn split_v_ratio(&self, rect: &Rect, ratio: f32, gap: f32) -> SplitRects {
        let g = gap.max(0.0);
        let usable = (rect.h - g).max(0.0);
        self.split_v(rect, usable * ratio.clamp(0.0, 1.0), gap)
    }

    // ── Builders ──

    pub fn shape(&mut self) -> ShapeBuilder<'_> {
        ShapeBuilder::new(self.ctx)
    }

    pub fn text(&mut self, text: &str) -> TextBuilder<'_> {
        TextBuilder::new(self.ctx, text)
    }

    pub fn label(&mut self, text: &str) -> LabelBuilder<'_> {
        LabelBuilder::new(self.ctx, text)
    }

    pub fn button(&mut self, label: &str) -> ButtonBuilder<'_> {
        ButtonBuilder::new(self.ctx, label)
    }

    pub fn slider<'b>(&'b mut self, label: &str, value: &'b mut f32) -> SliderFloatBuilder<'b> {
        SliderFloatBuilder::new(self.ctx, label, value)
    }

    pub fn input<'b>(&'b mut self, label: &str, value: &'b mut String) -> InputBuilder<'b> {
        InputBuilder::new(self.ctx, label, value)
    }

    pub fn progress(&mut self, label: &str, ratio: f32) -> ProgressBuilder<'_> {
        ProgressBuilder::new(self.ctx, label, ratio)
    }

    pub fn metric(&mut self, label: &str, value: &str) -> MetricBuilder<'_> {
        MetricBuilder::new(self.ctx, label, value)
    }

    pub fn image(&mut self, source: &str) -> ImageBuilder<'_> {
        ImageBuilder::new(self.ctx, source)
    }

    pub fn rectangle(&mut self) -> RectangleBuilder<'_> {
        RectangleBuilder::new(self.ctx)
    }

    pub fn panel(&mut self, title: &str) -> SurfaceBuilder<'_> {
        SurfaceBuilder::new(self.ctx, title)
    }

    pub fn card(&mut self, title: &str) -> SurfaceBuilder<'_> {
        SurfaceBuilder::new(self.ctx, title)
    }

    pub fn anchor(&self) -> AnchorBuilder<'_> {
        AnchorBuilder::new(self.ctx)
    }

    pub fn row(&mut self) -> RowBuilder<'_> {
        RowBuilder::new(self.ctx)
    }

    pub fn view(&mut self, rect: Rect) -> ViewBuilder<'_> {
        ViewBuilder::new(self.ctx, rect)
    }

    // ── Scopes via closures ──

    pub fn scope<F: FnOnce(&mut Context)>(&mut self, rect: Rect, f: F) {
        self.ctx.push_layout_rect(rect);
        f(self.ctx);
        self.ctx.pop_layout_rect();
    }

    pub fn stack<F: FnOnce(&mut Context)>(&mut self, rect: Rect, f: F) {
        self.ctx.push_layout_rect(rect);
        f(self.ctx);
        self.ctx.pop_layout_rect();
    }

    pub fn clip<F: FnOnce(&mut Context)>(&mut self, rect: Rect, f: F) {
        self.ctx.push_clip(rect);
        f(self.ctx);
        self.ctx.pop_clip();
    }

    pub fn spacer(&mut self, height: f32) {
        self.ctx.advance_cursor(height, 0.0);
    }

    // ── Paint primitives directly ──

    pub fn paint_rectangle(&mut self, prim: &RectanglePrimitive) -> Rect {
        crate::quick::primitive_painter::paint_rectangle(self.ctx, prim);
        crate::quick::primitive_painter::resolve_rectangle_rect(prim)
    }

    pub fn paint_icon(&mut self, prim: &IconPrimitive) {
        crate::quick::primitive_painter::paint_icon(self.ctx, prim);
    }

    pub fn paint_image(&mut self, prim: &ImagePrimitive) {
        crate::quick::primitive_painter::paint_image(self.ctx, prim);
    }

    pub fn resolve_anchor(&self, anchor: &AnchorRect) -> Rect {
        resolve_anchor_rect(anchor, &self.content_rect())
    }

    pub fn paint_filled_rect(&mut self, rect: Rect, color: Color, radius: f32) {
        self.ctx.paint_filled_rect(rect, color, radius);
    }

    pub fn paint_outline_rect(&mut self, rect: Rect, color: Color, thickness: f32, radius: f32) {
        self.ctx.paint_outline_rect(rect, color, thickness, radius);
    }

    pub fn cursor_y(&self) -> f32 {
        self.ctx.cursor_y()
    }
}
