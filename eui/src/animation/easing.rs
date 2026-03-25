#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CubicBezier {
    pub x1: f32,
    pub y1: f32,
    pub x2: f32,
    pub y2: f32,
}

impl Default for CubicBezier {
    fn default() -> Self {
        Self { x1: 0.25, y1: 0.10, x2: 0.25, y2: 1.00 }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u8)]
pub enum EasingPreset {
    Linear,
    #[default]
    Ease,
    EaseIn,
    EaseOut,
    EaseInOut,
    SpringSoft,
}

pub fn preset_bezier(preset: EasingPreset) -> CubicBezier {
    match preset {
        EasingPreset::Linear => CubicBezier { x1: 0.0, y1: 0.0, x2: 1.0, y2: 1.0 },
        EasingPreset::Ease => CubicBezier { x1: 0.25, y1: 0.10, x2: 0.25, y2: 1.0 },
        EasingPreset::EaseIn => CubicBezier { x1: 0.42, y1: 0.0, x2: 1.0, y2: 1.0 },
        EasingPreset::EaseOut => CubicBezier { x1: 0.0, y1: 0.0, x2: 0.58, y2: 1.0 },
        EasingPreset::EaseInOut => CubicBezier { x1: 0.42, y1: 0.0, x2: 0.58, y2: 1.0 },
        EasingPreset::SpringSoft => CubicBezier { x1: 0.20, y1: 0.85, x2: 0.25, y2: 1.10 },
    }
}

fn cubic_bezier_component(p0: f32, p1: f32, p2: f32, p3: f32, t: f32) -> f32 {
    let inv = 1.0 - t;
    inv * inv * inv * p0 + 3.0 * inv * inv * t * p1 + 3.0 * inv * t * t * p2 + t * t * t * p3
}

fn cubic_bezier_derivative(p0: f32, p1: f32, p2: f32, p3: f32, t: f32) -> f32 {
    let inv = 1.0 - t;
    3.0 * inv * inv * (p1 - p0) + 6.0 * inv * t * (p2 - p1) + 3.0 * t * t * (p3 - p2)
}

pub fn sample_bezier_y(bezier: &CubicBezier, progress: f32) -> f32 {
    let progress = progress.clamp(0.0, 1.0);
    if progress <= 0.0 || progress >= 1.0 {
        return progress;
    }

    // Newton-Raphson
    let mut t = progress;
    for _ in 0..6 {
        let x = cubic_bezier_component(0.0, bezier.x1, bezier.x2, 1.0, t);
        let dx = cubic_bezier_derivative(0.0, bezier.x1, bezier.x2, 1.0, t);
        if dx.abs() < 1e-5 {
            break;
        }
        t -= (x - progress) / dx;
        t = t.clamp(0.0, 1.0);
    }

    // Bisection refinement
    let mut low = 0.0_f32;
    let mut high = 1.0_f32;
    for _ in 0..8 {
        let x = cubic_bezier_component(0.0, bezier.x1, bezier.x2, 1.0, t);
        if (x - progress).abs() < 1e-4 {
            break;
        }
        if x < progress {
            low = t;
        } else {
            high = t;
        }
        t = 0.5 * (low + high);
    }

    cubic_bezier_component(0.0, bezier.y1, bezier.y2, 1.0, t).clamp(0.0, 1.0)
}

pub fn ease_bezier(bezier: &CubicBezier, progress: f32) -> f32 {
    sample_bezier_y(bezier, progress)
}

pub fn ease(preset: EasingPreset, progress: f32) -> f32 {
    sample_bezier_y(&preset_bezier(preset), progress)
}
