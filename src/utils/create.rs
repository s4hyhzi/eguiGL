use eframe::glow::{self, HasContext};

pub fn create_shader(
    gl: &glow::Context,
    shader_type: u32,
    source: &str,
) -> Result<<glow::Context as glow::HasContext>::Shader, String> {
    unsafe {
        let shader = gl.create_shader(shader_type).unwrap();
        gl.shader_source(shader, source);
        gl.compile_shader(shader);
        if !gl.get_shader_compile_status(shader) {
            let err = gl.get_shader_info_log(shader);
            gl.delete_shader(shader);
            return Err(err);
        }
        Ok(shader)
    }
}

pub fn create_program(gl: &glow::Context, vertex: &str, fragment: &str) -> Result<<glow::Context as glow::HasContext>::Program, String> {
    unsafe {
        let program = gl.create_program().unwrap();
        let vs = create_shader(gl, glow::VERTEX_SHADER, vertex)?;
        let fs = create_shader(gl, glow::FRAGMENT_SHADER, fragment)?;
        gl.attach_shader(program, vs);
        gl.attach_shader(program, fs);
        gl.link_program(program);
        if !gl.get_program_link_status(program) {
            let err = gl.get_program_info_log(program);
            gl.delete_program(program);
            return Err(err);
        }
        Ok(program)
    }
}