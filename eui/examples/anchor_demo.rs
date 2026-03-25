use eui::*;
use eui::quick::ui::UI;

fn main() {
    eui::run(move |_ctx, ui| {
        let content = ui.content_rect();

        // Center box
        let center = ui.anchor()
            .in_rect(content)
            .center_x(0.0)
            .center_y(0.0)
            .width(400.0)
            .height(300.0)
            .resolve();

        ui.card("Anchor Demo").rect(center).begin(|ctx| {
            let mut ui = UI::new(ctx);
            ui.label("This card is centered using anchors").draw();
            ui.spacer(8.0);

            // Anchor from parent
            let inner = ui.content_rect();
            let top_left = ui.anchor()
                .in_rect(inner)
                .left(0.0)
                .top(40.0)
                .width(120.0)
                .height(40.0)
                .resolve();
            ui.shape().rect(top_left).fill(rgba(0.2, 0.6, 1.0, 1.0)).radius(6.0).draw();
            ui.text("Top Left").rect(top_left).color(Color::WHITE).center().draw();

            let bottom_right = ui.anchor()
                .in_rect(inner)
                .right(0.0)
                .bottom(0.0)
                .width(120.0)
                .height(40.0)
                .resolve();
            ui.shape().rect(bottom_right).fill(rgba(1.0, 0.3, 0.3, 1.0)).radius(6.0).draw();
            ui.text("Bottom Right").rect(bottom_right).color(Color::WHITE).center().draw();

            let centered = ui.anchor()
                .in_rect(inner)
                .center_x(0.0)
                .center_y(0.0)
                .width(100.0)
                .height(40.0)
                .resolve();
            ui.shape().rect(centered).fill(rgba(0.2, 0.8, 0.4, 1.0)).radius(6.0).draw();
            ui.text("Center").rect(centered).color(Color::WHITE).center().draw();
        });
    });
}
