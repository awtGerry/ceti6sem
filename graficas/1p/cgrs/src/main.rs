use cgrs::window::Window;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

fn main() {
    let mut window = Window::new(WIDTH, HEIGHT, "Title");

    window.init();

    while !window.should_close() {

        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0); // black background
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::LineWidth(8.0);
            gl::DrawArrays(gl::LINES, 0, 5);
        }

        window.update();
    }
}
