// Phase 5: Full Context implementation

use std::collections::HashMap;
use std::sync::Arc;

use crate::color::{mix, rgba, Color};
use crate::core::context_state::*;
use crate::core::context_utils::*;
use crate::core::draw_command::*;
use crate::core::foundation::*;
use crate::graphics::effects::*;
use crate::graphics::primitives::*;
use crate::graphics::transforms::*;
use crate::rect::Rect;
use crate::text::measurement::TextMeasurer;

#[allow(dead_code)]
pub struct Context {
    // Drawing
    pub(crate) commands: Vec<DrawCommand>,
    pub(crate) text_arena: Vec<u8>,
    pub(crate) brush_payloads: Vec<Brush>,
    pub(crate) transform_payloads: Vec<Transform3D>,

    // Layout
    pub(crate) layout_stack: Vec<ContextLayoutRectState>,
    pub(crate) scope_stack: Vec<ContextScopeState>,
    pub(crate) clip_stack: Vec<Rect>,

    // State
    pub(crate) motion_states: HashMap<u64, ContextMotionState>,
    pub(crate) scroll_states: HashMap<u64, ContextScrollAreaState>,
    pub(crate) text_area_states: HashMap<u64, ContextTextAreaState>,

    // Input
    pub(crate) input: InputState,
    pub(crate) theme: Theme,
    pub(crate) frame_index: u64,

    // Viewport
    pub(crate) viewport_w: f32,
    pub(crate) viewport_h: f32,
    pub(crate) dpi_scale: f32,

    // Text
    pub(crate) text_measurer: Option<TextMeasurer>,

    // Hot/Active tracking
    pub(crate) hot_id: u64,
    pub(crate) active_id: u64,
    pub(crate) focus_id: u64,

    // Memory asset registry
    pub(crate) memory_assets: HashMap<String, Arc<Vec<u8>>>,

    // Hysteresis counters
    pub(crate) cmd_underuse: u32,
    pub(crate) text_underuse: u32,
}

impl Context {
    pub fn new() -> Self {
        Self {
            commands: Vec::with_capacity(512),
            text_arena: Vec::with_capacity(4096),
            brush_payloads: Vec::with_capacity(64),
            transform_payloads: Vec::with_capacity(16),
            layout_stack: Vec::with_capacity(16),
            scope_stack: Vec::with_capacity(8),
            clip_stack: Vec::with_capacity(8),
            motion_states: HashMap::new(),
            scroll_states: HashMap::new(),
            text_area_states: HashMap::new(),
            input: InputState::default(),
            theme: Theme::default(),
            frame_index: 0,
            viewport_w: 800.0,
            viewport_h: 600.0,
            dpi_scale: 1.0,
            text_measurer: None,
            hot_id: 0,
            active_id: 0,
            focus_id: 0,
            memory_assets: HashMap::new(),
            cmd_underuse: 0,
            text_underuse: 0,
        }
    }

    pub fn set_text_measurer(&mut self, measurer: TextMeasurer) {
        self.text_measurer = Some(measurer);
    }

    pub fn begin_frame(&mut self, viewport_w: f32, viewport_h: f32, dpi_scale: f32, input: InputState) {
        self.viewport_w = viewport_w;
        self.viewport_h = viewport_h;
        self.dpi_scale = dpi_scale;
        self.input = input;
        self.frame_index += 1;

        self.commands.clear();
        self.text_arena.clear();
        self.brush_payloads.clear();
        self.transform_payloads.clear();
        self.layout_stack.clear();
        self.scope_stack.clear();
        self.clip_stack.clear();

        // Push root layout
        self.layout_stack.push(ContextLayoutRectState {
            layout_rect: Rect::new(0.0, 0.0, viewport_w, viewport_h),
            cursor_y: 0.0,
            ..Default::default()
        });
    }

    pub fn end_frame(&mut self) {
        // Compute hashes for dirty checking
        for cmd in &mut self.commands {
            cmd.hash = context_hash_command_base(cmd);
        }

        // GC stale motion states
        let frame = self.frame_index;
        self.motion_states.retain(|_, state| frame - state.last_touched_frame < 120);
        self.scroll_states.retain(|_, state| frame - state.last_touched_frame < 120);
        self.text_area_states.retain(|_, state| frame - state.last_touched_frame < 120);
    }

    // ── Layout Rect ──

    pub fn layout_rect(&self) -> Rect {
        self.layout_stack.last()
            .map(|s| s.layout_rect)
            .unwrap_or(Rect::new(0.0, 0.0, self.viewport_w, self.viewport_h))
    }

    pub fn cursor_y(&self) -> f32 {
        self.layout_stack.last().map(|s| s.cursor_y).unwrap_or(0.0)
    }

    pub fn advance_cursor(&mut self, height: f32, gap: f32) {
        if let Some(state) = self.layout_stack.last_mut() {
            state.cursor_y += height + gap;
        }
    }

    pub fn push_layout_rect(&mut self, rect: Rect) {
        self.layout_stack.push(ContextLayoutRectState {
            layout_rect: rect,
            cursor_y: rect.y,
            ..Default::default()
        });
    }

    pub fn pop_layout_rect(&mut self) {
        if self.layout_stack.len() > 1 {
            self.layout_stack.pop();
        }
    }

    // ── Clip ──

    pub fn push_clip(&mut self, rect: Rect) {
        let clipped = if let Some(top) = self.clip_stack.last() {
            context_intersect_rects(top, &rect).unwrap_or(Rect::ZERO)
        } else {
            rect
        };
        self.clip_stack.push(clipped);
    }

    pub fn pop_clip(&mut self) {
        self.clip_stack.pop();
    }

    pub fn current_clip(&self) -> Option<Rect> {
        self.clip_stack.last().copied()
    }

    // ── Paint Commands ──

    fn push_command(&mut self, mut cmd: DrawCommand) -> usize {
        if let Some(clip) = self.current_clip() {
            cmd.has_clip = true;
            cmd.clip_rect = clip;
            if let Some(vis) = context_intersect_rects(&cmd.rect, &clip) {
                cmd.visible_rect = vis;
            } else {
                cmd.visible_rect = Rect::ZERO;
            }
        } else {
            cmd.visible_rect = cmd.rect;
        }
        let idx = self.commands.len();
        self.commands.push(cmd);
        idx
    }

    pub fn paint_filled_rect(&mut self, rect: Rect, color: Color, radius: f32) -> usize {
        self.push_command(DrawCommand {
            command_type: CommandType::FilledRect,
            rect,
            color,
            radius,
            ..Default::default()
        })
    }

    pub fn paint_filled_rect_with_brush(&mut self, rect: Rect, brush: Brush, radius: f32) -> usize {
        let payload_index = self.brush_payloads.len() as u32;
        self.brush_payloads.push(brush);
        let color = match brush.kind {
            BrushKind::Solid => Color::new(brush.solid.r, brush.solid.g, brush.solid.b, brush.solid.a),
            _ => Color::WHITE,
        };
        self.push_command(DrawCommand {
            command_type: CommandType::FilledRect,
            rect,
            color,
            radius,
            brush_payload_index: payload_index,
            payload_hash: context_hash_brush(&brush),
            ..Default::default()
        })
    }

    pub fn paint_outline_rect(&mut self, rect: Rect, color: Color, radius: f32, thickness: f32) -> usize {
        self.push_command(DrawCommand {
            command_type: CommandType::RectOutline,
            rect,
            color,
            radius,
            thickness,
            ..Default::default()
        })
    }

    pub fn paint_text(&mut self, rect: Rect, text: &str, font_size: f32, color: Color, align: TextAlign) -> usize {
        let offset = self.text_arena.len() as u32;
        self.text_arena.extend_from_slice(text.as_bytes());
        let length = text.len() as u32;
        self.push_command(DrawCommand {
            command_type: CommandType::Text,
            rect,
            color,
            font_size,
            align,
            text_offset: offset,
            text_length: length,
            ..Default::default()
        })
    }

    pub fn paint_image_rect(&mut self, rect: Rect, image_path: &str, fit: ImageFit, radius: f32) -> usize {
        let hash = context_hash_sv(image_path);
        let offset = self.text_arena.len() as u32;
        self.text_arena.extend_from_slice(image_path.as_bytes());
        let length = image_path.len() as u32;
        self.push_command(DrawCommand {
            command_type: CommandType::ImageRect,
            rect,
            radius,
            image_fit: fit,
            text_offset: offset,
            text_length: length,
            payload_hash: hash,
            ..Default::default()
        })
    }

    pub fn paint_backdrop_blur(&mut self, rect: Rect, blur_radius: f32, radius: f32) -> usize {
        self.push_command(DrawCommand {
            command_type: CommandType::BackdropBlur,
            rect,
            blur_radius,
            radius,
            ..Default::default()
        })
    }

    pub fn paint_chevron(&mut self, rect: Rect, color: Color, rotation: f32) -> usize {
        self.push_command(DrawCommand {
            command_type: CommandType::Chevron,
            rect,
            color,
            rotation,
            ..Default::default()
        })
    }

    // ── Dock/Split ──

    pub fn dock_top(&mut self, height: f32) -> Rect {
        let lr = self.layout_rect();
        let r = Rect::new(lr.x, lr.y, lr.w, height);
        if let Some(state) = self.layout_stack.last_mut() {
            state.layout_rect.y += height;
            state.layout_rect.h = (state.layout_rect.h - height).max(0.0);
            state.cursor_y = state.layout_rect.y;
        }
        r
    }

    pub fn dock_bottom(&mut self, height: f32) -> Rect {
        let lr = self.layout_rect();
        let r = Rect::new(lr.x, lr.y + lr.h - height, lr.w, height);
        if let Some(state) = self.layout_stack.last_mut() {
            state.layout_rect.h = (state.layout_rect.h - height).max(0.0);
        }
        r
    }

    pub fn dock_left(&mut self, width: f32) -> Rect {
        let lr = self.layout_rect();
        let r = Rect::new(lr.x, lr.y, width, lr.h);
        if let Some(state) = self.layout_stack.last_mut() {
            state.layout_rect.x += width;
            state.layout_rect.w = (state.layout_rect.w - width).max(0.0);
        }
        r
    }

    pub fn dock_right(&mut self, width: f32) -> Rect {
        let lr = self.layout_rect();
        let r = Rect::new(lr.x + lr.w - width, lr.y, width, lr.h);
        if let Some(state) = self.layout_stack.last_mut() {
            state.layout_rect.w = (state.layout_rect.w - width).max(0.0);
        }
        r
    }

    pub fn split_h(&self, left_width: f32) -> (Rect, Rect) {
        let lr = self.layout_rect();
        let left = Rect::new(lr.x, lr.y, left_width, lr.h);
        let right = Rect::new(lr.x + left_width, lr.y, (lr.w - left_width).max(0.0), lr.h);
        (left, right)
    }

    pub fn split_v(&self, top_height: f32) -> (Rect, Rect) {
        let lr = self.layout_rect();
        let top = Rect::new(lr.x, lr.y, lr.w, top_height);
        let bottom = Rect::new(lr.x, lr.y + top_height, lr.w, (lr.h - top_height).max(0.0));
        (top, bottom)
    }

    // ── Flex Row ──

    pub fn begin_flex_row(&mut self, items: &[FlexLength], gap: f32, align: FlexAlign) {
        let lr = self.layout_rect();
        let n = items.len();
        let total_gap = if n > 1 { gap * (n as f32 - 1.0) } else { 0.0 };
        let avail = (lr.w - total_gap).max(0.0);

        // Resolve widths
        let mut widths = vec![0.0f32; n];
        let mut total_fixed = 0.0f32;
        let mut total_flex = 0.0f32;

        for (i, item) in items.iter().enumerate() {
            match item {
                FlexLength::Fixed(px) => {
                    widths[i] = *px;
                    total_fixed += px;
                }
                FlexLength::Flex(w) => {
                    total_flex += w;
                }
                FlexLength::Content { min, .. } => {
                    widths[i] = *min;
                    total_fixed += min;
                }
            }
        }

        let remaining = (avail - total_fixed).max(0.0);
        if total_flex > 0.0 {
            for (i, item) in items.iter().enumerate() {
                if let FlexLength::Flex(w) = item {
                    widths[i] = remaining * w / total_flex;
                }
            }
        }

        // Compute rects
        let mut rects = Vec::with_capacity(n);
        let mut x = lr.x;
        let y = self.cursor_y();
        for &width in &widths {
            rects.push(Rect::new(x, y, width, 0.0)); // height TBD
            x += width + gap;
        }

        if let Some(state) = self.layout_stack.last_mut() {
            state.flex_row = ContextFlexRowState {
                active: true,
                items: items.to_vec(),
                widths: widths.clone(),
                heights: vec![0.0; n],
                item_rects: rects,
                cmd_begin: vec![0; n],
                cmd_end: vec![0; n],
                index: 0,
                gap,
                y,
                row_height: 0.0,
                align,
            };
        }
    }

    pub fn next_flex_item(&mut self) -> Option<Rect> {
        let state = self.layout_stack.last_mut()?;
        let fr = &mut state.flex_row;
        if !fr.active || fr.index as usize >= fr.item_rects.len() {
            return None;
        }
        let idx = fr.index as usize;
        fr.cmd_begin[idx] = self.commands.len();
        let rect = fr.item_rects[idx];
        fr.index += 1;
        Some(rect)
    }

    pub fn finish_flex_item(&mut self, height: f32) {
        if let Some(state) = self.layout_stack.last_mut() {
            let fr = &mut state.flex_row;
            if !fr.active {
                return;
            }
            let idx = (fr.index - 1) as usize;
            if idx < fr.heights.len() {
                fr.heights[idx] = height;
                fr.cmd_end[idx] = self.commands.len();
                if height > fr.row_height {
                    fr.row_height = height;
                }
            }
        }
    }

    pub fn end_flex_row(&mut self) {
        if let Some(state) = self.layout_stack.last_mut() {
            let row_height = state.flex_row.row_height;
            let align = state.flex_row.align;

            // Apply vertical alignment
            if align != FlexAlign::Top {
                let n = state.flex_row.item_rects.len();
                for i in 0..n {
                    let h = state.flex_row.heights[i];
                    let offset = match align {
                        FlexAlign::Center => (row_height - h) * 0.5,
                        FlexAlign::Bottom => row_height - h,
                        FlexAlign::Top => 0.0,
                    };
                    if offset.abs() > 0.5 {
                        let begin = state.flex_row.cmd_begin[i];
                        let end = state.flex_row.cmd_end[i];
                        for cmd_idx in begin..end.min(self.commands.len()) {
                            self.commands[cmd_idx].rect.y += offset;
                            if self.commands[cmd_idx].has_clip {
                                // Don't move clip rects
                            }
                            self.commands[cmd_idx].visible_rect.y += offset;
                        }
                    }
                }
            }

            state.cursor_y = state.flex_row.y + row_height;
            state.flex_row.active = false;
        }
    }

    // ── Row (grid) ──

    pub fn begin_row(&mut self, columns: i32, gap: f32) {
        if let Some(state) = self.layout_stack.last_mut() {
            state.row = ContextRowState {
                active: true,
                columns,
                index: 0,
                next_span: 1,
                gap,
                y: state.cursor_y,
                max_height: 0.0,
            };
        }
    }

    pub fn next_cell(&mut self) -> Rect {
        let lr = self.layout_rect();
        if let Some(state) = self.layout_stack.last_mut() {
            let row = &mut state.row;
            if !row.active {
                return Rect::ZERO;
            }
            let cols = row.columns.max(1) as f32;
            let total_gap = row.gap * (cols - 1.0);
            let cell_w = ((lr.w - total_gap) / cols).max(0.0);
            let col = row.index % row.columns;
            let span = row.next_span.max(1);
            let x = lr.x + col as f32 * (cell_w + row.gap);
            let w = cell_w * span as f32 + row.gap * (span - 1).max(0) as f32;
            let rect = Rect::new(x, row.y, w, 0.0);
            row.index += span;
            row.next_span = 1;
            rect
        } else {
            Rect::ZERO
        }
    }

    pub fn finish_cell(&mut self, height: f32) {
        if let Some(state) = self.layout_stack.last_mut() {
            let row = &mut state.row;
            if height > row.max_height {
                row.max_height = height;
            }
            if row.index >= row.columns {
                row.y += row.max_height + row.gap;
                row.max_height = 0.0;
                row.index = 0;
            }
        }
    }

    pub fn end_row(&mut self) {
        if let Some(state) = self.layout_stack.last_mut() {
            if state.row.max_height > 0.0 {
                state.row.y += state.row.max_height + state.row.gap;
            }
            state.cursor_y = state.row.y;
            state.row.active = false;
        }
    }

    // ── Motion (animation state) ──

    pub fn motion(&mut self, id: u64, hovered: bool, pressed: bool) -> MotionResult {
        let dt = self.input.time_seconds as f32;
        let speed = 8.0;
        let state = self.motion_states.entry(id).or_default();
        state.last_touched_frame = self.frame_index;

        let target_hover = if hovered { 1.0 } else { 0.0 };
        let target_press = if pressed { 1.0 } else { 0.0 };
        state.hover += (target_hover - state.hover) * (speed * dt.max(0.001)).min(1.0);
        state.press += (target_press - state.press) * (speed * dt.max(0.001)).min(1.0);

        MotionResult {
            hover: state.hover,
            press: state.press,
        }
    }

    pub fn presence(&mut self, id: u64, visible: bool) -> f32 {
        let dt = (self.input.time_seconds as f32).max(0.001);
        let speed = 6.0;
        let state = self.motion_states.entry(id).or_default();
        state.last_touched_frame = self.frame_index;

        let target = if visible { 1.0 } else { 0.0 };
        if !state.initialized {
            state.active = target;
            state.initialized = true;
        }
        state.active += (target - state.active) * (speed * dt).min(1.0);
        state.active
    }

    pub fn animated_value(&mut self, id: u64, target: f32) -> f32 {
        let dt = (self.input.time_seconds as f32).max(0.001);
        let speed = 8.0;
        let state = self.motion_states.entry(id).or_default();
        state.last_touched_frame = self.frame_index;

        if !state.value_initialized {
            state.value = target;
            state.value_initialized = true;
        }
        state.value += (target - state.value) * (speed * dt).min(1.0);
        state.value
    }

    // ── Hit testing ──

    pub fn is_hovered(&self, rect: &Rect) -> bool {
        rect.contains(self.input.mouse_x, self.input.mouse_y)
    }

    pub fn is_mouse_pressed(&self) -> bool {
        self.input.mouse_pressed
    }

    pub fn is_mouse_released(&self) -> bool {
        self.input.mouse_released
    }

    pub fn is_mouse_down(&self) -> bool {
        self.input.mouse_down
    }

    // ── Widget helpers ──

    pub fn button(&mut self, id: u64, rect: Rect, label: &str, style: ButtonStyle) -> bool {
        let hovered = self.is_hovered(&rect);
        let pressed = hovered && self.is_mouse_down();
        let clicked = hovered && self.is_mouse_released();
        let m = self.motion(id, hovered, pressed);

        let (bg_color, text_color) = match style {
            ButtonStyle::Primary => {
                let bg = mix(self.theme.primary, rgba(1.0, 1.0, 1.0, 1.0), m.hover * 0.1 + m.press * 0.05);
                (bg, self.theme.primary_text)
            }
            ButtonStyle::Secondary => {
                let bg = mix(self.theme.secondary, self.theme.secondary_hover, m.hover);
                let bg = mix(bg, self.theme.secondary_active, m.press);
                (bg, self.theme.text)
            }
            ButtonStyle::Ghost => {
                let bg = rgba(0.0, 0.0, 0.0, m.hover * 0.05 + m.press * 0.03);
                (bg, self.theme.text)
            }
        };

        self.paint_filled_rect(rect, bg_color, self.theme.radius);
        let text_rect = Rect::new(rect.x, rect.y, rect.w, rect.h);
        self.paint_text(text_rect, label, 13.0, text_color, TextAlign::Center);

        clicked
    }

    pub fn slider(&mut self, id: u64, rect: Rect, value: &mut f32, min: f32, max: f32) -> bool {
        let hovered = self.is_hovered(&rect);
        let range = (max - min).max(1e-6);
        let ratio = ((*value - min) / range).clamp(0.0, 1.0);

        // Track
        let track_h = 4.0;
        let track_y = rect.y + (rect.h - track_h) * 0.5;
        let track_rect = Rect::new(rect.x, track_y, rect.w, track_h);
        self.paint_filled_rect(track_rect, self.theme.track, 2.0);

        // Fill
        let fill_w = rect.w * ratio;
        let fill_rect = Rect::new(rect.x, track_y, fill_w, track_h);
        self.paint_filled_rect(fill_rect, self.theme.track_fill, 2.0);

        // Thumb
        let thumb_r = 8.0;
        let thumb_x = rect.x + fill_w - thumb_r;
        let thumb_rect = Rect::new(thumb_x, rect.y + (rect.h - thumb_r * 2.0) * 0.5, thumb_r * 2.0, thumb_r * 2.0);
        let m = self.motion(id, hovered, hovered && self.is_mouse_down());
        let thumb_color = mix(self.theme.primary, rgba(1.0, 1.0, 1.0, 1.0), m.hover * 0.15);
        self.paint_filled_rect(thumb_rect, thumb_color, thumb_r);

        // Interaction
        let mut changed = false;
        if hovered && self.is_mouse_down() {
            let new_ratio = ((self.input.mouse_x - rect.x) / rect.w).clamp(0.0, 1.0);
            *value = min + new_ratio * range;
            changed = true;
        }
        changed
    }

    pub fn progress(&mut self, rect: Rect, value: f32) {
        let ratio = value.clamp(0.0, 1.0);
        self.paint_filled_rect(rect, self.theme.track, 4.0);
        let fill_rect = Rect::new(rect.x, rect.y, rect.w * ratio, rect.h);
        self.paint_filled_rect(fill_rect, self.theme.track_fill, 4.0);
    }

    // ── Text measurement ──

    pub fn measure_text(&self, text: &str, font_size: f32) -> f32 {
        if let Some(ref measurer) = self.text_measurer {
            measurer.measure_width(text, font_size)
        } else {
            // Approximate: ~0.5 em per char
            text.len() as f32 * font_size * 0.5
        }
    }

    // ── Getters ──

    pub fn commands(&self) -> &[DrawCommand] {
        &self.commands
    }

    pub fn text_arena(&self) -> &[u8] {
        &self.text_arena
    }

    pub fn brush_payloads(&self) -> &[Brush] {
        &self.brush_payloads
    }

    pub fn transform_payloads(&self) -> &[Transform3D] {
        &self.transform_payloads
    }

    pub fn theme(&self) -> &Theme {
        &self.theme
    }

    pub fn set_theme(&mut self, theme: Theme) {
        self.theme = theme;
    }

    pub fn input(&self) -> &InputState {
        &self.input
    }

    pub fn viewport_size(&self) -> (f32, f32) {
        (self.viewport_w, self.viewport_h)
    }

    // ── Memory assets ──

    pub fn register_memory_asset(&mut self, name: &str, data: Vec<u8>) {
        if name.is_empty() {
            return;
        }
        if data.is_empty() {
            self.memory_assets.remove(name);
        } else {
            self.memory_assets.insert(name.to_string(), Arc::new(data));
        }
    }

    pub fn find_memory_asset(&self, name: &str) -> Option<Arc<Vec<u8>>> {
        self.memory_assets.get(name).cloned()
    }

    pub fn resolve_memory_asset_uri(&self, uri: &str) -> Option<Arc<Vec<u8>>> {
        let scheme = "asset://";
        if let Some(name) = uri.strip_prefix(scheme) {
            self.find_memory_asset(name)
        } else {
            None
        }
    }

    pub fn memory_asset_uri(name: &str) -> String {
        format!("asset://{}", name)
    }

    // ── Scroll Area ──

    pub fn begin_scroll_area(&mut self, id: u64, viewport: Rect) -> f32 {
        let state = self.scroll_states.entry(id).or_default();
        state.last_touched_frame = self.frame_index;
        let scroll = state.scroll;

        self.push_clip(viewport);
        self.push_layout_rect(Rect::new(viewport.x, viewport.y - scroll, viewport.w, 100000.0));
        scroll
    }

    pub fn end_scroll_area(&mut self, id: u64, viewport: &Rect) {
        let content_h = self.cursor_y() - (viewport.y - self.scroll_states.get(&id).map(|s| s.scroll).unwrap_or(0.0));
        self.pop_layout_rect();
        self.pop_clip();

        let max_scroll = (content_h - viewport.h).max(0.0);

        let hovered = viewport.contains(self.input.mouse_x, self.input.mouse_y);
        let wheel_y = self.input.mouse_wheel_y;

        if let Some(state) = self.scroll_states.get_mut(&id) {
            state.content_height = content_h;

            // Mouse wheel
            if hovered {
                state.scroll -= wheel_y * 40.0;
            }

            // Inertia
            state.scroll += state.velocity;
            state.velocity *= 0.92;

            state.scroll = state.scroll.clamp(0.0, max_scroll);
        }
    }

    // ── Text Input ──

    pub fn text_input_field(&mut self, id: u64, rect: Rect, text: &mut String) -> bool {
        let hovered = self.is_hovered(&rect);
        let clicked = hovered && self.is_mouse_pressed();
        let focused = self.focus_id == id;

        if clicked {
            self.focus_id = id;
        }

        let m = self.motion(id, hovered, false);

        // Background
        self.paint_filled_rect(rect, self.theme.input_bg, self.theme.radius);

        // Border
        let border_color = if focused {
            self.theme.focus_ring
        } else {
            mix(self.theme.input_border, self.theme.primary, m.hover * 0.3)
        };
        self.paint_outline_rect(rect, border_color, self.theme.radius, 1.0);

        // Text
        let text_rect = Rect::new(rect.x + 8.0, rect.y, rect.w - 16.0, rect.h);
        self.paint_text(text_rect, text, 13.0, self.theme.text, TextAlign::Left);

        // Handle input
        let mut changed = false;
        if focused {
            if !self.input.text_input.is_empty() {
                text.push_str(&self.input.text_input);
                changed = true;
            }
            if self.input.key_backspace && !text.is_empty() {
                text.pop();
                changed = true;
            }
        }

        changed
    }

    // ── Dropdown ──

    pub fn dropdown(&mut self, id: u64, rect: Rect, items: &[&str], selected: &mut usize) -> bool {
        let hovered = self.is_hovered(&rect);
        let m = self.motion(id, hovered, hovered && self.is_mouse_down());

        let bg = mix(self.theme.secondary, self.theme.secondary_hover, m.hover);
        self.paint_filled_rect(rect, bg, self.theme.radius);
        self.paint_outline_rect(rect, self.theme.outline, self.theme.radius, 1.0);

        let label = if *selected < items.len() { items[*selected] } else { "" };
        let text_rect = Rect::new(rect.x + 8.0, rect.y, rect.w - 32.0, rect.h);
        self.paint_text(text_rect, label, 13.0, self.theme.text, TextAlign::Left);

        // Chevron
        let chevron_rect = Rect::new(rect.x + rect.w - 24.0, rect.y + (rect.h - 16.0) * 0.5, 16.0, 16.0);
        self.paint_chevron(chevron_rect, self.theme.muted_text, 90.0);

        // Simple click-to-cycle for now
        let mut changed = false;
        if hovered && self.is_mouse_released() && !items.is_empty() {
            *selected = (*selected + 1) % items.len();
            changed = true;
        }
        changed
    }

    // ── Tabs ──

    pub fn tab_bar(&mut self, id: u64, rect: Rect, labels: &[&str], selected: &mut usize) -> bool {
        let n = labels.len();
        if n == 0 {
            return false;
        }
        let tab_w = rect.w / n as f32;
        let mut changed = false;

        for (i, label) in labels.iter().enumerate() {
            let tab_rect = Rect::new(rect.x + i as f32 * tab_w, rect.y, tab_w, rect.h);
            let is_selected = i == *selected;
            let hovered = self.is_hovered(&tab_rect);
            let tab_id = context_hash_mix(id, i as u64);
            let m = self.motion(tab_id, hovered, hovered && self.is_mouse_down());

            let bg = if is_selected {
                mix(self.theme.primary, rgba(1.0, 1.0, 1.0, 1.0), m.hover * 0.1)
            } else {
                rgba(0.0, 0.0, 0.0, m.hover * 0.05)
            };
            let text_color = if is_selected { self.theme.primary_text } else { self.theme.muted_text };

            self.paint_filled_rect(tab_rect, bg, 0.0);
            self.paint_text(tab_rect, label, 13.0, text_color, TextAlign::Center);

            if hovered && self.is_mouse_released() && !is_selected {
                *selected = i;
                changed = true;
            }
        }

        changed
    }

    // ── Transform payload ──

    pub fn push_transform_3d(&mut self, transform: Transform3D) -> u32 {
        let idx = self.transform_payloads.len() as u32;
        self.transform_payloads.push(transform);
        idx
    }
}

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MotionResult {
    pub hover: f32,
    pub press: f32,
}
