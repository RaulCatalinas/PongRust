use macroquad::window::Conf;

use crate::{
    constants,
    game::{Game, scenes::Scene, sfx, ui},
    physics,
    types::game_object::GameObject,
    window_conf,
};

impl Game {
    pub fn game_scene(&mut self) {
        self.update();
        self.draw();

        if self.win() {
            self.scene = Scene::Win;
        }
    }

    fn update(&mut self) {
        self.player1.update();
        self.player2.update();
        self.ball.update();

        self.handle_collisions();
    }

    fn draw(&self) {
        let Conf {
            window_width,
            window_height,
            ..
        } = window_conf();

        ui::render_game_scoreboard(self.score_player1, self.score_player2);
        ui::draw_center_line(window_width as f32, window_height as f32);
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
            sfx::play_sound_effect(&self.ball_bounce_off_the_paddle);
            self.ball.invert_velocity(true, false);
        }

        if ball_collision_with_wall_y {
            sfx::play_sound_effect(&self.bounce_off_the_walls);
            self.ball.invert_velocity(false, true);
        }

        if ball_collision_with_left_wall {
            sfx::play_sound_effect(&self.marked_point);
            self.score_player2 += 1;
            self.reset_game_objects();
        }

        if ball_collision_with_right_wall {
            sfx::play_sound_effect(&self.marked_point);
            self.score_player1 += 1;
            self.reset_game_objects();
        }
    }

    pub fn reset_game_objects(&mut self) {
        self.ball.reset();
        self.player1.reset();
        self.player2.reset();
    }

    pub fn win(&self) -> bool {
        self.score_player1 >= constants::WINNING_SCORE
            || self.score_player2 >= constants::WINNING_SCORE
    }

    pub fn restart_game(&mut self) {
        self.score_player1 = 0;
        self.score_player2 = 0;

        self.reset_game_objects();
    }
}
