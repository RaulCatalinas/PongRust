pub mod ui;

use macroquad::{
    color::BLACK,
    window::{clear_background, next_frame},
};

use crate::{
    constants,
    entities::{ball::Ball, paddle::Paddle},
    physics,
    types::game_object::GameObject,
};

pub struct Game {
    ball: Ball,
    player1: Paddle,
    player2: Paddle,
    window_width: f32,
    window_height: f32,
    score_player1: u8,
    score_player2: u8,
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
        }
    }

    pub async fn run(&mut self) {
        loop {
            clear_background(BLACK);

            if self.win() {
                let restart = ui::render_win_screen(self.score_player1 > self.score_player2);

                if restart {
                    self.restart_game();
                }

                next_frame().await;

                continue;
            }

            self.update();
            self.draw();

            next_frame().await
        }
    }

    fn update(&mut self) {
        self.player1.update();
        self.player2.update();
        self.ball.update();

        self.handle_collisions();
    }

    fn draw(&self) {
        ui::render_game_scoreboard(self.score_player1, self.score_player2);
        self.player1.draw();
        self.player2.draw();
        self.ball.draw();
    }

    fn handle_collisions(&mut self) {
        let ball_collision_with_player1 =
            physics::ball::check_ball_paddle_collision(&self.ball, &self.player1);
        let ball_collision_with_player2 =
            physics::ball::check_ball_paddle_collision(&self.ball, &self.player2);

        let (
            ball_collision_with_left_wall,
            ball_collision_with_right_wall,
            ball_collision_with_wall_y,
        ) = physics::ball::check_ball_wall_collision(
            &self.ball,
            self.window_width,
            self.window_height,
        );

        if ball_collision_with_player1 || ball_collision_with_player2 {
            self.ball.invert_velocity(true, false);
        }

        if ball_collision_with_wall_y {
            self.ball.invert_velocity(false, true);
        }

        if ball_collision_with_left_wall {
            self.score_player2 += 1;
            self.reset_game_objects();
        }

        if ball_collision_with_right_wall {
            self.score_player1 += 1;
            self.reset_game_objects();
        }
    }

    fn reset_game_objects(&mut self) {
        self.ball.reset();
        self.player1.reset();
        self.player2.reset();
    }

    fn win(&self) -> bool {
        self.score_player1 >= constants::WINNING_SCORE
            || self.score_player2 >= constants::WINNING_SCORE
    }

    fn restart_game(&mut self) {
        self.score_player1 = 0;
        self.score_player2 = 0;

        self.reset_game_objects();
    }
}
