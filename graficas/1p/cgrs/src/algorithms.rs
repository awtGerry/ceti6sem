fn dda_line(x1: i32, y1: i32, x2: i32, y2: i32) {
    let dx = x2 - x1;
    let dy = y2 - y1;

    let steps = if dx.abs() > dy.abs() { dx.abs() } else { dy.abs() };

    let x_inc = dx as f32 / steps as f32;
    let y_inc = dy as f32 / steps as f32;

    let mut x = x1 as f32;
    let mut y = y1 as f32;

    for _ in 0..steps {
        unsafe {
            gl::VertexP2ui(x as u32, y as u32);
        }
        x += x_inc;
        y += y_inc;
    }
}
