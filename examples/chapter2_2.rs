use std::ops::Add;

use macroquad::{prelude::*, window};

struct Mover {
    mass: f32,
    position: Vec2,
    velocity: Vec2,
    acceleration: Vec2,
}

impl Mover {
    fn new(x: f32, y: f32, m: f32) -> Self {
        return Self {
            mass: m,
            position: vec2(x, y),
            velocity: vec2(0.0, 0.0),
            acceleration: vec2(0.0, 0.0),
        };
    }

    fn apply_force(&mut self, force: Vec2) {
        let f = force / self.mass;
        self.acceleration = self.acceleration.add(f);
    }

    fn update(&mut self) {
        self.velocity = self.velocity.add(self.acceleration);
        self.position = self.position.add(self.velocity);
        self.acceleration = self.acceleration * 0.0;
    }

    fn show(&self) {
        let color = Color::from_rgba(127, 127, 127, 255);
        draw_circle(self.position.x, self.position.y, self.mass * 16.0, color);
    }

    fn check_edges(&mut self) {
        if self.position.x > screen_width() {
            self.position.x = screen_width();
            self.velocity.x = self.velocity.x * -1.0;
        } else if self.position.x < 0.0 {
            self.velocity.x = self.velocity.x * -1.0;
            self.position.x = 0.0;
        }

        if self.position.y > screen_height() {
            self.velocity.y = self.velocity.y * -1.0;
            self.position.y = screen_height();
        }
    }
}

fn setup() {
    window::request_new_screen_size(640.0, 240.0);
}

#[macroquad::main("Forces Acting on Two Objects")]
async fn main() {
    setup();

    let mut mover_a = Mover::new(200.0, 30.0, 10.0);
    let mut mover_b = Mover::new(400.0, 300.0, 2.0);

    loop {
        clear_background(WHITE);

        let gravity = vec2(0.0, 0.1);
        mover_a.apply_force(gravity);
        mover_b.apply_force(gravity);

        if is_mouse_button_down(MouseButton::Left) {
            let wind = vec2(0.1, 0.0);
            mover_a.apply_force(wind);
            mover_b.apply_force(wind);
        }

        mover_a.update();
        mover_a.show();
        mover_a.check_edges();

        mover_b.update();
        mover_b.show();
        mover_b.check_edges();

        next_frame().await
    }
}
