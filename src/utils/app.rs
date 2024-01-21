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
            gl.use_program(Some(program.clone().unwrap()));
            let vertex_array = gl.create_vertex_array().unwrap();
            gl.bind_vertex_array(Some(vertex_array));

            let indices: Vec<i32> = vec![0, 1, 3, 1, 2, 3];

            let indices_buffer = gl.create_buffer().unwrap();
            gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(indices_buffer));
            gl.buffer_data_u8_slice(
                glow::ELEMENT_ARRAY_BUFFER,
                std::slice::from_raw_parts(
                    indices.as_ptr() as *const u8,
                    indices.len() * std::mem::size_of::<i32>(),
                ),
                glow::STATIC_DRAW,
            );

            let positions: Vec<f32> = vec![0.5, 0.5, 0.5, -0.5, -0.5, -0.5, -0.5, 0.5];
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
                0,           // index
                2,           // size
                glow::FLOAT, // type
                false,       // normalized
                0,           // stride
                0,           // offset
            );
            gl.enable_vertex_attrib_array(0);

            let colors = vec![
                255, 255, 255, 255, 0, 255, 120, 255, 120, 122, 255, 255, 255, 255, 0, 255,
            ];
            // 检查颜色数组的长度是否是4的倍数
            assert!(
                colors.len() % 4 == 0,
                "Colors array length should be a multiple of 4."
            );

            // 将整数颜色值转换为Vec4数组，归一化到0.0到1.0的范围
            let normalized_colors: Vec<f32> = colors
                .chunks(1) // 将数组切分为长度为4的块
                .map(|chunk| {
                    // 将每个块（RGBA值）转换为Vec4
                    chunk[0] as f32 / 255.0
                })
                .collect();
            let colors_buffer = gl.create_buffer().unwrap();
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(colors_buffer));
            gl.buffer_data_u8_slice(
                glow::ARRAY_BUFFER,
                std::slice::from_raw_parts(
                    normalized_colors.as_ptr() as *const u8,
                    normalized_colors.len() * std::mem::size_of::<f32>(),
                ),
                glow::STATIC_DRAW,
            );
            gl.vertex_attrib_pointer_f32(
                1,           // index
                4,           // size
                glow::FLOAT, // type
                true,        // normalized
                0,           // stride
                0,           // offset
            );
            gl.enable_vertex_attrib_array(1);

            let u_color = gl.get_uniform_location(program.unwrap(), "u_color");
            gl.uniform_4_f32(u_color.as_ref(), 0.5, 1.0, 1.0, 1.0);

            gl.viewport(0, 0, 768, 768);
            gl.clear_color(0.0, 0.0, 0.0, 0.0); // purple
            gl.clear(glow::COLOR_BUFFER_BIT);

            gl.draw_elements(glow::TRIANGLES, 6, glow::UNSIGNED_INT, 0);
        }

        egui::Window::new("浮动窗口").show(ctx, |ui| {
            ui.label("这是背景上的浮动窗口");
        });
    }
}
