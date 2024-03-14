use std::ops::Add;

use macroquad::{prelude::*, window};

fn setup() {
    window::request_new_screen_size(640.0, 240.0);
}

struct Mover {
    position: Vec2,
    velocity: Vec2,
    acceleration: Vec2,
    top_speed: f32,
}

impl Mover {
    fn new() -> Self {
        return Self {
            position: vec2(screen_width() / 2.0, screen_height() / 2.0),
            velocity: vec2(0.0, 0.0),
            acceleration: vec2(0.0, 0.0),
            top_speed: 5.0,
        };
    }

    fn update(&mut self) {
        self.acceleration = vec2(rand::gen_range(-1.0, 1.0), rand::gen_range(-1.0, 1.0));
        self.acceleration = self.acceleration * rand::gen_range(0.0, 2.0);

        self.velocity = self.velocity.add(self.acceleration);
        self.velocity.clamp_length(0.0, self.top_speed);
        self.position = self.position.add(self.velocity);
    }

    fn show(&self) {
        let color = Color::from_rgba(127, 127, 127, 255);
        draw_circle(self.position.x, self.position.y, 48.0, color);
    }

    fn check_edges(&mut self) {
        if self.position.x > screen_width() {
            self.position.x = 0.0;
        } else if self.position.x < 0.0 {
            self.position.x = screen_width();
        }

        if self.position.y > screen_height() {
            self.position.y = 0.0;
        } else if self.position.y < 0.0 {
            self.position.y = screen_height();
        }
    }
}

#[macroquad::main("Motion 101 (Velocity and Random Acceleration)")]
async fn main() {
    setup();

    let mut mover = Mover::new();

    loop {
        clear_background(WHITE);

        mover.update();
        mover.check_edges();
        mover.show();

        next_frame().await
    }
}
