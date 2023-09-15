use engine::graphics::window::Window;
use engine::graphics::wrapper::*;

use gl::types::*;
use std::mem;
use std::ptr;

fn set_vao_vbo(vertices: &[GLfloat], size: usize) {
    let vao = Vao::new();
    vao.bind();
    let vbo = Buffer::new(gl::ARRAY_BUFFER, gl::STATIC_DRAW);
    vbo.bind();
    vbo.buffer_data(vertices);

    let position_attrib_location = Vertex::new(
        0,
        size as i32,
        gl::FLOAT,
        gl::FALSE,
        (size * mem::size_of::<GLfloat>()) as GLsizei,
        ptr::null(),
    );
    position_attrib_location.enable();
}

fn main() {
    let mut window = Window::new(800, 600, "Hello, world!");
    window.init();

    while !window.should_close() {
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        window.update();
    }
}
