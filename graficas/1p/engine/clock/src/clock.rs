use engine::graphics::shader::Shader;
use engine::graphics::wrapper::*;

use crate::figures::{draw_circle, draw_line, set_vao_vbo};

fn draw_clock() {
    unsafe {
        draw_circle(200.0, 200.0, 100.0);
    }
}
