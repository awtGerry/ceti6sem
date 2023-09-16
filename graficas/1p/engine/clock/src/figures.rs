use engine::graphics::wrapper::*;
use engine::graphics::shader::*;

#[allow(unused_imports)]
use std::{mem, ptr};
use std::os::raw::c_void;
use gl::types::*;

fn set_vao_vbo(vao: &Vao, vbo: &Buffer, vertices: &[f32], size: usize) {
    vao.bind();
    vbo.bind();
    vbo.buffer_data(vertices);

    let position_attrib_location = Vertex::new(
        0,
        size as i32,
        gl::FLOAT,
        gl::FALSE,
        (size * std::mem::size_of::<f32>()) as GLsizei,
        ptr::null(),
    );
    position_attrib_location.enable();
}
pub fn draw_pixel(x: i32, y: i32) {
    unsafe {
        gl::DrawArrays(gl::POINTS, 0, 2);
        gl::Enable(gl::SCISSOR_TEST);
        gl::Scissor(x, y, 1, 1);
        // gl::ClearColor(0.0, 0.0, 1.0, 0.0);
        // gl::Clear(gl::COLOR_BUFFER_BIT);
        gl::Disable(gl::SCISSOR_TEST);
        gl::DrawArrays(gl::LINES, 0, 5);
    }
}

pub fn draw_line(x1: f32, y1: f32, x2: f32, y2: f32) { // DDA
    let vao = Vao::new();
    let vbo = Buffer::new(gl::ARRAY_BUFFER, gl::STATIC_DRAW);

    set_vao_vbo(&vao, &vbo, &[x1, y1, x2, y2], 2);

    let dx = x2 - x1;
    let dy = y2 - y1;

    let steps = if dx.abs() > dy.abs() { dx.abs() } else { dy.abs() };

    let x_inc = dx as f32 / steps as f32;
    let y_inc = dy as f32 / steps as f32;

    let mut x = x1 as f32;
    let mut y = y1 as f32;

    for _ in 0..steps as i32 {
        draw_pixel(x as i32, y as i32);
        unsafe {
            gl::DrawArrays(gl::LINES, 0, 5);
        }
        x += x_inc;
        y += y_inc;
    }
}

pub fn draw_triangle(x1: f32, y1: f32, x2: f32, y2: f32, x3: f32) {
    let vertices: [f32; 9] = [
        // Positions
        x1, y1, 0.0,
        x2, y1, 0.0,
        x3, y2, 0.0,
    ];
    let indices: [u32; 3] = [
        0, 1, 2,
    ];

    let vao = Vao::new();
    let vbo = Buffer::new(gl::ARRAY_BUFFER, gl::STATIC_DRAW);
    set_vao_vbo(&vao, &vbo, &vertices, 3);
    let ebo = Buffer::new(gl::ELEMENT_ARRAY_BUFFER, gl::STATIC_DRAW);
    ebo.bind();
    ebo.buffer_data(&indices);

    unsafe {
        gl::DrawElements(gl::TRIANGLES, 3, gl::UNSIGNED_INT, ptr::null());
    }
}


// pub fn draw_circle() {
// }

#[allow(unused)]
pub fn draw_rectangle(x1: f32, y1: f32, x2: f32, y2: f32) {
    draw_line(x1, y1, x2, y1);
    draw_line(x1, y1, x1, y2);
    draw_line(x1, y2, x2, y2);
    draw_line(x2, y1, x2, y2);
}

pub fn draw_rectangle_fill(x1: f32, y1: f32, x2: f32, y2: f32) {
    let cordenates: [f32; 12] = [
        // Positions
        x1, y1, 0.0,
        x2, y1, 0.0,
        x1, y2, 0.0,
        x2, y2, 0.0,
    ];
    let indices: [u32; 6] = [
        0, 1, 2,
        1, 2, 3,
    ];

    let vao = Vao::new();
    let vbo = Buffer::new(gl::ARRAY_BUFFER, gl::STATIC_DRAW);
    set_vao_vbo(&vao, &vbo, &cordenates, 3);
    let ebo = Buffer::new(gl::ELEMENT_ARRAY_BUFFER, gl::STATIC_DRAW);
    ebo.bind();
    ebo.buffer_data(&indices);

    unsafe {
        gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
    }
}

// This will draw the roof of the tower, is a bit more complex so it has its own function
pub fn draw_roof(x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, x4: f32) {
    let cordenates: [f32; 12] = [
        // Positions
        x1, y1, 0.0,
        x2, y1, 0.0,
        x3, y2, 0.0,
        x4, y2, 0.0,
    ];
    let indices: [u32; 6] = [
        0, 1, 2,
        1, 2, 3,
    ];

    let vao = Vao::new();
    let vbo = Buffer::new(gl::ARRAY_BUFFER, gl::STATIC_DRAW);
    set_vao_vbo(&vao, &vbo, &cordenates, 3);
    let ebo = Buffer::new(gl::ELEMENT_ARRAY_BUFFER, gl::STATIC_DRAW);
    ebo.bind();
    ebo.buffer_data(&indices);

    unsafe {
        gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
    }
}
