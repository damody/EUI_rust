use crate::rect::Rect;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u8)]
pub enum AnchorUnit {
    #[default]
    Auto,
    Pixels,
    Percent,
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct AnchorValue {
    pub unit: AnchorUnit,
    pub value: f32,
}

impl AnchorValue {
    pub fn auto_val() -> Self {
        Self::default()
    }
    pub fn px(v: f32) -> Self {
        Self { unit: AnchorUnit::Pixels, value: v }
    }
    pub fn percent(v: f32) -> Self {
        Self { unit: AnchorUnit::Percent, value: v }
    }
    pub fn is_set(&self) -> bool {
        self.unit != AnchorUnit::Auto
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u8)]
pub enum AnchorReference {
    #[default]
    Parent,
    LastItem,
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct AnchorRect {
    pub left: AnchorValue,
    pub top: AnchorValue,
    pub right: AnchorValue,
    pub bottom: AnchorValue,
    pub center_x: AnchorValue,
    pub center_y: AnchorValue,
    pub width: AnchorValue,
    pub height: AnchorValue,
    pub reference: AnchorReference,
}

fn normalize_percent(value: f32) -> f32 {
    if value.abs() > 1.0 { value * 0.01 } else { value }
}

fn resolve_value_px(val: &AnchorValue, span: f32) -> f32 {
    match val.unit {
        AnchorUnit::Pixels => val.value,
        AnchorUnit::Percent => normalize_percent(val.value) * span,
        AnchorUnit::Auto => 0.0,
    }
}

pub fn resolve_anchor_rect(anchor: &AnchorRect, reference: &Rect) -> Rect {
    let has_left = anchor.left.is_set();
    let has_right = anchor.right.is_set();
    let has_top = anchor.top.is_set();
    let has_bottom = anchor.bottom.is_set();
    let has_width = anchor.width.is_set();
    let has_height = anchor.height.is_set();
    let has_cx = anchor.center_x.is_set();
    let has_cy = anchor.center_y.is_set();

    let left = resolve_value_px(&anchor.left, reference.w);
    let right = resolve_value_px(&anchor.right, reference.w);
    let top = resolve_value_px(&anchor.top, reference.h);
    let bottom = resolve_value_px(&anchor.bottom, reference.h);
    let width = resolve_value_px(&anchor.width, reference.w);
    let height = resolve_value_px(&anchor.height, reference.h);
    let cx = resolve_value_px(&anchor.center_x, reference.w);
    let cy = resolve_value_px(&anchor.center_y, reference.h);

    let w = if has_width {
        width.max(0.0)
    } else if has_left && has_right {
        (reference.w - left - right).max(0.0)
    } else {
        reference.w
    };

    let h = if has_height {
        height.max(0.0)
    } else if has_top && has_bottom {
        (reference.h - top - bottom).max(0.0)
    } else {
        reference.h
    };

    let x = if has_left {
        reference.x + left
    } else if has_right {
        reference.x + reference.w - right - w
    } else if has_cx {
        reference.x + reference.w * 0.5 + cx - w * 0.5
    } else {
        reference.x
    };

    let y = if has_top {
        reference.y + top
    } else if has_bottom {
        reference.y + reference.h - bottom - h
    } else if has_cy {
        reference.y + reference.h * 0.5 + cy - h * 0.5
    } else {
        reference.y
    };

    Rect { x, y, w: w.max(0.0), h: h.max(0.0) }
}
