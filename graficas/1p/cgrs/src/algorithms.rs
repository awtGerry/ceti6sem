// use crate::window::Window;
use crate::wrapper::*;

use std::ptr;

pub fn draw_pixel(x: i32, y: i32) {
    unsafe {
        gl::DrawArrays(gl::LINES, x, y); // Draw the line
    }
}

pub fn dda_line(x1: f32, y1: f32, x2: f32, y2: f32) {
    let cordenates: [f32; 4] = [x1, y1, x2, y2];

    let vao = VAO::new();
    vao.bind(); let vbo = BufferGL::new(gl::ARRAY_BUFFER, gl::STATIC_DRAW);
    vbo.bind();
    vbo.buffer_data(&cordenates);

    let position_attrib = VertexGL::new(
        0,
        2,
        gl::FLOAT,
        gl::FALSE,
        2 * std::mem::size_of::<f32>() as i32,
        ptr::null(),
    );

    position_attrib.enable();

    let dx = x2 - x1;
    let dy = y2 - y1;

    let steps = if dx.abs() > dy.abs() { dx.abs() } else { dy.abs() };

    let x_inc = dx as f32 / steps as f32;
    let y_inc = dy as f32 / steps as f32;

    let mut x = x1 as f32;
    let mut y = y1 as f32;

    for _ in 0..steps as i32 {
        draw_pixel(x as i32, y as i32);
        x += x_inc;
        y += y_inc;
    }
}

pub fn bresenham_line(x1: f32, y1: f32, x2: f32, y2: f32) {
    let cordenates: [f32; 4] = [x1, y1, x2, y2];

    let vao = VAO::new();
    vao.bind(); let vbo = BufferGL::new(gl::ARRAY_BUFFER, gl::STATIC_DRAW);
    vbo.bind();
    vbo.buffer_data(&cordenates);

    let position_attrib = VertexGL::new(
        0,
        4,
        gl::FLOAT,
        gl::FALSE,
        4 * std::mem::size_of::<f32>() as i32,
        ptr::null(),
    );

    position_attrib.enable();

    let mut dx = x2 - x1;
    let mut dy = y2 - y1;

    let mut x = x1 as f32;
    let mut y = y1 as f32;

    let mut p = 2.0 * dy - dx;

    let mut inc_x = 1.0;
    let mut inc_y = 1.0;

    if dx < 0.0 {
        inc_x.clone_from(&-1.0);
        dx = -dx;
    }

    if dy < 0.0 {
        inc_y.clone_from(&-1.0);
        dy = -dy;
    }

    draw_pixel(x as i32, y as i32);

    while x < x2 {
        if p >= 0.0 {
            draw_pixel(x as i32, y as i32);
            y += 1.0;
            p += 2.0 * dy - 2.0 * dx;
        } else {
            draw_pixel(x as i32, y as i32);
            p += 2.0 * dy;
        }
        x += 1.0;
    }
}
