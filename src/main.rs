use eframe::egui;

fn main() {
    let native_options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "Egui学习",
        native_options,
        Box::new(|cc| Box::new(MyEguiApp::new(cc))),
    );
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
struct MyEguiApp {
    name: String,
    age: u32,
}

impl MyEguiApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        setup_custom_fonts(&_cc.egui_ctx);
        Self::default()
    }

    fn default() -> Self {
        Self {
            name: "小煜".to_owned(),
            age: 42,
        }
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("你好世界!这是小米字体。");

            ui.horizontal(|ui| {
                let name_label = ui.label("你的名字: ");
                ui.text_edit_singleline(&mut self.name)
                    .labelled_by(name_label.id);
            });
            ui.add(egui::Slider::new(&mut self.age, 0..=120).text("岁"));
            if ui.button("点击增加一岁").clicked() {
                self.age += 1;
            }
            ui.label(format!("你好！ '{}', 年龄 {}", self.name, self.age));
        });
    }
}
