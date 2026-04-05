mod entities;
mod game;
mod physics;
mod types;

use macroquad::{main, window::Conf};

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
    let mut game = game::Game::new(
        window_conf().window_width as f32,
        window_conf().window_height as f32,
    );

    game.run().await;
}
