// Test that we can paint to the screen using glow directly.

mod utils;

use eframe::glow;
use eframe::{egui, epaint::Vec2};

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
        Box::new(|cc| Box::new(MyApp::new(cc))),
    )?;
    Ok(())
}

fn setup_custom_fonts(ctx: &egui::Context) {
    let mut font = egui::FontDefinitions::default();

    font.font_data.insert(
        "miSans".to_owned(),
        egui::FontData::from_static(include_bytes!("../assets/fonts/miSans.ttf")),
    );

    font.families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "miSans".to_owned());

    font.families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .push("miSans".to_owned());

    ctx.set_fonts(font);
}

#[derive(Default)]
struct MyApp {}

impl MyApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        setup_custom_fonts(&cc.egui_ctx);
        Self {}
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        use glow::HasContext as _;
        let gl = frame.gl().unwrap();

        #[allow(unsafe_code)]
        unsafe {
            gl.viewport(0, 0, 100, 100);
            gl.clear_color(0.0, 0.0, 0.0, 0.0); // purple
            gl.clear(glow::COLOR_BUFFER_BIT);
        }

        egui::Window::new("浮动窗口").show(ctx, |ui| {
            ui.label("这是背景上的浮动窗口");
        });
    }
}
