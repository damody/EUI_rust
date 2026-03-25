use crate::graphics::transforms::{Transform2D, Transform3D};
use crate::rect::{transform_3d_is_identity, Rect};

const DEG_TO_RAD: f32 = 0.017_453_292;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct ProjectedPoint {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub fn rotate_point_x(p: ProjectedPoint, radians: f32) -> ProjectedPoint {
    let c = radians.cos();
    let s = radians.sin();
    ProjectedPoint {
        x: p.x,
        y: p.y * c - p.z * s,
        z: p.y * s + p.z * c,
    }
}

pub fn rotate_point_y(p: ProjectedPoint, radians: f32) -> ProjectedPoint {
    let c = radians.cos();
    let s = radians.sin();
    ProjectedPoint {
        x: p.x * c + p.z * s,
        y: p.y,
        z: -p.x * s + p.z * c,
    }
}

pub fn rotate_point_z(p: ProjectedPoint, radians: f32) -> ProjectedPoint {
    let c = radians.cos();
    let s = radians.sin();
    ProjectedPoint {
        x: p.x * c - p.y * s,
        y: p.x * s + p.y * c,
        z: p.z,
    }
}

pub fn project_rect_point_3d(x: f32, y: f32, rect: &Rect, transform: &Transform3D) -> ProjectedPoint {
    let pivot_x = rect.x + transform.origin_x;
    let pivot_y = rect.y + transform.origin_y;
    let mut point = ProjectedPoint {
        x: (x - pivot_x) * transform.scale_x.max(0.0),
        y: (y - pivot_y) * transform.scale_y.max(0.0),
        z: -transform.origin_z * transform.scale_z.max(0.0),
    };

    point = rotate_point_x(point, transform.rotation_x_deg * DEG_TO_RAD);
    point = rotate_point_y(point, transform.rotation_y_deg * DEG_TO_RAD);
    point = rotate_point_z(point, transform.rotation_z_deg * DEG_TO_RAD);
    point.z += transform.translation_z;

    let perspective = transform.perspective.max(0.0);
    let factor = if perspective > 1e-4 {
        perspective / (perspective - point.z).max(32.0)
    } else {
        1.0
    };

    ProjectedPoint {
        x: pivot_x + transform.translation_x + point.x * factor,
        y: pivot_y + transform.translation_y + point.y * factor,
        z: point.z,
    }
}

pub fn projected_rect_bounds(rect: &Rect, transform: &Transform3D) -> Rect {
    if transform_3d_is_identity(transform) {
        return *rect;
    }

    let p0 = project_rect_point_3d(rect.x, rect.y, rect, transform);
    let p1 = project_rect_point_3d(rect.x + rect.w, rect.y, rect, transform);
    let p2 = project_rect_point_3d(rect.x + rect.w, rect.y + rect.h, rect, transform);
    let p3 = project_rect_point_3d(rect.x, rect.y + rect.h, rect, transform);
    let min_x = p0.x.min(p1.x).min(p2.x).min(p3.x);
    let min_y = p0.y.min(p1.y).min(p2.y).min(p3.y);
    let max_x = p0.x.max(p1.x).max(p2.x).max(p3.x);
    let max_y = p0.y.max(p1.y).max(p2.y).max(p3.y);
    Rect {
        x: min_x,
        y: min_y,
        w: (max_x - min_x).max(0.0),
        h: (max_y - min_y).max(0.0),
    }
}

pub fn apply_rect_transform_2d(rect: &Rect, transform: &Transform2D) -> Rect {
    let scale_x = transform.scale_x.max(0.0);
    let scale_y = transform.scale_y.max(0.0);
    let pivot_x = rect.x + transform.origin_x;
    let pivot_y = rect.y + transform.origin_y;

    let new_x = pivot_x + (rect.x - pivot_x) * scale_x + transform.translation_x;
    let new_y = pivot_y + (rect.y - pivot_y) * scale_y + transform.translation_y;
    let new_w = rect.w * scale_x;
    let new_h = rect.h * scale_y;

    Rect { x: new_x, y: new_y, w: new_w.max(0.0), h: new_h.max(0.0) }
}

pub fn apply_rect_transform_3d_fallback(rect: &Rect, transform: &Transform3D) -> Rect {
    let scale_x = transform.scale_x.max(0.0);
    let scale_y = transform.scale_y.max(0.0);
    let pivot_x = rect.x + transform.origin_x;
    let pivot_y = rect.y + transform.origin_y;

    let rot_x = transform.rotation_x_deg * DEG_TO_RAD;
    let rot_y = transform.rotation_y_deg * DEG_TO_RAD;
    let perspective = transform.perspective.max(160.0);
    let depth_scale = (1.0 + transform.translation_z / perspective).clamp(0.72, 1.35);
    let tilt_scale_x = (1.0 - rot_y.sin().abs() * 0.32).clamp(0.58, 1.0);
    let tilt_scale_y = (1.0 - rot_x.sin().abs() * 0.26).clamp(0.62, 1.0);
    let final_scale_x = scale_x * depth_scale * tilt_scale_x;
    let final_scale_y = scale_y * depth_scale * tilt_scale_y;

    let tilt_offset_x = rot_y.sin() * rect.h * 0.16 + transform.translation_z * rot_y.sin() * 0.04;
    let tilt_offset_y = -rot_x.sin() * rect.w * 0.12 - transform.translation_z * rot_x.sin() * 0.03;

    let new_x = pivot_x + (rect.x - pivot_x) * final_scale_x + transform.translation_x + tilt_offset_x;
    let new_y = pivot_y + (rect.y - pivot_y) * final_scale_y + transform.translation_y + tilt_offset_y;
    let new_w = rect.w * final_scale_x;
    let new_h = rect.h * final_scale_y;

    Rect { x: new_x, y: new_y, w: new_w.max(0.0), h: new_h.max(0.0) }
}
