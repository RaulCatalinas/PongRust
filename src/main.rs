mod entities;
mod types;

use macroquad::{
    main,
    prelude::{BLACK, Conf, clear_background, next_frame},
};

use crate::{
    entities::{ball::Ball, paddle::Paddle},
    types::game_object::GameObject,
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
    let mut ball = Ball::new();
    let mut player1 = Paddle::new(50.0, 250.0, true);
    let mut player2 = Paddle::new(740.0, 250.0, false);
    loop {
        clear_background(BLACK);

        player1.update();
        player1.draw();

        player2.update();
        player2.draw();

        ball.update();
        ball.draw();

        next_frame().await
    }
}
