#![allow(clippy::missing_safety_doc)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::new_without_default)]
#![allow(clippy::type_complexity)]

pub mod color;
pub mod rect;
pub mod math;
pub mod graphics;
pub mod core;
pub mod animation;
pub mod text;
pub mod runtime;
pub mod renderer;
pub mod quick;
pub mod platform;
pub mod app;

// Re-export glow so downstream crates use the same version
pub use glow;

// Re-exports for convenience
pub use color::{rgba, rgb, mix, Color};
pub use rect::{Rect, SplitRects};
pub use core::foundation::{FlexLength, FlexAlign, Theme, ThemeMode, ButtonStyle, InputState, px, fr, fit, make_theme};
pub use core::draw_command::{TextAlign, CommandType, DrawCommand};
pub use core::context::Context;
pub use graphics::effects::{Brush, Stroke, Shadow, Blur, GfxColor, BrushKind, Point, ColorStop};
pub use graphics::transforms::{Transform2D, Transform3D};
pub use graphics::primitives::{CornerRadius, ClipRect, ImageFit, RectanglePrimitive, ImagePrimitive, IconPrimitive};
pub use animation::easing::{CubicBezier, EasingPreset, ease, ease_bezier, sample_bezier_y};
pub use animation::timeline::{TimelineClip, ScalarTrack, PropertyKind};
pub use animation::animator::{lerp_scalar, animate_scalar, interpolate_transform_2d, interpolate_transform_3d};
pub use quick::ui::UI;
pub use quick::gfx;
pub use quick::builders;
pub use quick::anchor;
pub use app::options::AppOptions;
pub use app::run::{run, run_with_options};
pub use runtime::contracts::{WindowMetrics, FrameClock};
pub use renderer::contracts::{ClearState, DrawDataView, RendererBackend};
