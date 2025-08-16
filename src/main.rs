use chip8::app::App;
use eframe::egui;

// TODO: Make the code more robust (gracefully handle all errors)

// Non Essential Features
// TODO: Implement load/save state, color config (via gui), kbd shortcuts (first define what do you mean by kbd shortcuts), cpu speed control, output sizing options

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
        Box::new(|cc| Ok(Box::new(App::new(cc)))),
    )
}
