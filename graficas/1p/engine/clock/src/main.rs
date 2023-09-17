use engine::graphics::window::Window;

mod figures;
mod buildings;

const WIDTH: u32 = 1000;
const HEIGHT: u32 = 768;

fn main() {
    let mut window = Window::new(WIDTH, HEIGHT, "Clock");

    window.init();

    while !window.should_close() {
        unsafe {
            // Blue sky background
            gl::ClearColor(0.19, 0.24, 0.98, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            buildings::build_clock_tower();
            buildings::build_houses();
        }
        window.update();
    }
}
