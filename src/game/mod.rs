mod scenes;
mod sfx;
mod ui;

use macroquad::{
    audio::Sound,
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
    ball_bounce_off_the_paddle: Sound,
    marked_point: Sound,
    bounce_off_the_walls: Sound,
}

impl Game {
    pub async fn new(window_width: f32, window_height: f32) -> Self {
        Self {
            ball: Ball::new(),
            player1: Paddle::new(50.0, 250.0, true),
            player2: Paddle::new(740.0, 250.0, false),
            window_width,
            window_height,
            score_player1: 0,
            score_player2: 0,
            scene: scenes::Scene::MainMenu,
            ball_bounce_off_the_paddle: sfx::load_sound_effect(
                "sfx/ball_bounce_off_the_paddle.wav",
            )
            .await,
            marked_point: sfx::load_sound_effect("sfx/marked_point.wav").await,
            bounce_off_the_walls: sfx::load_sound_effect("sfx/bounce_off_the_walls.wav").await,
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
