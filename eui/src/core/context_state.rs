use crate::core::foundation::{FlexAlign, FlexLength};
use crate::rect::Rect;

pub const K_CONTEXT_INVALID_COMMAND_INDEX: usize = usize::MAX;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContextScopeKind {
    Card,
    DropdownBody,
    ScrollArea,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ContextRowState {
    pub active: bool,
    pub columns: i32,
    pub index: i32,
    pub next_span: i32,
    pub gap: f32,
    pub y: f32,
    pub max_height: f32,
}

impl Default for ContextRowState {
    fn default() -> Self {
        Self {
            active: false,
            columns: 1,
            index: 0,
            next_span: 1,
            gap: 8.0,
            y: 0.0,
            max_height: 0.0,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ContextWaterfallState {
    pub active: bool,
    pub columns: i32,
    pub gap: f32,
    pub start_y: f32,
    pub item_width: f32,
    pub column_y: Vec<f32>,
}

impl Default for ContextWaterfallState {
    fn default() -> Self {
        Self {
            active: false,
            columns: 1,
            gap: 8.0,
            start_y: 0.0,
            item_width: 0.0,
            column_y: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ContextFlexRowState {
    pub active: bool,
    pub items: Vec<FlexLength>,
    pub widths: Vec<f32>,
    pub heights: Vec<f32>,
    pub item_rects: Vec<Rect>,
    pub cmd_begin: Vec<usize>,
    pub cmd_end: Vec<usize>,
    pub index: i32,
    pub gap: f32,
    pub y: f32,
    pub row_height: f32,
    pub align: FlexAlign,
}

impl Default for ContextFlexRowState {
    fn default() -> Self {
        Self {
            active: false,
            items: Vec::new(),
            widths: Vec::new(),
            heights: Vec::new(),
            item_rects: Vec::new(),
            cmd_begin: Vec::new(),
            cmd_end: Vec::new(),
            index: 0,
            gap: 8.0,
            y: 0.0,
            row_height: 0.0,
            align: FlexAlign::Top,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ContextLayoutRectState {
    pub layout_rect: Rect,
    pub cursor_y: f32,
    pub last_item_rect: Rect,
    pub row: ContextRowState,
    pub flex_row: ContextFlexRowState,
    pub waterfall: ContextWaterfallState,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ContextTextAreaState {
    pub scroll: f32,
    pub preferred_x: f32,
    pub last_touched_frame: u64,
}

impl Default for ContextTextAreaState {
    fn default() -> Self {
        Self { scroll: 0.0, preferred_x: -1.0, last_touched_frame: 0 }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ContextScrollAreaState {
    pub scroll: f32,
    pub velocity: f32,
    pub content_height: f32,
    pub last_touched_frame: u64,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ContextMotionState {
    pub hover: f32,
    pub press: f32,
    pub focus: f32,
    pub active: f32,
    pub value: f32,
    pub initialized: bool,
    pub value_initialized: bool,
    pub last_touched_frame: u64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ContextGlowCommandRange {
    pub outer_cmd_index: usize,
    pub inner_cmd_index: usize,
    pub outer_spread: f32,
    pub inner_spread: f32,
}

impl Default for ContextGlowCommandRange {
    fn default() -> Self {
        Self {
            outer_cmd_index: K_CONTEXT_INVALID_COMMAND_INDEX,
            inner_cmd_index: K_CONTEXT_INVALID_COMMAND_INDEX,
            outer_spread: 0.0,
            inner_spread: 0.0,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ContextScopeState {
    pub kind: ContextScopeKind,
    pub content_x: f32,
    pub content_width: f32,
    pub layout_rect: Rect,
    pub cursor_y_after: f32,
    pub fill_cmd_index: usize,
    pub outline_cmd_index: usize,
    pub top_y: f32,
    pub min_height: f32,
    pub padding: f32,
    pub had_outer_row: bool,
    pub outer_row: ContextRowState,
    pub had_outer_flex_row: bool,
    pub outer_flex_row: ContextFlexRowState,
    pub outer_waterfall: ContextWaterfallState,
    pub in_waterfall: bool,
    pub column_index: i32,
    pub fixed_rect: Rect,
    pub lock_shell_rect: bool,
    pub scroll_state_id: u64,
    pub scroll_viewport: Rect,
    pub scroll_content_origin_y: f32,
    pub pushed_clip: bool,
    pub reveal: f32,
    pub glow_outer_cmd_index: usize,
    pub glow_inner_cmd_index: usize,
    pub glow_outer_spread: f32,
    pub glow_inner_spread: f32,
    pub content_cmd_begin: usize,
    pub show_content: bool,
}

impl Default for ContextScopeState {
    fn default() -> Self {
        Self {
            kind: ContextScopeKind::Card,
            content_x: 0.0,
            content_width: 0.0,
            layout_rect: Rect::default(),
            cursor_y_after: 0.0,
            fill_cmd_index: 0,
            outline_cmd_index: 0,
            top_y: 0.0,
            min_height: 0.0,
            padding: 0.0,
            had_outer_row: false,
            outer_row: ContextRowState::default(),
            had_outer_flex_row: false,
            outer_flex_row: ContextFlexRowState::default(),
            outer_waterfall: ContextWaterfallState::default(),
            in_waterfall: false,
            column_index: -1,
            fixed_rect: Rect::default(),
            lock_shell_rect: false,
            scroll_state_id: 0,
            scroll_viewport: Rect::default(),
            scroll_content_origin_y: 0.0,
            pushed_clip: false,
            reveal: 1.0,
            glow_outer_cmd_index: K_CONTEXT_INVALID_COMMAND_INDEX,
            glow_inner_cmd_index: K_CONTEXT_INVALID_COMMAND_INDEX,
            glow_outer_spread: 0.0,
            glow_inner_spread: 0.0,
            content_cmd_begin: 0,
            show_content: true,
        }
    }
}
