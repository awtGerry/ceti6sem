use engine::graphics::window::Window;

mod figures;

fn main() {
    let mut window = Window::new(1366, 720, "Clock");

    window.init();

    while !window.should_close() {
        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            // gl::Clear(gl::COLOR_BUFFER_BIT);
            figures::draw_rectangle_fill(-0.5, -0.5, 0.5, 0.5);
            // TODO: There is a bug here, the line draw twice for some reason
            // gl::DrawArrays(gl::LINES, 0, 5);
        }
        window.update();
    }
}
