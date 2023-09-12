// use crate::window::Window;
use crate::wrapper::*;

use std::ptr;

fn draw_pixel(x: i32, y: i32) {
    unsafe {
        gl::DrawArrays(gl::POINTS, x, y);
    }
}

#[allow(unused)]
fn set_vao_vbo(cordenates: &[f32], size: i32) {
    let vao = VAO::new();
    vao.bind();
    let vbo = BufferGL::new(gl::ARRAY_BUFFER, gl::STATIC_DRAW);
    vbo.bind();
    vbo.buffer_data(cordenates);
    let position_attrib = VertexGL::new(
        0,
        size,
        gl::FLOAT,
        gl::FALSE,
        size * std::mem::size_of::<f32>() as i32,
        ptr::null(),
    );
    position_attrib.enable();
}


pub fn dda_line(x1: f32, y1: f32, x2: f32, y2: f32) {
    let cordenates: [f32; 4] = [x1, y1, x2, y2];

    set_vao_vbo(&cordenates, 2);

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

    set_vao_vbo(&cordenates, 4);

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

pub fn draw_circle(x1: f32, y1: f32, xc: f32, yc: f32) {
    // get radious squared (r^2) = (x2 - x1)^2 + (y2 - y1)^2
    let mut r: f32 = ((xc - x1).powi(2) + (yc - y1).powi(2)) as f32;
    r = r.sqrt();

    for x in (xc - r as f32) as i32..=(xc + r as f32) as i32 {
        let top_y = (yc as f32 + (r as f32).powi(2) - (x as f32 - xc as f32).powi(2)).sqrt() as u32;
        let bottom_y = (yc as f32 - (r as f32).powi(2) - (x as f32 - xc as f32).powi(2)).sqrt() as u32;
        draw_pixel(x as i32, top_y as i32);
        draw_pixel(x as i32, bottom_y as i32);
    }

}

pub fn draw_rectangle(x1: f32, y1: f32, x2: f32, y2: f32) {
    let cordenates: [f32; 4] = [x1, y1, x2, y2];
    set_vao_vbo(&cordenates, 4);
    dda_line(x1, y1, x2, y1);
    dda_line(x1, y1, x1, y2);
    dda_line(x2, y1, x2, y2);
    dda_line(x1, y2, x2, y2);
}
