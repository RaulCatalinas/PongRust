mod scenes;
mod ui;

use macroquad::{
    color::BLACK,
    window::{clear_background, next_frame},
};

use crate::{
    entities::{ball::Ball, paddle::Paddle},
    game::scenes::Scene,
};

pub struct Game {
    ball: Ball,
    player1: Paddle,
    player2: Paddle,
    window_width: f32,
    window_height: f32,
    score_player1: u8,
    score_player2: u8,
    scene: scenes::Scene,
}

impl Game {
    pub fn new(window_width: f32, window_height: f32) -> Self {
        Self {
            ball: Ball::new(),
            player1: Paddle::new(50.0, 250.0, true),
            player2: Paddle::new(740.0, 250.0, false),
            window_width,
            window_height,
            score_player1: 0,
            score_player2: 0,
            scene: scenes::Scene::MainMenu,
        }
    }

    pub async fn run(&mut self) {
        loop {
            clear_background(BLACK);

            match self.scene {
                Scene::MainMenu => self.main_menu_scene(),
                Scene::Game => self.game_scene(),
                Scene::Win => self.win_scene(self.score_player1 > self.score_player2),
            }

            next_frame().await
        }
    }
}
