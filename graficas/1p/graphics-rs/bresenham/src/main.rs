extern crate glfw;
extern crate gl;

use glfw::{Context, Key, Action};
use gl::types::*;

#[allow(unused)]
const WIDTH: usize = 640;
#[allow(unused)]
const HEIGHT: usize = 480;

fn bresenham(x1: i32, y1: i32, x2: i32, y2: i32) -> Vec<(i32, i32)> {
    let mut points = Vec::new();
    let x = x1;
    let mut y = y1;
    let dx = x2 - x1;
    let dy = y2 - y1;
    let mut d = 2 * dy - dx;
    points.push((x, y));
    for x in x1..x2 {
        if d > 0 {
            y += 1;
            d += 2 * (dy - dx);
        } else {
            d += 2 * dy;
        }
        points.push((x, y));
    }
    points
}

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    let (mut window, events) = glfw.create_window(800, 600, "Line Drawing", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.make_current();

    gl::load_with(|s| window.get_proc_address(s) as *const _);
    
    let start_x = 100.0;
    let start_y = 100.0;
    let end_x = 700.0;
    let end_y = 500.0;

    // Main loop
    while !window.should_close() {
        // Poll events
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    window.set_should_close(true);
                },
                _ => {}
            }
        }

        // Render the line
        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::DrawArrays(gl::TRIANGLES, 0, 3)

        }

        // Swap front and back buffers
        window.swap_buffers();
    }
    let points = bresenham(0, 0, 10, 5);
    for (x, y) in points {
        println!("({}, {})", x, y);
    }
}

