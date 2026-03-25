#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WindowMetrics {
    pub window_w: i32,
    pub window_h: i32,
    pub framebuffer_w: i32,
    pub framebuffer_h: i32,
    pub dpi_scale_x: f32,
    pub dpi_scale_y: f32,
    pub dpi_scale: f32,
}

impl Default for WindowMetrics {
    fn default() -> Self {
        Self {
            window_w: 1,
            window_h: 1,
            framebuffer_w: 1,
            framebuffer_h: 1,
            dpi_scale_x: 1.0,
            dpi_scale_y: 1.0,
            dpi_scale: 1.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct FrameClock {
    pub frame_index: u64,
    pub now_seconds: f64,
    pub delta_seconds: f64,
}

pub trait PlatformBackend {
    fn should_close(&self) -> bool;
    fn poll_events(&mut self, blocking: bool, timeout_seconds: f64);
    fn query_metrics(&self) -> WindowMetrics;
    fn get_clipboard_text(&mut self) -> String;
    fn set_clipboard_text(&mut self, text: &str);
    fn now_seconds(&self) -> f64;
}
