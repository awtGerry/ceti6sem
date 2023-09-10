use graphics::Graphics;
use opengl_graphics::{OpenGL, GlGraphics};
use piston::{
    WindowSettings,
    RenderArgs,
    UpdateArgs, Events, EventSettings, RenderEvent, UpdateEvent
};
use piston_window::PistonWindow;

const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

struct App {
    pub(crate) gl: GlGraphics,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        self.gl.draw(args.viewport(), |_c, gl| {
            gl.clear_color(BLACK)
        });
    }

    fn update(&mut self, _args: &UpdateArgs) { }
}

fn run_app(app: &mut App, w: &mut PistonWindow) {
    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(w) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }
        if let Some(args) = e.update_args() {
            app.update(&args);
        }
    }
}

fn main() {
    let opengl = OpenGL::V4_5;
    let mut window: PistonWindow = WindowSettings::new(
        "Title: Piston Window",
        [800, 600])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();
    let gl = GlGraphics::new(opengl);

    let mut app = App { gl };
    run_app(&mut app, &mut window);
}
