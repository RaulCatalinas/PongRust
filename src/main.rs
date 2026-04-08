mod constants;
mod engine;
mod entities;
mod game;
mod macros;
mod physics;
mod types;

use macroquad::window::Conf;

use crate::engine::{game::Game, window::WindowConfig};

fn window_conf() -> Conf {
    Conf {
        window_title: String::from("Pong"),
        window_width: 800,
        window_height: 600,
        window_resizable: false,
        ..Default::default()
    }
}

struct PongGame;

impl Game for PongGame {
    fn start(&mut self) {
        println!("Game started!");
    }

    fn update(&mut self) {
        engine::ui::text::draw("Hello World", 0.0, 0.0, 20.0);
        engine::ui::buttons::create_button(engine::ui::buttons::ButtonConfig {
            text: "Click Me",
            x: 350.0,
            y: 280.0,
            width: 100.0,
            height: 40.0,
            on_click: || {
                println!("Button clicked!");
            },
        });
    }
}

fn main() {
    engine::game_builder::GameBuilder::new()
        .with_window(WindowConfig {
            title: String::from("Pong in Rust"),
            resizable: false,
            maximizable: false,
            ..Default::default()
        })
        .run(PongGame);
}
