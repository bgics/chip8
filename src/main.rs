use chip8::app::App;
use eframe::egui;

// TODO: Make the code more robust (gracefully handle all errors)

// Essential Features
// TODO: Implement pause, loading rom, keymap config (via gui)
// Completed -> { pause, loading ram }

// Non Essential Features
// TODO: Implement load/save state, color config (via gui), kbd shortcuts, cpu speed control, output sizing options

// TODO: Add tests

fn main() -> eframe::Result {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_min_inner_size([640.0, 320.0])
            .with_title("Chip8"),
        ..Default::default()
    };

    eframe::run_native(
        "chip8",
        native_options,
        Box::new(|cc| {
            let mut style = (*cc.egui_ctx.style()).clone();

            if let Some(body_font_id) = style.text_styles.get_mut(&egui::TextStyle::Body) {
                *body_font_id = egui::FontId::new(20.0, egui::FontFamily::Proportional)
            }

            cc.egui_ctx.set_style(style);

            Ok(Box::new(App::new(cc)))
        }),
    )
}
