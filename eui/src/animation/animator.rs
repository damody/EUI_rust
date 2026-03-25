use crate::animation::easing::ease_bezier;
use crate::animation::timeline::TimelineClip;
use crate::graphics::transforms::{Transform2D, Transform3D};

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct TransformOrigin2D {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct TransformOrigin3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TransformAnimation2D {
    pub from: Transform2D,
    pub to: Transform2D,
    pub origin: TransformOrigin2D,
    pub clip: TimelineClip,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TransformAnimation3D {
    pub from: Transform3D,
    pub to: Transform3D,
    pub origin: TransformOrigin3D,
    pub clip: TimelineClip,
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct AnimatorState {
    pub frame_index: u64,
    pub now_seconds: f64,
    pub delta_seconds: f64,
}

pub fn lerp_scalar(from: f32, to: f32, t: f32) -> f32 {
    from + (to - from) * t.clamp(0.0, 1.0)
}

pub fn evaluate_timeline_progress(clip: &TimelineClip, elapsed_seconds: f32) -> f32 {
    let delayed = (elapsed_seconds - clip.delay_seconds.max(0.0)).max(0.0);
    let duration = clip.duration_seconds.max(1e-5);
    ease_bezier(&clip.easing, delayed / duration)
}

pub fn animate_scalar(clip: &TimelineClip, elapsed_seconds: f32) -> f32 {
    lerp_scalar(
        clip.scalar.from,
        clip.scalar.to,
        evaluate_timeline_progress(clip, elapsed_seconds),
    )
}

pub fn interpolate_transform_2d(from: &Transform2D, to: &Transform2D, t: f32) -> Transform2D {
    let t = t.clamp(0.0, 1.0);
    Transform2D {
        translation_x: lerp_scalar(from.translation_x, to.translation_x, t),
        translation_y: lerp_scalar(from.translation_y, to.translation_y, t),
        scale_x: lerp_scalar(from.scale_x, to.scale_x, t),
        scale_y: lerp_scalar(from.scale_y, to.scale_y, t),
        rotation_deg: lerp_scalar(from.rotation_deg, to.rotation_deg, t),
        origin_x: lerp_scalar(from.origin_x, to.origin_x, t),
        origin_y: lerp_scalar(from.origin_y, to.origin_y, t),
    }
}

pub fn interpolate_transform_3d(from: &Transform3D, to: &Transform3D, t: f32) -> Transform3D {
    let t = t.clamp(0.0, 1.0);
    Transform3D {
        translation_x: lerp_scalar(from.translation_x, to.translation_x, t),
        translation_y: lerp_scalar(from.translation_y, to.translation_y, t),
        translation_z: lerp_scalar(from.translation_z, to.translation_z, t),
        scale_x: lerp_scalar(from.scale_x, to.scale_x, t),
        scale_y: lerp_scalar(from.scale_y, to.scale_y, t),
        scale_z: lerp_scalar(from.scale_z, to.scale_z, t),
        rotation_x_deg: lerp_scalar(from.rotation_x_deg, to.rotation_x_deg, t),
        rotation_y_deg: lerp_scalar(from.rotation_y_deg, to.rotation_y_deg, t),
        rotation_z_deg: lerp_scalar(from.rotation_z_deg, to.rotation_z_deg, t),
        origin_x: lerp_scalar(from.origin_x, to.origin_x, t),
        origin_y: lerp_scalar(from.origin_y, to.origin_y, t),
        origin_z: lerp_scalar(from.origin_z, to.origin_z, t),
        perspective: lerp_scalar(from.perspective, to.perspective, t),
    }
}

pub fn animate_transform_2d(animation: &TransformAnimation2D, elapsed_seconds: f32) -> Transform2D {
    interpolate_transform_2d(
        &animation.from,
        &animation.to,
        evaluate_timeline_progress(&animation.clip, elapsed_seconds),
    )
}

pub fn animate_transform_3d(animation: &TransformAnimation3D, elapsed_seconds: f32) -> Transform3D {
    interpolate_transform_3d(
        &animation.from,
        &animation.to,
        evaluate_timeline_progress(&animation.clip, elapsed_seconds),
    )
}
