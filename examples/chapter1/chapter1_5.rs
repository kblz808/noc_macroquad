use macroquad::{prelude::*, window};

fn setup() {
    window::request_new_screen_size(640.0, 240.0);
}

#[macroquad::main("Vector Magnitude")]
async fn main() {
    setup();

    let stroke_width = 5.0;

    loop {
        clear_background(WHITE);

        let mut mouse = vec2(mouse_position().0, mouse_position().1);
        let center = vec2(screen_width() / 2.0, screen_height() / 2.0);
        mouse = mouse - center;

        let m = mouse.length();
        draw_rectangle(0.0, 0.0, m, 10.0, BLACK);

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
