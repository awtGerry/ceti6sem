use engine::graphics::window::Window;

use engine::algorithms::figures::fill_rectangle;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

fn main() {
    let mut window = Window::new(WIDTH, HEIGHT, "Title");

    window.init();

    while !window.should_close() {

        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            fill_rectangle(-0.5, -0.5, 0.5, 0.5);
            gl::DrawArrays(gl::LINES, 0, 5);
        }

        window.update();
    }
}
