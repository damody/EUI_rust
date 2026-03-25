use crate::color::{rgba, Color};
use crate::core::context::Context;
use crate::core::draw_command::TextAlign;
use crate::core::foundation::{ButtonStyle, FlexAlign, FlexLength};
use crate::graphics::effects::*;
use crate::graphics::primitives::*;
use crate::graphics::transforms::*;
use crate::quick::anchor::{AnchorRect, AnchorValue, resolve_anchor_rect};
use crate::quick::gfx;
use crate::quick::primitive_painter;
use crate::rect::Rect;

// ── Placement helper ──

#[derive(Debug, Clone, Copy, Default)]
pub struct Placement {
    pub has_rect: bool,
    pub rect: Rect,
    pub has_pos: bool,
    pub x: f32,
    pub y: f32,
    pub has_size: bool,
    pub w: f32,
    pub h: f32,
    pub use_parent: bool,
}

fn resolve_placement(p: &Placement, content: Rect) -> Rect {
    if p.has_rect {
        return p.rect;
    }
    if p.use_parent {
        return content;
    }
    let x = if p.has_pos { p.x } else { content.x };
    let y = if p.has_pos { p.y } else { content.y };
    let w = if p.has_size { p.w } else { content.w };
    let h = if p.has_size { p.h } else { content.h };
    Rect::new(x, y, w, h)
}

// ── ShapeBuilder ──

pub struct ShapeBuilder<'a> {
    ctx: &'a mut Context,
    placement: Placement,
    fill_color: Option<Color>,
    fill_brush: Option<Brush>,
    radius: f32,
    stroke_color: Option<Color>,
    stroke_width: f32,
}

impl<'a> ShapeBuilder<'a> {
    pub fn new(ctx: &'a mut Context) -> Self {
        Self {
            ctx,
            placement: Placement::default(),
            fill_color: None,
            fill_brush: None,
            radius: 0.0,
            stroke_color: None,
            stroke_width: 1.0,
        }
    }

    pub fn rect(mut self, r: Rect) -> Self { self.placement.has_rect = true; self.placement.rect = r; self }
    pub fn at(mut self, x: f32, y: f32) -> Self { self.placement.has_pos = true; self.placement.x = x; self.placement.y = y; self }
    pub fn size(mut self, w: f32, h: f32) -> Self { self.placement.has_size = true; self.placement.w = w; self.placement.h = h; self }
    pub fn radius(mut self, r: f32) -> Self { self.radius = r; self }

    pub fn fill(mut self, color: Color) -> Self { self.fill_color = Some(color); self }
    pub fn fill_hex(mut self, hex: u32, alpha: f32) -> Self {
        let inv = 1.0 / 255.0;
        self.fill_color = Some(rgba(
            ((hex >> 16) & 0xff) as f32 * inv,
            ((hex >> 8) & 0xff) as f32 * inv,
            (hex & 0xff) as f32 * inv,
            alpha,
        ));
        self
    }
    pub fn fill_brush(mut self, brush: Brush) -> Self { self.fill_brush = Some(brush); self }
    pub fn stroke(mut self, color: Color, width: f32) -> Self { self.stroke_color = Some(color); self.stroke_width = width; self }

    pub fn draw(self) -> Rect {
        let content = self.ctx.layout_rect();
        let r = resolve_placement(&self.placement, content);
        if let Some(brush) = self.fill_brush {
            self.ctx.paint_filled_rect_with_brush(r, brush, self.radius);
        } else if let Some(color) = self.fill_color {
            self.ctx.paint_filled_rect(r, color, self.radius);
        }
        if let Some(sc) = self.stroke_color {
            self.ctx.paint_outline_rect(r, sc, self.radius, self.stroke_width);
        }
        r
    }
}

// ── TextBuilder ──

pub struct TextBuilder<'a> {
    ctx: &'a mut Context,
    text: String,
    placement: Placement,
    font_size: f32,
    color: Option<Color>,
    align: TextAlign,
}

impl<'a> TextBuilder<'a> {
    pub fn new(ctx: &'a mut Context, text: &str) -> Self {
        Self {
            ctx,
            text: text.to_string(),
            placement: Placement::default(),
            font_size: 13.0,
            color: None,
            align: TextAlign::Left,
        }
    }

    pub fn rect(mut self, r: Rect) -> Self { self.placement.has_rect = true; self.placement.rect = r; self }
    pub fn at(mut self, x: f32, y: f32) -> Self { self.placement.has_pos = true; self.placement.x = x; self.placement.y = y; self }
    pub fn size(mut self, w: f32, h: f32) -> Self { self.placement.has_size = true; self.placement.w = w; self.placement.h = h; self }
    pub fn font_size(mut self, s: f32) -> Self { self.font_size = s; self }
    pub fn color(mut self, c: Color) -> Self { self.color = Some(c); self }
    pub fn align(mut self, a: TextAlign) -> Self { self.align = a; self }
    pub fn center(mut self) -> Self { self.align = TextAlign::Center; self }
    pub fn right(mut self) -> Self { self.align = TextAlign::Right; self }

    pub fn draw(self) -> Rect {
        let content = self.ctx.layout_rect();
        let theme_text = self.ctx.theme().text;
        let r = resolve_placement(&self.placement, content);
        let color = self.color.unwrap_or(theme_text);
        self.ctx.paint_text(r, &self.text, self.font_size, color, self.align);
        r
    }
}

// ── LabelBuilder ──

pub struct LabelBuilder<'a> {
    ctx: &'a mut Context,
    text: String,
    font_size: f32,
    muted: bool,
    height: f32,
}

impl<'a> LabelBuilder<'a> {
    pub fn new(ctx: &'a mut Context, text: &str) -> Self {
        Self { ctx, text: text.to_string(), font_size: 13.0, muted: false, height: 24.0 }
    }

    pub fn font_size(mut self, s: f32) -> Self { self.font_size = s; self }
    pub fn muted(mut self) -> Self { self.muted = true; self }
    pub fn height(mut self, h: f32) -> Self { self.height = h; self }

    pub fn draw(self) -> Rect {
        let lr = self.ctx.layout_rect();
        let y = self.ctx.cursor_y();
        let r = Rect::new(lr.x, y, lr.w, self.height);
        let color = if self.muted { self.ctx.theme().muted_text } else { self.ctx.theme().text };
        self.ctx.paint_text(r, &self.text, self.font_size, color, TextAlign::Left);
        self.ctx.advance_cursor(self.height, 4.0);
        r
    }
}

// ── ButtonBuilder ──

pub struct ButtonBuilder<'a> {
    ctx: &'a mut Context,
    label: String,
    style: ButtonStyle,
    height: f32,
    placement: Placement,
}

impl<'a> ButtonBuilder<'a> {
    pub fn new(ctx: &'a mut Context, label: &str) -> Self {
        Self { ctx, label: label.to_string(), style: ButtonStyle::Primary, height: 36.0, placement: Placement::default() }
    }

    pub fn rect(mut self, r: Rect) -> Self { self.placement.has_rect = true; self.placement.rect = r; self }
    pub fn style(mut self, s: ButtonStyle) -> Self { self.style = s; self }
    pub fn secondary(mut self) -> Self { self.style = ButtonStyle::Secondary; self }
    pub fn ghost(mut self) -> Self { self.style = ButtonStyle::Ghost; self }
    pub fn height(mut self, h: f32) -> Self { self.height = h; self }

    pub fn draw(self) -> bool {
        let lr = self.ctx.layout_rect();
        let y = self.ctx.cursor_y();
        let r = if self.placement.has_rect {
            self.placement.rect
        } else {
            Rect::new(lr.x, y, lr.w, self.height)
        };
        let id = crate::core::context_utils::context_hash_sv(&self.label);
        let clicked = self.ctx.button(id, r, &self.label, self.style);
        if !self.placement.has_rect {
            self.ctx.advance_cursor(self.height, 4.0);
        }
        clicked
    }
}

// ── SliderFloatBuilder ──

pub struct SliderFloatBuilder<'a> {
    ctx: &'a mut Context,
    label: String,
    value: &'a mut f32,
    min: f32,
    max: f32,
    height: f32,
}

impl<'a> SliderFloatBuilder<'a> {
    pub fn new(ctx: &'a mut Context, label: &str, value: &'a mut f32) -> Self {
        Self { ctx, label: label.to_string(), value, min: 0.0, max: 1.0, height: 36.0 }
    }

    pub fn range(mut self, min: f32, max: f32) -> Self { self.min = min; self.max = max; self }
    pub fn height(mut self, h: f32) -> Self { self.height = h; self }

    pub fn draw(self) -> bool {
        let lr = self.ctx.layout_rect();
        let y = self.ctx.cursor_y();
        let r = Rect::new(lr.x, y, lr.w, self.height);
        // Label
        let label_r = Rect::new(r.x, r.y, r.w, 16.0);
        let muted = self.ctx.theme().muted_text;
        self.ctx.paint_text(label_r, &self.label, 11.0, muted, TextAlign::Left);
        // Slider
        let slider_r = Rect::new(r.x, r.y + 16.0, r.w, self.height - 16.0);
        let id = crate::core::context_utils::context_hash_sv(&self.label);
        let changed = self.ctx.slider(id, slider_r, self.value, self.min, self.max);
        self.ctx.advance_cursor(self.height, 4.0);
        changed
    }
}

// ── InputBuilder ──

pub struct InputBuilder<'a> {
    ctx: &'a mut Context,
    label: String,
    value: &'a mut String,
    height: f32,
}

impl<'a> InputBuilder<'a> {
    pub fn new(ctx: &'a mut Context, label: &str, value: &'a mut String) -> Self {
        Self { ctx, label: label.to_string(), value, height: 36.0 }
    }

    pub fn height(mut self, h: f32) -> Self { self.height = h; self }

    pub fn draw(self) -> bool {
        let lr = self.ctx.layout_rect();
        let y = self.ctx.cursor_y();
        let r = Rect::new(lr.x, y, lr.w, self.height);
        if !self.label.is_empty() {
            let label_r = Rect::new(r.x, r.y, r.w, 16.0);
            let muted = self.ctx.theme().muted_text;
            self.ctx.paint_text(label_r, &self.label, 11.0, muted, TextAlign::Left);
        }
        let field_r = if self.label.is_empty() {
            r
        } else {
            Rect::new(r.x, r.y + 16.0, r.w, self.height - 16.0)
        };
        let id = crate::core::context_utils::context_hash_sv(&self.label);
        let changed = self.ctx.text_input_field(id, field_r, self.value);
        self.ctx.advance_cursor(self.height, 4.0);
        changed
    }
}

// ── ProgressBuilder ──

pub struct ProgressBuilder<'a> {
    ctx: &'a mut Context,
    label: String,
    ratio: f32,
    height: f32,
}

impl<'a> ProgressBuilder<'a> {
    pub fn new(ctx: &'a mut Context, label: &str, ratio: f32) -> Self {
        Self { ctx, label: label.to_string(), ratio, height: 24.0 }
    }

    pub fn height(mut self, h: f32) -> Self { self.height = h; self }

    pub fn draw(self) -> Rect {
        let lr = self.ctx.layout_rect();
        let y = self.ctx.cursor_y();
        let r = Rect::new(lr.x, y, lr.w, self.height);
        if !self.label.is_empty() {
            let label_r = Rect::new(r.x, r.y, r.w, 14.0);
            let muted = self.ctx.theme().muted_text;
            self.ctx.paint_text(label_r, &self.label, 11.0, muted, TextAlign::Left);
        }
        let bar_r = Rect::new(r.x, r.y + 14.0, r.w, 6.0);
        self.ctx.progress_bar(bar_r, self.ratio);
        self.ctx.advance_cursor(self.height, 4.0);
        r
    }
}

// ── AnchorBuilder ──

pub struct AnchorBuilder<'a> {
    ctx: &'a Context,
    spec: AnchorRect,
    reference: Option<Rect>,
}

impl<'a> AnchorBuilder<'a> {
    pub fn new(ctx: &'a Context) -> Self {
        Self { ctx, spec: AnchorRect::default(), reference: None }
    }

    pub fn in_rect(mut self, r: Rect) -> Self { self.reference = Some(r); self }
    pub fn left(mut self, px: f32) -> Self { self.spec.left = AnchorValue::px(px); self }
    pub fn right(mut self, px: f32) -> Self { self.spec.right = AnchorValue::px(px); self }
    pub fn top(mut self, px: f32) -> Self { self.spec.top = AnchorValue::px(px); self }
    pub fn bottom(mut self, px: f32) -> Self { self.spec.bottom = AnchorValue::px(px); self }
    pub fn center_x(mut self, offset: f32) -> Self { self.spec.center_x = AnchorValue::px(offset); self }
    pub fn center_y(mut self, offset: f32) -> Self { self.spec.center_y = AnchorValue::px(offset); self }
    pub fn width(mut self, px: f32) -> Self { self.spec.width = AnchorValue::px(px); self }
    pub fn height(mut self, px: f32) -> Self { self.spec.height = AnchorValue::px(px); self }
    pub fn size(self, w: f32, h: f32) -> Self { self.width(w).height(h) }

    pub fn left_percent(mut self, p: f32) -> Self { self.spec.left = AnchorValue::percent(p); self }
    pub fn right_percent(mut self, p: f32) -> Self { self.spec.right = AnchorValue::percent(p); self }
    pub fn top_percent(mut self, p: f32) -> Self { self.spec.top = AnchorValue::percent(p); self }
    pub fn bottom_percent(mut self, p: f32) -> Self { self.spec.bottom = AnchorValue::percent(p); self }
    pub fn width_percent(mut self, p: f32) -> Self { self.spec.width = AnchorValue::percent(p); self }
    pub fn height_percent(mut self, p: f32) -> Self { self.spec.height = AnchorValue::percent(p); self }

    pub fn resolve(self) -> Rect {
        let reference = self.reference.unwrap_or(self.ctx.layout_rect());
        resolve_anchor_rect(&self.spec, &reference)
    }
}

// ── ImageBuilder ──

pub struct ImageBuilder<'a> {
    ctx: &'a mut Context,
    source: String,
    placement: Placement,
    radius: f32,
    fit: ImageFit,
}

impl<'a> ImageBuilder<'a> {
    pub fn new(ctx: &'a mut Context, source: &str) -> Self {
        Self { ctx, source: source.to_string(), placement: Placement::default(), radius: 0.0, fit: ImageFit::Cover }
    }

    pub fn rect(mut self, r: Rect) -> Self { self.placement.has_rect = true; self.placement.rect = r; self }
    pub fn radius(mut self, r: f32) -> Self { self.radius = r; self }
    pub fn fit(mut self, f: ImageFit) -> Self { self.fit = f; self }

    pub fn draw(self) -> Rect {
        let content = self.ctx.layout_rect();
        let r = resolve_placement(&self.placement, content);
        self.ctx.paint_image_rect(r, &self.source, self.fit, self.radius);
        r
    }
}

// ── RectangleBuilder (wraps graphics::RectanglePrimitive) ──

pub struct RectangleBuilder<'a> {
    ctx: &'a mut Context,
    prim: RectanglePrimitive,
    has_rect: bool,
}

impl<'a> RectangleBuilder<'a> {
    pub fn new(ctx: &'a mut Context) -> Self {
        Self { ctx, prim: RectanglePrimitive::new(), has_rect: false }
    }

    pub fn rect(mut self, r: Rect) -> Self { self.prim.rect = r; self.has_rect = true; self }
    pub fn fill(mut self, brush: Brush) -> Self { self.prim.fill = brush; self }
    pub fn fill_color(mut self, c: Color) -> Self { self.prim.fill = gfx::solid_color(&c, 1.0); self }
    pub fn radius(mut self, r: f32) -> Self { self.prim.radius = CornerRadius::uniform(r); self }
    pub fn corner_radius(mut self, r: CornerRadius) -> Self { self.prim.radius = r; self }
    pub fn stroke(mut self, s: Stroke) -> Self { self.prim.stroke = s; self }
    pub fn shadow(mut self, s: Shadow) -> Self { self.prim.shadow = s; self }
    pub fn blur(mut self, b: Blur) -> Self { self.prim.blur = b; self }
    pub fn opacity(mut self, o: f32) -> Self { self.prim.opacity = o; self }
    pub fn image(mut self, src: &str) -> Self { self.prim.image_source = src.to_string(); self }
    pub fn image_fit(mut self, f: ImageFit) -> Self { self.prim.image_fit = f; self }
    pub fn clip(mut self, c: ClipRect) -> Self { self.prim.clip = c; self }
    pub fn transform_2d(mut self, t: Transform2D) -> Self { self.prim.transform_2d = t; self }
    pub fn transform_3d(mut self, t: Transform3D) -> Self { self.prim.transform_3d = t; self }

    pub fn draw(self) -> Rect {
        let content = self.ctx.layout_rect();
        let mut prim = self.prim;
        if !self.has_rect {
            prim.rect = content;
        }
        primitive_painter::paint_rectangle(self.ctx, &prim);
        prim.rect
    }
}

// ── SurfaceBuilder (panel/card) ──

pub struct SurfaceBuilder<'a> {
    ctx: &'a mut Context,
    title: String,
    placement: Placement,
    padding: f32,
    radius: f32,
}

impl<'a> SurfaceBuilder<'a> {
    pub fn new(ctx: &'a mut Context, title: &str) -> Self {
        Self {
            ctx, title: title.to_string(),
            placement: Placement::default(),
            padding: 16.0,
            radius: 8.0,
        }
    }

    pub fn rect(mut self, r: Rect) -> Self { self.placement.has_rect = true; self.placement.rect = r; self }
    pub fn padding(mut self, p: f32) -> Self { self.padding = p; self }
    pub fn radius(mut self, r: f32) -> Self { self.radius = r; self }

    pub fn begin<F: FnOnce(&mut Context)>(self, f: F) {
        let content = self.ctx.layout_rect();
        let r = resolve_placement(&self.placement, content);

        // Background
        let panel_color = self.ctx.theme().panel;
        let border_color = self.ctx.theme().panel_border;
        self.ctx.paint_filled_rect(r, panel_color, self.radius);
        self.ctx.paint_outline_rect(r, border_color, self.radius, 1.0);

        // Title
        let inner = Rect::new(r.x + self.padding, r.y + self.padding, (r.w - self.padding * 2.0).max(0.0), (r.h - self.padding * 2.0).max(0.0));
        if !self.title.is_empty() {
            let title_r = Rect::new(inner.x, inner.y, inner.w, 20.0);
            let text_color = self.ctx.theme().text;
            self.ctx.paint_text(title_r, &self.title, 14.0, text_color, TextAlign::Left);
            let body = Rect::new(inner.x, inner.y + 28.0, inner.w, (inner.h - 28.0).max(0.0));
            self.ctx.push_layout_rect(body);
        } else {
            self.ctx.push_layout_rect(inner);
        }

        f(self.ctx);
        self.ctx.pop_layout_rect();
    }
}

// ── MetricBuilder ──

pub struct MetricBuilder<'a> {
    ctx: &'a mut Context,
    label: String,
    value: String,
    height: f32,
}

impl<'a> MetricBuilder<'a> {
    pub fn new(ctx: &'a mut Context, label: &str, value: &str) -> Self {
        Self { ctx, label: label.to_string(), value: value.to_string(), height: 48.0 }
    }

    pub fn height(mut self, h: f32) -> Self { self.height = h; self }

    pub fn draw(self) -> Rect {
        let lr = self.ctx.layout_rect();
        let y = self.ctx.cursor_y();
        let r = Rect::new(lr.x, y, lr.w, self.height);
        let muted = self.ctx.theme().muted_text;
        let text_color = self.ctx.theme().text;
        self.ctx.paint_text(Rect::new(r.x, r.y, r.w, 16.0), &self.label, 11.0, muted, TextAlign::Left);
        self.ctx.paint_text(Rect::new(r.x, r.y + 18.0, r.w, 24.0), &self.value, 20.0, text_color, TextAlign::Left);
        self.ctx.advance_cursor(self.height, 4.0);
        r
    }
}

// ── RowBuilder ──

pub struct RowBuilder<'a> {
    ctx: &'a mut Context,
    items: Vec<FlexLength>,
    gap: f32,
    align: FlexAlign,
}

impl<'a> RowBuilder<'a> {
    pub fn new(ctx: &'a mut Context) -> Self {
        Self { ctx, items: Vec::new(), gap: 8.0, align: FlexAlign::Top }
    }

    pub fn item(mut self, fl: FlexLength) -> Self { self.items.push(fl); self }
    pub fn items(mut self, items: &[FlexLength]) -> Self { self.items.extend_from_slice(items); self }
    pub fn gap(mut self, g: f32) -> Self { self.gap = g; self }
    pub fn align(mut self, a: FlexAlign) -> Self { self.align = a; self }

    pub fn begin<F: FnOnce(&mut Context)>(self, f: F) {
        self.ctx.begin_flex_row(&self.items, self.gap, self.align);
        f(self.ctx);
        self.ctx.end_flex_row();
    }
}
