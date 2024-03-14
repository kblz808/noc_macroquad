use macroquad::{prelude::*, window};

fn setup() {
    window::request_new_screen_size(640.0, 240.0);
}

#[macroquad::main("Bouncing Ball with No Vectors")]
async fn main() {
    setup();

    let mut x = 100.0;
    let mut y = 100.0;
    let mut xspeed = 2.5;
    let mut yspeed = 2.0;

    loop {
        clear_background(WHITE);

        x = x + xspeed;
        y = y + yspeed;

        if x > screen_width() || x < 0.0 {
            xspeed = xspeed * -1.0;
        }
        if y > screen_height() || y < 0.0 {
            yspeed = yspeed * -1.0;
        }

        let color = Color::from_rgba(127, 127, 127, 255);

        draw_circle(x, y, 48.0, color);

        next_frame().await
    }
}
