use gl::types::GLsizei;

// use crate::window::Window;
use crate::wrapper::*;

use std::ptr;

pub fn draw_pixel(x: i32, y: i32) {
    unsafe {
        // gl::Viewport(0, 0, x, y);
        gl::DrawArrays(gl::POINTS, x, y);
        gl::Enable(gl::SCISSOR_TEST);
        gl::Scissor(x, y, 1, 1);
        gl::ClearColor(1.0, 1.0, 1.0, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);
        gl::Disable(gl::SCISSOR_TEST);
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
        (size * std::mem::size_of::<f32>()) as GLsizei,
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

    set_vao_vbo(&cordenates, 3);

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
    let r: f32 = ((x1 - xc).powi(2) + (y1 - yc).powi(2)).sqrt();

    set_vao_vbo(&[x1, y1, xc, yc], 4);

    for x in (xc - r) as i32..=(xc + r) as i32 {
        let top_y = yc as i32 + (((r).powi(2) - (x as f32 - xc ).powi(2)).sqrt()) as i32;
        let bottom_y = yc as i32 - ((r as f32).powi(2) - (x as f32 - xc as f32).powi(2)).sqrt() as i32;
        draw_pixel(x, top_y);
        draw_pixel(x, bottom_y);
    }

}

pub fn draw_rectangle(x1: f32, y1: f32, x2: f32, y2: f32) {
    dda_line(x1, y1, x2, y1);
    dda_line(x2, y1, x2, y2);
    dda_line(x2, y2, x1, y2);
    dda_line(x1, y2, x1, y1);

    let cordenates: [f32; 12] = [
        x1, y1, 0.0, x2, y1, 0.0, x2, y2, 0.0, x1, y2, 0.0,
    ];
    let indices: [u32; 6] = [0, 1, 2, 2, 3, 0];

    let vao = VAO::new();
    vao.bind();
    let vbo = BufferGL::new(gl::ARRAY_BUFFER, gl::STATIC_DRAW);
    vbo.bind();
    vbo.buffer_data(&cordenates);

    // TODO: Las lineas se borran por lo tanto agregue un fondo blanco temporal para que se quede el rectangulo
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

    unsafe {
        gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null()); // Agregar fondo blanco
    }
}

pub fn draw_mid_point_circle(xc: f32, yc: f32, r: f32) {
    let mut x = 0.0;
    let mut y = r;

    let mut d = 1.0 - r;

    while x < y {
        draw_pixel(xc as i32 + x as i32, yc as i32 + y as i32);
        draw_pixel(xc as i32 + y as i32, yc as i32 + x as i32);
        draw_pixel(xc as i32 - x as i32, yc as i32 + y as i32);
        draw_pixel(xc as i32 - y as i32, yc as i32 + x as i32);
        draw_pixel(xc as i32 + x as i32, yc as i32 - y as i32);
        draw_pixel(xc as i32 + y as i32, yc as i32 - x as i32);
        draw_pixel(xc as i32 - x as i32, yc as i32 - y as i32);
        draw_pixel(xc as i32 - y as i32, yc as i32 - x as i32);

        if d < 0.0 {
            d += 2.0 * x + 3.0;
        } else {
            d += 2.0 * (x - y) + 5.0;
            y -= 1.0;
        }
        x += 1.0;
    }
}

pub fn polar_circle(x: f32, y: f32, xc: f32, yc: f32) {
    let mut theta: f32 = 0.0;
    let segments = 360.0;
    let r = ((x - xc).powi(2) + (y - yc).powi(2)).sqrt();
    
    for _ in 0..segments as i32 {
        let x = xc + r * theta.cos();
        let y = yc + r * theta.sin();
        draw_pixel(x as i32, y as i32);
        theta += 2.0 * std::f32::consts::PI / segments;
    }
}

pub fn ellipse(xc: f32, yc: f32, rx: f32, ry: f32) {
    let mut x = 0.0;
    let mut y = ry;

    let mut d1 = (ry.powi(2) - rx.powi(2) * ry + 0.25 * rx.powi(2)) as f32;
    let mut dx = 2.0 * ry.powi(2) * x;
    let mut dy = 2.0 * rx.powi(2) * y;

    while dx < dy {
        draw_pixel(xc as i32 + x as i32, yc as i32 + y as i32);
        draw_pixel(xc as i32 - x as i32, yc as i32 + y as i32);
        draw_pixel(xc as i32 + x as i32, yc as i32 - y as i32);
        draw_pixel(xc as i32 - x as i32, yc as i32 - y as i32);

        if d1 < 0.0 {
            x += 1.0;
            dx += 2.0 * ry.powi(2);
            d1 += dx + ry.powi(2);
        } else {
            x += 1.0;
            y -= 1.0;
            dx += 2.0 * ry.powi(2);
            dy -= 2.0 * rx.powi(2);
            d1 += dx - dy + ry.powi(2);
        }
    }

    let mut d2 = ((ry.powi(2) * (x + 0.5).powi(2)) + (rx.powi(2) * (y - 1.0).powi(2)) - rx.powi(2) * ry.powi(2)) as f32;

    while y >= 0.0 {
        draw_pixel(xc as i32 + x as i32, yc as i32 + y as i32);
        draw_pixel(xc as i32 - x as i32, yc as i32 + y as i32);
        draw_pixel(xc as i32 + x as i32, yc as i32 - y as i32);
        draw_pixel(xc as i32 - x as i32, yc as i32 - y as i32);

        if d2 > 0.0 {
            y -= 1.0;
            dy -= 2.0 * rx.powi(2);
            d2 += rx.powi(2) - dy;
        } else {
            y -= 1.0;
            x += 1.0;
            dx += 2.0 * ry.powi(2);
            dy -= 2.0 * rx.powi(2);
            d2 += dx - dy + rx.powi(2);
        }

    }
}
