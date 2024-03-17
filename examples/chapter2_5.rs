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

    fn check_edges(&mut self) {
        if self.position.y > screen_height() - self.radius {
            self.velocity.y *= -0.9;
            self.position.y = screen_height() - self.radius;
        }
    }
}

struct Liquid {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    c: f32,
}

impl Liquid {
    fn new(x: f32, y: f32, w: f32, h: f32, c: f32) -> Self {
        return Self { x, y, w, h, c };
    }

    fn contains(&self, mover: &Mover) -> bool {
        let pos = mover.position;
        return pos.x > self.x
            && pos.x < self.x + self.w
            && pos.y > self.y
            && pos.y < self.y + self.h;
    }

    fn calculate_drag(&self, mover: &Mover) -> Vec2 {
        let speed = mover.velocity.length();
        let drag_magnitude = self.c * speed * speed;

        let mut drag_force = mover.velocity;
        drag_force *= -1.0;

        drag_force.clamp_length(drag_magnitude, drag_magnitude);
        return drag_force;
    }

    fn show(&self) {
        let color = Color::from_rgba(220, 220, 220, 255);
        draw_rectangle(self.x, self.y, self.w, self.h, color);
    }
}

fn setup() {
    window::request_new_screen_size(640.0, 240.0);
}

#[macroquad::main("Including Friction")]
async fn main() {
    setup();

    let liquid = Liquid::new(
        0.0,
        screen_height() / 2.0,
        screen_width(),
        screen_height() / 2.0,
        0.1,
    );

    let mut movers = vec![];

    let reset = |movers: &mut Vec<Mover>| {
        movers.clear();
        for i in 0..9 {
            movers.push(Mover::new(
                40.0 + i as f32 * 70.0,
                0.0,
                rand::gen_range(0.5, 3.0),
            ));
        }
    };
    reset(&mut movers);

    loop {
        clear_background(WHITE);

        liquid.show();

        for i in 0..movers.len() {
            if liquid.contains(&movers[i]) {
                let dragged_force = liquid.calculate_drag(&movers[i]);
                movers[i].apply_force(dragged_force);
            }

            let gravity = vec2(0.0, 0.1 * movers[i].mass);
            movers[i].apply_force(gravity);

            movers[i].update();
            movers[i].show();
            movers[i].check_edges();

            if is_mouse_button_down(MouseButton::Left) {
                reset(&mut movers);
            }
        }

        next_frame().await
    }
}
