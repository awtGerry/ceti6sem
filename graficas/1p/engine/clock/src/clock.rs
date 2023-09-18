// use engine::graphics::shader::Shader;
// use engine::graphics::wrapper::*;

#[allow(unused_imports)]
use crate::figures;
use engine::graphics::shader::Shader;

extern crate gl;

extern crate chrono;
use chrono::Local;

const CLOCK_RADIUS: f32 = 80.0;

// Clock design
pub fn draw_clock(hours: u32, minutes: u32) {
    figures::draw_circle_fill(165.0, 300.0, CLOCK_RADIUS);
    clock_hours(hours, minutes);
}

#[allow(unused_variables)]
fn clock_hours(hours: u32, minutes: u32) {
    let shader = Shader::new("clock/shaders/black.shader.vs", "clock/shaders/black.shader.fs");

    /* Just the seconds are going to be used in the loop, minutes and hours are going to be
       updated manually to save memory. */

    let seconds: f32 = chrono::Timelike::second(&Local::now()) as f32;
    println!("Seconds: {}", seconds);
    let seconds: f32 = (seconds * 6.0) - 90.0;

    unsafe {
        shader.bind();
        // Make lines bigger
        gl::LineWidth(2.0);

        // 0 seconds = (-0.675, 0.0)
        // 15 seconds = (-0.5, -0.2)
        // 30 seconds = (-0.675, -0.4)
        // 45 seconds = (-0.85, -0.2)
        let x = seconds.to_radians().cos() * CLOCK_RADIUS;
        let y = -(seconds.to_radians().sin() * CLOCK_RADIUS);

        println!("x: {}, y: {}", x, y);

        figures::draw_small_line(
            x,
            y,
            -0.675,
            -0.2
        );
    }
}
