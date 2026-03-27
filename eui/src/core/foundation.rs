use crate::color::{brighten_primary_for_dark_mode, mix, rgba, Color};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FlexLength {
    Fixed(f32),
    Flex(f32),
    Content { min: f32, max: f32 },
}

impl Default for FlexLength {
    fn default() -> Self {
        FlexLength::Flex(1.0)
    }
}

pub fn px(v: f32) -> FlexLength {
    FlexLength::Fixed(v.max(0.0))
}

pub fn fr(w: f32) -> FlexLength {
    FlexLength::Flex(w.max(0.0))
}

pub fn fit(min_width: f32, max_width: f32) -> FlexLength {
    let min_w = min_width.max(0.0);
    let max_w = max_width.max(min_w);
    FlexLength::Content { min: min_w, max: max_w }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FlexAlign {
    #[default]
    Top,
    Center,
    Bottom,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ThemeMode {
    Light,
    #[default]
    Dark,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ButtonStyle {
    #[default]
    Primary,
    Secondary,
    Ghost,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Theme {
    pub background: Color,
    pub panel: Color,
    pub panel_border: Color,
    pub text: Color,
    pub muted_text: Color,
    pub primary: Color,
    pub primary_text: Color,
    pub secondary: Color,
    pub secondary_hover: Color,
    pub secondary_active: Color,
    pub track: Color,
    pub track_fill: Color,
    pub outline: Color,
    pub input_bg: Color,
    pub input_border: Color,
    pub focus_ring: Color,
    pub radius: f32,
}

impl Default for Theme {
    fn default() -> Self {
        make_theme(ThemeMode::Dark, &rgba(0.25, 0.52, 0.96, 1.0))
    }
}

pub fn make_theme(mode: ThemeMode, primary: &Color) -> Theme {
    let primary_color = if mode == ThemeMode::Dark {
        brighten_primary_for_dark_mode(primary)
    } else {
        *primary
    };

    if mode == ThemeMode::Dark {
        let background = rgba(0.05, 0.07, 0.10, 1.0);
        let panel = rgba(0.09, 0.12, 0.16, 1.0);
        let panel_border = rgba(0.18, 0.23, 0.30, 1.0);
        let text = rgba(0.92, 0.95, 0.98, 1.0);
        let muted_text = rgba(0.63, 0.70, 0.79, 1.0);
        let primary_text = rgba(0.06, 0.10, 0.17, 1.0);
        let secondary = rgba(0.15, 0.20, 0.27, 1.0);
        let secondary_hover = mix(secondary, primary_color, 0.18);
        let secondary_active = mix(secondary, primary_color, 0.32);
        let track = rgba(0.18, 0.23, 0.31, 1.0);
        let track_fill = primary_color;
        let outline = rgba(0.25, 0.31, 0.40, 1.0);
        let input_bg = rgba(0.08, 0.11, 0.15, 1.0);
        let input_border = mix(rgba(0.26, 0.33, 0.42, 1.0), primary_color, 0.20);
        let focus_ring = mix(primary_color, rgba(1.0, 1.0, 1.0, 1.0), 0.18);

        Theme {
            background, panel, panel_border, text, muted_text,
            primary: primary_color, primary_text, secondary,
            secondary_hover, secondary_active, track, track_fill,
            outline, input_bg, input_border, focus_ring, radius: 8.0,
        }
    } else {
        let background = rgba(0.96, 0.97, 0.99, 1.0);
        let panel = rgba(1.0, 1.0, 1.0, 1.0);
        let panel_border = rgba(0.84, 0.88, 0.93, 1.0);
        let text = rgba(0.11, 0.15, 0.22, 1.0);
        let muted_text = rgba(0.41, 0.47, 0.58, 1.0);
        let primary_text = rgba(0.96, 0.98, 1.0, 1.0);
        let secondary = rgba(0.92, 0.94, 0.97, 1.0);
        let secondary_hover = mix(secondary, primary_color, 0.12);
        let secondary_active = mix(secondary, primary_color, 0.24);
        let track = rgba(0.90, 0.92, 0.96, 1.0);
        let track_fill = primary_color;
        let outline = rgba(0.80, 0.85, 0.92, 1.0);
        let input_bg = rgba(1.0, 1.0, 1.0, 1.0);
        let input_border = mix(rgba(0.79, 0.84, 0.91, 1.0), primary_color, 0.28);
        let focus_ring = mix(primary_color, rgba(1.0, 1.0, 1.0, 1.0), 0.10);

        Theme {
            background, panel, panel_border, text, muted_text,
            primary: primary_color, primary_text, secondary,
            secondary_hover, secondary_active, track, track_fill,
            outline, input_bg, input_border, focus_ring, radius: 8.0,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct InputState {
    pub mouse_x: f32,
    pub mouse_y: f32,
    pub mouse_wheel_y: f32,
    pub mouse_down: bool,
    pub mouse_pressed: bool,
    pub mouse_released: bool,
    pub mouse_right_down: bool,
    pub mouse_right_pressed: bool,
    pub mouse_right_released: bool,
    pub key_backspace: bool,
    pub key_delete: bool,
    pub key_enter: bool,
    pub key_escape: bool,
    pub key_left: bool,
    pub key_right: bool,
    pub key_up: bool,
    pub key_down: bool,
    pub key_home: bool,
    pub key_end: bool,
    pub key_select_all: bool,
    pub key_copy: bool,
    pub key_cut: bool,
    pub key_paste: bool,
    pub key_shift: bool,
    pub key_w: bool,
    pub key_a: bool,
    pub key_s: bool,
    pub key_d: bool,
    pub text_input: String,
    pub clipboard_text: String,
    pub clipboard_out: String,
    pub time_seconds: f64,
    pub dropped_files: Vec<std::path::PathBuf>,
    pub title_request: Option<String>,
}
