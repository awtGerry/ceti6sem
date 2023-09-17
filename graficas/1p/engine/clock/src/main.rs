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
            // gl::UseProgram(shader_program);
            // gl::BindVertexArray(vao);
            buildings::draw_buildings();
            // MOON
            figures::draw_circle(500.0, 500.0, 200.0);
        }
        window.update();
    }
}
