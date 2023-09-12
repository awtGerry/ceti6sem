#[allow(unused_imports)]
use glfw::{Action, Context, Key, WindowEvent};

use crate::algorithms::*;

/* -- Manejar nuevos eventos de ventana --
  Crear argumentos para la función de callback de eventos de ventana

  Ejemplo:
  let (mut window, event) = window::init(800, 600, "Title");
  window.set_key_polling(true);
  window.init_gl();
  etc...

*/

pub struct Window {
    #[allow(unused)]
    glfw: glfw::Glfw,
    window: glfw::Window,
    events: std::sync::mpsc::Receiver<(f64, glfw::WindowEvent)>,
}

impl Window {
    // Funcion para crear una nueva ventana
    pub fn new(width: u32, height: u32, title: &str) -> Window {
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
        let (mut window, events) = glfw
            .create_window(width, height, title, glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window.");

        window.make_current();
        window.set_key_polling(true);
        window.set_framebuffer_size_polling(true);

        Window {
            glfw,
            window,
            events,
        }
    }

    pub fn init(&mut self) {
        self.window.make_current();
        gl::load_with(|symbol| self.window.get_proc_address(symbol) as *const _);
    }

    pub fn should_close(&mut self) -> bool {
        self.window.should_close()
    }

    pub fn update(&mut self) {
        self.handle_events();
        self.window.swap_buffers();
        self.glfw.poll_events();
    }

    fn handle_events(&mut self) {
        for(_, event) in glfw::flush_messages(&self.events) {
            match event {
                glfw::WindowEvent::FramebufferSize(width, height) => unsafe {
                    gl::Viewport(0, 0, width, height);
                },
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    self.window.set_should_close(true);
                },

                glfw::WindowEvent::Key(Key::X, _, Action::Press, _) => { // DER, IZQ
                    unsafe {
                        gl::Viewport(0, 0, 800, 600);
                        gl::Clear(gl::COLOR_BUFFER_BIT);
                    }
                    draw_pixel(0, 0);
                },

                /* LINES EVENTS */
                glfw::WindowEvent::Key(Key::D, _, Action::Press, _) => { // DER, IZQ
                    unsafe {
                        gl::Viewport(0, 0, 800, 600);
                        gl::Clear(gl::COLOR_BUFFER_BIT);
                    } // De momento el eje se define por algoritmo
                    // TODO: Arreglar esto, si las coordenadas son numeros grandes el dibujo se desvanece
                    // dda_line(100.0, 100.0, -200.0, 200.0)
                    // dda_line(-0.2, 0.0, -0.8, 0.9)
                    dda_line(0.2, 0.0, 0.8, 0.9)
                },
                glfw::WindowEvent::Key(Key::I, _, Action::Press, _) => { // IZQ, DER
                    unsafe {
                        gl::Viewport(0, 0, 800, 600);
                        gl::Clear(gl::COLOR_BUFFER_BIT);
                    } // De momento el eje se define por algoritmo
                    dda_line(-0.2, 0.0, -0.8, 0.9)
                },
                glfw::WindowEvent::Key(Key::B, _, Action::Press, _) => {
                    unsafe {
                        gl::Viewport(-400, -300, 800, 600);
                        gl::Clear(gl::COLOR_BUFFER_BIT);
                    } // De momento el eje se define por algoritmo
                    bresenham_line(0.9, 0.9, 0.9, 0.9) // Decimales como enteros por el momento
                },
                glfw::WindowEvent::Key(Key::R, _, Action::Press, _) => {
                    unsafe {
                        gl::Viewport(0, 0, 800, 600);
                        gl::Clear(gl::COLOR_BUFFER_BIT);
                    } // De momento el eje se define por algoritmo
                    draw_rectangle(0.5, 0.5, -0.5, -0.5)
                },
                /* END OF LINES EVENTS */

                /* CIRCLES EVENTS */
                glfw::WindowEvent::Key(Key::C, _, Action::Press, _) => {
                    unsafe {
                        gl::Viewport(0, 0, 800, 600);
                        gl::Clear(gl::COLOR_BUFFER_BIT);
                    } // De momento el eje se define por algoritmo
                    draw_circle(20.0, 200.0, 100.0, 200.0)
                },
                glfw::WindowEvent::Key(Key::M, _, Action::Press, _) => {
                    unsafe {
                        gl::Viewport(0, 0, 800, 600);
                        gl::Clear(gl::COLOR_BUFFER_BIT);
                    } // De momento el eje se define por algoritmo
                    draw_mid_point_circle(200.0, 200.0, 100.0)
                },
                glfw::WindowEvent::Key(Key::P, _, Action::Press, _) => {
                    unsafe {
                        gl::Viewport(0, 0, 800, 600);
                        gl::Clear(gl::COLOR_BUFFER_BIT);
                    } // De momento el eje se define por algoritmo
                    polar_circle(20.0, 200.0, 100.0, 200.0)
                },
                /* END OF CIRCLES EVENTS */

                glfw::WindowEvent::Key(Key::E, _, Action::Press, _) => {
                    unsafe {
                        gl::Viewport(0, 0, 800, 600);
                        gl::Clear(gl::COLOR_BUFFER_BIT);
                    } // De momento el eje se define por algoritmo
                    ellipse(200.0, 200.0, 100.0, 50.0)
                },
                _ => {}
            }
        }
    }
}
