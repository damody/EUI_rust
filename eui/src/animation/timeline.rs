use crate::animation::easing::CubicBezier;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u8)]
pub enum PropertyKind {
    Opacity,
    Color,
    Position,
    Size,
    Radius,
    Blur,
    Shadow,
    Transform2D,
    Transform3D,
    #[default]
    CustomScalar,
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct ScalarTrack {
    pub from: f32,
    pub to: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TimelineClip {
    pub id: String,
    pub property: PropertyKind,
    pub scalar: ScalarTrack,
    pub duration_seconds: f32,
    pub delay_seconds: f32,
    pub easing: CubicBezier,
}

impl Default for TimelineClip {
    fn default() -> Self {
        Self {
            id: String::new(),
            property: PropertyKind::CustomScalar,
            scalar: ScalarTrack::default(),
            duration_seconds: 0.2,
            delay_seconds: 0.0,
            easing: CubicBezier::default(),
        }
    }
}
