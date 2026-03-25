use crate::graphics::transforms::Transform3D;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl Rect {
    pub const ZERO: Self = Self { x: 0.0, y: 0.0, w: 0.0, h: 0.0 };

    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self { x, y, w, h }
    }

    pub fn contains(&self, px: f32, py: f32) -> bool {
        px >= self.x && px <= self.x + self.w && py >= self.y && py <= self.y + self.h
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct SplitRects {
    pub first: Rect,
    pub second: Rect,
}

pub fn transform_3d_is_identity(t: &Transform3D) -> bool {
    t.translation_x.abs() <= 1e-6
        && t.translation_y.abs() <= 1e-6
        && t.translation_z.abs() <= 1e-6
        && (t.scale_x - 1.0).abs() <= 1e-6
        && (t.scale_y - 1.0).abs() <= 1e-6
        && (t.scale_z - 1.0).abs() <= 1e-6
        && t.rotation_x_deg.abs() <= 1e-6
        && t.rotation_y_deg.abs() <= 1e-6
        && t.rotation_z_deg.abs() <= 1e-6
        && t.origin_x.abs() <= 1e-6
        && t.origin_y.abs() <= 1e-6
        && t.origin_z.abs() <= 1e-6
        && t.perspective.abs() <= 1e-6
}
