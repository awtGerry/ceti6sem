extern crate glfw;
extern crate gl;

use glfw::{Context, Key, Action};
use gl::types::*;

fn main() {
    // Initialize GLFW
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    // Create a windowed mode window and its OpenGL context
    let (mut window, events) = glfw.create_window(800, 600, "Line Drawing", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    // Make the window's context current
    window.make_current();

    // Initialize OpenGL (make sure you have the necessary OpenGL bindings)
    gl::load_with(|s| window.get_proc_address(s) as *const _);
    
    // Coordinates of the two points to draw a line between
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
}
