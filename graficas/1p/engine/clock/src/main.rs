use engine::graphics::window::Window;

mod figures;
mod buildings;
mod clock;

const WIDTH: u32 = 1000;
const HEIGHT: u32 = 768;

fn main() {
    let mut window = Window::new(WIDTH, HEIGHT, "Clock");

    window.init();
    while !window.should_close() {
        unsafe {
            gl::ClearColor(0.11, 0.16, 0.38, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            buildings::draw_buildings();
            figures::draw_circle_fill(580.0, 500.0, 200.0);
            clock::draw_clock();
        }
        window.update();
    }
}
