use engine::graphics::window::Window;

mod figures;

use std::ptr;

const WIDTH: u32 = 1000;
const HEIGHT: u32 = 768;
const R: f32 = 80.0;

fn main() {
    let mut window = Window::new(WIDTH, HEIGHT, "Class testing");

    window.init();
    while !window.should_close() {
        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            figures::draw_rectangle(0.5, 0.5, -0.5, -0.5);
            figures::scan_line(-0.5, 0.5);
            figures::draw_circle_fill(165.0, 120.0, R);
            // gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
            gl::DrawArrays(gl::LINES, 0, 5);
        }
        window.update();
    }
}
