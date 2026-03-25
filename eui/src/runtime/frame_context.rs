use crate::core::context::Context;
use crate::runtime::contracts::{FrameClock, WindowMetrics};

pub struct FrameContext<'a> {
    pub ui: &'a mut Context,
    pub clock: FrameClock,
    pub metrics: WindowMetrics,
    pub repaint_flag: Option<&'a mut bool>,
}

impl<'a> FrameContext<'a> {
    pub fn context(&mut self) -> &mut Context {
        self.ui
    }

    pub fn delta_seconds(&self) -> f64 {
        self.clock.delta_seconds
    }

    pub fn delta_seconds_f32(&self) -> f32 {
        self.clock.delta_seconds as f32
    }

    pub fn request_next_frame(&mut self) {
        if let Some(flag) = self.repaint_flag.as_mut() {
            **flag = true;
        }
    }

    pub fn window_metrics(&self) -> WindowMetrics {
        self.metrics
    }
}
