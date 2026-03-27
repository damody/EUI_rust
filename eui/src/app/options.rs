pub struct AppOptions {
    pub width: i32,
    pub height: i32,
    pub title: String,
    pub vsync: bool,
    pub continuous_render: bool,
    pub idle_wait_seconds: f64,
    pub max_fps: f64,
    pub enable_dirty_cache: bool,
    pub text_font_family: String,
    pub text_font_weight: i32,
    pub text_font_file: Option<String>,
    pub icon_font_family: String,
    pub icon_font_file: Option<String>,
    pub enable_icon_font_fallback: bool,
    /// Called after EUI rendering, before swap_buffers. Receives (gl, fb_width, fb_height).
    pub post_render: Option<Box<dyn FnMut(&glow::Context, f32, f32)>>,
}

impl Default for AppOptions {
    fn default() -> Self {
        Self {
            width: 1150,
            height: 820,
            title: "EUI App".to_string(),
            vsync: true,
            continuous_render: false,
            idle_wait_seconds: 0.25,
            max_fps: 60.0,
            enable_dirty_cache: true,
            text_font_family: "Segoe UI".to_string(),
            text_font_weight: 600,
            text_font_file: None,
            icon_font_family: "Font Awesome 7 Free Solid".to_string(),
            icon_font_file: Some("include/Font Awesome 7 Free-Solid-900.otf".to_string()),
            enable_icon_font_fallback: true,
            post_render: None,
        }
    }
}
