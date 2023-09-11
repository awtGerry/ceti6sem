mod window;

const BACKGROUND: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
const RECTANGLE_FG: [f32; 4] = [0.0, 1.0, 0.0, 1.0];

fn main() {
    window::create_window(BACKGROUND);
}
