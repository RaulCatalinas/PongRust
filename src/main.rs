#![windows_subsystem = "windows"]

mod constants;
mod engine;
mod entities;
mod game;
mod macros;
mod physics;
mod types;

use macroquad::window::Conf;

use crate::engine::window::WindowConfig;

fn window_conf() -> Conf {
    Conf {
        window_title: String::from("Pong"),
        window_width: 800,
        window_height: 600,
        window_resizable: false,
        ..Default::default()
    }
}

fn main() {
    engine::window::create_window(WindowConfig {
        title: String::from("Pong in Rust"),
        ..Default::default()
    });
}
