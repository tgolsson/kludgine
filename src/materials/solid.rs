use crate::shaders::{Program, ProgramSource};

const VERTEX_SHADER_SOURCE: &str = r#"
    #version 140
    uniform mat4 projection;
    uniform mat4 model;
    in vec3 position;
    void main() {
        vec4 transformed = model * vec4(position, 1.0);

        gl_Position = projection * transformed;
    }
"#;

const FRAGMENT_SHADER_SOURCE: &str = r#"
    #version 140
    uniform vec4 color;
    out vec4 f_color;
    void main() {
        f_color = color;
    }
"#;

pub(crate) fn program() -> Program {
    ProgramSource {
        vertex_shader: Some(VERTEX_SHADER_SOURCE.to_owned()),
        fragment_shader: Some(FRAGMENT_SHADER_SOURCE.to_owned()),
    }
    .into()
}
