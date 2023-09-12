mod window;
mod wrapper;

use window::Window;
use wrapper::*;

use std::ptr;

fn main() {
    let mut window = Window::new(800, 600, "Title");

    let vertices: [f32; 12] = [
        0.5, 0.5, 0.0, // top right
        0.5, -0.5, 0.0, // bottom right
        -0.5, -0.5, 0.0, // bottom left
        -0.5, 0.5, 0.0, // top left
    ];

    let indices: [u32; 6] = [
        0, 1, 3, // first triangle
        1, 2, 3, // second triangle
    ];

    window.init();

    let vao = VAO::new();
    vao.bind();
    let vbo = BufferGL::new(gl::ARRAY_BUFFER, gl::STATIC_DRAW);
    vbo.bind();
    vbo.buffer_data(&vertices);
    let ebo = BufferGL::new(gl::ELEMENT_ARRAY_BUFFER, gl::STATIC_DRAW);
    ebo.bind();
    ebo.buffer_data(&indices);

    let position_attrib = VertexGL::new(
        0,
        3,
        gl::FLOAT,
        gl::FALSE,
        3 * std::mem::size_of::<f32>() as i32,
        ptr::null(),
    );

    position_attrib.enable();

    while !window.should_close() {

        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            // gl::DrawArrays(gl::TRIANGLES, 0, 4);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
        }

        window.update();
    }
}
