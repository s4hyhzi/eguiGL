use eframe::{egui,glow};

fn setup_custom_fonts(ctx: &egui::Context) {
    let mut font = egui::FontDefinitions::default();

    font.font_data.insert(
        "miSans".to_owned(),
        egui::FontData::from_static(include_bytes!("../../assets/fonts/miSans.ttf")),
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
pub struct App {}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        setup_custom_fonts(&cc.egui_ctx);
        Self {}
    }
}

impl eframe::App for App {
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
