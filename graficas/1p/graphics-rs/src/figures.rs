use graphics::{Context, Rectangle};
use opengl_graphics::GlGraphics;

const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

pub trait DrawRectangle {
    fn draw_rectangle(
        &mut self,
        r: [f64;4],
        color: [f32; 4],
        c: &Context
    );
}

impl DrawRectangle for GlGraphics {
    fn draw_rectangle(
        &mut self,
        r: [f64;4],
        color: [f32; 4],
        c: &Context
    ) {
        Rectangle::new(color).draw(r, &c.draw_state, c.transform, self);
    }

}

pub fn render(gl: &mut GlGraphics) {
    gl.draw_rectangle([0.0, 0.0, 100.0, 100.0], RED, &Context::new_abs(0.0, 0.0));
}

pub fn draw_txt() {
    // let ttf = Pathbuf::from("assets/FiraSans-Regular.ttf");
}
