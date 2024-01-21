mod utils;

// Test that we can paint to the screen using glow directly.
use eframe::{egui, epaint::Vec2};
use utils::app::App;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        renderer: eframe::Renderer::Glow,
        viewport: egui::ViewportBuilder {
            min_inner_size: Some(Vec2::new(1366.0, 768.0)),
            ..Default::default()
        },
        ..Default::default()
    };
    eframe::run_native(
        "Egui+GL学习",
        options,
        Box::new(|cc| Box::new(App::new(cc))),
    )?;
    Ok(())
}