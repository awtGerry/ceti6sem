#![allow(unused_imports)]
use graphics::Context;
use opengl_graphics::{OpenGL, GlGraphics};
use piston::{Window, WindowSettings, RenderArgs, Events, EventSettings};

fn create_color([r, g, b, a]: [f32; 4]) -> [f32; 4] {
    let color = [r, g, b, a];
    color
}

const BLACK: [f32; 4] = create_color([0.0, 0.0, 0.0, 1.0]);
const BLUE: [f32; 4] = create_color([0.0, 0.0, 1.0, 1.0]);

struct App {
    pub(crate) gl: GlGraphics,
}

impl App {
    fn render_window(&mut self, args: &RenderArgs) {
        self.gl.draw(args.viewport(), |c,g| {
            graphics::clear(BLACK, g);
            graphics::rectangle(BLUE, [0.0, 0.0, 100.0, 100.0], c.transform, g);
        })
    }
}

/* fn draw_pixel() {

    let opengl = OpenGL::V4_5;
    let window = WindowSettings::new("Hello Piston!", [640, 480])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();
    let mut gl = GlGraphics::new(opengl);
} */

pub fn run_app(app: &mut App, window: &mut Window) {
    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(window) {
        if let Some(args) = e.render_args() {
            app.render_window(&args);
        }

        if let Some(_args) = e.update_args() {
            app.update_window(&_args);
        }
    }
    let mut app = App {
        gl: GlGraphics::new(OpenGL::V4_5),
    };

    run_app(&mut app, &mut window);
}
