use glow::HasContext;

pub const VERTEX_SHADER_SOURCE: &str = r#"
#version 120
attribute vec2 a_position;
attribute vec4 a_color;
attribute vec2 a_texcoord;

varying vec4 v_color;
varying vec2 v_texcoord;

uniform vec2 u_viewport;

void main() {
    vec2 pos = (a_position / u_viewport) * 2.0 - 1.0;
    pos.y = -pos.y;
    gl_Position = vec4(pos, 0.0, 1.0);
    v_color = a_color;
    v_texcoord = a_texcoord;
}
"#;

pub const FRAGMENT_SHADER_SOURCE: &str = r#"
#version 120

varying vec4 v_color;
varying vec2 v_texcoord;

uniform sampler2D u_texture;
uniform int u_texture_mode;

void main() {
    if (u_texture_mode == 1) {
        float alpha = texture2D(u_texture, v_texcoord).r;
        gl_FragColor = vec4(v_color.rgb, v_color.a * alpha);
    } else if (u_texture_mode == 2) {
        gl_FragColor = v_color * texture2D(u_texture, v_texcoord);
    } else {
        gl_FragColor = v_color;
    }
}
"#;

pub unsafe fn compile_shader(gl: &glow::Context, shader_type: u32, source: &str) -> Result<glow::Shader, String> {
    let shader = gl.create_shader(shader_type).map_err(|e| e.to_string())?;
    gl.shader_source(shader, source);
    gl.compile_shader(shader);
    if !gl.get_shader_compile_status(shader) {
        let log = gl.get_shader_info_log(shader);
        gl.delete_shader(shader);
        return Err(log);
    }
    Ok(shader)
}

pub unsafe fn create_program(gl: &glow::Context) -> Result<glow::Program, String> {
    let vs = compile_shader(gl, glow::VERTEX_SHADER, VERTEX_SHADER_SOURCE)?;
    let fs = compile_shader(gl, glow::FRAGMENT_SHADER, FRAGMENT_SHADER_SOURCE)?;

    let program = gl.create_program().map_err(|e| e.to_string())?;
    gl.attach_shader(program, vs);
    gl.attach_shader(program, fs);

    gl.bind_attrib_location(program, 0, "a_position");
    gl.bind_attrib_location(program, 1, "a_color");
    gl.bind_attrib_location(program, 2, "a_texcoord");

    gl.link_program(program);
    if !gl.get_program_link_status(program) {
        let log = gl.get_program_info_log(program);
        gl.delete_program(program);
        gl.delete_shader(vs);
        gl.delete_shader(fs);
        return Err(log);
    }

    gl.delete_shader(vs);
    gl.delete_shader(fs);
    Ok(program)
}
