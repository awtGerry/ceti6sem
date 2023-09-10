#[allow(unused_imports)]
extern crate gl;
extern crate glutin;

use glutin::dpi::LogicalSize;
use glutin::event_loop::EventLoop;
use glutin::window::WindowBuilder;
use glutin::ContextBuilder;
use glutin::event::{Event, WindowEvent};
use gl::types::*;

fn main() {
    // Create an event loop
    let event_loop = EventLoop::new();

    // Create a window
    let wb = WindowBuilder::new()
        .with_title("Rust OpenGL Pixel")
        .with_inner_size(LogicalSize::new(800.0, 600.0));
    let cb = ContextBuilder::new().with_vsync(true);
    let display = glutin::Context::new(wb, cb, &event_loop).unwrap();

    // Initialize OpenGL
    gl::load_with(|symbol| display.get_proc_address(symbol) as *const _);
    unsafe {
        gl::ClearColor(0.0, 0.0, 0.0, 1.0);
    }

    // Main event loop
    event_loop.run(move |event, _, control_flow| {
        *control_flow = glutin::event_loop::ControlFlow::Poll;

        match event {
            Event::WindowEvent { event, window_id } if window_id == display.window().id() => match event {
                WindowEvent::CloseRequested => *control_flow = glutin::event_loop::ControlFlow::Exit,
                _ => (),
            },
            Event::LoopDestroyed => return,
            _ => (),
        }

        // Render a pixel
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::PointSize(10.0);
            gl::Begin(gl::POINTS);
            gl::Color3f(1.0, 0.0, 0.0); // Red color
            gl::Vertex2f(0.0, 0.0);     // Coordinates of the pixel
            gl::End();
        }

        // Swap buffers
        display.swap_buffers().unwrap();
    });
}
