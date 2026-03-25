use eui::*;
use eui::quick::ui::*;

fn main() {
    let mut counter = 0i32;
    let mut slider_val = 0.5f32;
    let mut name = String::from("World");

    eui::run(move |_ctx, ui| {
        let content = ui.content_rect();
        let padding = 24.0;
        let inner = inset(&content, padding, padding);

        ui.scope(inner, |ctx| {
            let mut ui = UI::new(ctx);

            // Title
            ui.label("EUI Rust Demo").font_size(24.0).height(36.0).draw();
            ui.spacer(8.0);

            // Counter
            ui.label(&format!("Counter: {}", counter)).draw();
            if ui.button("Increment").draw() {
                counter += 1;
            }
            if ui.button("Decrement").secondary().draw() {
                counter -= 1;
            }
            ui.spacer(12.0);

            // Slider
            ui.slider("Opacity", &mut slider_val).range(0.0, 1.0).draw();
            ui.spacer(12.0);

            // Text input
            ui.input("Name", &mut name).draw();
            ui.spacer(8.0);
            ui.label(&format!("Hello, {}!", name)).draw();

            // Progress
            ui.spacer(12.0);
            ui.progress("Progress", slider_val).draw();
        });
    });
}
