#[allow(unused_imports)]
use glfw::{Action, Context, Key, WindowEvent};

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

    #[allow(unused)]
    fn handle_events(&mut self) {
        for(_, event) in glfw::flush_messages(&self.events) {
            match event {
                glfw::WindowEvent::FramebufferSize(width, height) => unsafe {
                    gl::Viewport(0, 0, width, height);
                },
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    self.window.set_should_close(true);
                },
                _ => {}
            }
        }
    }
}
