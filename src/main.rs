mod entities;
mod types;

use macroquad::{
    main,
    prelude::{BLACK, Conf, clear_background, next_frame},
};

use crate::{entities::ball::Ball, types::game_object::GameObject};

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
    let mut ball = Ball::new();

    ball.reset_position();

    loop {
        clear_background(BLACK);

        ball.update();
        ball.draw();

        next_frame().await
    }
}
