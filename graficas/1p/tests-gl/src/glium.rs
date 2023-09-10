#[allow(unused)]
#[allow(unused_imports)]

use glium::Surface;

// fn loop() {
//     let event_loop = winit::eve
// }

// pub fn glium() -> String {
//     format!("Hello from glium")
// }

fn loop() {
    let event_loop = winit::event_loop::EventLoopBuilder::new().build();
    let (_window, display) = glium::glutin::window::WindowBuilder::new().build(&event_loop);

    let mut frame = display.draw();
    frame.clear_color(0.0, 0.0, 1.0, 1.0);
    frame.finish().unwrap();

    event_loop.run(move |event, _, control_flow| {
        match event {
            winit::event::Event::WindowEvent { event, .. } => match event {
                winit::event::WindowEvent::CloseRequested => control_flow.set_exit(),
                _ => (),
            },
            _ => (),
        };
    });
}
