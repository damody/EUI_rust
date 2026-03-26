use eui::*;
use eui::quick::ui::*;
use eui::quick::layouts::*;
use eui::core::context_utils::{context_scale_rect_from_center, context_translate_rect};

// ── State ──

#[allow(dead_code)]
struct GalleryState {
    selected_page: usize,
    selected_animation_demo: usize,
    selected_control_icon: usize,
    control_slider: f32,
    progress_ratio: f32,
    layout_gap: f32,
    layout_radius: f32,
    settings_blur: f32,
    light_mode: bool,
    controls_dropdown_open: bool,
    design_icon_library_open: bool,
    controls_mode: usize,
    design_icon_page: usize,
    accent_index: usize,
    custom_accent_r: f32,
    custom_accent_g: f32,
    custom_accent_b: f32,
    controls_multi_select: [bool; 3],
    search_text: String,
    notes_text: String,
}

impl Default for GalleryState {
    fn default() -> Self {
        Self {
            selected_page: 0,
            selected_animation_demo: 0,
            selected_control_icon: 0,
            control_slider: 0.64,
            progress_ratio: 0.72,
            layout_gap: 18.0,
            layout_radius: 18.0,
            settings_blur: 18.0,
            light_mode: false,
            controls_dropdown_open: false,
            design_icon_library_open: false,
            controls_mode: 0,
            design_icon_page: 0,
            accent_index: 0,
            custom_accent_r: 96.0,
            custom_accent_g: 165.0,
            custom_accent_b: 250.0,
            controls_multi_select: [true, false, true],
            search_text: String::from("Search gallery"),
            notes_text: String::from("Panels, text, blur and motion all run through the same immediate-mode renderer.\n\n\
                Use this editor to test multiline input, line wrapping and longer form content while you tune the surrounding layout.\n\n\
                The gallery is meant to double as a developer reference, so the default text is intentionally longer than a placeholder.\n\n\
                Try editing this copy, pasting multiple paragraphs, and scrolling through the document while sliders and theme settings update around it.\n\n\
                A healthy default body here is useful because developers can immediately validate caret movement, selection painting, wheel scrolling, clipboard flow and long-form wrapping without seeding data first.\n\n\
                This extra block is only here to guarantee the scroll bar stays active during demo startup."),
        }
    }
}

// ── Palette ──

#[derive(Clone, Copy)]
struct GalleryPalette {
    light: bool,
    shell_top: u32,
    shell_bottom: u32,
    surface: u32,
    surface_alt: u32,
    surface_deep: u32,
    border: u32,
    border_soft: u32,
    text: u32,
    muted: u32,
    accent: u32,
    accent_soft: u32,
    grid: u32,
}

// ── Constants ──

const PAGE_NAMES: [&str; 8] = [
    "Basic Controls", "Design", "Layout", "Animation",
    "Dashboard Example", "Settings", "Image", "About",
];
const PAGE_SUMMARIES: [&str; 8] = [
    "Buttons, sliders, inputs and text surfaces.",
    "Typography, icon ids and palette tokens for fast UI work.",
    "Declarative row, column, grid and stack composition.",
    "Motion lives here instead of in the main sidebar.",
    "A dashboard-style composition preview.",
    "Theme mode, accent color and gallery tuning.",
    "Image widgets, textured fills and fit modes.",
    "Project identity, repository link and contact details.",
];
const PAGE_API_LABELS: [&str; 8] = [
    "buttons + inputs", "fonts + icons + colors", "row() + column() + grid()",
    "motion showcase", "dashboard preview", "theme + accent",
    "ui.image() + fill_image()", "project + license",
];

const DEMO_NAMES: [&str; 9] = [
    "Translate", "Scale", "Rotate", "Arc Motion", "Bezier Path",
    "Color Shift", "Opacity", "Blur", "Combo",
];
const DEMO_SUMMARIES: [&str; 9] = [
    "Move a rounded rect on one axis",
    "Grow and shrink around the center",
    "Rotate a single rounded rect in place",
    "Move on a curved arc instead of a line",
    "Follow a cubic Bezier trajectory",
    "Crossfade the actor between two palettes",
    "Fade between solid and ghosted states",
    "Animate backdrop blur on the actor",
    "Translation, scale, rotate, color, opacity and blur",
];
const DEMO_API_LABELS: [&str; 9] = [
    "translate() + ease_in_out", "scale() + eased scalar", "rotate() + linear loop",
    "curve path + ease_out", "Bezier path + custom cubic", "gradient() + color lerp",
    "opacity() + ease_in_out", "blur() + eased scalar", "combined channels",
];

const ACCENT_HEXES: [u32; 6] = [0x60A5FA, 0x22C55E, 0xF97316, 0xA855F7, 0xEAB308, 0x14B8A6];

const PAGE_ICONS: [u32; 8] = [0xF1DE, 0xF1FC, 0xF0DB, 0xF061, 0xF080, 0xF013, 0xF03E, 0xF05A];

const ICON_LIBRARY: [(u32, &str, &str); 12] = [
    (0xF002, "Search", "Query / filter"),
    (0xF004, "Heart", "Favorite / like"),
    (0xF005, "Star", "Rating / featured"),
    (0xF007, "User", "Profile / identity"),
    (0xF015, "Home", "Landing / dashboard"),
    (0xF017, "Clock", "Time / recent"),
    (0xF030, "Camera", "Capture / media"),
    (0xF03E, "Image", "Artwork / preview"),
    (0xF03D, "Video", "Playback / motion"),
    (0xF00C, "Check", "Confirm / success"),
    (0xF00D, "Close", "Dismiss / remove"),
    (0xF061, "Arrow", "Forward / next"),
];

// ── Helpers ──

fn color_from_hex(hex: u32, alpha: f32) -> Color {
    rgb_hex(hex, alpha)
}

fn mix_color(a: Color, b: Color, t: f32) -> Color {
    let t = t.clamp(0.0, 1.0);
    Color::new(
        a.r + (b.r - a.r) * t,
        a.g + (b.g - a.g) * t,
        a.b + (b.b - a.b) * t,
        a.a + (b.a - a.a) * t,
    )
}

fn mix_hex(a: u32, b: u32, t: f32) -> u32 {
    let t = t.clamp(0.0, 1.0);
    let mix_ch = |shift: u32| -> u32 {
        let va = ((a >> shift) & 0xff) as f32;
        let vb = ((b >> shift) & 0xff) as f32;
        (va + (vb - va) * t).round() as u32
    };
    (mix_ch(16) << 16) | (mix_ch(8) << 8) | mix_ch(0)
}

fn pack_rgb_hex(r: f32, g: f32, b: f32) -> u32 {
    let to_u8 = |v: f32| -> u32 { v.clamp(0.0, 255.0).round() as u32 };
    (to_u8(r) << 16) | (to_u8(g) << 8) | to_u8(b)
}


fn format_pixels(value: f32) -> String {
    format!("{:.0} px", value)
}

fn format_hex_color(hex: u32) -> String {
    format!("0x{:06X}", hex & 0xFFFFFF)
}

fn format_codepoint(cp: u32) -> String {
    format!("0x{:04X}", cp & 0xFFFF)
}

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t.clamp(0.0, 1.0)
}

fn inset_rect(r: &Rect, px: f32, py: f32) -> Rect {
    Rect::new(r.x + px, r.y + py, (r.w - px * 2.0).max(0.0), (r.h - py * 2.0).max(0.0))
}

fn loop_progress(time: f64, duration: f32) -> f32 {
    let d = duration.max(1e-4);
    let mut phase = (time as f32) % d;
    if phase < 0.0 { phase += d; }
    phase / d
}

fn timeline_ping_pong(time: f64, duration: f32, preset: EasingPreset) -> f32 {
    let d = duration.max(1e-4);
    let total = d * 2.0;
    let looped = loop_progress(time, total) * total;
    let t = if looped <= d { looped / d } else { 1.0 - (looped - d) / d };
    ease(preset, t.clamp(0.0, 1.0))
}

fn cubic_bezier_ease(x1: f32, y1: f32, x2: f32, y2: f32, t: f32) -> f32 {
    // Attempt to solve for the parameter u such that bezier_x(u) == t
    let mut u = t;
    for _ in 0..8 {
        let bx = 3.0 * (1.0 - u) * (1.0 - u) * u * x1
            + 3.0 * (1.0 - u) * u * u * x2
            + u * u * u;
        let dx = 3.0 * (1.0 - u) * (1.0 - u) * x1
            + 6.0 * (1.0 - u) * u * (x2 - x1)
            + 3.0 * u * u * (1.0 - x2);
        if dx.abs() < 1e-6 { break; }
        u -= (bx - t) / dx;
        u = u.clamp(0.0, 1.0);
    }
    3.0 * (1.0 - u) * (1.0 - u) * u * y1
        + 3.0 * (1.0 - u) * u * u * y2
        + u * u * u
}

fn cubic_bezier_component(p0: f32, p1: f32, p2: f32, p3: f32, t: f32) -> f32 {
    let inv = 1.0 - t;
    inv * inv * inv * p0 + 3.0 * inv * inv * t * p1 + 3.0 * inv * t * t * p2 + t * t * t * p3
}

fn cubic_bezier_point(p0: (f32, f32), p1: (f32, f32), p2: (f32, f32), p3: (f32, f32), t: f32) -> (f32, f32) {
    let t = t.clamp(0.0, 1.0);
    (
        cubic_bezier_component(p0.0, p1.0, p2.0, p3.0, t),
        cubic_bezier_component(p0.1, p1.1, p2.1, p3.1, t),
    )
}

fn font_display(s: f32) -> f32 { 22.0 * s }
fn font_heading(s: f32) -> f32 { 16.0 * s }
fn font_body(s: f32) -> f32 { 14.0 * s }
fn font_meta(s: f32) -> f32 { 12.2 * s }

fn custom_accent_slot() -> usize { ACCENT_HEXES.len() }

fn accent_uses_custom(state: &GalleryState) -> bool {
    state.accent_index >= custom_accent_slot()
}

fn custom_accent_hex(state: &GalleryState) -> u32 {
    pack_rgb_hex(state.custom_accent_r, state.custom_accent_g, state.custom_accent_b)
}

fn accent_hex(state: &GalleryState) -> u32 {
    if accent_uses_custom(state) {
        custom_accent_hex(state)
    } else {
        ACCENT_HEXES[state.accent_index.min(ACCENT_HEXES.len() - 1)]
    }
}

fn make_gallery_palette(state: &GalleryState) -> GalleryPalette {
    let accent = accent_hex(state);
    if state.light_mode {
        GalleryPalette {
            light: true,
            shell_top: 0xEEF4FA, shell_bottom: 0xE1EBF5,
            surface: 0xFBFDFF, surface_alt: 0xF3F8FC, surface_deep: 0xEAF1F7,
            border: 0xC8D5E1, border_soft: 0xD6E0E8,
            text: 0x102033, muted: 0x5F738A,
            accent, accent_soft: mix_hex(0xF3F8FC, accent, 0.20), grid: 0xD3DEE7,
        }
    } else {
        GalleryPalette {
            light: false,
            shell_top: 0x060D17, shell_bottom: 0x0C1625,
            surface: 0x07111C, surface_alt: 0x09131F, surface_deep: 0x0F1B2A,
            border: 0x20324A, border_soft: 0x26364E,
            text: 0xF5FAFF, muted: 0x8EA0BA,
            accent, accent_soft: mix_hex(0x0E1A2A, accent, 0.18), grid: 0x203247,
        }
    }
}

// ── Palette-derived colors ──

fn nav_selected_fill(p: &GalleryPalette) -> u32 {
    mix_hex(p.surface_deep, p.accent, if p.light { 0.18 } else { 0.24 })
}
fn nav_idle_fill(p: &GalleryPalette) -> u32 {
    if p.light { mix_hex(p.surface_deep, p.border, 0.18) } else { p.surface_alt }
}
fn actor_primary_top(p: &GalleryPalette) -> u32 {
    if p.light { mix_hex(0xFFFFFF, p.accent, 0.34) } else { mix_hex(0x72B2FF, p.accent, 0.18) }
}
fn actor_primary_bottom(p: &GalleryPalette) -> u32 {
    if p.light { mix_hex(0xDCEAFE, p.accent, 0.76) } else { mix_hex(0x2563EB, p.accent, 0.60) }
}
fn actor_focus_top(p: &GalleryPalette) -> u32 {
    if p.light { mix_hex(0xF8FCFF, p.accent, 0.22) } else { mix_hex(0xA7E2FF, p.accent, 0.18) }
}
fn actor_focus_bottom(p: &GalleryPalette) -> u32 {
    if p.light { mix_hex(0xE2EEFF, p.accent, 0.58) } else { mix_hex(0x2563EB, p.accent, 0.72) }
}
fn actor_glass_top(p: &GalleryPalette) -> u32 {
    if p.light { 0xFFFFFF } else { 0xF8FBFF }
}
fn actor_glass_bottom(p: &GalleryPalette) -> u32 {
    if p.light { mix_hex(0xEEF5FD, p.accent, 0.22) } else { mix_hex(0xD7EAFE, p.accent, 0.16) }
}
fn actor_stroke(p: &GalleryPalette) -> u32 {
    if p.light { mix_hex(0xD7E6F7, p.accent, 0.18) } else { mix_hex(0xDBECFF, p.accent, 0.16) }
}
fn actor_outline(p: &GalleryPalette) -> u32 {
    mix_hex(p.border_soft, p.accent, if p.light { 0.24 } else { 0.18 })
}
fn demo_track(p: &GalleryPalette) -> u32 {
    mix_hex(p.surface_deep, p.accent, if p.light { 0.12 } else { 0.18 })
}
fn demo_axis(p: &GalleryPalette) -> u32 {
    mix_hex(p.border, p.accent, if p.light { 0.18 } else { 0.24 })
}
fn demo_curve(p: &GalleryPalette) -> u32 {
    mix_hex(p.border_soft, p.accent, if p.light { 0.42 } else { 0.22 })
}
fn demo_curve_soft(p: &GalleryPalette) -> u32 {
    mix_hex(p.border_soft, p.accent, if p.light { 0.34 } else { 0.16 })
}
fn demo_anchor(p: &GalleryPalette) -> u32 {
    if p.light { mix_hex(0xFFFFFF, p.accent, 0.32) } else { mix_hex(0x7DD3FC, p.accent, 0.22) }
}
fn demo_anchor_end(p: &GalleryPalette) -> u32 {
    mix_hex(p.accent, 0x38BDF8, if p.light { 0.24 } else { 0.52 })
}
fn demo_anchor_muted(p: &GalleryPalette) -> u32 {
    mix_hex(p.surface_deep, p.text, if p.light { 0.18 } else { 0.28 })
}
fn preview_backdrop(p: &GalleryPalette) -> u32 {
    mix_hex(p.surface_deep, p.accent, if p.light { 0.10 } else { 0.06 })
}
fn blur_reference_hexes(p: &GalleryPalette) -> [u32; 5] {
    [
        mix_hex(p.accent, 0x22C55E, if p.light { 0.34 } else { 0.16 }),
        mix_hex(p.accent, 0xF97316, if p.light { 0.56 } else { 0.74 }),
        mix_hex(p.accent, 0xA855F7, if p.light { 0.62 } else { 0.76 }),
        mix_hex(p.accent, 0x38BDF8, if p.light { 0.36 } else { 0.58 }),
        mix_hex(p.accent, 0xEAB308, if p.light { 0.58 } else { 0.72 }),
    ]
}

fn panel_shadow_hex(p: &GalleryPalette) -> u32 {
    if p.light { 0x93A8BC } else { 0x020617 }
}

// ── Drawing helpers ──

fn draw_shadow(ctx: &mut Context, r: Rect, radius: f32, offset_y: f32, blur: f32, hex: u32, alpha: f32) {
    let shadow = Shadow {
        offset_x: 0.0,
        offset_y,
        blur_radius: blur,
        spread: 0.0,
        color: GfxColor::from(color_from_hex(hex, 1.0)),
    };
    eui::quick::primitive_painter::paint_shadow_approx(ctx, &r, radius, &shadow, alpha);
}

fn draw_fill(ctx: &mut Context, r: Rect, hex: u32, radius: f32, alpha: f32) {
    ctx.paint_filled_rect(r, color_from_hex(hex, alpha), radius);
}

fn draw_gradient(ctx: &mut Context, r: Rect, top_hex: u32, bottom_hex: u32, radius: f32, alpha: f32) {
    let top_c = color_from_hex(top_hex, alpha);
    let bottom_c = color_from_hex(bottom_hex, alpha);
    // Use normalized coordinates (0..1) matching C++ gfx::vertical_gradient
    let brush = Brush {
        kind: BrushKind::LinearGradient,
        solid: GfxColor::from(top_c),
        linear: eui::graphics::effects::LinearGradient {
            start: Point { x: 0.0, y: 0.0 },
            end: Point { x: 0.0, y: 1.0 },
            stops: [
                ColorStop { position: 0.0, color: GfxColor::from(top_c) },
                ColorStop { position: 1.0, color: GfxColor::from(bottom_c) },
                ColorStop::default(),
                ColorStop::default(),
            ],
            stop_count: 2,
        },
        radial: eui::graphics::effects::RadialGradient::default(),
    };
    ctx.paint_filled_rect_with_brush(r, brush, radius);
}

fn draw_stroke(ctx: &mut Context, r: Rect, hex: u32, radius: f32, width: f32, alpha: f32) {
    ctx.paint_outline_rect(r, color_from_hex(hex, alpha), radius, width);
}

fn draw_text_left(ctx: &mut Context, text: &str, r: Rect, font_size: f32, hex: u32, alpha: f32) {
    ctx.paint_text_clipped(r, text, font_size, color_from_hex(hex, alpha), TextAlign::Left, Some(&r));
}

fn draw_text_center(ctx: &mut Context, text: &str, r: Rect, font_size: f32, hex: u32, alpha: f32) {
    ctx.paint_text_clipped(r, text, font_size, color_from_hex(hex, alpha), TextAlign::Center, Some(&r));
}

fn draw_text_right(ctx: &mut Context, text: &str, r: Rect, font_size: f32, hex: u32, alpha: f32) {
    ctx.paint_text_clipped(r, text, font_size, color_from_hex(hex, alpha), TextAlign::Right, Some(&r));
}

fn draw_icon(ctx: &mut Context, codepoint: u32, r: Rect, hex: u32, alpha: f32) {
    ctx.paint_glyph(r, codepoint, color_from_hex(hex, alpha), r.h.min(r.w) * 0.72);
}

fn hovered(ctx: &Context, r: &Rect) -> bool {
    ctx.is_hovered(r)
}

fn clicked(ctx: &Context, r: &Rect) -> bool {
    hovered(ctx, r) && ctx.is_mouse_pressed()
}

// ── Stage background ──

fn draw_stage_background(ctx: &mut Context, rect: Rect, s: f32, p: &GalleryPalette) {
    draw_fill(ctx, rect, p.surface_alt, 22.0 * s, 1.0);
    draw_stroke(ctx, rect, p.border, 22.0 * s, 1.0, 0.92);
    let gap = 34.0 * s;
    let mut x = rect.x + gap;
    while x < rect.x + rect.w {
        draw_fill(ctx, Rect::new(x, rect.y, 1.0, rect.h), p.grid, 0.0, if p.light { 0.36 } else { 0.46 });
        x += gap;
    }
    let mut y = rect.y + gap;
    while y < rect.y + rect.h {
        draw_fill(ctx, Rect::new(rect.x, y, rect.w, 1.0), p.grid, 0.0, if p.light { 0.36 } else { 0.46 });
        y += gap;
    }
}

// ── Actor ──

fn draw_actor(ctx: &mut Context, rect: Rect, s: f32, p: &GalleryPalette, top: u32, bottom: u32, alpha: f32) {
    // Shadow matching C++: shadow(0.0, 8.0*s, 20.0*s, panel_shadow_hex, light?0.08:0.12)
    draw_shadow(ctx, rect, 18.0 * s, 8.0 * s, 20.0 * s, panel_shadow_hex(p), if p.light { 0.08 } else { 0.12 });
    // Gradient fill matching C++: .gradient(top, bottom, alpha)
    draw_gradient(ctx, rect, top, bottom, 18.0 * s, alpha);
    // Stroke
    draw_stroke(ctx, rect, actor_stroke(p), 18.0 * s, 1.0, 0.92 * alpha);
}

#[allow(clippy::too_many_arguments)]
fn draw_actor_ex(ctx: &mut Context, rect: Rect, s: f32, p: &GalleryPalette, top: u32, bottom: u32, alpha: f32, blur_radius: f32, fill_alpha: f32) {
    // Shadow matching C++
    draw_shadow(ctx, rect, 18.0 * s, 8.0 * s, 20.0 * s, panel_shadow_hex(p), if p.light { 0.08 } else { 0.12 });
    // When blur > 0, draw a backdrop blur effect behind the actor
    if blur_radius > 0.0 {
        ctx.paint_backdrop_blur(rect, blur_radius, 18.0 * s);
    }
    // Gradient fill matching C++
    draw_gradient(ctx, rect, top, bottom, 18.0 * s, fill_alpha);
    draw_stroke(ctx, rect, actor_stroke(p), 18.0 * s, 1.0, 0.92 * alpha);
}

fn draw_actor_default(ctx: &mut Context, rect: Rect, s: f32, p: &GalleryPalette, alpha: f32) {
    draw_actor(ctx, rect, s, p, actor_primary_top(p), actor_primary_bottom(p), alpha);
}

// ── Blur reference ──

fn draw_blur_reference(ctx: &mut Context, rect: Rect, s: f32, p: &GalleryPalette) {
    let cx = rect.x + rect.w * 0.5;
    let cy = rect.y + rect.h * 0.5;
    let colors = blur_reference_hexes(p);
    draw_fill(ctx, Rect::new(cx - 240.0 * s, cy - 94.0 * s, 122.0 * s, 122.0 * s), colors[0], 61.0 * s, 0.90);
    draw_fill(ctx, Rect::new(cx + 116.0 * s, cy - 86.0 * s, 130.0 * s, 130.0 * s), colors[1], 65.0 * s, 0.88);
    draw_fill(ctx, Rect::new(cx - 214.0 * s, cy + 72.0 * s, 168.0 * s, 22.0 * s), colors[2], 11.0 * s, 0.82);
    draw_fill(ctx, Rect::new(cx + 34.0 * s, cy + 62.0 * s, 150.0 * s, 22.0 * s), colors[3], 11.0 * s, 0.84);
    draw_fill(ctx, Rect::new(cx - 9.0 * s, cy - 116.0 * s, 18.0 * s, 220.0 * s), colors[4], 9.0 * s, 0.78);
}

// ── Animation demos ──

fn draw_translate_demo(ctx: &mut Context, rect: Rect, s: f32, time: f64, p: &GalleryPalette) {
    draw_stage_background(ctx, rect, s, p);
    let track = Rect::new(rect.x + 48.0 * s, rect.y + rect.h * 0.5 - 3.0 * s, rect.w - 96.0 * s, 6.0 * s);
    draw_fill(ctx, track, demo_track(p), 3.0 * s, 0.86);
    draw_fill(ctx, Rect::new(track.x - 8.0 * s, track.y - 8.0 * s, 16.0 * s, 16.0 * s), demo_anchor(p), 8.0 * s, 1.0);
    draw_fill(ctx, Rect::new(track.x + track.w - 8.0 * s, track.y - 8.0 * s, 16.0 * s, 16.0 * s), demo_anchor_end(p), 8.0 * s, 1.0);
    let travel = track.w - 72.0 * s;
    let t = timeline_ping_pong(time, 1.35, EasingPreset::EaseInOut);
    draw_actor_default(ctx, Rect::new(track.x + travel * t, rect.y + rect.h * 0.5 - 36.0 * s, 72.0 * s, 72.0 * s), s, p, 1.0);
}

fn draw_scale_demo(ctx: &mut Context, rect: Rect, s: f32, time: f64, p: &GalleryPalette) {
    draw_stage_background(ctx, rect, s, p);
    let t = timeline_ping_pong(time, 1.55, EasingPreset::EaseInOut);
    let actor_scale = lerp(0.62, 1.28, t);
    let w = 144.0 * s * actor_scale;
    let h = 144.0 * s * actor_scale;
    let cx = rect.x + rect.w * 0.5;
    let cy = rect.y + rect.h * 0.5;
    draw_actor_default(ctx, Rect::new(cx - w * 0.5, cy - h * 0.5, w, h), s, p, 1.0);
}

fn draw_rotate_demo(ctx: &mut Context, rect: Rect, s: f32, time: f64, p: &GalleryPalette) {
    draw_stage_background(ctx, rect, s, p);
    let cx = rect.x + rect.w * 0.5;
    let cy = rect.y + rect.h * 0.5;
    // Crosshairs
    draw_fill(ctx, Rect::new(cx - 1.0, rect.y + 48.0 * s, 2.0, rect.h - 96.0 * s), demo_axis(p), 0.0, 0.66);
    draw_fill(ctx, Rect::new(rect.x + 48.0 * s, cy - 1.0, rect.w - 96.0 * s, 2.0), demo_axis(p), 0.0, 0.66);
    // Circle outline
    draw_stroke(ctx, Rect::new(cx - 132.0 * s, cy - 132.0 * s, 264.0 * s, 264.0 * s), demo_axis(p), 132.0 * s, 1.0 * s, 0.42);
    // Actor (no rotation support in our renderer, just show oscillating position)
    let _t = loop_progress(time, 2.40);
    draw_actor(ctx, Rect::new(cx - 92.0 * s, cy - 42.0 * s, 184.0 * s, 84.0 * s), s, p, actor_focus_top(p), actor_focus_bottom(p), 1.0);
    // Center dot
    draw_fill(ctx, Rect::new(cx - 7.0 * s, cy - 7.0 * s, 14.0 * s, 14.0 * s), actor_glass_top(p), 7.0 * s, 0.98);
}

fn draw_arc_motion_demo(ctx: &mut Context, rect: Rect, s: f32, time: f64, p: &GalleryPalette) {
    draw_stage_background(ctx, rect, s, p);
    let t = timeline_ping_pong(time, 1.75, EasingPreset::EaseOut);
    let left = rect.x + 52.0 * s;
    let right = rect.x + rect.w - 52.0 * s;
    let base_y = rect.y + rect.h * 0.70;
    let height = rect.h * 0.34;

    for i in 0..=30 {
        let sample = i as f32 / 30.0;
        let x = lerp(left, right, sample);
        let y = base_y - (sample * std::f32::consts::PI).sin() * height;
        draw_fill(ctx, Rect::new(x - 3.0 * s, y - 3.0 * s, 6.0 * s, 6.0 * s), demo_curve_soft(p), 3.0 * s, 0.70);
    }

    let x = lerp(left, right, t);
    let y = base_y - (t * std::f32::consts::PI).sin() * height;
    draw_actor_default(ctx, Rect::new(x - 36.0 * s, y - 36.0 * s, 72.0 * s, 72.0 * s), s, p, 1.0);
}

fn draw_bezier_demo(ctx: &mut Context, rect: Rect, s: f32, time: f64, p: &GalleryPalette) {
    draw_stage_background(ctx, rect, s, p);
    let p0 = (rect.x + 58.0 * s, rect.y + rect.h - 74.0 * s);
    let p1 = (rect.x + rect.w * 0.30, rect.y + 38.0 * s);
    let p2 = (rect.x + rect.w * 0.68, rect.y + rect.h - 42.0 * s);
    let p3 = (rect.x + rect.w - 58.0 * s, rect.y + 74.0 * s);

    for i in 0..=40 {
        let sample = i as f32 / 40.0;
        let pt = cubic_bezier_point(p0, p1, p2, p3, sample);
        let a = if sample < 0.5 { 0.72 } else { 0.90 };
        draw_fill(ctx, Rect::new(pt.0 - 3.0 * s, pt.1 - 3.0 * s, 6.0 * s, 6.0 * s), demo_curve(p), 3.0 * s, a);
    }

    draw_fill(ctx, Rect::new(p0.0 - 6.0 * s, p0.1 - 6.0 * s, 12.0 * s, 12.0 * s), demo_anchor(p), 6.0 * s, 1.0);
    draw_fill(ctx, Rect::new(p1.0 - 6.0 * s, p1.1 - 6.0 * s, 12.0 * s, 12.0 * s), demo_anchor_muted(p), 6.0 * s, 0.92);
    draw_fill(ctx, Rect::new(p2.0 - 6.0 * s, p2.1 - 6.0 * s, 12.0 * s, 12.0 * s), demo_anchor_muted(p), 6.0 * s, 0.92);
    draw_fill(ctx, Rect::new(p3.0 - 6.0 * s, p3.1 - 6.0 * s, 12.0 * s, 12.0 * s), demo_anchor_end(p), 6.0 * s, 1.0);

    let raw_t = {
        let d = 2.10_f32.max(1e-4);
        let total = d * 2.0;
        let looped = loop_progress(time, total) * total;
        if looped <= d { looped / d } else { 1.0 - (looped - d) / d }
    };
    let t = cubic_bezier_ease(0.20, 0.0, 0.10, 1.0, raw_t.clamp(0.0, 1.0));
    let actor_pt = cubic_bezier_point(p0, p1, p2, p3, t);
    draw_actor_default(ctx, Rect::new(actor_pt.0 - 34.0 * s, actor_pt.1 - 34.0 * s, 68.0 * s, 68.0 * s), s, p, 1.0);
}

fn draw_color_demo(ctx: &mut Context, rect: Rect, s: f32, time: f64, p: &GalleryPalette) {
    draw_stage_background(ctx, rect, s, p);
    let t = timeline_ping_pong(time, 1.80, EasingPreset::EaseInOut);
    let top = mix_hex(actor_primary_top(p), mix_hex(p.accent, 0xF59E0B, 0.72), t);
    let bottom = mix_hex(actor_primary_bottom(p), mix_hex(p.accent, 0xEF4444, 0.68), t);
    let actor = Rect::new(rect.x + rect.w * 0.5 - 88.0 * s, rect.y + rect.h * 0.5 - 52.0 * s, 176.0 * s, 104.0 * s);
    draw_actor(ctx, actor, s, p, top, bottom, 1.0);
    // Color swatches at bottom
    draw_fill(ctx, Rect::new(rect.x + 58.0 * s, rect.y + rect.h - 78.0 * s, 84.0 * s, 18.0 * s), actor_primary_top(p), 9.0 * s, 0.92);
    let mid = mix_hex(actor_primary_top(p), mix_hex(p.accent, 0xF59E0B, 0.72), 0.5);
    draw_fill(ctx, Rect::new(rect.x + 154.0 * s, rect.y + rect.h - 78.0 * s, 84.0 * s, 18.0 * s), mid, 9.0 * s, 0.92);
    draw_fill(ctx, Rect::new(rect.x + 250.0 * s, rect.y + rect.h - 78.0 * s, 84.0 * s, 18.0 * s), mix_hex(p.accent, 0xEF4444, 0.68), 9.0 * s, 0.92);
}

fn draw_opacity_demo(ctx: &mut Context, rect: Rect, s: f32, time: f64, p: &GalleryPalette) {
    draw_stage_background(ctx, rect, s, p);
    let actor = Rect::new(rect.x + rect.w * 0.5 - 88.0 * s, rect.y + rect.h * 0.5 - 52.0 * s, 176.0 * s, 104.0 * s);
    let alpha = lerp(0.20, 1.0, timeline_ping_pong(time, 1.60, EasingPreset::EaseInOut));
    draw_stroke(ctx, actor, actor_outline(p), 18.0 * s, 1.0, 0.26);
    draw_actor_default(ctx, actor, s, p, alpha);
}

fn draw_blur_demo(ctx: &mut Context, rect: Rect, s: f32, time: f64, p: &GalleryPalette) {
    draw_stage_background(ctx, rect, s, p);
    draw_blur_reference(ctx, inset_rect(&rect, 18.0 * s, 18.0 * s), s, p);
    let t = timeline_ping_pong(time, 1.85, EasingPreset::EaseInOut);
    let blur_value = lerp(0.0, 22.0 * s, t);
    let left_actor = Rect::new(rect.x + rect.w * 0.5 - 214.0 * s, rect.y + rect.h * 0.5 - 58.0 * s, 188.0 * s, 116.0 * s);
    let right_actor = Rect::new(rect.x + rect.w * 0.5 + 26.0 * s, rect.y + rect.h * 0.5 - 58.0 * s, 188.0 * s, 116.0 * s);
    draw_text_center(ctx, "No blur", Rect::new(left_actor.x, left_actor.y - 34.0 * s, left_actor.w, 18.0 * s), font_body(s), p.muted, 0.96);
    draw_text_center(ctx, "Animated blur", Rect::new(right_actor.x, right_actor.y - 34.0 * s, right_actor.w, 18.0 * s), font_body(s), p.muted, 0.96);
    draw_actor_ex(ctx, left_actor, s, p, actor_glass_top(p), actor_glass_bottom(p), 1.0, 0.0, 0.12);
    draw_actor_ex(ctx, right_actor, s, p, actor_glass_top(p), actor_glass_bottom(p), 1.0, blur_value, 0.12);
}

fn draw_combo_demo(ctx: &mut Context, rect: Rect, s: f32, time: f64, p: &GalleryPalette) {
    draw_stage_background(ctx, rect, s, p);
    draw_blur_reference(ctx, inset_rect(&rect, 24.0 * s, 24.0 * s), s, p);
    let translate_t = timeline_ping_pong(time, 1.65, EasingPreset::EaseInOut);
    let scale_t = timeline_ping_pong(time + 0.28, 1.65, EasingPreset::EaseOut);
    let _rotate_t = loop_progress(time, 2.60);
    let color_t = timeline_ping_pong(time + 0.44, 2.00, EasingPreset::EaseInOut);
    let alpha_t = timeline_ping_pong(time + 0.18, 1.70, EasingPreset::EaseInOut);
    let blur_t = timeline_ping_pong(time + 0.33, 1.90, EasingPreset::EaseInOut);

    let dx = lerp(-rect.w * 0.18, rect.w * 0.18, translate_t);
    let actor_scale = lerp(0.72, 1.18, scale_t);
    let alpha = lerp(0.42, 1.0, alpha_t);
    let blur_value = lerp(0.0, 16.0 * s, blur_t);
    let top = mix_hex(actor_primary_top(p), mix_hex(p.accent, 0xA855F7, 0.62), color_t);
    let bottom = mix_hex(actor_primary_bottom(p), mix_hex(p.accent, 0xF97316, 0.72), color_t);
    let w = 188.0 * s * actor_scale;
    let h = 116.0 * s * actor_scale;
    let cx = rect.x + rect.w * 0.5 + dx;
    let cy = rect.y + rect.h * 0.5;
    draw_actor_ex(ctx, Rect::new(cx - w * 0.5, cy - h * 0.5, w, h), s, p, top, bottom, alpha, blur_value, 0.24);
}

fn draw_demo_scene(ctx: &mut Context, demo: usize, rect: Rect, s: f32, time: f64, p: &GalleryPalette) {
    match demo {
        0 => draw_translate_demo(ctx, rect, s, time, p),
        1 => draw_scale_demo(ctx, rect, s, time, p),
        2 => draw_rotate_demo(ctx, rect, s, time, p),
        3 => draw_arc_motion_demo(ctx, rect, s, time, p),
        4 => draw_bezier_demo(ctx, rect, s, time, p),
        5 => draw_color_demo(ctx, rect, s, time, p),
        6 => draw_opacity_demo(ctx, rect, s, time, p),
        7 => draw_blur_demo(ctx, rect, s, time, p),
        _ => draw_combo_demo(ctx, rect, s, time, p),
    }
}

// ── Page: Basic Controls ──

fn draw_basic_controls_page(ctx: &mut Context, state: &mut GalleryState, rect: Rect, s: f32) {
    let p = make_gallery_palette(state);
    let gap = 18.0 * s;
    let cols = LinearLayout::row(rect).gap(gap)
        .items(&[fr(1.0), fr(1.0), fr(1.0), fr(1.0)]).resolve();

    let control_icons: [(u32, &str); 3] = [(0xF002, "Search"), (0xF0DB, "Layout"), (0xF061, "Motion")];

    // Card 1: Buttons
    if let Some(col) = cols.first() {
        draw_card(ctx, *col, "Buttons", s, &p, |ctx, content| {
            let rows = LinearLayout::column(content).gap(12.0 * s)
                .items(&[px(72.0 * s), fr(1.0), px(18.0 * s)]).resolve();

            // Icon chips
            if let Some(icon_row) = rows.first() {
                let chips = LinearLayout::row(*icon_row).gap(8.0 * s)
                    .items(&[fr(1.0), fr(1.0), fr(1.0)]).resolve();
                for (i, chip_rect) in chips.iter().enumerate() {
                    let is_selected = state.selected_control_icon == i;
                    let is_hovered = hovered(ctx, chip_rect);
                    let mix = ctx.presence(hash_str(&format!("basic_icon_chip_{}", i + 1)), is_hovered || is_selected);
                    let bg = if is_selected { nav_selected_fill(&p) } else { mix_hex(p.surface_deep, p.accent, (0.10 * mix) as f64 as f32) };
                    let border = if is_selected { p.accent } else { mix_hex(p.border_soft, p.accent, (0.18 * mix) as f64 as f32) };
                    draw_fill(ctx, *chip_rect, bg, 16.0 * s, 0.96);
                    draw_stroke(ctx, *chip_rect, border, 16.0 * s, 1.0, 0.88);
                    draw_icon(ctx, control_icons[i].0,
                        Rect::new(chip_rect.x + chip_rect.w * 0.5 - 8.0 * s, chip_rect.y + 12.0 * s, 16.0 * s, 16.0 * s),
                        if is_selected { p.accent } else { p.text }, 0.98);
                    draw_text_center(ctx, control_icons[i].1,
                        Rect::new(chip_rect.x + 6.0 * s, chip_rect.y + 38.0 * s, chip_rect.w - 12.0 * s, 18.0 * s),
                        font_meta(s), if is_selected { p.text } else { p.muted }, 0.98);
                    if clicked(ctx, chip_rect) { state.selected_control_icon = i; }
                }
            }

            // Buttons
            if rows.len() > 1 {
                let btn_rows = LinearLayout::column(rows[1]).gap(8.0 * s)
                    .items(&[px(36.0 * s), px(32.0 * s), px(32.0 * s)]).resolve();
                if let Some(r) = btn_rows.first() {
                    ctx.button(hash_str("primary_action"), *r, "Primary Action", ButtonStyle::Primary);
                }
                if btn_rows.len() > 1
                    && ctx.button(hash_str("reset_progress"), btn_rows[1], "Reset Progress", ButtonStyle::Ghost) {
                    state.progress_ratio = 0.18;
                }
                if btn_rows.len() > 2 {
                    let label = control_icons[state.selected_control_icon.min(2)].1;
                    draw_readonly(ctx, btn_rows[2], "Selected", label, s, &p);
                }
            }

            // Summary
            if rows.len() > 2 {
                draw_text_left(ctx, "Compact button sizing keeps the page readable.", rows[2], font_meta(s), p.muted, 0.96);
            }
        });
    }

    // Card 2: Selection — match C++ ui.tab() widget rendering
    if cols.len() > 1 {
        let control_modes = ["Compact", "Balanced", "Comfortable"];
        let control_toggles = ["Toolbar", "Cards", "Blur"];
        // Extract theme colors before the closure borrows ctx
        let theme_secondary = ctx.theme().secondary;
        let theme_primary = ctx.theme().primary;
        let theme_panel = ctx.theme().panel;
        let theme_outline = ctx.theme().outline;
        let theme_text = ctx.theme().text;
        let theme_muted_text = ctx.theme().muted_text;
        let tab_radius = (ctx.theme().radius - 2.0_f32).max(0.0);
        draw_card(ctx, cols[1], "Selection", s, &p, |ctx, content| {
            let row_h = 32.0 * s;
            let rows = LinearLayout::column(content).gap(8.0 * s)
                .items(&[px(row_h); 9]).resolve();

            if !rows.is_empty() { draw_readonly(ctx, rows[0], "Single", "Exclusive tab group", s, &p); }
            for i in 0..3 {
                if rows.len() > i + 1 {
                    let is_selected = state.controls_mode == i;
                    let active_v: f32 = if is_selected { 1.0 } else { 0.0 };
                    let press_v: f32 = 0.0; // static first frame
                    // C++: visual_scale = 1.0 + active_v * 0.004 - press_v * 0.014
                    let visual_scale = 1.0 + active_v * 0.004 - press_v * 0.014;
                    let mut visual_rect = context_scale_rect_from_center(&rows[i + 1], visual_scale, visual_scale);
                    visual_rect = context_translate_rect(&visual_rect, 0.0, press_v * 0.3);
                    // C++ fill: mix(secondary, mix(primary, panel, 0.72), active_v)
                    let fill = mix_color(theme_secondary, mix_color(theme_primary, theme_panel, 0.72), active_v);
                    // C++ outline: mix(mix(outline, panel, 0.6), primary, active_v * 0.78)
                    let outline = mix_color(mix_color(theme_outline, theme_panel, 0.6), theme_primary, active_v * 0.78);
                    let thickness = 1.0 + active_v * 0.36;
                    let text_size = (rows[i + 1].h * 0.42).clamp(13.0, 26.0);
                    // C++ text: mix(muted_text, text, 0.38 + active_v * 0.62)
                    let text_color = mix_color(theme_muted_text, theme_text, 0.38 + active_v * 0.62);
                    if is_selected {
                        ctx.paint_soft_glow(visual_rect, theme_primary, tab_radius, active_v * 0.34, 5.0);
                    }
                    ctx.paint_filled_rect(visual_rect, fill, tab_radius);
                    ctx.paint_outline_rect(visual_rect, outline, tab_radius, thickness);
                    ctx.paint_text(visual_rect, control_modes[i], text_size, text_color, TextAlign::Center);
                    if clicked(ctx, &rows[i + 1]) { state.controls_mode = i; }
                }
            }
            if rows.len() > 4 { draw_readonly(ctx, rows[4], "Multi", "Independent toggles", s, &p); }
            for i in 0..3 {
                if rows.len() > i + 5 {
                    let is_on = state.controls_multi_select[i];
                    let active_v: f32 = if is_on { 1.0 } else { 0.0 };
                    let press_v: f32 = 0.0;
                    let visual_scale = 1.0 + active_v * 0.004 - press_v * 0.014;
                    let mut visual_rect = context_scale_rect_from_center(&rows[i + 5], visual_scale, visual_scale);
                    visual_rect = context_translate_rect(&visual_rect, 0.0, press_v * 0.3);
                    let fill = mix_color(theme_secondary, mix_color(theme_primary, theme_panel, 0.72), active_v);
                    let outline = mix_color(mix_color(theme_outline, theme_panel, 0.6), theme_primary, active_v * 0.78);
                    let thickness = 1.0 + active_v * 0.36;
                    let text_size = (rows[i + 5].h * 0.42).clamp(13.0, 26.0);
                    let text_color = mix_color(theme_muted_text, theme_text, 0.38 + active_v * 0.62);
                    if is_on {
                        ctx.paint_soft_glow(visual_rect, theme_primary, tab_radius, active_v * 0.34, 5.0);
                    }
                    ctx.paint_filled_rect(visual_rect, fill, tab_radius);
                    ctx.paint_outline_rect(visual_rect, outline, tab_radius, thickness);
                    ctx.paint_text(visual_rect, control_toggles[i], text_size, text_color, TextAlign::Center);
                    if clicked(ctx, &rows[i + 5]) { state.controls_multi_select[i] = !state.controls_multi_select[i]; }
                }
            }
            if rows.len() > 8 {
                let mut enabled = String::new();
                if state.controls_multi_select[0] { enabled.push_str("Toolbar "); }
                if state.controls_multi_select[1] { enabled.push_str("Cards "); }
                if state.controls_multi_select[2] { enabled.push_str("Blur"); }
                let enabled = if enabled.is_empty() { "None".to_string() } else { enabled };
                draw_readonly(ctx, rows[8], "Enabled", &enabled, s, &p);
            }
        });
    }

    // Card 3: Inputs (matches C++ order: search, dropdown, slider, slider, progress, readonly, button)
    if cols.len() > 2 {
        draw_card(ctx, cols[2], "Inputs", s, &p, |ctx, content| {
            let compact_h = 32.0 * s;
            let normal_h = 36.0 * s;
            let progress_bar_h = 10.0 * s;
            let progress_label_h = (progress_bar_h * 1.6).clamp(14.0, 26.0);
            let progress_track_h = progress_bar_h + (progress_label_h + 4.0).max(8.0);
            let control_modes_labels = ["Compact", "Balanced", "Comfortable"];
            let density_text = control_modes_labels[state.controls_mode.min(2)];
            let dropdown_label = format!("Density: {}", density_text);
            // C++ dropdown_track_h = max(34, dropdown_padding*3) + (open ? body : 0)
            let dropdown_padding = 10.0 * s;
            let dropdown_header_h = 34.0_f32.max(dropdown_padding * 3.0);
            let dropdown_body_h = 104.0 * s;
            let dropdown_track_h = if state.controls_dropdown_open {
                dropdown_header_h + dropdown_body_h
            } else {
                dropdown_header_h
            };
            let rows = LinearLayout::column(content).gap(8.0 * s)
                .items(&[px(38.0 * s), px(dropdown_track_h), px(normal_h), px(normal_h), px(progress_track_h), px(compact_h), px(normal_h)]).resolve();

            // Row 0: Search input (with "Search" label matching C++ ui.input("Search", ...))
            if !rows.is_empty() {
                ctx.text_input_field_ex(hash_str("search_input"), rows[0], "Search", &mut state.search_text, "Type to filter");
            }
            // Row 1: Density dropdown — expanding with selectable items
            if rows.len() > 1 {
                ctx.dropdown_select(
                    hash_str("density_dropdown"), rows[1], &dropdown_label,
                    &mut state.controls_dropdown_open,
                    &control_modes_labels, &mut state.controls_mode,
                    dropdown_body_h, dropdown_padding,
                );
            }
            // Row 2: Progress slider (C++ ui.slider("Progress", ...) — label drawn internally)
            if rows.len() > 2 {
                ctx.slider_labeled(hash_str("progress_slider"), rows[2], "Progress", &mut state.progress_ratio, 0.0, 1.0);
            }
            // Row 3: Live Value slider (C++ ui.slider("Live Value", ...) — label drawn internally)
            if rows.len() > 3 {
                ctx.slider_labeled(hash_str("control_slider"), rows[3], "Live Value", &mut state.control_slider, 0.0, 1.0);
            }
            // Row 4: Completion progress bar (C++ ui.progress("Completion", ...) — label drawn internally)
            if rows.len() > 4 {
                let pr = state.progress_ratio;
                ctx.progress(hash_str("completion_progress"), rows[4], "Completion", pr, progress_bar_h);
            }
            // Row 5: Gap / Accent readonly
            if rows.len() > 5 {
                let info = format!("{} / {}", format_pixels(state.layout_gap), format_hex_color(accent_hex(state)));
                draw_readonly(ctx, rows[5], "Gap / Accent", &info, s, &p);
            }
            // Row 6: Jump To Animation button
            if rows.len() > 6
                && ctx.button(hash_str("jump_animation"), rows[6], "Jump To Animation Page", ButtonStyle::Secondary) {
                state.selected_page = 3;
            }
        });
    }

    // Card 4: Editor
    if cols.len() > 3 {
        draw_card(ctx, cols[3], "Editor", s, &p, |ctx, content| {
            draw_readonly(ctx, Rect::new(content.x, content.y, content.w, 32.0 * s), "Purpose", "Multiline input, wrapping and scroll behavior", s, &p);
            let compact_h = 32.0 * s;
            let item_spacing = 8.0; // C++ item_spacing_ default
            let text_area_y = content.y + compact_h + item_spacing;
            let text_area_h = (160.0_f32 * s).max(content.h - 40.0 * s); // C++: max(160*scale, card.content().h - 40*scale)
            let text_area_rect = Rect::new(content.x, text_area_y, content.w, text_area_h);
            // "Notes" label matching C++ text_area("Notes", ...) label rendering
            let label_font = (text_area_rect.h * 0.12).clamp(12.0, 18.0);
            let outer_pad = (text_area_rect.h * 0.04).clamp(6.0, 12.0);
            // C++ add_text(label, label_rect, muted_text, label_font, Left) — no clip
            let muted_text_col = ctx.theme().muted_text;
            ctx.paint_text(Rect::new(text_area_rect.x + outer_pad, text_area_rect.y + outer_pad, text_area_rect.w - outer_pad * 2.0, label_font), "Notes", label_font, muted_text_col, TextAlign::Left);
            // Text area box below the label
            let box_y = text_area_rect.y + outer_pad + label_font + 6.0;
            let box_rect = Rect::new(text_area_rect.x + outer_pad, box_y, text_area_rect.w - outer_pad * 2.0, (text_area_rect.h - (label_font + outer_pad + 12.0)).max(0.0));
            ctx.text_input_field(hash_str("notes_area"), box_rect, &mut state.notes_text);
        });
    }
}

// ── Page: Design ──

fn draw_design_page(ctx: &mut Context, state: &mut GalleryState, rect: Rect, s: f32) {
    let p = make_gallery_palette(state);
    let gap = 18.0 * s;
    let cols = LinearLayout::row(rect).gap(gap)
        .items(&[fr(1.05), fr(1.0), fr(1.0)]).resolve();

    // Typography
    if let Some(col) = cols.first() {
        draw_card(ctx, *col, "Typography", s, &p, |ctx, content| {
            let type_rows: [(& str, &str, &str, f32); 4] = [
                ("font_display", "Page title / shell header", "Display Title", font_display(s)),
                ("font_heading", "Card / section heading", "Section Heading", font_heading(s)),
                ("font_body", "Navigation / readable text", "Readable Body", font_body(s)),
                ("font_meta", "Summary / caption / helper", "Meta Caption", font_meta(s)),
            ];
            let rows = LinearLayout::column(content).gap(8.0 * s)
                .items(&[fr(1.0), fr(1.0), fr(1.0), fr(1.0), px(92.0 * s)]).resolve();
            for (i, row) in rows.iter().take(4).enumerate() {
                let row_size = type_rows[i].3;
                let px_label = format!("{:.1} px", row_size / s.max(1.0));
                draw_fill(ctx, *row, p.surface_deep, 14.0 * s, 0.94);
                draw_stroke(ctx, *row, p.border_soft, 14.0 * s, 1.0, 0.86);
                draw_text_left(ctx, type_rows[i].0, Rect::new(row.x + 12.0 * s, row.y + 8.0 * s, row.w - 132.0 * s, 18.0 * s), font_body(s), p.text, 0.98);
                draw_text_right(ctx, &px_label, Rect::new(row.x + row.w - 120.0 * s, row.y + 8.0 * s, 108.0 * s, 18.0 * s), font_meta(s), p.muted, 0.98);
                draw_text_left(ctx, type_rows[i].1, Rect::new(row.x + 12.0 * s, row.y + 28.0 * s, row.w - 24.0 * s, 16.0 * s), font_meta(s), p.muted, 0.96);
                draw_text_left(ctx, type_rows[i].2, Rect::new(row.x + 12.0 * s, row.y + 44.0 * s, row.w - 24.0 * s, 24.0 * s), row_size, mix_hex(p.text, p.accent, 0.18), 0.98);
            }
            if rows.len() > 4 {
                let code = rows[4];
                draw_fill(ctx, code, p.surface_deep, 16.0 * s, 0.92);
                draw_stroke(ctx, code, p.border_soft, 16.0 * s, 1.0, 0.86);
                let code_lines = [
                    "float font_display(float scale) { return 22.0f * scale; }",
                    "float font_heading(float scale) { return 16.0f * scale; }",
                    "float font_body(float scale) { return 14.0f * scale; }",
                    "float font_meta(float scale) { return 12.2f * scale; }",
                ];
                for (i, line) in code_lines.iter().enumerate() {
                    draw_text_left(ctx, line, Rect::new(code.x + 12.0 * s, code.y + 10.0 * s + i as f32 * 20.0 * s, code.w - 24.0 * s, 18.0 * s), font_meta(s), p.muted, 0.98);
                }
            }
        });
    }

    // Color System
    if cols.len() > 1 {
        draw_card(ctx, cols[1], "Color System", s, &p, |ctx, content| {
            let sections = LinearLayout::column(content).gap(12.0 * s)
                .items(&[fr(1.0), fr(2.1)]).resolve();

            // Accent presets
            if let Some(accents_rect) = sections.first() {
                draw_fill(ctx, *accents_rect, p.surface_deep, 18.0 * s, 0.92);
                draw_stroke(ctx, *accents_rect, p.border_soft, 18.0 * s, 1.0, 0.90);
                let inner = inset_rect(accents_rect, 14.0 * s, 14.0 * s);
                draw_text_left(ctx, "Built-in Accent Presets", Rect::new(inner.x, inner.y, inner.w, 18.0 * s), font_body(s), p.text, 0.98);
                let grid_rect = Rect::new(inner.x, inner.y + 30.0 * s, inner.w, (inner.h - 30.0 * s).max(0.0));
                let grid = GridLayout::new(grid_rect, 3).rows(2).gap(10.0 * s);
                for (i, &accent_h) in ACCENT_HEXES.iter().enumerate() {
                    let tile = grid.cell(i);
                    draw_fill(ctx, tile, p.surface_alt, 14.0 * s, 0.96);
                    draw_stroke(ctx, tile, p.border_soft, 14.0 * s, 1.0, 0.86);
                    draw_fill(ctx, Rect::new(tile.x + 10.0 * s, tile.y + 10.0 * s, tile.w - 20.0 * s, 14.0 * s), accent_h, 7.0 * s, 0.98);
                    draw_text_left(ctx, &format_hex_color(accent_h), Rect::new(tile.x + 10.0 * s, tile.y + 30.0 * s, tile.w - 20.0 * s, 14.0 * s), font_meta(s), p.text, 0.98);
                }
            }

            // Theme tokens
            if sections.len() > 1 {
                let tokens_area = sections[1];
                draw_fill(ctx, tokens_area, p.surface_deep, 18.0 * s, 0.92);
                draw_stroke(ctx, tokens_area, p.border_soft, 18.0 * s, 1.0, 0.90);
                let inner = inset_rect(&tokens_area, 14.0 * s, 14.0 * s);
                let title = if state.light_mode { "Current Theme Tokens: Light" } else { "Current Theme Tokens: Dark" };
                draw_text_left(ctx, title, Rect::new(inner.x, inner.y, inner.w, 18.0 * s), font_body(s), p.text, 0.98);
                let tokens: [(&str, u32); 10] = [
                    ("shell_top", p.shell_top), ("shell_bottom", p.shell_bottom),
                    ("surface", p.surface), ("surface_alt", p.surface_alt),
                    ("surface_deep", p.surface_deep), ("border", p.border),
                    ("border_soft", p.border_soft), ("accent", p.accent),
                    ("accent_soft", p.accent_soft), ("muted", p.muted),
                ];
                let grid_rect = Rect::new(inner.x, inner.y + 30.0 * s, inner.w, (inner.h - 30.0 * s).max(0.0));
                let grid = GridLayout::new(grid_rect, 2).rows(5).gap(8.0 * s);
                for (i, (name, hex)) in tokens.iter().enumerate() {
                    let tile = grid.cell(i);
                    draw_fill(ctx, tile, p.surface_alt, 14.0 * s, 0.96);
                    draw_stroke(ctx, tile, p.border_soft, 14.0 * s, 1.0, 0.86);
                    let swatch_h = (tile.h - 20.0 * s).max(14.0 * s);
                    let swatch = Rect::new(tile.x + 10.0 * s, tile.y + 10.0 * s, 16.0 * s, swatch_h);
                    draw_fill(ctx, swatch, *hex, swatch.w.min(8.0 * s), 0.98);
                    draw_text_left(ctx, name, Rect::new(tile.x + 34.0 * s, tile.y + 10.0 * s, tile.w - 46.0 * s, 16.0 * s), font_body(s), p.text, 0.98);
                    draw_text_left(ctx, &format_hex_color(*hex), Rect::new(tile.x + 34.0 * s, tile.y + tile.h - 22.0 * s, tile.w - 46.0 * s, 14.0 * s), font_meta(s), p.muted, 0.96);
                }
            }
        });
    }

    // Icon Reference
    if cols.len() > 2 {
        draw_card(ctx, cols[2], "Icon Reference", s, &p, |ctx, content| {
            let rows = LinearLayout::column(content).gap(12.0 * s)
                .items(&[px(38.0 * s), fr(1.0)]).resolve();

            // "Open Icon Sidebar" action row
            if let Some(action_row) = rows.first() {
                let ar = *action_row;
                let open_hovered = hovered(ctx, &ar);
                let open_mix = ctx.presence(hash_str("design_open_icon_sidebar"), open_hovered || state.design_icon_library_open);
                let fill_hex = if state.design_icon_library_open { nav_selected_fill(&p) } else { mix_hex(p.surface_deep, p.accent, 0.10 * open_mix) };
                let stroke_hex = if state.design_icon_library_open { p.accent } else { mix_hex(p.border_soft, p.accent, 0.18 * open_mix) };
                let icon_hex = if state.design_icon_library_open { p.accent } else { p.text };
                let text_hex = if state.design_icon_library_open { p.text } else { p.muted };
                draw_fill(ctx, ar, fill_hex, ar.h * 0.5, 0.96);
                draw_stroke(ctx, ar, stroke_hex, ar.h * 0.5, 1.0, 0.88);
                draw_icon(ctx, 0xF03A, Rect::new(ar.x + 12.0 * s, ar.y + (ar.h - 16.0 * s) * 0.5, 16.0 * s, 16.0 * s), icon_hex, 0.98);
                draw_text_left(ctx, "Open Icon Sidebar", Rect::new(ar.x + 34.0 * s, ar.y + 10.0 * s, ar.w - 46.0 * s, 18.0 * s), font_body(s), text_hex, 0.98);
                if clicked(ctx, &ar) { state.design_icon_library_open = true; }
            }

            // Icon grid (4 rows x 3 columns)
            if rows.len() > 1 {
                let grid_content = rows[1];
                let grid = GridLayout::new(grid_content, 3).rows(4).gap(10.0 * s);
                for (i, (cp, label, usage)) in ICON_LIBRARY.iter().enumerate() {
                    let tile = grid.cell(i);
                    draw_fill(ctx, tile, p.surface_deep, 16.0 * s, 0.94);
                    draw_stroke(ctx, tile, if i == 0 { p.accent } else { p.border_soft }, 16.0 * s, 1.0, 0.90);
                    draw_icon(ctx, *cp, Rect::new(tile.x + 12.0 * s, tile.y + 12.0 * s, 18.0 * s, 18.0 * s),
                        if i == 0 { p.accent } else { p.text }, 0.98);
                    draw_text_left(ctx, label, Rect::new(tile.x + 38.0 * s, tile.y + 10.0 * s, tile.w - 50.0 * s, 18.0 * s), font_body(s), p.text, 0.98);
                    draw_text_left(ctx, &format_codepoint(*cp), Rect::new(tile.x + 12.0 * s, tile.y + 38.0 * s, tile.w - 24.0 * s, 16.0 * s), font_meta(s), p.accent, 0.98);
                    draw_text_left(ctx, usage, Rect::new(tile.x + 12.0 * s, tile.y + 56.0 * s, tile.w - 24.0 * s, (tile.h - 60.0 * s).max(0.0)), font_meta(s), p.muted, 0.96);
                }
            }
        });
    }
}

// ── Page: Layout ──

fn draw_layout_page(ctx: &mut Context, state: &mut GalleryState, rect: Rect, s: f32) {
    let p = make_gallery_palette(state);
    let accent = accent_hex(state);
    let rows = LinearLayout::column(rect).gap(18.0 * s)
        .items(&[px(168.0 * s), fr(1.0)]).resolve();

    // Controls
    if let Some(controls_rect) = rows.first() {
        draw_card(ctx, *controls_rect, "Layout Controls", s, &p, |ctx, content| {
            let control_rows = LinearLayout::column(content).gap(10.0 * s)
                .items(&[px(40.0 * s), px(40.0 * s), px(36.0 * s)]).resolve();
            if !control_rows.is_empty() {
                ctx.slider_labeled(hash_str("layout_gap"), control_rows[0], "Gap", &mut state.layout_gap, 8.0, 28.0);
            }
            if control_rows.len() > 1 {
                ctx.slider_labeled(hash_str("layout_radius"), control_rows[1], "Radius", &mut state.layout_radius, 8.0, 28.0);
            }
            if control_rows.len() > 2 {
                draw_readonly(ctx, control_rows[2], "Core APIs", "view(), row(), column(), grid(), zstack()", s, &p);
            }
        });
    }

    // Layout Composition preview
    if rows.len() > 1 {
        let preview = rows[1];
        let gap = state.layout_gap * s;
        let radius = state.layout_radius * s;
        draw_card(ctx, preview, "Layout Composition", s, &p, |ctx, content| {
            let main_rows = LinearLayout::column(content).gap(gap)
                .items(&[px(52.0 * s), fr(1.0)]).resolve();

            if let Some(toolbar) = main_rows.first() {
                draw_fill(ctx, *toolbar, p.surface_deep, radius, 0.98);
                draw_stroke(ctx, *toolbar, accent, radius, 1.0, 0.52);
                draw_text_left(ctx, "Toolbar / Filters", inset_rect(toolbar, 16.0 * s, 14.0 * s), font_heading(s), p.text, 0.98);
            }

            if main_rows.len() > 1 {
                let body = main_rows[1];
                let body_cols = LinearLayout::row(body).gap(gap)
                    .items(&[px(120.0 * s), fr(1.8), px(160.0 * s)]).resolve();

                if let Some(sidebar) = body_cols.first() {
                    draw_fill(ctx, *sidebar, p.surface_deep, radius, 0.98);
                    draw_stroke(ctx, *sidebar, p.border_soft, radius, 1.0, 0.96);
                    draw_text_left(ctx, "Sidebar", inset_rect(sidebar, 14.0 * s, 14.0 * s), font_heading(s), p.text, 0.98);
                }

                if body_cols.len() > 1 {
                    let center = body_cols[1];
                    let center_rows = LinearLayout::column(center).gap(gap)
                        .items(&[fr(3.0), fr(1.0)]).resolve();

                    if let Some(canvas) = center_rows.first() {
                        draw_fill(ctx, *canvas, p.surface, radius, 0.98);
                        draw_stroke(ctx, *canvas, p.border_soft, radius, 1.0, 0.96);
                        let center_inner = inset_rect(canvas, 18.0 * s, 18.0 * s);
                        draw_fill(ctx, center_inner, mix_hex(p.surface_deep, accent, 0.08), 18.0 * s, 0.92);
                        draw_stroke(ctx, center_inner, p.border_soft, 18.0 * s, 1.0, 0.88);
                        let title_r = Rect::new(center_inner.x + 32.0 * s, center_inner.y + center_inner.h * 0.5 - 18.0 * s, center_inner.w - 64.0 * s, 20.0 * s);
                        draw_text_center(ctx, "Centered Content", title_r, font_heading(s), p.text, 0.98);
                        let sub_r = Rect::new(center_inner.x + 40.0 * s, title_r.y + 22.0 * s, center_inner.w - 80.0 * s, 18.0 * s);
                        draw_text_center(ctx, "zstack() keeps the middle region aligned while side panels resize.", sub_r, font_meta(s), p.muted, 0.96);
                    }

                    if center_rows.len() > 1 {
                        draw_fill(ctx, center_rows[1], p.surface_deep, radius, 0.98);
                        draw_stroke(ctx, center_rows[1], p.border_soft, radius, 1.0, 0.96);
                        draw_text_left(ctx, "Footer Tools", inset_rect(&center_rows[1], 18.0 * s, 16.0 * s), font_heading(s), p.text, 0.98);
                    }
                }

                if body_cols.len() > 2 {
                    draw_fill(ctx, body_cols[2], p.surface_deep, radius, 0.98);
                    draw_stroke(ctx, body_cols[2], mix_hex(p.border_soft, accent, 0.26), radius, 1.0, 0.96);
                    draw_text_left(ctx, "Inspector", inset_rect(&body_cols[2], 14.0 * s, 14.0 * s), font_heading(s), p.text, 0.98);
                }
            }
        });
    }
}

// ── Page: Animation ──

fn draw_animation_page(ctx: &mut Context, state: &mut GalleryState, rect: Rect, s: f32, time: f64) {
    let p = make_gallery_palette(state);
    let accent = accent_hex(state);
    let rows = LinearLayout::column(rect).gap(12.0 * s)
        .items(&[px(154.0 * s), fr(1.0)]).resolve();

    // Selector
    if let Some(selector_rect) = rows.first() {
        draw_card(ctx, *selector_rect, "Animation Catalog", s, &p, |ctx, content| {
            let sections = LinearLayout::column(content).gap(12.0 * s)
                .items(&[fr(1.0), px(24.0 * s)]).resolve();

            if let Some(grid_rect) = sections.first() {
                let grid = GridLayout::new(*grid_rect, 3).rows(3).gap(10.0 * s);
                for (i, &demo_name) in DEMO_NAMES.iter().enumerate() {
                    let item = grid.cell(i);
                    let is_selected = state.selected_animation_demo == i;
                    let bg = if is_selected { mix_hex(p.surface_deep, accent, 0.28) } else { p.surface_deep };
                    draw_fill(ctx, item, bg, item.h * 0.5, if is_selected { 0.98 } else { 0.84 });
                    if is_selected {
                        draw_stroke(ctx, item, accent, item.h * 0.5, 1.0, 0.96);
                    }
                    draw_text_center(ctx, demo_name, item, font_body(s), if is_selected { p.text } else { p.muted }, 0.98);
                    if clicked(ctx, &item) { state.selected_animation_demo = i; }
                }
            }

            if sections.len() > 1 {
                let info = sections[1];
                let info_cols = LinearLayout::row(info).gap(14.0 * s)
                    .items(&[fr(1.0), px(222.0 * s)]).resolve();
                let demo_idx = state.selected_animation_demo.min(8);
                if let Some(summary_r) = info_cols.first() {
                    draw_text_left(ctx, DEMO_SUMMARIES[demo_idx], *summary_r, font_meta(s), p.muted, 0.96);
                }
                if info_cols.len() > 1 {
                    draw_fill(ctx, info_cols[1], p.surface_deep, info_cols[1].h * 0.5, 0.96);
                    draw_stroke(ctx, info_cols[1], accent, info_cols[1].h * 0.5, 1.0, 0.90);
                    draw_text_center(ctx, DEMO_API_LABELS[demo_idx], info_cols[1], font_meta(s), mix_hex(p.text, p.accent, 0.40), 0.98);
                }
            }
        });
    }

    // Stage
    if rows.len() > 1 {
        draw_card(ctx, rows[1], "Animation Stage", s, &p, |ctx, content| {
            // Draw current demo with crossfade using global_alpha
            for i in 0..9usize {
                let is_current = state.selected_animation_demo == i;
                let mix = ctx.presence(hash_str(&format!("anim_view_{}", i)), is_current);
                if mix <= 0.01 { continue; }
                let alpha = mix * mix * (3.0 - 2.0 * mix);
                ctx.set_global_alpha(alpha);
                ctx.push_clip(content);
                let scene_rect = Rect::new(content.x + (1.0 - alpha) * 14.0 * s, content.y, content.w, content.h);
                draw_demo_scene(ctx, i, scene_rect, s, time, &p);
                ctx.pop_clip();
                ctx.set_global_alpha(1.0);
            }
        });
    }
}

// ── Page: Dashboard ──

fn draw_dashboard_page(ctx: &mut Context, state: &GalleryState, rect: Rect, s: f32) {
    let p = make_gallery_palette(state);
    let accent = accent_hex(state);
    let page_rows = LinearLayout::column(rect).gap(18.0 * s)
        .items(&[fr(2.1), fr(1.0)]).resolve();

    if let Some(preview) = page_rows.first() {
        draw_card(ctx, *preview, "Dashboard Preview", s, &p, |ctx, content| {
            let cols = LinearLayout::row(content).gap(16.0 * s)
                .items(&[px(120.0 * s), fr(1.0)]).resolve();

            // Sidebar
            if let Some(sidebar) = cols.first() {
                draw_fill(ctx, *sidebar, p.surface_deep, 18.0 * s, 0.98);
                draw_stroke(ctx, *sidebar, p.border_soft, 18.0 * s, 1.0, 0.96);
                draw_text_left(ctx, "Dashboard", Rect::new(sidebar.x + 14.0 * s, sidebar.y + 16.0 * s, sidebar.w - 28.0 * s, 18.0 * s), font_heading(s), p.text, 1.0);
                let nav_items = ["Overview", "Orders", "Activity", "Settings"];
                for (i, label) in nav_items.iter().enumerate() {
                    let item = Rect::new(sidebar.x + 12.0 * s, sidebar.y + 48.0 * s + i as f32 * 42.0 * s, sidebar.w - 24.0 * s, 30.0 * s);
                    let bg = if i == 1 { nav_selected_fill(&p) } else { nav_idle_fill(&p) };
                    draw_fill(ctx, item, bg, 15.0 * s, if i == 1 { 0.98 } else { 0.84 });
                    draw_text_left(ctx, label, Rect::new(item.x + 12.0 * s, item.y + 7.0 * s, item.w - 24.0 * s, 16.0 * s),
                        font_body(s), if i == 1 { p.text } else { p.muted }, 0.98);
                }
            }

            // Main area
            if cols.len() > 1 {
                let main = cols[1];
                let main_rows = LinearLayout::column(main).gap(14.0 * s)
                    .items(&[px(76.0 * s), px(94.0 * s), fr(1.0)]).resolve();

                // Hero
                if let Some(hero) = main_rows.first() {
                    draw_fill(ctx, *hero, p.surface_deep, 18.0 * s, 0.98);
                    draw_stroke(ctx, *hero, p.border_soft, 18.0 * s, 1.0, 0.96);
                    draw_text_left(ctx, "Dashboard Preview", Rect::new(hero.x + 18.0 * s, hero.y + 14.0 * s, hero.w - 36.0 * s, 18.0 * s), font_heading(s), p.text, 1.0);
                    draw_text_left(ctx, "The full standalone example remains available as reference_dashboard_demo.cpp", Rect::new(hero.x + 18.0 * s, hero.y + 40.0 * s, hero.w - 36.0 * s, 16.0 * s), font_meta(s), p.muted, 0.96);
                }

                // Metric cards
                if main_rows.len() > 1 {
                    let cards = LinearLayout::row(main_rows[1]).gap(12.0 * s)
                        .items(&[fr(1.0), fr(1.0), fr(1.0)]).resolve();
                    let metrics: [(&str, &str, &str, &str); 3] = [
                        ("Revenue", "$128k", "up", "Month over month +18%"),
                        ("Orders", "1,284", "live", "Refreshed from shared state"),
                        ("Retention", "92%", "stable", "Reusable metric surface"),
                    ];
                    for (i, card_rect) in cards.iter().enumerate() {
                        let bg = mix_hex(p.surface_deep, accent, 0.10);
                        draw_fill(ctx, *card_rect, bg, 18.0 * s, 0.98);
                        draw_stroke(ctx, *card_rect, p.border_soft, 18.0 * s, 1.0, 0.90);
                        draw_text_left(ctx, metrics[i].0, Rect::new(card_rect.x + 16.0 * s, card_rect.y + 14.0 * s, card_rect.w - 32.0 * s, 16.0 * s), font_body(s), p.muted, 0.98);
                        draw_text_left(ctx, metrics[i].1, Rect::new(card_rect.x + 16.0 * s, card_rect.y + 38.0 * s, card_rect.w - 32.0 * s, 24.0 * s), font_heading(s), p.text, 0.98);
                        // Tag chip
                        let tag_w = 48.0 * s;
                        let tag_r = Rect::new(card_rect.x + card_rect.w - tag_w - 12.0 * s, card_rect.y + 14.0 * s, tag_w, 18.0 * s);
                        draw_fill(ctx, tag_r, mix_hex(p.surface_deep, accent, 0.24), tag_r.h * 0.5, 0.94);
                        draw_text_center(ctx, metrics[i].2, tag_r, font_meta(s), p.text, 0.98);
                        // Caption
                        draw_text_left(ctx, metrics[i].3, Rect::new(card_rect.x + 16.0 * s, card_rect.y + 66.0 * s, card_rect.w - 32.0 * s, 16.0 * s), font_meta(s), p.muted, 0.96);
                    }
                }

                // Activity chart
                if main_rows.len() > 2 {
                    let activity = main_rows[2];
                    draw_fill(ctx, activity, p.surface_deep, 18.0 * s, 0.98);
                    draw_stroke(ctx, activity, p.border_soft, 18.0 * s, 1.0, 0.96);
                    draw_text_left(ctx, "Activity", Rect::new(activity.x + 18.0 * s, activity.y + 14.0 * s, 120.0 * s, 18.0 * s), font_heading(s), p.text, 1.0);
                    let plot_bottom = activity.y + activity.h - 26.0 * s;
                    let mut bar_x = activity.x + 24.0 * s;
                    for i in 0..7 {
                        let height = (0.28 + ((i * 37) % 53) as f32 / 100.0) * (activity.h - 66.0 * s);
                        let hex = if i == 5 { accent } else { demo_track(&p) };
                        draw_fill(ctx, Rect::new(bar_x, plot_bottom - height, 18.0 * s, height), hex, 9.0 * s, if i == 5 { 0.98 } else { 0.86 });
                        bar_x += 28.0 * s;
                    }
                }
            }
        });
    }

    // Bottom info cards
    if page_rows.len() > 1 {
        let stats_cols = LinearLayout::row(page_rows[1]).gap(18.0 * s)
            .items(&[fr(1.0), fr(1.0)]).resolve();
        if let Some(c) = stats_cols.first() {
            draw_card(ctx, *c, "What This Proves", s, &p, |ctx, content| {
                draw_readonly(ctx, Rect::new(content.x, content.y, content.w, 36.0 * s), "Composition", "Sidebar + cards + chart + metrics", s, &p);
                draw_readonly(ctx, Rect::new(content.x, content.y + 44.0 * s, content.w, 36.0 * s), "Reuse", "Same primitives power the standalone dashboard", s, &p);
            });
        }
        if stats_cols.len() > 1 {
            draw_card(ctx, stats_cols[1], "Usage", s, &p, |ctx, content| {
                draw_text_left(ctx, "reference_dashboard_demo.cpp remains as the dedicated dashboard example target.", content, font_body(s), p.muted, 0.96);
            });
        }
    }
}

// ── Page: Settings ──

fn draw_settings_page(ctx: &mut Context, state: &mut GalleryState, rect: Rect, s: f32) {
    let p = make_gallery_palette(state);
    let cols = LinearLayout::row(rect).gap(18.0 * s)
        .items(&[fr(1.08), fr(0.92)]).resolve();

    // Settings panel
    if let Some(settings_rect) = cols.first() {
        draw_card(ctx, *settings_rect, "Gallery Settings", s, &p, |ctx, content| {
            let rows = LinearLayout::column(content).gap(12.0 * s)
                .items(&[
                    px(14.0 * s), px(38.0 * s),  // theme mode
                    px(14.0 * s), px(40.0 * s),  // accent
                    px(14.0 * s), px(42.0 * s),  // custom
                    px(36.0 * s), px(36.0 * s), px(36.0 * s),  // RGB
                    px(36.0 * s), px(36.0 * s),  // radius, blur
                ]).resolve();

            // Theme Mode
            if !rows.is_empty() { draw_text_left(ctx, "Theme Mode", rows[0], font_meta(s), p.muted, 0.96); }
            if rows.len() > 1 {
                let mode_cols = LinearLayout::row(rows[1]).gap(10.0 * s)
                    .items(&[fr(1.0), fr(1.0)]).resolve();
                if mode_cols.len() >= 2 {
                    let accent = accent_hex(state);
                    // Dark button
                    let dark_bg = if state.light_mode { p.surface_deep } else { nav_selected_fill(&p) };
                    let dark_border = if state.light_mode { p.border_soft } else { accent };
                    draw_fill(ctx, mode_cols[0], dark_bg, mode_cols[0].h * 0.5, 0.98);
                    draw_stroke(ctx, mode_cols[0], dark_border, mode_cols[0].h * 0.5, 1.0, 0.96);
                    draw_text_center(ctx, "Dark", mode_cols[0], font_body(s), if state.light_mode { p.muted } else { p.text }, 0.98);
                    // Light button
                    let light_bg = if state.light_mode { nav_selected_fill(&p) } else { p.surface_deep };
                    let light_border = if state.light_mode { accent } else { p.border_soft };
                    draw_fill(ctx, mode_cols[1], light_bg, mode_cols[1].h * 0.5, 0.98);
                    draw_stroke(ctx, mode_cols[1], light_border, mode_cols[1].h * 0.5, 1.0, 0.96);
                    draw_text_center(ctx, "Light", mode_cols[1], font_body(s), if state.light_mode { p.text } else { p.muted }, 0.98);
                    if clicked(ctx, &mode_cols[0]) { state.light_mode = false; }
                    if clicked(ctx, &mode_cols[1]) { state.light_mode = true; }
                }
            }

            // Accent Color
            if rows.len() > 2 { draw_text_left(ctx, "Accent Color", rows[2], font_meta(s), p.muted, 0.96); }
            if rows.len() > 3 {
                let swatch_size = 28.0 * s;
                let total = custom_accent_slot() + 1;
                let swatch_items: Vec<FlexLength> = (0..total).map(|_| px(swatch_size)).collect();
                let chips = LinearLayout::row(rows[3]).gap(10.0 * s).items(&swatch_items).resolve();
                let active = if accent_uses_custom(state) { custom_accent_slot() } else { state.accent_index };
                for (i, chip) in chips.iter().enumerate() {
                    let hex = if i == custom_accent_slot() { custom_accent_hex(state) } else { ACCENT_HEXES[i] };
                    draw_fill(ctx, *chip, hex, chip.w * 0.5, 0.98);
                    if i == custom_accent_slot() {
                        draw_stroke(ctx, inset_rect(chip, 4.0 * s, 4.0 * s), p.surface_alt, (chip.w - 8.0 * s) * 0.5, 1.0, 0.90);
                    }
                    if active == i {
                        draw_stroke(ctx, Rect::new(chip.x - 3.0, chip.y - 3.0, chip.w + 6.0, chip.h + 6.0), p.text, (chip.w + 6.0) * 0.5, 1.0, 0.92);
                    }
                    if clicked(ctx, chip) { state.accent_index = i; }
                }
            }

            // Custom RGB
            if rows.len() > 4 { draw_text_left(ctx, "Custom RGB", rows[4], font_meta(s), p.muted, 0.96); }
            if rows.len() > 5 {
                let custom_cols = LinearLayout::row(rows[5]).gap(10.0 * s)
                    .items(&[fr(0.24), fr(0.76)]).resolve();
                if custom_cols.len() >= 2 {
                    let custom_hex = custom_accent_hex(state);
                    draw_fill(ctx, custom_cols[0], custom_hex, 14.0 * s, 0.98);
                    let active = if accent_uses_custom(state) { custom_accent_slot() } else { state.accent_index };
                    draw_stroke(ctx, custom_cols[0], if active == custom_accent_slot() { p.text } else { p.border_soft }, 14.0 * s, 1.0, if active == custom_accent_slot() { 0.94 } else { 0.86 });
                    draw_fill(ctx, custom_cols[1], p.surface_deep, custom_cols[1].h * 0.5, 0.96);
                    draw_stroke(ctx, custom_cols[1], p.border_soft, custom_cols[1].h * 0.5, 1.0, 0.86);
                    draw_text_left(ctx, "Live custom accent", Rect::new(custom_cols[1].x + 12.0 * s, custom_cols[1].y + 6.0 * s, custom_cols[1].w - 24.0 * s, 14.0 * s), font_meta(s), p.muted, 0.96);
                    draw_text_left(ctx, &format_hex_color(custom_hex), Rect::new(custom_cols[1].x + 12.0 * s, custom_cols[1].y + 20.0 * s, custom_cols[1].w - 24.0 * s, 16.0 * s), font_body(s), p.text, 0.98);
                    if clicked(ctx, &rows[5]) { state.accent_index = custom_accent_slot(); }
                }
            }

            // RGB sliders
            if rows.len() > 6 {
                if ctx.slider_labeled(hash_str("red_slider"), rows[6], "Red", &mut state.custom_accent_r, 0.0, 255.0) {
                    state.accent_index = custom_accent_slot();
                }
            }
            if rows.len() > 7 {
                if ctx.slider_labeled(hash_str("green_slider"), rows[7], "Green", &mut state.custom_accent_g, 0.0, 255.0) {
                    state.accent_index = custom_accent_slot();
                }
            }
            if rows.len() > 8 {
                if ctx.slider_labeled(hash_str("blue_slider"), rows[8], "Blue", &mut state.custom_accent_b, 0.0, 255.0) {
                    state.accent_index = custom_accent_slot();
                }
            }
            if rows.len() > 9 {
                ctx.slider_labeled(hash_str("corner_radius"), rows[9], "Corner Radius", &mut state.layout_radius, 8.0, 28.0);
            }
            if rows.len() > 10 {
                ctx.slider_labeled(hash_str("glass_blur"), rows[10], "Glass Blur", &mut state.settings_blur, 0.0, 28.0);
            }
        });
    }

    // Live Preview
    if cols.len() > 1 {
        draw_card(ctx, cols[1], "Live Preview", s, &p, |ctx, content| {
            let preview_radius = state.layout_radius * s;
            let preview = inset_rect(&content, 6.0 * s, 6.0 * s);
            draw_fill(ctx, preview, preview_backdrop(&p), preview_radius, 1.0);
            draw_stroke(ctx, preview, accent_hex(state), preview_radius, 1.0, 0.76);
            let inner = inset_rect(&preview, 18.0 * s, 18.0 * s);
            let palette = make_gallery_palette(state);
            draw_stage_background(ctx, inner, s, &palette);
            draw_blur_reference(ctx, inset_rect(&inner, 34.0 * s, 34.0 * s), s * 0.82, &p);
            draw_actor_ex(ctx, Rect::new(inner.x + inner.w * 0.5 - 80.0 * s, inner.y + inner.h * 0.5 - 50.0 * s, 160.0 * s, 100.0 * s),
                s, &p, actor_glass_top(&p), actor_glass_bottom(&p), 1.0, state.settings_blur * s, 0.12);
        });
    }
}

// ── Page: Image ──

fn draw_image_page(ctx: &mut Context, state: &GalleryState, rect: Rect, s: f32) {
    let p = make_gallery_palette(state);
    let rows = LinearLayout::column(rect).gap(16.0 * s)
        .items(&[fr(1.18), fr(1.0)]).resolve();

    if let Some(top) = rows.first() {
        let top_cols = LinearLayout::row(*top).gap(16.0 * s)
            .items(&[fr(1.2), px(320.0 * s)]).resolve();

        if let Some(preview) = top_cols.first() {
            draw_card(ctx, *preview, "ui.image(...)", s, &p, |ctx, content| {
                // Placeholder for image - show colored rect
                draw_fill(ctx, content, mix_hex(p.surface_deep, p.accent, 0.15), 18.0 * s, 0.94);
                draw_text_center(ctx, "Image Preview", Rect::new(content.x, content.y + content.h * 0.5 - 10.0, content.w, 20.0), font_heading(s), p.text, 0.7);
                // Cover chip
                let chip = Rect::new(content.x + content.w - 140.0 * s, content.y + 16.0 * s, 124.0 * s, 28.0 * s);
                draw_fill(ctx, chip, mix_hex(p.surface_deep, p.accent, 0.24), chip.h * 0.5, 0.94);
                draw_stroke(ctx, chip, p.accent, chip.h * 0.5, 1.0, 0.90);
                draw_text_center(ctx, "cover()", chip, font_meta(s), p.text, 0.98);
            });
        }

        if top_cols.len() > 1 {
            draw_card(ctx, top_cols[1], "API Snapshot", s, &p, |ctx, content| {
                draw_fill(ctx, content, p.surface_deep, 18.0 * s, 0.94);
                draw_stroke(ctx, content, p.border_soft, 18.0 * s, 1.0, 0.88);
                draw_text_left(ctx, "Standalone image", Rect::new(content.x + 16.0 * s, content.y + 16.0 * s, content.w - 32.0 * s, 18.0 * s), font_body(s), p.text, 0.98);
                draw_text_left(ctx, "ui.image(path).in(rect).cover().draw();", Rect::new(content.x + 16.0 * s, content.y + 40.0 * s, content.w - 32.0 * s, 18.0 * s), font_meta(s), p.accent, 0.98);
                draw_text_left(ctx, "Texture fill on shape", Rect::new(content.x + 16.0 * s, content.y + 84.0 * s, content.w - 32.0 * s, 18.0 * s), font_body(s), p.text, 0.98);
                draw_text_left(ctx, "ui.shape().fill_image(path).image_cover().draw();", Rect::new(content.x + 16.0 * s, content.y + 108.0 * s, content.w - 32.0 * s, 18.0 * s), font_meta(s), p.accent, 0.98);
                draw_text_left(ctx, "Fit modes", Rect::new(content.x + 16.0 * s, content.y + 152.0 * s, content.w - 32.0 * s, 18.0 * s), font_body(s), p.text, 0.98);
                draw_text_left(ctx, "cover / contain / stretch / center", Rect::new(content.x + 16.0 * s, content.y + 176.0 * s, content.w - 32.0 * s, 18.0 * s), font_meta(s), p.muted, 0.98);
            });
        }
    }

    // Fit mode cards
    if rows.len() > 1 {
        let mode_cols = LinearLayout::row(rows[1]).gap(12.0 * s)
            .items(&[fr(1.0), fr(1.0), fr(1.0), fr(1.0)]).resolve();
        let modes = ["Cover", "Contain", "Stretch", "Center"];
        for (i, col) in mode_cols.iter().enumerate() {
            draw_card(ctx, *col, modes[i], s, &p, |ctx, content| {
                let stage = Rect::new(content.x, content.y, content.w, (content.h - 22.0 * s).max(0.0));
                draw_fill(ctx, stage, mix_hex(p.surface_deep, p.accent, 0.08 + 0.04 * i as f32), 16.0 * s, 0.96);
                draw_stroke(ctx, stage, p.border_soft, 16.0 * s, 1.0, 0.88);
                draw_text_center(ctx, modes[i], Rect::new(stage.x, stage.y + stage.h * 0.5 - 10.0, stage.w, 20.0), font_heading(s), p.text, 0.5);
                let footer = Rect::new(content.x, content.y + content.h - 18.0 * s, content.w, 18.0 * s);
                draw_text_center(ctx, modes[i], footer, font_meta(s), p.muted, 0.96);
            });
        }
    }
}

// ── Page: About ──

fn draw_about_page(ctx: &mut Context, state: &GalleryState, rect: Rect, s: f32) {
    let p = make_gallery_palette(state);
    let accent = accent_hex(state);
    let content = inset_rect(&rect, 32.0 * s, 32.0 * s);
    let hero_w = content.w.min(720.0 * s);
    let hero_h = (content.h * 0.48).min(272.0 * s);

    let rows = LinearLayout::column(content).gap(18.0 * s)
        .items(&[px(hero_h), fr(1.0)]).resolve();

    // Hero
    if let Some(hero_slot) = rows.first() {
        let hero_cols = LinearLayout::row(*hero_slot).gap(0.0)
            .items(&[fr(1.0), px(hero_w), fr(1.0)]).resolve();
        if hero_cols.len() > 1 {
            let hero = hero_cols[1];
            draw_fill(ctx, hero, p.surface_deep, 24.0 * s, 0.96);
            draw_stroke(ctx, hero, mix_hex(p.border_soft, accent, 0.18), 24.0 * s, 1.0, 0.90);
            let inner = inset_rect(&hero, 20.0 * s, 20.0 * s);
            let avatar_size = 72.0 * s;

            // Avatar placeholder
            let avatar = Rect::new(inner.x + (inner.w - avatar_size) * 0.5, inner.y, avatar_size, avatar_size);
            draw_fill(ctx, avatar, mix_hex(p.accent, 0xFFFFFF, 0.3), avatar_size * 0.5, 0.98);
            draw_stroke(ctx, avatar, mix_hex(accent, 0xFFFFFF, if p.light { 0.22 } else { 0.10 }), avatar_size * 0.5, 2.0, 0.96);
            draw_text_center(ctx, "E", avatar, font_display(s) * 1.5, p.text, 0.8);

            let title_y = inner.y + avatar_size + 8.0 * s;
            draw_text_center(ctx, "EUI", Rect::new(inner.x, title_y, inner.w, 28.0 * s), font_heading(s) * 2.0, mix_hex(p.text, accent, 0.24), 0.98);
            draw_text_center(ctx, "Created by SudoEvolve", Rect::new(inner.x, title_y + 32.0 * s, inner.w, 18.0 * s), font_body(s), p.text, 0.98);
            draw_text_center(ctx, "Immediate-mode GUI for crisp text, glass surfaces and fast desktop tooling iteration.",
                Rect::new(inner.x, title_y + 54.0 * s, inner.w, 18.0 * s), font_body(s), p.muted, 0.98);

            // Buttons
            let buttons_w = inner.w.min(420.0 * s);
            let btn_y = inner.y + inner.h - 42.0 * s;
            let btn_cols = LinearLayout::row(Rect::new(inner.x + (inner.w - buttons_w) * 0.5, btn_y, buttons_w, 42.0 * s))
                .gap(16.0 * s).items(&[fr(1.0), fr(1.0)]).resolve();
            if let Some(github) = btn_cols.first() {
                let github_hovered = hovered(ctx, github);
                let github_mix = ctx.presence(hash_str("about_github"), github_hovered);
                draw_fill(ctx, *github, mix_hex(p.accent, 0xFFFFFF, if p.light { 0.14 } else { 0.04 * github_mix }), github.h * 0.5, 0.98);
                draw_stroke(ctx, *github, mix_hex(p.accent, p.text, 0.12), github.h * 0.5, 1.0, 0.94);
                draw_icon(ctx, 0xF121, Rect::new(github.x + 24.0 * s, github.y + 13.0 * s, 18.0 * s, 18.0 * s),
                    if p.light { 0x0F172A } else { 0xFFFFFF }, 0.98);
                draw_text_left(ctx, "GitHub", Rect::new(github.x + 52.0 * s, github.y + 13.0 * s, github.w - 68.0 * s, 18.0 * s),
                    font_body(s), if p.light { 0x0F172A } else { 0xFFFFFF }, 0.98);
            }
            if btn_cols.len() > 1 {
                let mail = btn_cols[1];
                let mail_hovered = hovered(ctx, &mail);
                let mail_mix = ctx.presence(hash_str("about_mail"), mail_hovered);
                draw_fill(ctx, mail, p.surface_deep, mail.h * 0.5, 0.96);
                draw_stroke(ctx, mail, mix_hex(p.border_soft, accent, 0.20 + 0.18 * mail_mix), mail.h * 0.5, 1.0, 0.90);
                draw_icon(ctx, 0xF0E0, Rect::new(mail.x + 24.0 * s, mail.y + 13.0 * s, 18.0 * s, 18.0 * s), p.text, 0.98);
                draw_text_left(ctx, "Email", Rect::new(mail.x + 52.0 * s, mail.y + 13.0 * s, mail.w - 68.0 * s, 18.0 * s), font_body(s), p.text, 0.98);
            }
        }
    }

    // Info grid
    if rows.len() > 1 {
        let info_cols = LinearLayout::row(rows[1]).gap(16.0 * s)
            .items(&[fr(1.0), fr(1.0)]).resolve();

        if let Some(project) = info_cols.first() {
            draw_fill(ctx, *project, p.surface_deep, 22.0 * s, 0.96);
            draw_stroke(ctx, *project, p.border_soft, 22.0 * s, 1.0, 0.88);
            draw_text_left(ctx, "Project", Rect::new(project.x + 20.0 * s, project.y + 18.0 * s, project.w - 40.0 * s, 18.0 * s), font_heading(s), p.text, 0.98);
            draw_text_left(ctx, "Name: EUI", Rect::new(project.x + 20.0 * s, project.y + 52.0 * s, project.w - 40.0 * s, 16.0 * s), font_body(s), p.text, 0.98);
            draw_text_left(ctx, "Author: SudoEvolve", Rect::new(project.x + 20.0 * s, project.y + 78.0 * s, project.w - 40.0 * s, 16.0 * s), font_body(s), p.text, 0.98);
            draw_text_left(ctx, "Renderer: Modern OpenGL", Rect::new(project.x + 20.0 * s, project.y + 104.0 * s, project.w - 40.0 * s, 16.0 * s), font_meta(s), p.muted, 0.98);
            draw_text_left(ctx, "Repository: github.com/sudoevolve", Rect::new(project.x + 20.0 * s, project.y + 130.0 * s, project.w - 40.0 * s, 16.0 * s), font_meta(s), p.muted, 0.98);
        }

        if info_cols.len() > 1 {
            let license = info_cols[1];
            draw_fill(ctx, license, p.surface_deep, 22.0 * s, 0.96);
            draw_stroke(ctx, license, p.border_soft, 22.0 * s, 1.0, 0.88);
            draw_text_left(ctx, "License & Contact", Rect::new(license.x + 20.0 * s, license.y + 18.0 * s, license.w - 40.0 * s, 18.0 * s), font_heading(s), p.text, 0.98);
            draw_text_left(ctx, "MIT License", Rect::new(license.x + 20.0 * s, license.y + 52.0 * s, license.w - 40.0 * s, 16.0 * s), font_body(s), p.text, 0.98);
            draw_text_left(ctx, "Copyright (c) 2026 SudoEvolve", Rect::new(license.x + 20.0 * s, license.y + 78.0 * s, license.w - 40.0 * s, 16.0 * s), font_meta(s), p.muted, 0.98);
            draw_text_left(ctx, "Email: sudoevolve@gmail.com", Rect::new(license.x + 20.0 * s, license.y + 104.0 * s, license.w - 40.0 * s, 16.0 * s), font_body(s), p.text, 0.98);
            draw_text_left(ctx, "GitHub button opens the profile directly in your browser.", Rect::new(license.x + 20.0 * s, license.y + 132.0 * s, license.w - 40.0 * s, 28.0 * s), font_meta(s), p.muted, 0.96);
        }
    }
}

// ── Sidebar ──

fn draw_sidebar(ctx: &mut Context, state: &mut GalleryState, rect: Rect, s: f32) {
    let p = make_gallery_palette(state);
    let accent = accent_hex(state);
    draw_fill(ctx, rect, p.surface, 26.0 * s, 1.0);
    draw_stroke(ctx, rect, p.border, 26.0 * s, 1.0, 1.0);

    let inner = inset_rect(&rect, 18.0 * s, 18.0 * s);
    let main_rows = LinearLayout::column(inner).gap(8.0 * s)
        .items(&[px(54.0 * s), fr(1.0)]).resolve();

    // Header
    if let Some(header) = main_rows.first() {
        draw_text_left(ctx, "EUI Gallery", Rect::new(header.x, header.y, header.w, 22.0 * s), font_display(s), p.text, 1.0);
        draw_text_left(ctx, "Controls, design, layout, motion, dashboard, settings, image and about.",
            Rect::new(header.x, header.y + 26.0 * s, header.w, 18.0 * s), font_meta(s), p.muted, 0.96);
    }

    // Nav list
    if main_rows.len() > 1 {
        let list_rect = main_rows[1];
        let row_h = 42.0 * s;
        let items: Vec<FlexLength> = (0..8).map(|_| px(row_h)).collect();
        let nav_rows = LinearLayout::column(list_rect).gap(8.0 * s).items(&items).resolve();

        // Animated indicator
        let selected_idx = state.selected_page.min(7);
        if let Some(selected_row) = nav_rows.get(selected_idx) {
            let target_y = selected_row.y;
            let indicator_y = ctx.animated_value(hash_str("sidebar_indicator"), target_y);
            draw_fill(ctx, Rect::new(selected_row.x, indicator_y, selected_row.w, row_h), nav_selected_fill(&p), 18.0 * s, 0.96);
            draw_stroke(ctx, Rect::new(selected_row.x, indicator_y, selected_row.w, row_h), accent, 18.0 * s, 1.0, 0.72);
            draw_fill(ctx, Rect::new(selected_row.x, indicator_y, 4.0 * s, row_h), accent, 2.0 * s, 0.98);
        }

        // Items
        for (i, row) in nav_rows.iter().enumerate() {
            let is_selected = state.selected_page == i;
            let is_hovered = hovered(ctx, row);
            let hover_mix = ctx.presence(hash_str(&format!("sidebar_hover_{}", i)), is_hovered);

            if !is_selected && is_hovered {
                draw_fill(ctx, *row, p.surface_deep, 18.0 * s, 0.92 * hover_mix);
                draw_stroke(ctx, *row, p.border_soft, 18.0 * s, 1.0, 0.88 * hover_mix);
            }

            if clicked(ctx, row) { state.selected_page = i; }

            draw_icon(ctx, PAGE_ICONS[i],
                Rect::new(row.x + 14.0 * s, row.y + (row.h - 14.0 * s) * 0.5, 14.0 * s, 14.0 * s),
                if is_selected { accent } else { p.muted }, 0.98);
            draw_text_left(ctx, PAGE_NAMES[i],
                Rect::new(row.x + 38.0 * s, row.y + 12.0 * s, row.w - 54.0 * s, 16.0 * s),
                font_body(s), if is_selected { p.text } else { mix_hex(p.text, p.muted, 0.22) }, 0.98);
        }
    }
}

// ── Stage dispatcher ──

fn draw_stage(ctx: &mut Context, state: &mut GalleryState, rect: Rect, s: f32, time: f64) {
    let p = make_gallery_palette(state);
    let page_idx = state.selected_page.min(7);

    draw_fill(ctx, rect, p.surface, 26.0 * s, 1.0);
    draw_stroke(ctx, rect, p.border, 26.0 * s, 1.0, 1.0);

    let inner = inset_rect(&rect, 22.0 * s, 22.0 * s);
    let rows = LinearLayout::column(inner).gap(12.0 * s)
        .items(&[px(24.0 * s), px(18.0 * s), px(30.0 * s), fr(1.0)]).resolve();

    if !rows.is_empty() {
        draw_text_left(ctx, PAGE_NAMES[page_idx], rows[0], font_display(s), p.text, 1.0);
    }
    if rows.len() > 1 {
        draw_text_left(ctx, PAGE_SUMMARIES[page_idx], rows[1], font_meta(s), p.muted, 0.96);
    }
    if rows.len() > 2 {
        let chip = rows[2];
        draw_fill(ctx, chip, p.surface_deep, chip.h * 0.5, 0.96);
        draw_stroke(ctx, chip, p.border_soft, chip.h * 0.5, 1.0, 0.90);
        draw_text_center(ctx, PAGE_API_LABELS[page_idx], chip, font_meta(s), mix_hex(p.text, p.accent, 0.40), 0.98);
    }
    if rows.len() > 3 {
        let stage_rect = rows[3];
        match state.selected_page {
            0 => draw_basic_controls_page(ctx, state, stage_rect, s),
            1 => draw_design_page(ctx, state, stage_rect, s),
            2 => draw_layout_page(ctx, state, stage_rect, s),
            3 => draw_animation_page(ctx, state, stage_rect, s, time),
            4 => draw_dashboard_page(ctx, state, stage_rect, s),
            5 => draw_settings_page(ctx, state, stage_rect, s),
            6 => draw_image_page(ctx, state, stage_rect, s),
            _ => draw_about_page(ctx, state, stage_rect, s),
        }
    }
}

// ── Card helper ──

fn draw_card(ctx: &mut Context, rect: Rect, title: &str, s: f32, p: &GalleryPalette, body: impl FnOnce(&mut Context, Rect)) {
    // Card radius/padding/shadow matching C++ SurfaceBuilder(card): radius=22*s, padding=16 (unscaled), shadow(0,10,18)
    let card_radius = 22.0 * s;
    draw_shadow(ctx, rect, card_radius, 10.0, 18.0, 0x020617, 0.12);
    draw_fill(ctx, rect, p.surface_alt, card_radius, 1.0);
    draw_stroke(ctx, rect, p.border, card_radius, 1.0, 1.0);
    // C++ SurfaceBuilder does NOT push_clip for cards
    let inner = inset_rect(&rect, 16.0, 16.0);
    if !title.is_empty() {
        let title_font = font_heading(s);
        let title_height = title_font + 4.0;
        // C++ SurfaceBuilder: paint_text(title, title_rect, ..., &shell)
        // Uses theme text color and clips to entire card shell rect, not title_rect
        let title_rect = Rect::new(inner.x, inner.y, inner.w, title_height);
        let title_color = ctx.theme().text;
        ctx.paint_text_clipped(title_rect, title, title_font, title_color, TextAlign::Left, Some(&rect));
        let body_rect = Rect::new(inner.x, inner.y + title_height + 8.0, inner.w, (inner.h - title_height - 8.0).max(0.0));
        body(ctx, body_rect);
    } else {
        body(ctx, inner);
    }
}

// ── Readonly helper ──

fn draw_readonly(ctx: &mut Context, rect: Rect, label: &str, value: &str, _s: f32, _p: &GalleryPalette) {
    // Use the proper input_readonly widget matching C++ internal_input_readonly
    let id = hash_str(label) ^ hash_str(value);
    ctx.input_readonly(id, rect, label, value);
}

// ── Hash helper ──

fn hash_str(s: &str) -> u64 {
    let mut hash: u64 = 5381;
    for b in s.bytes() {
        hash = hash.wrapping_mul(33).wrapping_add(b as u64);
    }
    hash
}

// ── Main ──

fn main() {
    let mut state = GalleryState::default();

    let options = AppOptions {
        title: "EUI Gallery".to_string(),
        width: 1420,
        height: 860,
        continuous_render: true,
        ..Default::default()
    };

    eui::run_with_options(move |ctx, _ui| {
        let (vw, vh) = ctx.viewport_size();
        let s = 1.0_f32.max(ctx.dpi_scale());
        let time = ctx.input().time_seconds;

        // Set theme based on state
        let accent = accent_hex(&state);
        let accent_color = color_from_hex(accent, 1.0);
        let mode = if state.light_mode { ThemeMode::Light } else { ThemeMode::Dark };
        ctx.set_theme(make_theme(mode, &accent_color));
        ctx.set_corner_radius(8.0_f32.max(state.layout_radius * s * 0.7));

        let p = make_gallery_palette(&state);
        let margin = 18.0 * s;
        let frame_rect = Rect::new(margin, margin, (vw - margin * 2.0).max(0.0), (vh - margin * 2.0).max(0.0));

        // Shell background with shadow + gradient matching C++
        draw_shadow(ctx, frame_rect, 30.0 * s, 14.0 * s, 28.0 * s, panel_shadow_hex(&p), if p.light { 0.10 } else { 0.18 });
        draw_gradient(ctx, frame_rect, p.shell_top, p.shell_bottom, 30.0 * s, 1.0);
        draw_stroke(ctx, frame_rect, p.border, 30.0 * s, 1.0, 1.0);

        let shell_inner = inset_rect(&frame_rect, 18.0 * s, 18.0 * s);
        let shell_rows = LinearLayout::column(shell_inner).gap(16.0 * s)
            .items(&[px(68.0 * s), fr(1.0)]).resolve();

        // Header
        if let Some(header) = shell_rows.first() {
            draw_fill(ctx, *header, p.surface_alt, 22.0 * s, 0.96);
            draw_stroke(ctx, *header, p.border, 22.0 * s, 1.0, 1.0);
            draw_text_left(ctx, "EUI Gallery",
                Rect::new(header.x + 22.0 * s, header.y + 14.0 * s, header.w - 44.0 * s, 20.0 * s),
                font_display(s), p.text, 1.0);
            draw_text_left(ctx, "Gallery now includes a dedicated image page for ui.image() and textured shape fills.",
                Rect::new(header.x + 22.0 * s, header.y + 36.0 * s, header.w - 44.0 * s, 16.0 * s),
                font_meta(s), p.muted, 0.96);
        }

        // Body: sidebar + stage
        if shell_rows.len() > 1 {
            let body = shell_rows[1];
            let sidebar_w = (320.0 * s).min(body.w * 0.26);
            let body_cols = LinearLayout::row(body).gap(18.0 * s)
                .items(&[px(sidebar_w), fr(1.0)]).resolve();

            if let Some(sidebar_rect) = body_cols.first() {
                draw_sidebar(ctx, &mut state, *sidebar_rect, s);
            }
            if body_cols.len() > 1 {
                draw_stage(ctx, &mut state, body_cols[1], s, time);
            }
        }
    }, options);
}
