use graphics::{Context, Rectangle, Graphics};
use opengl_graphics::GlGraphics;
use opengl_graphics::OpenGL;
use piston::{
    WindowSettings,
    RenderArgs,
    UpdateArgs,
    Events,
    EventSettings,
    RenderEvent,
    UpdateEvent
};
use piston_window::PistonWindow;

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

pub fn render(gl: &mut GlGraphics, c: &Context) {
    gl.draw_rectangle(
        [0.0, 0.0, 100.0, 100.0],
        RED,
        &c
    );
}

struct App {
    pub(crate) gl: GlGraphics,
}

impl App {
    fn render(&mut self, args: &RenderArgs, color: [f32; 4]) {
        self.gl.draw(args.viewport(), |_c, gl| {
            gl.clear_color(color)
        });
    }

    fn update(&mut self, _args: &UpdateArgs) { }
}

fn run_app(app: &mut App, w: &mut PistonWindow, color: [f32; 4]) {
    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(w) {
        if let Some(args) = e.render_args() {
            app.render(&args, color);
        }
        if let Some(args) = e.update_args() {
            app.update(&args);
        }
    }
}

pub fn create_window(color: [f32; 4]) {
    let opengl = OpenGL::V4_5;
    let mut window: PistonWindow = WindowSettings::new(
        "Rusty",
        [800, 600])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();
    let gl = GlGraphics::new(opengl);

    let mut app = App { gl };
    run_app(&mut app, &mut window, color);
    render(&mut app.gl, &Context::new());
}
