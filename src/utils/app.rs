use crate::utils::creat::{compile_shader, link_program};
use eframe::{egui, glow};

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
            let vertex_shader_source = include_str!("../../assets/shaders/custom_material.vert");
            let fragment_shader_source = include_str!("../../assets/shaders/custom_material.frag");
            let vertex_shader = compile_shader(gl, glow::VERTEX_SHADER, vertex_shader_source);
            let fragment_shader = compile_shader(gl, glow::FRAGMENT_SHADER, fragment_shader_source);
            let program = link_program(
                gl,
                [vertex_shader.unwrap(), fragment_shader.unwrap()].iter(),
            );
            gl.use_program(Some(program.unwrap()));
            let vertex_array = gl.create_vertex_array().unwrap();
            gl.bind_vertex_array(Some(vertex_array));

            let positions:Vec<f32> = vec![
                0.0, 0.5, 0.0, // top
                -0.5, -0.5, 0.0, // left
                0.5, -0.5, 0.0, // right
            ];

            let positions_buffer = gl.create_buffer().unwrap();
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(positions_buffer));
            gl.buffer_data_u8_slice(
                glow::ARRAY_BUFFER,
                std::slice::from_raw_parts(
                    positions.as_ptr() as *const u8,
                    positions.len() * std::mem::size_of::<f32>(),
                ),
                glow::STATIC_DRAW,
            );
            gl.vertex_attrib_pointer_f32(
                0, // index
                3, // size
                glow::FLOAT, // type
                false, // normalized
                0, // stride
                0, // offset
            );
            gl.enable_vertex_attrib_array(0);


            gl.viewport(0, 0, 768, 768);
            gl.clear_color(0.0, 0.0, 0.0, 0.0); // purple
            gl.clear(glow::COLOR_BUFFER_BIT);
            gl.draw_arrays(glow::TRIANGLES, 0, 3);
        }

        egui::Window::new("浮动窗口").show(ctx, |ui| {
            ui.label("这是背景上的浮动窗口");
        });
    }
}
