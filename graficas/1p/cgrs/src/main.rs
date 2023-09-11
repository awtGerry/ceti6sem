use cgrs::window::Window;
use cgrs::wrapper::*;

use std::ptr;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

#[allow(unused)]
fn draw_pixel(x: i32, y: i32) {
    unsafe {
        gl::DrawArrays(gl::LINES, x, y);
    }
}

#[allow(unused)]
fn dda_line(x1: i32, y1: i32, x2: i32, y2: i32) {
    let dx = x2 - x1;
    let dy = y2 - y1;

    let steps = if dx.abs() > dy.abs() { dx.abs() } else { dy.abs() };

    let x_inc = dx as f32 / steps as f32;
    let y_inc = dy as f32 / steps as f32;

    let mut x = x1 as f32;
    let mut y = y1 as f32;

    for _ in 0..steps {
        draw_pixel(x as i32, y as i32);
        x += x_inc;
        y += y_inc;
    }
}

fn main() {
    let mut window = Window::new(WIDTH, HEIGHT, "Title");

    let cordenates: [f32; 4] = [
        -0.5, // x1
        -0.5, // y1
        0.5,  // x2
        0.5,  // y2
    ];

    window.init();

    let vao = VAO::new();
    vao.bind();
    let vbo = BufferGL::new(gl::ARRAY_BUFFER, gl::STATIC_DRAW);
    vbo.bind();
    vbo.buffer_data(&cordenates);

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
            gl::ClearColor(0.0, 0.0, 0.0, 1.0); // black background
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::PointSize(10.0);
            // Draw cordenates
            gl::DrawArrays(gl::LINES, 0, 3);
        }

        window.update();
    }
}
