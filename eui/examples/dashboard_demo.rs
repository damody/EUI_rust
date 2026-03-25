use eui::*;
use eui::quick::ui::*;

fn main() {
    let mut tab = 0usize;

    eui::run(move |_ctx, ui| {
        let content = ui.content_rect();
        let bg = ui.theme().background;
        ui.paint_filled_rect(content, bg, 0.0);

        // Header
        let header_h = 56.0;
        let header = Rect::new(content.x, content.y, content.w, header_h);
        let panel_color = ui.theme().panel;
        ui.paint_filled_rect(header, panel_color, 0.0);

        let header_inner = inset(&header, 16.0, 0.0);
        let text_color = ui.theme().text;
        ui.text("Dashboard").rect(Rect::new(header_inner.x, header_inner.y, 200.0, header_h))
            .font_size(18.0).color(text_color).draw();

        let body = Rect::new(content.x, content.y + header_h, content.w, (content.h - header_h).max(0.0));
        let sides = ui.split_h(&body, 200.0, 0.0);

        // Sidebar
        ui.scope(sides.first, |ctx| {
            let mut ui = UI::new(ctx);
            let r = ui.content_rect();
            let panel = ui.theme().panel;
            ui.paint_filled_rect(r, panel, 0.0);

            let inner = inset(&r, 12.0, 12.0);
            ui.scope(inner, |ctx| {
                let mut ui = UI::new(ctx);
                let labels = ["Overview", "Analytics", "Reports", "Settings"];
                for (i, label) in labels.iter().enumerate() {
                    let style = if i == tab { ButtonStyle::Primary } else { ButtonStyle::Ghost };
                    if ui.button(label).style(style).draw() {
                        tab = i;
                    }
                    ui.spacer(2.0);
                }
            });
        });

        // Main content
        ui.scope(sides.second, |ctx| {
            let mut ui = UI::new(ctx);
            let r = ui.content_rect();
            let inner = inset(&r, 24.0, 24.0);

            ui.scope(inner, |ctx| {
                let mut ui = UI::new(ctx);

                match tab {
                    0 => {
                        ui.label("Overview").font_size(20.0).height(32.0).draw();
                        ui.spacer(16.0);

                        // Metrics row
                        let metrics_area = Rect::new(
                            ui.content_rect().x,
                            ui.cursor_y(),
                            ui.content_rect().w,
                            80.0,
                        );
                        let col1 = ui.split_h(&metrics_area, metrics_area.w / 3.0, 12.0);
                        let col2 = ui.split_h(&col1.second, col1.second.w / 2.0, 12.0);

                        ui.card("").rect(col1.first).padding(12.0).begin(|ctx| {
                            let mut ui = UI::new(ctx);
                            ui.metric("Users", "12,847").draw();
                        });
                        ui.card("").rect(col2.first).padding(12.0).begin(|ctx| {
                            let mut ui = UI::new(ctx);
                            ui.metric("Revenue", "$45,230").draw();
                        });
                        ui.card("").rect(col2.second).padding(12.0).begin(|ctx| {
                            let mut ui = UI::new(ctx);
                            ui.metric("Growth", "+23%").draw();
                        });
                    }
                    1 => {
                        ui.label("Analytics").font_size(20.0).height(32.0).draw();
                        ui.spacer(16.0);
                        ui.label("Charts and analytics would go here.").muted().draw();
                    }
                    2 => {
                        ui.label("Reports").font_size(20.0).height(32.0).draw();
                        ui.spacer(16.0);
                        ui.label("Report generation interface.").muted().draw();
                    }
                    3 => {
                        ui.label("Settings").font_size(20.0).height(32.0).draw();
                        ui.spacer(16.0);
                        ui.label("Application settings.").muted().draw();
                    }
                    _ => {}
                }
            });
        });
    });
}
