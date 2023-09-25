#[allow(unused_imports)]
use engine::graphics::shader::Shader;

use crate::figures;

extern crate gl;

extern crate chrono;
use chrono::Local;

const CLOCK_RADIUS: f32 = 80.0;

// Clock design
pub fn draw_clock() {
    figures::draw_circle_fill(165.0, 300.0, CLOCK_RADIUS);
    clock_hours();
}

#[allow(unused_variables)]
fn clock_hours() {
    let mut seconds: f32 = chrono::Timelike::second(&Local::now()) as f32;
    let mut minutes: f32 = chrono::Timelike::minute(&Local::now()) as f32;
    let mut hours: f32 = chrono::Timelike::hour(&Local::now()) as f32;

    // Convertion
    seconds = (seconds * 6.0) - 90.0;
    minutes = (minutes as f32 * 6.0) - 90.0;
    hours = (hours as f32 * 30.0) - 90.0;

    unsafe {
        gl::LineWidth(4.0);

        let shader = Shader::new("clock/shaders/red.shader.vs", "clock/shaders/red.shader.fs");
        shader.bind();
        figures::draw_polar_hand(
            -0.675,
            -0.2,
            0.14,
            seconds,
        );
        let shader = Shader::new("clock/shaders/blue.shader.vs", "clock/shaders/blue.shader.fs");
        shader.bind();
        figures::draw_polar_hand(
            -0.675,
            -0.2,
            0.11,
            minutes,
        );
        let shader = Shader::new("clock/shaders/black.shader.vs", "clock/shaders/black.shader.fs");
        shader.bind();
        figures::draw_polar_hand(
            -0.675,
            -0.2,
            0.08,
            hours,
        );

    }
}
