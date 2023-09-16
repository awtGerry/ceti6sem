use engine::graphics::window::Window;

mod figures;

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
            /* CLOCK TOWER {{ */
            figures::draw_rectangle_fill(-0.9, 0.0, -0.45, -1.0);
            figures::draw_rectangle_fill(-0.85, -1.0, -0.5, 0.15);

            // LIGHTS HERE

            figures::draw_roof(-0.85, 0.1, -0.5, 0.4, -0.75, -0.6);
            figures::draw_line(-0.77, 0.4, -0.58, 0.4);
            gl::DrawArrays(gl::LINES, 0, 5);
            figures::draw_rectangle_fill(-0.75, 0.4, -0.6, 0.55);

            // LIGHTS HERE

            figures::draw_triangle(-0.75, 0.55, -0.6, 0.75, -0.675);
            figures::draw_line(-0.675, 0.74, -0.675, 0.85);
            gl::DrawArrays(gl::LINES, 0, 5);
            /* }} END: CLOCK TOWER */

            /* BUILDINGS {{ */
            figures::draw_rectangle_fill(-0.45, -1.0, -0.25, -0.8);
            figures::draw_rectangle_fill(0.1, -1.0, 0.4, -0.65);
            figures::draw_rectangle_fill(0.75, -1.0, 1.0, -0.75);

            figures::draw_rectangle_fill(-0.25, -1.0, 0.1, -0.5);
            figures::draw_rectangle_fill(-0.23, -0.5, -0.18, -0.41);
            figures::draw_triangle(-0.23, -0.41, -0.18, -0.33, -0.205);
            for i in 0..9 {
                figures::draw_line(-0.16 + (i as f32 * 0.02), -0.5, -0.16 + (i as f32 * 0.02), -0.47);
                gl::DrawArrays(gl::LINES, 0, 5);
            }
            figures::draw_rectangle_fill(0.02, -0.5, 0.07, -0.41);
            figures::draw_triangle(0.02, -0.41, 0.07, -0.33, 0.045);

            figures::draw_rectangle_fill(0.4, -1.0, 0.75, -0.5);
            figures::draw_rectangle_fill(0.42, -0.5, 0.47, -0.41);
            figures::draw_triangle(0.42, -0.41, 0.47, -0.33, 0.445);
            for i in 0..9 {
                figures::draw_line(0.49 + (i as f32 * 0.02), -0.5, 0.49 + (i as f32 * 0.02), -0.47);
                gl::DrawArrays(gl::LINES, 0, 5);
            }
            figures::draw_rectangle_fill(0.67, -0.5, 0.72, -0.41);
            figures::draw_triangle(0.67, -0.41, 0.72, -0.33, 0.695);
            /* }} END: BUILDINGS */
        }
        window.update();
    }
}
