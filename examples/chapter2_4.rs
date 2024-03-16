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

    fn show(&self) {
        let color = Color::from_rgba(127, 127, 127, 255);
        draw_circle(self.position.x, self.position.y, self.radius * 2.0, color);
    }

    fn contact_edge(&self) -> bool {
        return self.position.y > screen_height() - self.radius - 1.0;
    }

    fn bounce_endges(&mut self) {
        let bounce = -0.9;

        if self.position.x > screen_width() - self.radius {
            self.position.x = screen_width() - self.radius;
            self.velocity.x *= bounce;
        } else if self.position.x < self.radius {
            self.position.x = self.radius;
            self.velocity.x *= bounce
        }

        if self.position.y > screen_height() - self.radius {
            self.position.y = screen_height() - self.radius;
            self.velocity.y *= bounce;
        }
    }
}

fn setup() {
    window::request_new_screen_size(640.0, 240.0);
}

#[macroquad::main("Including Friction")]
async fn main() {
    setup();

    let mut mover = Mover::new(screen_width() / 2.0, 30.0, 5.0);

    loop {
        clear_background(WHITE);

        let gravity = vec2(0.0, 1.0);
        mover.apply_force(gravity);

        if is_mouse_button_down(MouseButton::Left) {
            let wind = vec2(0.5, 0.0);
            mover.apply_force(wind);
        }

        if mover.contact_edge() {
            let c = 0.1;
            let mut friction = mover.velocity;
            friction *= -1.0;

            friction.clamp_length(c, c);

            mover.apply_force(friction);
        }

        mover.bounce_endges();
        mover.update();
        mover.show();

        next_frame().await
    }
}
