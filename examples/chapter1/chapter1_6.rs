use macroquad::{prelude::*, window};

fn setup() {
    window::request_new_screen_size(640.0, 240.0);
}

#[macroquad::main("Normalizing a Vector")]
async fn main() {
    setup();

    loop {
        clear_background(WHITE);

        let mut mouse = vec2(mouse_position().0, mouse_position().1);
        let center = vec2(screen_width() / 2.0, screen_height() / 2.0);
        mouse = mouse - center;

        let color = Color::from_rgba(200, 200, 200, 255);

        let mut stroke_width = 4.0;

        draw_line(
            center.x,
            center.y,
            center.x + mouse.x,
            center.y + mouse.y,
            stroke_width,
            color,
        );

        mouse = mouse.normalize();
        mouse = mouse * 50.0;

        stroke_width = 12.0;

        draw_line(
            center.x,
            center.y,
            center.x + mouse.x,
            center.y + mouse.y,
            stroke_width,
            BLACK,
        );

        next_frame().await
    }
}
