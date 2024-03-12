use std::ops::Add;

use macroquad::{prelude::*, window};

fn setup() {
    window::request_new_screen_size(640.0, 240.0);
}

#[macroquad::main("Bouncing Ball with Vectors")]
async fn main() {
    setup();

    let mut position = vec2(100.0, 100.0);
    let mut velocity = vec2(2.5, 2.0);

    loop {
        clear_background(WHITE);

        // adding 2 vectors returns a new vector
        position = position.add(velocity);

        if position.x > screen_width() || position.x < 0.0 {
            velocity.x = velocity.x * -1.0;
        }

        if position.y > screen_height() || position.y < 0.0 {
            velocity.y = velocity.y * -1.0;
        }

        let color = Color::new(127.0 / 255.0, 127.0 / 255.0, 127.0 / 255.0, 1.0);

        draw_circle(position.x, position.y, 48.0, color);

        next_frame().await
    }
}
