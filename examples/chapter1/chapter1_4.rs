use macroquad::{prelude::*, window};

fn setup() {
    window::request_new_screen_size(640.0, 240.0);
}

#[macroquad::main("Multiplying a Vector")]
async fn main() {
    setup();

    let stroke_width = 5.0;

    loop {
        clear_background(WHITE);

        let mut mouse = vec2(mouse_position().0, mouse_position().1);
        let center = vec2(screen_width() / 2.0, screen_height() / 2.0);
        mouse = mouse - center;

        let stroke_color = Color::from_rgba(200, 200, 200, 255);
        draw_line(
            center.x,
            center.y,
            center.x + mouse.x,
            center.y + mouse.y,
            stroke_width,
            stroke_color,
        );

        mouse = mouse * 0.5;
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
