use std::ops::Add;

use macroquad::{prelude::*, window};

struct Mover {
    mass: f32,
    radius: f32,
    position: Vec2,
    velocity: Vec2,
    acceleration: Vec2,
}

impl Mover {
    fn new(x: f32, y: f32, m: f32) -> Self {
        return Self {
            mass: m,
            radius: m * 8.0,
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

    fn display(&self) {
        let color = Color::from_rgba(127, 127, 127, 255);
        draw_circle(self.position.x, self.position.y, self.radius * 2.0, color);
    }

    fn check_edges(&mut self) {
        if self.position.x > screen_width() - self.radius {
            self.position.x = screen_width() - self.radius;
            self.velocity.x *= -1.0;
        } else if self.position.x < self.radius {
            self.position.x = self.radius;
            self.velocity.x *= -1.0;
        }

        if self.position.y > screen_height() - self.radius {
            self.position.y = screen_height() - self.radius;
            self.velocity.y *= -1.0;
        }
    }
}

fn setup() {
    window::request_new_screen_size(640.0, 240.0);
}

#[macroquad::main("Gravity Scaled by Mass")]
async fn main() {
    setup();

    let mut mover_a = Mover::new(200.0, 30.0, 10.0);
    let mut mover_b = Mover::new(400.0, 300.0, 2.0);

    loop {
        clear_background(WHITE);

        let gravity = vec2(0.0, 0.1);

        let gravity_a = gravity * mover_a.mass;
        mover_a.apply_force(gravity_a);

        let gravity_b = gravity * mover_b.mass;
        mover_b.apply_force(gravity_b);

        if is_mouse_button_down(MouseButton::Left) {
            let wind = vec2(0.1, 0.0);
            mover_a.apply_force(wind);
            mover_b.apply_force(wind);
        }

        mover_a.update();
        mover_a.display();
        mover_a.check_edges();

        mover_b.update();
        mover_b.display();
        mover_b.check_edges();

        next_frame().await
    }
}
