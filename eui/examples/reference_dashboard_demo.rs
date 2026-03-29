use eui::*;
use eui::core::context_utils::{context_hash_sv, context_hash_mix};
use eui::quick::gfx;
use eui::quick::primitive_painter;

// ── Enums ──

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
enum NavItem {
    Dashboard = 0,
    Deliveries = 1,
    Documents = 2,
    Wallet = 3,
    Analytics = 4,
}

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
enum OpenMenu {
    None = 0,
    Orders = 1,
    Activity = 2,
    Couriers = 3,
}

// ── Structs ──

#[derive(Clone, Copy)]
#[allow(dead_code)]
struct Palette {
    dark: bool,
    shell: u32,
    panel: u32,
    panel_alt: u32,
    divider: u32,
    divider_strong: u32,
    text: u32,
    muted: u32,
    soft: u32,
    accent: u32,
    accent_bright: u32,
    accent_soft: u32,
    chip: u32,
    chip_active: u32,
    chip_active_text: u32,
    pill_dark: u32,
    pill_blue: u32,
    pill_blue_text: u32,
}

struct MetricCard {
    title: &'static str,
    value: &'static str,
    delta: &'static str,
}

struct OrderRow {
    id: &'static str,
    address: &'static str,
    expires: &'static str,
    status: &'static str,
    in_transit: bool,
}

struct Courier {
    name: &'static str,
    orders: &'static str,
    avatar_top: u32,
    avatar_bottom: u32,
    hair: u32,
    shirt: u32,
    skin: u32,
}

struct PageCopy {
    badge: &'static str,
    orders_title: &'static str,
    activity_title: &'static str,
    couriers_title: &'static str,
    highlighted_metric: i32,
}

struct AccentTheme {
    _name: &'static str,
    accent: u32,
}

struct DashboardState {
    nav: NavItem,
    filter_index: i32,
    selected_order: i32,
    orders_range: i32,
    activity_range: i32,
    couriers_range: i32,
    hovered_bar: i32,
    search_text: String,
    open_menu: OpenMenu,
    settings_open: bool,
    theme_dark: bool,
    accent_index: i32,
    overlay_block_rect: Rect,
    overlay_block_active: bool,
}

impl Default for DashboardState {
    fn default() -> Self {
        Self {
            nav: NavItem::Dashboard,
            filter_index: 2,
            selected_order: 4,
            orders_range: 0,
            activity_range: 1,
            couriers_range: 0,
            hovered_bar: -1,
            search_text: String::new(),
            open_menu: OpenMenu::None,
            settings_open: false,
            theme_dark: true,
            accent_index: 0,
            overlay_block_rect: Rect::ZERO,
            overlay_block_active: false,
        }
    }
}

// ── Static data ──

const K_METRICS: [MetricCard; 4] = [
    MetricCard { title: "On-Time Deliveries", value: "96%", delta: "+2% vs last month" },
    MetricCard { title: "Total Deliveries", value: "568", delta: "+24% vs last month" },
    MetricCard { title: "Total Vehicles", value: "159", delta: "+27 vs last month" },
    MetricCard { title: "Driver Behavior Score", value: "95%", delta: "+1% vs last month" },
];
const K_DELIVERY_METRICS: [MetricCard; 4] = [
    MetricCard { title: "Active Routes", value: "128", delta: "+14 vs last week" },
    MetricCard { title: "Delivered Parcels", value: "1,284", delta: "+11% vs last week" },
    MetricCard { title: "Vehicles On Road", value: "64", delta: "+6 vs yesterday" },
    MetricCard { title: "Missed Windows", value: "3.2%", delta: "-0.8% vs last week" },
];
const K_DOCUMENT_METRICS: [MetricCard; 4] = [
    MetricCard { title: "Open Waybills", value: "182", delta: "+17 pending" },
    MetricCard { title: "Approved Today", value: "96", delta: "+22 vs yesterday" },
    MetricCard { title: "Awaiting Signature", value: "34", delta: "-4 vs yesterday" },
    MetricCard { title: "Document Accuracy", value: "99.1%", delta: "+0.3% vs last week" },
];
const K_WALLET_METRICS: [MetricCard; 4] = [
    MetricCard { title: "Pending Payouts", value: "$18.4k", delta: "+$2.2k today" },
    MetricCard { title: "Processed Today", value: "$42.8k", delta: "+12% vs yesterday" },
    MetricCard { title: "Driver Bonuses", value: "$6.9k", delta: "+$940 today" },
    MetricCard { title: "Failed Charges", value: "0.4%", delta: "-0.2% vs last week" },
];
const K_ANALYTICS_METRICS: [MetricCard; 4] = [
    MetricCard { title: "Conversion Rate", value: "18.2%", delta: "+1.4% vs last week" },
    MetricCard { title: "Avg Route Time", value: "28 min", delta: "-3 min vs last week" },
    MetricCard { title: "Demand Peaks", value: "4", delta: "+1 region" },
    MetricCard { title: "Forecast Score", value: "92%", delta: "+2% vs last month" },
];

const K_ORDERS: [OrderRow; 6] = [
    OrderRow { id: "#10234", address: "47 Maple Ave, Cambridge", expires: "26 Jan", status: "Picked up", in_transit: false },
    OrderRow { id: "#10444", address: "99 Ocean Rd, Brooklyn, NY", expires: "28 Jan", status: "In transit", in_transit: true },
    OrderRow { id: "#10345", address: "22 Palm St, San Diego", expires: "30 Jan", status: "In transit", in_transit: true },
    OrderRow { id: "#10376", address: "21 King Rd, Ottawa", expires: "6 Feb", status: "Picked up", in_transit: false },
    OrderRow { id: "#10254", address: "912nd Ave, Portland", expires: "3 Feb", status: "Picked up", in_transit: false },
    OrderRow { id: "#10254", address: "79 Lakeshore Blvd, Kelowna", expires: "8 Feb", status: "Picked up", in_transit: false },
];

const K_COURIERS: [Courier; 4] = [
    Courier { name: "Adem Barnes", orders: "23 orders", avatar_top: 0xE3EEF9, avatar_bottom: 0xC9D8EA, hair: 0x4A5565, shirt: 0x6E8EB7, skin: 0xF1C8A8 },
    Courier { name: "Annette Black", orders: "23 orders", avatar_top: 0xF6D5E3, avatar_bottom: 0xD59EC0, hair: 0x915775, shirt: 0x4F6FA8, skin: 0xF2C5AD },
    Courier { name: "Kristin Miles", orders: "21 orders", avatar_top: 0xF7E7D7, avatar_bottom: 0xE1C9AF, hair: 0xC6A66C, shirt: 0x90A1C4, skin: 0xF4CFB2 },
    Courier { name: "Marvin McKinney", orders: "21 orders", avatar_top: 0xECE7E0, avatar_bottom: 0xD2C4B5, hair: 0x5C4B3F, shirt: 0x8F8177, skin: 0xE8C09A },
];

const K_BARS_WEEK_A: [f32; 14] = [4.0, 15.0, 29.0, 35.0, 25.0, 16.0, 30.0, 38.0, 34.0, 14.0, 20.0, 26.0, 40.0, 36.0];
const K_BARS_WEEK_B: [f32; 14] = [6.0, 18.0, 24.0, 31.0, 27.0, 19.0, 28.0, 36.0, 32.0, 17.0, 23.0, 29.0, 37.0, 34.0];
const K_BARS_MONTH: [f32; 14] = [8.0, 11.0, 19.0, 27.0, 24.0, 22.0, 25.0, 33.0, 30.0, 26.0, 29.0, 36.0, 41.0, 39.0];
const K_BAR_LABELS: [&str; 14] = [
    "5 Jan", "6 Jan", "7 Jan", "8 Jan", "9 Jan", "10 Jan", "11 Jan",
    "12 Jan", "13 Jan", "14 Jan", "15 Jan", "16 Jan", "17 Jan", "18 Jan",
];

const K_ORDERS_RANGES: [&str; 3] = ["This week", "This month", "This quarter"];
const K_ACTIVITY_RANGES: [&str; 3] = ["Last 7 days", "Last 2 weeks", "Last month"];
const K_COURIERS_RANGES: [&str; 2] = ["This week", "This month"];
const K_FILTER_LABELS: [&str; 4] = ["Pending", "Responded", "Assigned", "Completed"];
const K_FILTER_COUNTS: [&str; 4] = ["21", "24", "15", "76"];
const K_SIDEBAR_ICONS: [u32; 5] = [0xF00A, 0xF0D1, 0xF15C, 0xF555, 0xF080];
const K_PAGES: [PageCopy; 5] = [
    PageCopy { badge: "Dashboard", orders_title: "Orders", activity_title: "Activity", couriers_title: "Top couriers", highlighted_metric: 0 },
    PageCopy { badge: "Deliveries", orders_title: "Delivery queue", activity_title: "Route activity", couriers_title: "Drivers on shift", highlighted_metric: 1 },
    PageCopy { badge: "Documents", orders_title: "Waybills", activity_title: "Approval flow", couriers_title: "Owners", highlighted_metric: 2 },
    PageCopy { badge: "Wallet", orders_title: "Payouts", activity_title: "Cash movement", couriers_title: "Top earners", highlighted_metric: 3 },
    PageCopy { badge: "Analytics", orders_title: "Segments", activity_title: "Traffic volume", couriers_title: "Top regions", highlighted_metric: 1 },
];

const K_ACCENT_THEMES: [AccentTheme; 12] = [
    AccentTheme { _name: "Indigo", accent: 0x5E76D8 },
    AccentTheme { _name: "Sky", accent: 0x46A9FF },
    AccentTheme { _name: "Cyan", accent: 0x33B7D6 },
    AccentTheme { _name: "Mint", accent: 0x38C89D },
    AccentTheme { _name: "Emerald", accent: 0x32B882 },
    AccentTheme { _name: "Lime", accent: 0x79C64A },
    AccentTheme { _name: "Amber", accent: 0xD7A43B },
    AccentTheme { _name: "Orange", accent: 0xE58C42 },
    AccentTheme { _name: "Coral", accent: 0xE06A78 },
    AccentTheme { _name: "Rose", accent: 0xD96CB3 },
    AccentTheme { _name: "Violet", accent: 0x8B72E8 },
    AccentTheme { _name: "Slate", accent: 0x7C8AA6 },
];

// ── Utility functions ──

fn mix_hex(lhs: u32, rhs: u32, t: f32) -> u32 {
    let t = t.clamp(0.0, 1.0);
    let mix_ch = |shift: u32| -> u32 {
        let a = ((lhs >> shift) & 0xff) as f32;
        let b = ((rhs >> shift) & 0xff) as f32;
        (a + (b - a) * t).round() as u32
    };
    (mix_ch(16) << 16) | (mix_ch(8) << 8) | mix_ch(0)
}

fn lighten_hex(hex: u32, amount: f32) -> u32 { mix_hex(hex, 0xFFFFFF, amount) }
fn darken_hex(hex: u32, amount: f32) -> u32 { mix_hex(hex, 0x000000, amount) }

fn make_palette(dark: bool, accent_index: i32) -> Palette {
    let safe_index = accent_index.clamp(0, K_ACCENT_THEMES.len() as i32 - 1) as usize;
    let accent = K_ACCENT_THEMES[safe_index].accent;
    let accent_bright = if dark { lighten_hex(accent, 0.42) } else { darken_hex(accent, 0.10) };
    let accent_soft = if dark { mix_hex(accent, 0x0D1322, 0.78) } else { lighten_hex(accent, 0.86) };

    if dark {
        Palette {
            dark: true,
            shell: 0x171A28,
            panel: 0x1A1D2C,
            panel_alt: 0x1E2234,
            divider: 0x33384B,
            divider_strong: 0x42485F,
            text: 0xF5F6FB,
            muted: 0xA2A8BF,
            soft: 0x7D839C,
            accent,
            accent_bright,
            accent_soft,
            chip: 0x222738,
            chip_active: lighten_hex(accent, 0.76),
            chip_active_text: 0x171A28,
            pill_dark: 0x222737,
            pill_blue: lighten_hex(accent, 0.72),
            pill_blue_text: 0x1B2436,
        }
    } else {
        Palette {
            dark: false,
            shell: 0xEDF2F8,
            panel: 0xFFFFFF,
            panel_alt: 0xF3F6FB,
            divider: 0xD9E1EC,
            divider_strong: 0xC7D2E1,
            text: 0x182033,
            muted: 0x6D778A,
            soft: 0x8F98AA,
            accent,
            accent_bright,
            accent_soft,
            chip: 0xEEF3F9,
            chip_active: accent,
            chip_active_text: 0xF9FBFF,
            pill_dark: 0xE6ECF5,
            pill_blue: lighten_hex(accent, 0.82),
            pill_blue_text: 0x182033,
        }
    }
}

fn active_bars(state: &DashboardState) -> &[f32; 14] {
    if state.activity_range == 0 { &K_BARS_WEEK_B }
    else if state.activity_range == 2 { &K_BARS_MONTH }
    else { &K_BARS_WEEK_A }
}

fn page_copy(nav: NavItem) -> &'static PageCopy {
    &K_PAGES[nav as usize]
}

fn page_metrics(nav: NavItem) -> &'static [MetricCard; 4] {
    match nav {
        NavItem::Deliveries => &K_DELIVERY_METRICS,
        NavItem::Documents => &K_DOCUMENT_METRICS,
        NavItem::Wallet => &K_WALLET_METRICS,
        NavItem::Analytics => &K_ANALYTICS_METRICS,
        NavItem::Dashboard => &K_METRICS,
    }
}

fn clamp01(v: f32) -> f32 { v.clamp(0.0, 1.0) }
fn smoothstep01(v: f32) -> f32 { let t = clamp01(v); t * t * (3.0 - 2.0 * t) }
fn menu_slot(menu: OpenMenu) -> usize { (menu as i32 - 1) as usize }

fn inset_rect(r: Rect, padding: f32) -> Rect {
    Rect::new(r.x + padding, r.y + padding, (r.w - padding * 2.0).max(0.0), (r.h - padding * 2.0).max(0.0))
}

fn inset_rect_hv(r: Rect, h: f32, v: f32) -> Rect {
    Rect::new(r.x + h, r.y + v, (r.w - h * 2.0).max(0.0), (r.h - v * 2.0).max(0.0))
}

fn inset_rect_ltrb(r: Rect, left: f32, top: f32, right: f32, bottom: f32) -> Rect {
    Rect::new(r.x + left, r.y + top, (r.w - left - right).max(0.0), (r.h - top - bottom).max(0.0))
}

fn color_from_hex(hex: u32, alpha: f32) -> Color {
    let inv = 1.0 / 255.0;
    Color::new(
        ((hex >> 16) & 0xff) as f32 * inv,
        ((hex >> 8) & 0xff) as f32 * inv,
        (hex & 0xff) as f32 * inv,
        alpha,
    )
}

// ── Input helpers ──

fn pointer_captured_by_overlay(state: &DashboardState, input: &InputState) -> bool {
    state.overlay_block_active && state.overlay_block_rect.contains(input.mouse_x, input.mouse_y)
}

fn set_overlay_block(state: &mut DashboardState, rect: Rect) {
    state.overlay_block_rect = rect;
    state.overlay_block_active = rect.w > 0.0 && rect.h > 0.0;
}

fn is_hovered(input: &InputState, rect: &Rect) -> bool {
    rect.contains(input.mouse_x, input.mouse_y)
}

fn is_clicked(input: &InputState, rect: &Rect) -> bool {
    input.mouse_pressed && is_hovered(input, rect)
}

fn interactive_hovered(state: &DashboardState, input: &InputState, rect: &Rect) -> bool {
    !pointer_captured_by_overlay(state, input) && is_hovered(input, rect)
}

fn interactive_clicked(state: &DashboardState, input: &InputState, rect: &Rect) -> bool {
    input.mouse_pressed && interactive_hovered(state, input, rect)
}

// ── Drawing helpers ──

fn draw_fill(ctx: &mut Context, r: Rect, hex: u32, radius: f32, alpha: f32) {
    ctx.paint_filled_rect(r, color_from_hex(hex, alpha), radius);
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

fn draw_icon(ctx: &mut Context, codepoint: u32, r: Rect, hex: u32, alpha: f32) {
    ctx.paint_glyph(r, codepoint, color_from_hex(hex, alpha), r.h.min(r.w) * 0.72);
}

fn draw_soft_glow(ctx: &mut Context, r: Rect, inner: u32, outer: u32, radius: f32, alpha: f32) {
    let brush = gfx::radial_gradient_brush(
        gfx::color_from_hex(inner, alpha),
        gfx::color_from_hex(outer, 0.0),
        radius,
    );
    ctx.paint_filled_rect_with_brush(r, brush, r.w.max(r.h) * 0.5);
}

/// Paint a shape with fill + stroke + shadow + blur + opacity (maps to C++ ui.shape()...draw())
fn paint_shape(ctx: &mut Context, r: Rect, radius: f32, fill_hex: u32, fill_alpha: f32,
               stroke_hex: u32, stroke_width: f32, stroke_alpha: f32,
               shadow_ox: f32, shadow_oy: f32, shadow_blur: f32, shadow_hex: u32, shadow_alpha: f32,
               blur_radius: f32, backdrop_blur: f32, opacity: f32) {
    use eui::graphics::primitives::*;
    let mut prim = RectanglePrimitive::new();
    prim.rect = r;
    prim.radius = CornerRadius::uniform(radius);
    prim.fill = gfx::solid_hex(fill_hex, fill_alpha);
    if stroke_width > 0.0 && stroke_alpha > 0.0 {
        prim.stroke = Stroke {
            width: stroke_width,
            brush: gfx::solid_hex(stroke_hex, stroke_alpha),
        };
    }
    if shadow_alpha > 0.0 {
        prim.shadow = Shadow {
            offset_x: shadow_ox,
            offset_y: shadow_oy,
            blur_radius: shadow_blur,
            spread: 0.0,
            color: GfxColor::from(color_from_hex(shadow_hex, shadow_alpha)),
        };
    }
    if blur_radius > 0.0 || backdrop_blur > 0.0 {
        prim.blur = Blur { radius: blur_radius, backdrop_radius: backdrop_blur };
    }
    if opacity < 1.0 {
        prim.opacity = opacity;
    }
    primitive_painter::paint_rectangle(ctx, &prim);
}

/// Simplified paint_shape for common fill+stroke case
fn paint_fill_stroke(ctx: &mut Context, r: Rect, radius: f32,
                     fill_hex: u32, fill_alpha: f32,
                     stroke_hex: u32, stroke_width: f32, stroke_alpha: f32) {
    draw_fill(ctx, r, fill_hex, radius, fill_alpha);
    if stroke_width > 0.0 && stroke_alpha > 0.0 {
        draw_stroke(ctx, r, stroke_hex, radius, stroke_width, stroke_alpha);
    }
}

/// Paint a shape with fill + shadow (no blur)
fn paint_fill_shadow(ctx: &mut Context, r: Rect, radius: f32,
                     fill_hex: u32, fill_alpha: f32,
                     shadow_oy: f32, shadow_blur: f32, shadow_hex: u32, shadow_alpha: f32) {
    paint_shape(ctx, r, radius, fill_hex, fill_alpha, 0, 0.0, 0.0,
                0.0, shadow_oy, shadow_blur, shadow_hex, shadow_alpha, 0.0, 0.0, 1.0);
}

/// Paint a gradient fill using vertical_gradient brush
fn paint_gradient(ctx: &mut Context, r: Rect, top_hex: u32, bottom_hex: u32, alpha: f32) {
    let brush = gfx::vertical_gradient(
        gfx::color_from_hex(top_hex, alpha),
        gfx::color_from_hex(bottom_hex, alpha),
    );
    ctx.paint_filled_rect_with_brush(r, brush, 0.0);
}

// ── UI Components ──

fn draw_avatar(ctx: &mut Context, rect: Rect, avatar: &Courier, scale: f32) {
    let radius = rect.w * 0.5;
    draw_fill(ctx, rect, 0xFFFFFF, radius, 0.92);
    let inner = inset_rect(rect, 2.0 * scale);
    let ir = inner.w * 0.5;
    paint_gradient(ctx, inner, avatar.avatar_top, avatar.avatar_bottom, 1.0);
    // Overdraw with rounded clip approximation via large radius
    draw_fill(ctx, Rect::new(inner.x + inner.w * 0.18, inner.y + inner.h * 0.58, inner.w * 0.64, inner.h * 0.34), avatar.shirt, inner.h * 0.17, 0.95);
    draw_fill(ctx, Rect::new(inner.x + inner.w * 0.43, inner.y + inner.h * 0.46, inner.w * 0.14, inner.h * 0.11), avatar.skin, inner.w * 0.06, 1.0);
    draw_fill(ctx, Rect::new(inner.x + inner.w * 0.30, inner.y + inner.h * 0.19, inner.w * 0.40, inner.h * 0.40), avatar.skin, inner.w * 0.20, 1.0);
    draw_fill(ctx, Rect::new(inner.x + inner.w * 0.20, inner.y + inner.h * 0.06, inner.w * 0.60, inner.h * 0.36), avatar.hair, inner.w * 0.30, 0.98);
    draw_fill(ctx, Rect::new(inner.x + inner.w * 0.19, inner.y + inner.h * 0.24, inner.w * 0.12, inner.h * 0.22), avatar.hair, inner.w * 0.06, 0.98);
    draw_fill(ctx, Rect::new(inner.x + inner.w * 0.69, inner.y + inner.h * 0.24, inner.w * 0.12, inner.h * 0.22), avatar.hair, inner.w * 0.06, 0.98);
    // Re-clip avatar circle by drawing an outline on top (the gradient doesn't have radius, so we approximate)
    let _ = ir; // avatar_top gradient is actually drawn without radius, matching C++ ui.shape().in(inner).radius(ir).gradient(...)
}

fn draw_top_profile(ctx: &mut Context, rect: Rect, scale: f32, palette: &Palette) {
    let hero = Courier { name: "Zaina Riddle", orders: "Admin", avatar_top: 0xF0D7E4, avatar_bottom: 0xD89FBE, hair: 0x9A6381, shirt: 0x5D76A9, skin: 0xF2C7AF };
    let avatar_rect = Rect::new(rect.x + 18.0 * scale, rect.y + (rect.h - 40.0 * scale) * 0.5, 40.0 * scale, 40.0 * scale);
    draw_avatar(ctx, avatar_rect, &hero, scale);
    draw_text_left(ctx, "Zaina Riddle",
        Rect::new(avatar_rect.x + avatar_rect.w + 14.0 * scale, rect.y + 10.0 * scale,
                  rect.w - avatar_rect.w - 40.0 * scale, 18.0 * scale),
        13.5 * scale, palette.text, 0.98);
    draw_text_left(ctx, "Admin",
        Rect::new(avatar_rect.x + avatar_rect.w + 14.0 * scale, rect.y + 32.0 * scale,
                  rect.w - avatar_rect.w - 40.0 * scale, 16.0 * scale),
        11.0 * scale, palette.muted, 0.95);
}

fn draw_dropdown_control(
    ctx: &mut Context,
    input: &InputState,
    button: Rect,
    menu_id: OpenMenu,
    state: &mut DashboardState,
    selected_index: &mut i32,
    labels: &[&str],
    scale: f32,
    palette: &Palette,
) {
    let menu_index = menu_slot(menu_id);
    let menu_motion_id = context_hash_mix(context_hash_sv("dashboard/dropdown"), (menu_index + 1) as u64);
    let radius = button.h * 0.5;
    let button_hovered = interactive_hovered(state, input, &button);
    if interactive_clicked(state, input, &button) {
        if state.open_menu != menu_id {
            ctx.animated_value_reset(menu_motion_id, 0.0);
        }
        state.open_menu = if state.open_menu == menu_id { OpenMenu::None } else { menu_id };
    }

    let menu_mix = smoothstep01(ctx.presence_ex(menu_motion_id, state.open_menu == menu_id, 18.0, 14.0));

    paint_fill_stroke(ctx, button, radius,
        palette.chip, if button_hovered { 0.96 } else { 0.90 },
        if button_hovered { palette.accent_bright } else { palette.divider },
        1.0, if button_hovered { 0.34 } else { 1.0 });
    draw_text_center(ctx, labels[*selected_index as usize],
        inset_rect_ltrb(button, 12.0 * scale, 0.0, 22.0 * scale, 0.0),
        11.5 * scale, palette.text, 0.96);
    draw_icon(ctx, 0xF078,
        Rect::new(button.x + button.w - 22.0 * scale, button.y + (button.h - 12.0 * scale) * 0.5, 12.0 * scale, 12.0 * scale),
        palette.text, 0.82);

    if menu_mix <= 0.01 {
        return;
    }

    let n = labels.len();
    let base_menu = Rect::new(
        button.x,
        button.y + button.h + 8.0 * scale,
        button.w.max(132.0 * scale),
        n as f32 * 34.0 * scale + 10.0 * scale,
    );
    let menu = Rect::new(
        base_menu.x,
        base_menu.y + (1.0 - menu_mix) * 10.0 * scale,
        base_menu.w,
        base_menu.h,
    );
    set_overlay_block(state, Rect::new(menu.x - 10.0 * scale, menu.y - 10.0 * scale, menu.w + 20.0 * scale, menu.h + 20.0 * scale));

    // Backdrop blur + fill
    paint_shape(ctx, menu, 12.0 * scale,
        0xF7FAFF, (if palette.dark { 0.05 } else { 0.12 }) * menu_mix,
        0xFFFFFF, 1.0, (if palette.dark { 0.10 } else { 0.16 }) * menu_mix,
        0.0, 2.0 * scale, 8.0 * scale, 0x030712, 0.06 * menu_mix,
        26.0 * scale, 26.0 * scale, 1.0);
    paint_fill_stroke(ctx, inset_rect(menu, 1.0), 11.0 * scale,
        palette.panel_alt, (if palette.dark { 0.34 } else { 0.46 }) * menu_mix,
        palette.divider, 1.0, 0.68 * menu_mix);

    ctx.set_global_alpha(menu_mix);
    ctx.push_clip(menu);
    for i in 0..n {
        let item = Rect::new(
            menu.x + 5.0 * scale,
            menu.y + 5.0 * scale + i as f32 * 34.0 * scale + (1.0 - menu_mix) * 6.0 * scale,
            menu.w - 10.0 * scale,
            30.0 * scale,
        );
        let item_hovered = is_hovered(input, &item);
        let item_selected = *selected_index == i as i32;
        if item_selected || item_hovered {
            paint_fill_shadow(ctx, item, item.h * 0.5,
                if item_selected { palette.chip_active } else { palette.panel_alt },
                if item_selected { 1.0 } else { 0.92 },
                2.0 * scale, 6.0 * scale, 0x030712,
                if item_selected { 0.08 } else { 0.04 });
        }
        draw_text_left(ctx, labels[i],
            Rect::new(item.x + 12.0 * scale, item.y + 7.0 * scale, item.w - 24.0 * scale, 16.0 * scale),
            11.5 * scale, if item_selected { palette.chip_active_text } else { palette.text }, 0.98);
        if menu_mix > 0.9 && is_clicked(input, &item) {
            *selected_index = i as i32;
            state.open_menu = OpenMenu::None;
        }
    }
    ctx.pop_clip();
    ctx.set_global_alpha(1.0);

    if input.mouse_pressed && !button.contains(input.mouse_x, input.mouse_y) && !base_menu.contains(input.mouse_x, input.mouse_y) {
        state.open_menu = OpenMenu::None;
    }
}

fn draw_sidebar(ctx: &mut Context, input: &InputState, state: &mut DashboardState, rect: Rect, scale: f32, palette: &Palette) {
    draw_fill(ctx, rect, palette.panel, 0.0, 1.0);
    draw_fill(ctx, Rect::new(rect.x + rect.w - 1.0, rect.y, 1.0, rect.h), palette.divider, 0.0, 0.95);

    let slot_h = 58.0 * scale;
    let indicator_target_y = rect.y + (state.nav as i32) as f32 * slot_h;
    let indicator_motion_id = context_hash_sv("dashboard/sidebar-indicator");
    if ctx.animated_value_read(indicator_motion_id, -1.0) < 0.0 {
        ctx.animated_value_reset(indicator_motion_id, indicator_target_y);
    }
    let indicator_y = ctx.animated_value_ex(indicator_motion_id, indicator_target_y, 16.0);
    let indicator = Rect::new(rect.x, indicator_y, rect.w, slot_h);
    draw_fill(ctx, indicator, 0xF8F8FA, 0.0, 1.0);
    draw_fill(ctx, Rect::new(indicator.x, indicator.y, 4.0 * scale, indicator.h), palette.accent, 0.0, 1.0);

    for i in 0..K_SIDEBAR_ICONS.len() {
        let slot = Rect::new(rect.x, rect.y + i as f32 * slot_h, rect.w, slot_h);
        if interactive_clicked(state, input, &slot) {
            let next = match i {
                0 => NavItem::Dashboard,
                1 => NavItem::Deliveries,
                2 => NavItem::Documents,
                3 => NavItem::Wallet,
                4 => NavItem::Analytics,
                _ => NavItem::Dashboard,
            };
            if next != state.nav {
                state.nav = next;
                ctx.animated_value_reset(context_hash_sv("dashboard/page-reveal"), 0.0);
                state.hovered_bar = -1;
                state.settings_open = false;
            }
        }
        let selected = state.nav as i32 == i as i32;
        if !selected && interactive_hovered(state, input, &slot) {
            draw_fill(ctx, slot, 0xFFFFFF, 0.0, 0.05);
        }
        draw_icon(ctx, K_SIDEBAR_ICONS[i],
            Rect::new(slot.x + (slot.w - 18.0 * scale) * 0.5, slot.y + (slot.h - 18.0 * scale) * 0.5, 18.0 * scale, 18.0 * scale),
            if selected { 0x171A28 } else { palette.text }, if selected { 1.0 } else { 0.90 });
        draw_fill(ctx, Rect::new(slot.x, slot.y + slot.h - 1.0, slot.w, 1.0), palette.divider, 0.0, 1.0);
    }

    let gear_slot = Rect::new(rect.x, rect.y + rect.h - slot_h * 2.0, rect.w, slot_h);
    let logout_slot = Rect::new(rect.x, rect.y + rect.h - slot_h, rect.w, slot_h);
    if interactive_clicked(state, input, &gear_slot) {
        if !state.settings_open {
            ctx.animated_value_reset(context_hash_sv("dashboard/settings-panel"), 0.0);
        }
        state.settings_open = !state.settings_open;
        state.open_menu = OpenMenu::None;
    }
    let gear_hovered = interactive_hovered(state, input, &gear_slot);
    if state.settings_open || gear_hovered {
        draw_fill(ctx, gear_slot,
            if state.settings_open { palette.panel_alt } else { 0xFFFFFF }, 0.0,
            if state.settings_open { 0.92 } else { 0.06 });
    }
    draw_fill(ctx, Rect::new(gear_slot.x, gear_slot.y, gear_slot.w, 1.0), palette.divider, 0.0, 1.0);
    draw_icon(ctx, 0xF013,
        Rect::new(gear_slot.x + (gear_slot.w - 18.0 * scale) * 0.5, gear_slot.y + (gear_slot.h - 18.0 * scale) * 0.5, 18.0 * scale, 18.0 * scale),
        if state.settings_open { palette.accent_bright } else { palette.text },
        if state.settings_open { 1.0 } else { 0.88 });
    draw_icon(ctx, 0xF2F5,
        Rect::new(logout_slot.x + (logout_slot.w - 18.0 * scale) * 0.5, logout_slot.y + (logout_slot.h - 18.0 * scale) * 0.5, 18.0 * scale, 18.0 * scale),
        palette.text, 0.88);
}

fn draw_header(ctx: &mut Context, rect: Rect, scale: f32, state: &mut DashboardState, palette: &Palette, sidebar_w: f32, page: &PageCopy) {
    draw_fill(ctx, rect, palette.panel, 0.0, 1.0);
    draw_fill(ctx, Rect::new(rect.x, rect.y + rect.h - 1.0, rect.w, 1.0), palette.divider, 0.0, 1.0);
    draw_fill(ctx, Rect::new(rect.x + sidebar_w, rect.y, 1.0, rect.h), palette.divider, 0.0, 1.0);

    let profile_w = 250.0 * scale;
    let bell_w = 62.0 * scale;
    let bell_rect = Rect::new(rect.x + rect.w - profile_w - bell_w, rect.y, bell_w, rect.h);
    let profile_rect = Rect::new(rect.x + rect.w - profile_w, rect.y, profile_w, rect.h);

    draw_fill(ctx, Rect::new(bell_rect.x, bell_rect.y, 1.0, bell_rect.h), palette.divider, 0.0, 1.0);
    draw_fill(ctx, Rect::new(profile_rect.x, profile_rect.y, 1.0, profile_rect.h), palette.divider, 0.0, 1.0);

    let badge_rect = Rect::new(rect.x + sidebar_w + 18.0 * scale, rect.y + 12.0 * scale, 124.0 * scale, 32.0 * scale);
    paint_fill_stroke(ctx, badge_rect, badge_rect.h * 0.5, palette.panel_alt, 1.0, palette.divider, 1.0, 1.0);
    draw_text_center(ctx, page.badge, badge_rect, 11.2 * scale, palette.text, 0.96);

    let search_rect = Rect::new(badge_rect.x + badge_rect.w + 14.0 * scale, rect.y + 10.0 * scale, 206.0 * scale, 36.0 * scale);
    draw_icon(ctx, 0xF002,
        Rect::new(search_rect.x + 14.0 * scale, search_rect.y + (search_rect.h - 16.0 * scale) * 0.5, 16.0 * scale, 16.0 * scale),
        palette.text, 0.80);
    let input_rect = Rect::new(search_rect.x + 34.0 * scale, search_rect.y, search_rect.w - 34.0 * scale, search_rect.h);
    let input_id = context_hash_sv("dashboard/search-input");
    ctx.push_layout_rect(input_rect);
    ctx.text_input_field_ex(input_id, input_rect, "", &mut state.search_text, "Search...");
    ctx.pop_layout_rect();

    draw_icon(ctx, 0xF0F3,
        Rect::new(bell_rect.x + (bell_rect.w - 18.0 * scale) * 0.5, bell_rect.y + (bell_rect.h - 18.0 * scale) * 0.5, 18.0 * scale, 18.0 * scale),
        palette.text, 0.92);
    draw_top_profile(ctx, profile_rect, scale, palette);
}

fn draw_metric_card(ctx: &mut Context, rect: Rect, card: &MetricCard, highlighted: bool, scale: f32, palette: &Palette) {
    draw_fill(ctx, rect, palette.panel, 0.0, 1.0);
    draw_stroke(ctx, rect, palette.divider, 0.0, 1.0, 1.0);

    if highlighted {
        let top_hex = if palette.dark { darken_hex(palette.accent, 0.58) } else { lighten_hex(palette.accent, 0.84) };
        let bottom_hex = if palette.dark { lighten_hex(palette.accent, 0.24) } else { lighten_hex(palette.accent, 0.10) };
        paint_gradient(ctx, rect, top_hex, bottom_hex, 1.0);
        paint_fill_shadow(ctx, rect, 0.0, 0x000000, 0.0, 14.0 * scale, 32.0 * scale, 0x07101A, 0.28);
        ctx.push_clip(rect);
        draw_soft_glow(ctx, Rect::new(rect.x + rect.w * 0.46, rect.y + rect.h * 0.12, rect.w * 0.60, rect.h * 0.88),
            palette.accent_bright, 0xFFFFFF, 0.84, if palette.dark { 0.34 } else { 0.18 });
        draw_soft_glow(ctx, Rect::new(rect.x + rect.w * 0.02, rect.y + rect.h * 0.18, rect.w * 0.54, rect.h * 0.74),
            palette.accent, if palette.dark { palette.shell } else { 0xFFFFFF }, 0.84, if palette.dark { 0.32 } else { 0.14 });
        ctx.pop_clip();
    }

    let title_rect = Rect::new(rect.x + 26.0 * scale, rect.y + 26.0 * scale, rect.w - 124.0 * scale, 18.0 * scale);
    let value_rect = Rect::new(rect.x + 26.0 * scale, rect.y + rect.h - 64.0 * scale, 98.0 * scale, 40.0 * scale);
    let delta_rect = Rect::new(value_rect.x + value_rect.w + 10.0 * scale, value_rect.y + 12.0 * scale,
        rect.w - value_rect.w - 126.0 * scale, 16.0 * scale);
    let circle = Rect::new(rect.x + rect.w - 58.0 * scale, rect.y + 24.0 * scale, 28.0 * scale, 28.0 * scale);

    draw_fill(ctx, circle,
        if highlighted { if palette.dark { 0xFFFFFF } else { palette.accent } } else { palette.panel_alt },
        circle.w * 0.5, if highlighted { 0.96 } else { 0.92 });
    draw_icon(ctx, 0xF061,
        Rect::new(circle.x + 6.0 * scale, circle.y + 6.0 * scale, 16.0 * scale, 16.0 * scale),
        if highlighted { if palette.dark { 0x1B2032 } else { 0xFFFFFF } } else { palette.text }, 0.96);
    draw_text_left(ctx, card.title, title_rect, 11.8 * scale, palette.text, 0.96);
    draw_text_left(ctx, card.value, value_rect, 29.0 * scale, palette.text, 1.0);
    draw_text_left(ctx, card.delta, delta_rect, 11.8 * scale, palette.text, 0.95);
}

fn draw_filter_chip(ctx: &mut Context, input: &InputState, state: &mut DashboardState, filter_index: i32, rect: Rect, scale: f32, palette: &Palette) {
    let selected = state.filter_index == filter_index;
    if interactive_clicked(state, input, &rect) {
        state.filter_index = filter_index;
    }

    let index = filter_index as usize;
    let chip_id = context_hash_mix(context_hash_sv("dashboard/filter-chip"), index as u64);
    let active_mix = ctx.presence_ex(chip_id, selected, 18.0, 18.0);
    let visual = Rect::new(rect.x, rect.y - active_mix * 2.0 * scale, rect.w, rect.h);
    let radius = visual.h * 0.5;
    draw_fill(ctx, visual, palette.chip, radius, 1.0);
    if active_mix > 0.01 {
        paint_fill_shadow(ctx, visual, radius,
            palette.chip_active, active_mix,
            2.0 * scale, 6.0 * scale, 0x040913, active_mix * 0.07);
    } else {
        draw_stroke(ctx, visual, palette.divider, radius, 1.0, 1.0);
    }

    let count_w = (visual.h - 8.0 * scale).max(ctx.measure_text(K_FILTER_COUNTS[index], 11.0 * scale) + 14.0 * scale);
    let count_rect = Rect::new(visual.x + visual.w - count_w - 6.0 * scale, visual.y + 4.0 * scale, count_w, visual.h - 8.0 * scale);
    draw_fill(ctx, count_rect,
        if selected { if palette.dark { 0x171A28 } else { darken_hex(palette.accent, 0.18) } } else { if palette.dark { 0xF8F8FB } else { 0xFFFFFF } },
        count_rect.h * 0.5, 1.0);
    draw_text_center(ctx, K_FILTER_COUNTS[index], count_rect, 11.0 * scale,
        if selected { 0xF5F6FB } else { if palette.dark { 0x171A28 } else { palette.text } }, 1.0);
    draw_text_left(ctx, K_FILTER_LABELS[index],
        Rect::new(visual.x + 16.0 * scale, visual.y + 9.0 * scale, visual.w - count_rect.w - 24.0 * scale, 16.0 * scale),
        11.0 * scale, if selected { palette.chip_active_text } else { palette.text }, 0.98);
}

fn draw_status_pill(ctx: &mut Context, rect: Rect, row: &OrderRow, scale: f32, palette: &Palette) {
    let radius = rect.h * 0.5;
    if row.in_transit {
        draw_fill(ctx, rect, palette.pill_blue, radius, 1.0);
        draw_text_center(ctx, row.status, rect, 11.5 * scale, palette.pill_blue_text, 1.0);
    } else {
        draw_fill(ctx, rect, palette.pill_dark, radius, 0.92);
        draw_text_center(ctx, row.status, rect, 11.0 * scale, palette.text, 1.0);
    }
}

fn draw_order_row(ctx: &mut Context, input: &InputState, state: &mut DashboardState, index: i32, rect: Rect, row: &OrderRow, scale: f32, palette: &Palette) {
    if interactive_clicked(state, input, &rect) {
        state.selected_order = index;
    }

    let selected = state.selected_order == index;
    let row_id = context_hash_mix(context_hash_sv("dashboard/order-row"), index as u64);
    let selected_mix = ctx.presence_ex(row_id, selected, 10.0, 10.0);
    let visual = rect;
    if selected_mix > 0.01 {
        draw_fill(ctx, visual, palette.panel_alt, 14.0 * scale, selected_mix * 0.92);
        ctx.push_clip(visual);
        draw_soft_glow(ctx, Rect::new(visual.x + visual.w * 0.10, visual.y - visual.h * 0.20, visual.w * 0.60, visual.h * 1.40),
            palette.accent, if palette.dark { 0x151926 } else { 0xFFFFFF }, 0.82, 0.22 * selected_mix);
        ctx.pop_clip();
    }
    draw_fill(ctx, Rect::new(visual.x, visual.y, visual.w, 1.0), palette.divider, 0.0, 1.0);

    let radio_outer = Rect::new(visual.x + 20.0 * scale, visual.y + 21.0 * scale, 14.0 * scale, 14.0 * scale);
    if selected_mix > 0.01 {
        let radio_halo = inset_rect(radio_outer, -1.2 * scale * selected_mix);
        draw_fill(ctx, radio_halo, palette.accent, radio_halo.w * 0.5, 0.045 * selected_mix);
    }
    draw_fill(ctx, radio_outer, if palette.dark { 0x161C2A } else { 0xFFFFFF }, radio_outer.w * 0.5, 1.0);
    draw_stroke(ctx, radio_outer,
        if selected_mix > 0.12 { palette.accent_bright } else { palette.divider },
        radio_outer.w * 0.5, 1.0 + selected_mix * 0.8, 1.0);
    if selected_mix > 0.01 {
        let inset_val = (1.9_f32 * scale).max(5.0 * scale - selected_mix * 2.6 * scale);
        let radio_inner = inset_rect(radio_outer, inset_val);
        draw_fill(ctx, radio_inner, palette.accent_bright, radio_inner.w * 0.5, 0.96 * selected_mix);
        let radio_core = inset_rect(radio_inner, 1.3 * scale);
        draw_fill(ctx, radio_core,
            if palette.dark { 0xFFFFFF } else { lighten_hex(palette.accent_bright, 0.16) },
            radio_core.w * 0.5, 0.36 * selected_mix);
    }

    draw_text_left(ctx, row.id, Rect::new(visual.x + 46.0 * scale, visual.y + 20.0 * scale, visual.w * 0.16, 20.0 * scale), 12.0 * scale, palette.text, 0.96);
    draw_text_left(ctx, row.address, Rect::new(visual.x + visual.w * 0.23, visual.y + 20.0 * scale, visual.w * 0.41, 20.0 * scale), 12.0 * scale, palette.text, 0.94);
    draw_text_left(ctx, row.expires, Rect::new(visual.x + visual.w * 0.64, visual.y + 20.0 * scale, visual.w * 0.10, 20.0 * scale), 12.0 * scale, palette.text, 0.94);
    draw_status_pill(ctx, Rect::new(visual.x + visual.w - 126.0 * scale, visual.y + 12.0 * scale, 104.0 * scale, 34.0 * scale), row, scale, palette);
}

fn draw_orders_panel(ctx: &mut Context, input: &InputState, state: &mut DashboardState, rect: Rect, scale: f32, palette: &Palette, title: &str) {
    draw_fill(ctx, rect, palette.panel, 0.0, 1.0);
    draw_stroke(ctx, rect, palette.divider, 0.0, 1.0, 1.0);

    let inner = inset_rect(rect, 26.0 * scale);
    draw_text_left(ctx, title, Rect::new(inner.x, inner.y + 8.0 * scale, 220.0 * scale, 28.0 * scale), 23.0 * scale, palette.text, 1.0);

    // Dropdown (needs mutable access to state)
    let mut orders_range = state.orders_range;
    draw_dropdown_control(ctx, input,
        Rect::new(inner.x + inner.w - 128.0 * scale, inner.y, 128.0 * scale, 42.0 * scale),
        OpenMenu::Orders, state, &mut orders_range, &K_ORDERS_RANGES, scale, palette);
    state.orders_range = orders_range;

    let chip_y = inner.y + 56.0 * scale;
    let mut chip_x = inner.x;
    for i in 0..4 {
        let w = ctx.measure_text(K_FILTER_LABELS[i], 11.0 * scale) + 52.0 * scale;
        let count_w = (24.0_f32 * scale).max(ctx.measure_text(K_FILTER_COUNTS[i], 11.0 * scale) + 14.0 * scale);
        let chip = Rect::new(chip_x, chip_y, w + count_w, 34.0 * scale);
        draw_filter_chip(ctx, input, state, i as i32, chip, scale, palette);
        chip_x += chip.w + 10.0 * scale;
    }

    let header_y = chip_y + 54.0 * scale;
    draw_text_left(ctx, "Order ID", Rect::new(inner.x + 24.0 * scale, header_y, 110.0 * scale, 16.0 * scale), 11.0 * scale, palette.muted, 0.95);
    draw_text_left(ctx, "Delivery address", Rect::new(inner.x + inner.w * 0.23, header_y, 180.0 * scale, 16.0 * scale), 11.0 * scale, palette.muted, 0.95);
    draw_text_left(ctx, "Expires", Rect::new(inner.x + inner.w * 0.64, header_y, 80.0 * scale, 16.0 * scale), 11.0 * scale, palette.muted, 0.95);
    draw_text_left(ctx, "Status", Rect::new(inner.x + inner.w - 126.0 * scale, header_y, 80.0 * scale, 16.0 * scale), 11.0 * scale, palette.muted, 0.95);
    draw_fill(ctx, Rect::new(inner.x, header_y + 24.0 * scale, inner.w, 1.0), palette.divider, 0.0, 1.0);

    let row_h = 74.0 * scale;
    let mut row_y = header_y + 26.0 * scale;
    for i in 0..K_ORDERS.len() {
        draw_order_row(ctx, input, state, i as i32, Rect::new(inner.x, row_y, inner.w, row_h), &K_ORDERS[i], scale, palette);
        row_y += row_h;
    }
}

fn draw_activity_panel(ctx: &mut Context, input: &InputState, state: &mut DashboardState, rect: Rect, scale: f32, palette: &Palette, title: &str) {
    draw_fill(ctx, rect, palette.panel, 0.0, 1.0);
    draw_stroke(ctx, rect, palette.divider, 0.0, 1.0, 1.0);

    ctx.push_clip(rect);
    let haze_a = Rect::new(rect.x + rect.w * 0.04, rect.y + rect.h * 0.12, rect.w * 0.36, rect.h * 0.46);
    let haze_b = Rect::new(rect.x + rect.w * 0.28, rect.y + rect.h * 0.08, rect.w * 0.42, rect.h * 0.54);
    let haze_c = Rect::new(rect.x + rect.w * 0.60, rect.y + rect.h * 0.10, rect.w * 0.24, rect.h * 0.40);
    draw_fill(ctx, haze_a, 0xFFFFFF, haze_a.h * 0.5, 0.038);
    draw_fill(ctx, haze_b, 0xD9E2FF, haze_b.h * 0.5, 0.032);
    draw_fill(ctx, haze_c, 0xAFC0FF, haze_c.h * 0.5, 0.026);
    draw_fill(ctx, Rect::new(rect.x + rect.w * 0.18, rect.y + rect.h * 0.58, rect.w * 0.22, rect.h * 0.02), 0xFFFFFF, 8.0 * scale, 0.05);
    draw_fill(ctx, Rect::new(rect.x + rect.w * 0.48, rect.y + rect.h * 0.52, rect.w * 0.26, rect.h * 0.02), 0xD7E1FF, 8.0 * scale, 0.04);
    ctx.pop_clip();

    // Backdrop blur layer
    paint_shape(ctx, inset_rect(rect, 1.0), 0.0,
        0xF7F9FF, if palette.dark { 0.040 } else { 0.085 },
        0xFFFFFF, 1.0, if palette.dark { 0.055 } else { 0.18 },
        0.0, 0.0, 0.0, 0, 0.0,
        12.0 * scale, 12.0 * scale, 1.0);
    draw_fill(ctx, inset_rect(rect, 1.0),
        if palette.dark { 0x0B1020 } else { 0xFFFFFF }, 0.0,
        if palette.dark { 0.155 } else { 0.48 });
    draw_fill(ctx, inset_rect(rect, 8.0 * scale),
        if palette.dark { 0x0D1322 } else { lighten_hex(palette.accent, 0.90) }, 0.0,
        if palette.dark { 0.060 } else { 0.34 });

    let inner = inset_rect_hv(rect, 28.0 * scale, 22.0 * scale);
    draw_text_left(ctx, title, Rect::new(inner.x, inner.y, 180.0 * scale, 28.0 * scale), 22.0 * scale, palette.text, 1.0);

    let mut activity_range = state.activity_range;
    draw_dropdown_control(ctx, input,
        Rect::new(inner.x + inner.w - 134.0 * scale, inner.y - 2.0 * scale, 134.0 * scale, 42.0 * scale),
        OpenMenu::Activity, state, &mut activity_range, &K_ACTIVITY_RANGES, scale, palette);
    state.activity_range = activity_range;

    let chart_rect = inset_rect_ltrb(
        Rect::new(inner.x, inner.y + 44.0 * scale, inner.w, inner.h - 44.0 * scale),
        0.0, 18.0 * scale, 0.0, 16.0 * scale);
    let plot_rect = Rect::new(chart_rect.x + 34.0 * scale, chart_rect.y, chart_rect.w - 34.0 * scale, chart_rect.h - 26.0 * scale);
    let max_value = 50.0_f32;
    let gap = 10.0 * scale;
    let bar_w = (plot_rect.w - gap * (K_BAR_LABELS.len() as f32 - 1.0)) / K_BAR_LABELS.len() as f32;
    let bars = *active_bars(state);
    let mut hovered_index: i32 = -1;

    for value in (0..=50).step_by(10) {
        let t = value as f32 / max_value;
        let y = plot_rect.y + plot_rect.h - plot_rect.h * t;
        let label = match value { 0 => "0", 10 => "10", 20 => "20", 30 => "30", 40 => "40", _ => "50" };
        draw_text_left(ctx, label, Rect::new(chart_rect.x, y - 8.0 * scale, 24.0 * scale, 16.0 * scale), 10.5 * scale, palette.muted, 0.74);
    }

    if !pointer_captured_by_overlay(state, input) && plot_rect.contains(input.mouse_x, input.mouse_y) {
        let relative_x = input.mouse_x - plot_rect.x;
        let step = bar_w + gap;
        hovered_index = (relative_x / step) as i32;
        hovered_index = hovered_index.clamp(0, bars.len() as i32 - 1);
    }

    let mut bar_x = plot_rect.x;
    for i in 0..bars.len() {
        let h = plot_rect.h * (bars[i] / max_value);
        let bar = Rect::new(bar_x, plot_rect.y + plot_rect.h - h, bar_w, h);
        let active = hovered_index == i as i32;
        let base_alpha = if hovered_index >= 0 { if active { 1.0 } else { 0.18 } } else { 0.42 + 0.024 * i as f32 };
        let inactive_bar = if palette.dark { lighten_hex(palette.accent, 0.38) } else { lighten_hex(palette.accent, 0.14) };
        let active_bar = palette.accent;
        if active {
            let bar_glow = inset_rect_hv(bar, -4.0 * scale, -6.0 * scale);
            draw_fill(ctx, bar_glow, palette.accent, 6.0 * scale, 0.14);
        }
        draw_fill(ctx, bar, if active { active_bar } else { inactive_bar }, 3.0 * scale, base_alpha);
        draw_text_center(ctx, K_BAR_LABELS[i],
            Rect::new(bar.x - gap * 0.25, plot_rect.y + plot_rect.h + 6.0 * scale, bar.w + gap * 0.5, 14.0 * scale),
            9.4 * scale, if active { palette.accent_bright } else { palette.text }, if active { 1.0 } else { 0.84 });
        bar_x += bar_w + gap;
    }

    state.hovered_bar = hovered_index;
    let tooltip_alpha_id = context_hash_sv("dashboard/activity-tooltip-alpha");
    let tooltip_x_id = context_hash_sv("dashboard/activity-tooltip-x");
    let tooltip_y_id = context_hash_sv("dashboard/activity-tooltip-y");
    let tooltip_alpha = ctx.presence_ex(tooltip_alpha_id, hovered_index >= 0, 18.0, 12.0);

    if hovered_index >= 0 {
        let hi = hovered_index as usize;
        let hovered_h = plot_rect.h * (bars[hi] / max_value);
        let hovered_x = plot_rect.x + hovered_index as f32 * (bar_w + gap);
        let target_center_x = hovered_x + bar_w * 0.5;
        let target_anchor_y = plot_rect.y + plot_rect.h - hovered_h;
        if tooltip_alpha < 0.02 {
            ctx.animated_value_reset(tooltip_x_id, target_center_x);
            ctx.animated_value_reset(tooltip_y_id, target_anchor_y);
        }
        ctx.animated_value_ex(tooltip_x_id, target_center_x, 18.0);
        ctx.animated_value_ex(tooltip_y_id, target_anchor_y, 18.0);
    }

    if tooltip_alpha > 0.02 {
        let tooltip_center_x = ctx.animated_value_read(tooltip_x_id, plot_rect.x + plot_rect.w * 0.5);
        let tooltip_anchor_y = ctx.animated_value_read(tooltip_y_id, plot_rect.y + plot_rect.h * 0.5);
        let bubble_w = 108.0 * scale;
        let bubble_h = 36.0 * scale;
        let bubble_x = (tooltip_center_x - bubble_w * 0.5).clamp(plot_rect.x, plot_rect.x + plot_rect.w - bubble_w);
        let bubble_y = (tooltip_anchor_y - bubble_h - 18.0 * scale).clamp(
            chart_rect.y + 6.0 * scale,
            plot_rect.y + plot_rect.h - bubble_h - 8.0 * scale);
        let bubble = Rect::new(bubble_x, bubble_y, bubble_w, bubble_h);
        let arrow_offset = (tooltip_center_x - bubble.x - 6.0 * scale).clamp(16.0 * scale, bubble.w - 16.0 * scale);
        let arrow = Rect::new(bubble.x + arrow_offset, bubble.y + bubble.h - 4.0 * scale, 12.0 * scale, 12.0 * scale);
        let safe_index = hovered_index.max(0).clamp(0, bars.len() as i32 - 1) as usize;
        let tooltip_text = format!("{} orders", bars[safe_index].round() as i32);

        paint_shape(ctx, bubble, bubble.h * 0.5,
            0xFBFBFD, tooltip_alpha * 0.95,
            0xFFFFFF, 1.0, tooltip_alpha,
            0.0, 4.0 * scale, 10.0 * scale, 0x020617, tooltip_alpha * 0.12,
            0.0, 0.0, 1.0);
        draw_text_center(ctx, &tooltip_text, bubble, 11.4 * scale, 0x181B2A, tooltip_alpha);
        // Arrow with rotation
        ctx.push_rotation(45.0, arrow.w * 0.5, arrow.h * 0.5);
        draw_fill(ctx, arrow, 0xFBFBFD, 2.0 * scale, tooltip_alpha * 0.95);
        ctx.pop_transform();
    }
}

fn draw_courier_entry(ctx: &mut Context, rect: Rect, courier: &Courier, scale: f32, palette: &Palette) {
    draw_avatar(ctx, Rect::new(rect.x, rect.y + 4.0 * scale, 40.0 * scale, 40.0 * scale), courier, scale);
    draw_text_left(ctx, courier.name, Rect::new(rect.x + 56.0 * scale, rect.y + 8.0 * scale, rect.w - 96.0 * scale, 18.0 * scale),
        12.8 * scale, palette.text, 0.98);
    draw_text_left(ctx, courier.orders, Rect::new(rect.x + 56.0 * scale, rect.y + 32.0 * scale, rect.w - 96.0 * scale, 16.0 * scale),
        11.0 * scale, palette.muted, 0.92);
    draw_icon(ctx, 0xF061, Rect::new(rect.x + rect.w - 24.0 * scale, rect.y + 14.0 * scale, 16.0 * scale, 16.0 * scale), palette.text, 0.92);
}

fn draw_couriers_panel(ctx: &mut Context, input: &InputState, state: &mut DashboardState, rect: Rect, scale: f32, palette: &Palette, title: &str) {
    draw_fill(ctx, rect, palette.panel, 0.0, 1.0);
    draw_stroke(ctx, rect, palette.divider, 0.0, 1.0, 1.0);

    let inner = inset_rect_hv(rect, 28.0 * scale, 22.0 * scale);
    draw_text_left(ctx, title, Rect::new(inner.x, inner.y, 220.0 * scale, 28.0 * scale), 22.0 * scale, palette.text, 1.0);

    let mut couriers_range = state.couriers_range;
    draw_dropdown_control(ctx, input,
        Rect::new(inner.x + inner.w - 116.0 * scale, inner.y - 4.0 * scale, 116.0 * scale, 40.0 * scale),
        OpenMenu::Couriers, state, &mut couriers_range, &K_COURIERS_RANGES, scale, palette);
    state.couriers_range = couriers_range;

    let content = Rect::new(inner.x, inner.y + 60.0 * scale, inner.w, inner.h - 60.0 * scale);
    let gap_x = 26.0 * scale;
    let gap_y = 18.0 * scale;
    let cell_w = (content.w - gap_x) * 0.5;
    let cell_h = (content.h - gap_y) * 0.5;

    draw_courier_entry(ctx, Rect::new(content.x, content.y, cell_w, cell_h), &K_COURIERS[0], scale, palette);
    draw_courier_entry(ctx, Rect::new(content.x + cell_w + gap_x, content.y, cell_w, cell_h), &K_COURIERS[1], scale, palette);
    draw_courier_entry(ctx, Rect::new(content.x, content.y + cell_h + gap_y, cell_w, cell_h), &K_COURIERS[2], scale, palette);
    draw_courier_entry(ctx, Rect::new(content.x + cell_w + gap_x, content.y + cell_h + gap_y, cell_w, cell_h), &K_COURIERS[3], scale, palette);

    draw_fill(ctx, Rect::new(content.x, content.y + cell_h + gap_y * 0.5, content.w, 1.0), palette.divider, 0.0, 1.0);
    draw_fill(ctx, Rect::new(content.x + cell_w + gap_x * 0.5, content.y, 1.0, content.h), palette.divider, 0.0, 1.0);
}

fn settings_panel_rect(rect: Rect, scale: f32, mix_val: f32) -> Rect {
    let panel_w = 336.0 * scale;
    let panel_h = rect.h - 28.0 * scale;
    let target_x = rect.x + rect.w - panel_w - 14.0 * scale;
    let slide = 1.0 - mix_val;
    Rect::new(
        target_x + slide * (panel_w + 28.0 * scale),
        rect.y + 14.0 * scale,
        panel_w,
        panel_h,
    )
}

fn draw_settings_panel(ctx: &mut Context, input: &InputState, state: &mut DashboardState, rect: Rect, scale: f32, palette: &Palette, panel_mix: f32) {
    let panel_mix = panel_mix.clamp(0.0, 1.0);
    let panel = settings_panel_rect(rect, scale, panel_mix);

    // Backdrop blur panel
    paint_shape(ctx, panel, 22.0 * scale,
        0xF7FAFF, if palette.dark { 0.07 } else { 0.18 },
        0xFFFFFF, 1.0, if palette.dark { 0.10 } else { 0.18 },
        0.0, 6.0 * scale, 14.0 * scale, 0x020617, 0.08,
        26.0 * scale, 26.0 * scale, 1.0);
    paint_fill_stroke(ctx, inset_rect(panel, 1.0), 21.0 * scale,
        palette.panel, if palette.dark { 0.78 } else { 0.84 },
        palette.divider, 1.0, 0.62);

    ctx.set_global_alpha(1.0);
    let inner = inset_rect(panel, 24.0 * scale);
    draw_text_left(ctx, "Settings", Rect::new(inner.x, inner.y, inner.w - 44.0 * scale, 28.0 * scale), 22.0 * scale, palette.text, 1.0);
    draw_text_left(ctx, "Theme color and daytime appearance",
        Rect::new(inner.x, inner.y + 28.0 * scale, inner.w, 18.0 * scale), 11.0 * scale, palette.muted, 0.96);

    let close_rect = Rect::new(panel.x + panel.w - 44.0 * scale, panel.y + 18.0 * scale, 28.0 * scale, 28.0 * scale);
    if is_clicked(input, &close_rect) {
        state.settings_open = false;
    }
    paint_fill_stroke(ctx, close_rect, close_rect.w * 0.5,
        palette.panel_alt, if is_hovered(input, &close_rect) { 1.0 } else { 0.92 },
        palette.divider, 1.0, 1.0);
    draw_icon(ctx, 0xF00D, Rect::new(close_rect.x + 7.0 * scale, close_rect.y + 7.0 * scale, 14.0 * scale, 14.0 * scale), palette.text, 0.92);

    let appearance_rect = Rect::new(inner.x, inner.y + 72.0 * scale, inner.w, 150.0 * scale);
    draw_fill(ctx, appearance_rect, palette.panel_alt, 18.0 * scale, 0.86);
    draw_stroke(ctx, appearance_rect, palette.divider, 18.0 * scale, 1.0, 1.0);
    draw_text_left(ctx, "Appearance",
        Rect::new(appearance_rect.x + 18.0 * scale, appearance_rect.y + 18.0 * scale, appearance_rect.w - 36.0 * scale, 18.0 * scale),
        13.0 * scale, palette.text, 0.98);
    draw_text_left(ctx, "Switch between dark and daytime mode",
        Rect::new(appearance_rect.x + 18.0 * scale, appearance_rect.y + 40.0 * scale, appearance_rect.w - 36.0 * scale, 16.0 * scale),
        10.8 * scale, palette.muted, 0.94);

    let mode_wrap = Rect::new(appearance_rect.x + 18.0 * scale, appearance_rect.y + 74.0 * scale, appearance_rect.w - 36.0 * scale, 46.0 * scale);
    draw_fill(ctx, mode_wrap, palette.chip, mode_wrap.h * 0.5, 1.0);
    let mode_gap = 8.0 * scale;
    let mode_w = (mode_wrap.w - mode_gap) * 0.5;
    let dark_rect = Rect::new(mode_wrap.x, mode_wrap.y, mode_w, mode_wrap.h);
    let light_rect = Rect::new(mode_wrap.x + mode_w + mode_gap, mode_wrap.y, mode_w, mode_wrap.h);
    if is_clicked(input, &dark_rect) { state.theme_dark = true; }
    if is_clicked(input, &light_rect) { state.theme_dark = false; }

    let draw_mode_button = |ctx: &mut Context, button: Rect, active: bool, label: &str, icon: u32| {
        paint_fill_stroke(ctx, button, button.h * 0.5,
            if active { palette.accent } else { palette.panel }, if active { 0.96 } else { 0.92 },
            if active { palette.accent_bright } else { palette.divider }, 1.0, if active { 0.55 } else { 1.0 });
        draw_icon(ctx, icon,
            Rect::new(button.x + 14.0 * scale, button.y + 14.0 * scale, 16.0 * scale, 16.0 * scale),
            if active { palette.chip_active_text } else { palette.text }, 0.96);
        draw_text_left(ctx, label,
            Rect::new(button.x + 36.0 * scale, button.y + 13.0 * scale, button.w - 48.0 * scale, 18.0 * scale),
            11.5 * scale, if active { palette.chip_active_text } else { palette.text }, 0.98);
    };
    draw_mode_button(ctx, dark_rect, state.theme_dark, "Dark", 0xF186);
    draw_mode_button(ctx, light_rect, !state.theme_dark, "Day", 0xF185);

    let accent_rect = Rect::new(inner.x, appearance_rect.y + appearance_rect.h + 16.0 * scale, inner.w, 240.0 * scale);
    draw_fill(ctx, accent_rect, palette.panel_alt, 18.0 * scale, 0.86);
    draw_stroke(ctx, accent_rect, palette.divider, 18.0 * scale, 1.0, 1.0);
    draw_text_left(ctx, "Theme Color",
        Rect::new(accent_rect.x + 18.0 * scale, accent_rect.y + 18.0 * scale, accent_rect.w - 36.0 * scale, 18.0 * scale),
        13.0 * scale, palette.text, 0.98);
    draw_text_left(ctx, "Pick a primary accent",
        Rect::new(accent_rect.x + 18.0 * scale, accent_rect.y + 40.0 * scale, accent_rect.w - 36.0 * scale, 16.0 * scale),
        10.8 * scale, palette.muted, 0.94);

    let swatch_gap = 12.0 * scale;
    let swatch_cell_w = (accent_rect.w - 36.0 * scale - swatch_gap * 3.0) / 4.0;
    let swatch_size = 36.0 * scale;
    for i in 0..K_ACCENT_THEMES.len() {
        let row = i / 4;
        let col = i % 4;
        let cell = Rect::new(
            accent_rect.x + 18.0 * scale + col as f32 * (swatch_cell_w + swatch_gap),
            accent_rect.y + 74.0 * scale + row as f32 * (swatch_size + 18.0 * scale),
            swatch_cell_w,
            swatch_size,
        );
        let swatch = Rect::new(cell.x + (cell.w - swatch_size) * 0.5, cell.y, swatch_size, swatch_size);
        if is_clicked(input, &swatch) {
            state.accent_index = i as i32;
        }
        let selected = state.accent_index == i as i32;
        let swatch_color = K_ACCENT_THEMES[i].accent;
        paint_fill_stroke(ctx, swatch, swatch.w * 0.5,
            swatch_color, 1.0,
            if selected { lighten_hex(swatch_color, 0.42) } else { palette.divider },
            if selected { 2.0 } else { 1.0 }, 1.0);
        if selected {
            draw_fill(ctx,
                Rect::new(swatch.x + 10.0 * scale, swatch.y + 10.0 * scale, swatch.w - 20.0 * scale, swatch.h - 20.0 * scale),
                if palette.dark { 0xFFFFFF } else { 0x182033 },
                (swatch.w - 20.0 * scale) * 0.5, 0.92);
        }
    }
    ctx.set_global_alpha(1.0);
}

// ── Main layout ──

fn draw_dashboard(ctx: &mut Context, input: &InputState, state: &mut DashboardState, rect: Rect, scale: f32, palette: &Palette) {
    let page = page_copy(state.nav);
    draw_fill(ctx, rect, palette.shell, 10.0 * scale, 1.0);
    draw_stroke(ctx, rect, palette.divider_strong, 10.0 * scale, 1.0, 1.0);

    let header_h = 56.0 * scale;
    let sidebar_w = 58.0 * scale;
    let header_rect = Rect::new(rect.x, rect.y, rect.w, header_h);
    let body_rect = Rect::new(rect.x, rect.y + header_h, rect.w, rect.h - header_h);
    let sidebar_rect = Rect::new(body_rect.x, body_rect.y, sidebar_w, body_rect.h);
    let main_rect = Rect::new(body_rect.x + sidebar_w, body_rect.y, body_rect.w - sidebar_w, body_rect.h);

    draw_header(ctx, header_rect, scale, state, palette, sidebar_w, page);
    draw_sidebar(ctx, input, state, sidebar_rect, scale, palette);

    let settings_mix = ctx.presence_ex(context_hash_sv("dashboard/settings-panel"), state.settings_open, 7.0, 7.4);
    if settings_mix > 0.01 {
        set_overlay_block(state, Rect::new(rect.x, rect.y, rect.w, rect.h));
    }

    let reveal = smoothstep01(ctx.animated_value_ex(context_hash_sv("dashboard/page-reveal"), 1.0, 8.0));
    let reveal_offset_x = (1.0 - reveal) * 26.0 * scale;
    let reveal_offset_y = (1.0 - reveal) * 10.0 * scale;
    ctx.set_global_alpha(reveal);
    ctx.push_clip(main_rect);
    {
        let animated_main = Rect::new(main_rect.x + reveal_offset_x, main_rect.y + reveal_offset_y, main_rect.w, main_rect.h);
        let metrics = page_metrics(state.nav);
        let metric_h = 120.0 * scale;
        let card_w = animated_main.w * 0.25;
        for i in 0..metrics.len() {
            draw_metric_card(ctx,
                Rect::new(animated_main.x + card_w * i as f32, animated_main.y, card_w, metric_h),
                &metrics[i], i as i32 == page.highlighted_metric, scale, palette);
        }

        let lower_rect = Rect::new(animated_main.x, animated_main.y + metric_h, animated_main.w, animated_main.h - metric_h);
        match state.nav {
            NavItem::Deliveries => {
                let left_w = lower_rect.w * 0.62;
                let activity_rect = Rect::new(lower_rect.x, lower_rect.y, left_w, lower_rect.h);
                let side_rect = Rect::new(lower_rect.x + left_w, lower_rect.y, lower_rect.w - left_w, lower_rect.h);
                let top_h = side_rect.h * 0.54;
                draw_activity_panel(ctx, input, state, activity_rect, scale, palette, page.activity_title);
                draw_orders_panel(ctx, input, state, Rect::new(side_rect.x, side_rect.y, side_rect.w, top_h), scale, palette, page.orders_title);
                draw_couriers_panel(ctx, input, state, Rect::new(side_rect.x, side_rect.y + top_h, side_rect.w, side_rect.h - top_h), scale, palette, page.couriers_title);
            }
            NavItem::Documents => {
                let top_h = lower_rect.h * 0.56;
                let top_rect = Rect::new(lower_rect.x, lower_rect.y, lower_rect.w, top_h);
                let bottom_rect = Rect::new(lower_rect.x, lower_rect.y + top_h, lower_rect.w, lower_rect.h - top_h);
                let split_w = bottom_rect.w * 0.58;
                draw_orders_panel(ctx, input, state, top_rect, scale, palette, page.orders_title);
                draw_activity_panel(ctx, input, state, Rect::new(bottom_rect.x, bottom_rect.y, split_w, bottom_rect.h), scale, palette, page.activity_title);
                draw_couriers_panel(ctx, input, state, Rect::new(bottom_rect.x + split_w, bottom_rect.y, bottom_rect.w - split_w, bottom_rect.h), scale, palette, page.couriers_title);
            }
            NavItem::Wallet => {
                let top_h = lower_rect.h * 0.46;
                let top_rect = Rect::new(lower_rect.x, lower_rect.y, lower_rect.w, top_h);
                let bottom_rect = Rect::new(lower_rect.x, lower_rect.y + top_h, lower_rect.w, lower_rect.h - top_h);
                let left_w = bottom_rect.w * 0.57;
                draw_activity_panel(ctx, input, state, top_rect, scale, palette, page.activity_title);
                draw_orders_panel(ctx, input, state, Rect::new(bottom_rect.x, bottom_rect.y, left_w, bottom_rect.h), scale, palette, page.orders_title);
                draw_couriers_panel(ctx, input, state, Rect::new(bottom_rect.x + left_w, bottom_rect.y, bottom_rect.w - left_w, bottom_rect.h), scale, palette, page.couriers_title);
            }
            NavItem::Analytics => {
                let chart_h = lower_rect.h * 0.66;
                let chart_rect = Rect::new(lower_rect.x, lower_rect.y, lower_rect.w, chart_h);
                let bottom_rect = Rect::new(lower_rect.x, lower_rect.y + chart_h, lower_rect.w, lower_rect.h - chart_h);
                let left_w = bottom_rect.w * 0.52;
                draw_activity_panel(ctx, input, state, chart_rect, scale, palette, page.activity_title);
                draw_orders_panel(ctx, input, state, Rect::new(bottom_rect.x, bottom_rect.y, left_w, bottom_rect.h), scale, palette, page.orders_title);
                draw_couriers_panel(ctx, input, state, Rect::new(bottom_rect.x + left_w, bottom_rect.y, bottom_rect.w - left_w, bottom_rect.h), scale, palette, page.couriers_title);
            }
            NavItem::Dashboard => {
                let left_w = lower_rect.w * 0.51;
                let orders_rect = Rect::new(lower_rect.x, lower_rect.y, left_w, lower_rect.h);
                let right_rect = Rect::new(lower_rect.x + left_w, lower_rect.y, lower_rect.w - left_w, lower_rect.h);
                let activity_h = right_rect.h * 0.62;
                draw_orders_panel(ctx, input, state, orders_rect, scale, palette, page.orders_title);
                draw_activity_panel(ctx, input, state, Rect::new(right_rect.x, right_rect.y, right_rect.w, activity_h), scale, palette, page.activity_title);
                draw_couriers_panel(ctx, input, state, Rect::new(right_rect.x, right_rect.y + activity_h, right_rect.w, right_rect.h - activity_h), scale, palette, page.couriers_title);
            }
        }
    }
    ctx.pop_clip();
    ctx.set_global_alpha(1.0);

    if settings_mix > 0.01 {
        draw_fill(ctx, rect, 0x020617, 10.0 * scale, 0.10 * smoothstep01(settings_mix));
        draw_settings_panel(ctx, input, state, rect, scale, palette, settings_mix);
    }
}

// ── Entry point ──

fn main() {
    let mut state = DashboardState::default();

    let options = AppOptions {
        width: 1200,
        height: 800,
        title: "EUI Reference Dashboard Demo".to_string(),
        continuous_render: true,
        ..Default::default()
    };

    eui::run_with_options(move |ctx, _ui| {
        let (vw, vh) = ctx.viewport_size();
        let scale = (vw / 1440.0).min(vh / 900.0);
        let margin = 18.0 * scale;

        let palette = make_palette(state.theme_dark, state.accent_index);

        // Set theme based on state
        let accent_hex = K_ACCENT_THEMES[state.accent_index.clamp(0, K_ACCENT_THEMES.len() as i32 - 1) as usize].accent;
        let accent_color = color_from_hex(accent_hex, 1.0);
        let mode = if state.theme_dark { ThemeMode::Dark } else { ThemeMode::Light };
        ctx.set_theme(make_theme(mode, &accent_color));
        ctx.set_corner_radius(10.0 * scale);

        // Clone input to avoid borrow conflicts
        let input = ctx.input().clone();

        state.overlay_block_active = false;
        state.overlay_block_rect = Rect::ZERO;

        draw_dashboard(ctx, &input, &mut state,
            Rect::new(margin, margin, vw - margin * 2.0, vh - margin * 2.0),
            scale, &palette);
    }, options);
}
