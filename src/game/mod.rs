use macroquad::{
    color::BLACK,
    window::{clear_background, next_frame},
};

use crate::{
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
}

impl Game {
    pub fn new(window_width: f32, window_height: f32) -> Self {
        Self {
            ball: Ball::new(),
            player1: Paddle::new(50.0, 250.0, true),
            player2: Paddle::new(740.0, 250.0, false),
            window_width,
            window_height,
        }
    }

    pub async fn run(&mut self) {
        loop {
            clear_background(BLACK);

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
        self.player1.draw();
        self.player2.draw();
        self.ball.draw();
    }

    fn handle_collisions(&mut self) {
        let ball_collision_with_player1 =
            physics::ball::check_ball_paddle_collision(&self.ball, &self.player1);
        let ball_collision_with_player2 =
            physics::ball::check_ball_paddle_collision(&self.ball, &self.player2);

        let (ball_collision_with_wall_x, ball_collision_with_wall_y) =
            physics::ball::check_ball_wall_collision(
                &self.ball,
                self.window_width,
                self.window_height,
            );

        if ball_collision_with_player1 || ball_collision_with_player2 || ball_collision_with_wall_x
        {
            self.ball.invert_velocity(true, false);
        }

        if ball_collision_with_wall_y {
            self.ball.invert_velocity(false, true);
        }
    }
}
