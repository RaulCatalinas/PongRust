use macroquad::{
    main,
    prelude::{
        BLACK, BLUE, Conf, DARKGRAY, GREEN, clear_background, draw_line, draw_rectangle, draw_text,
        next_frame, screen_width,
    },
};

fn window_conf() -> Conf {
    Conf {
        window_title: String::from("Pong"),
        window_width: 800,
        window_height: 600,
        window_resizable: false,
        ..Default::default()
    }
}

#[main(window_conf)]
async fn main() {
    loop {
        clear_background(BLACK);

        draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);

        draw_text("Hello, Macroquad!", 20.0, 20.0, 30.0, DARKGRAY);

        next_frame().await
    }
}
