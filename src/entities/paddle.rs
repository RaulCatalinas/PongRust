use macroquad::{
    color::WHITE,
    input::{KeyCode, is_key_down},
    shapes::draw_rectangle,
};

use crate::types;

pub struct Paddle {
    x: f32,
    y: f32,
    velocity_y: f32,
    width: f32,
    height: f32,
    is_player_one: bool,
}

impl types::game_object::GameObject for Paddle {
    fn update(&mut self) {
        self.velocity_y = self.get_input_velocity();
        self.y += self.velocity_y;
    }

    fn draw(&self) {
        draw_rectangle(self.x, self.y, self.width, self.height, WHITE);
    }

    fn reset_position(&mut self) {}
}

impl Paddle {
    pub fn new(x: f32, y: f32, is_player_one: bool) -> Self {
        Self {
            x,
            y,
            is_player_one,
            velocity_y: 0.0,
            width: 10.0,
            height: 100.0,
        }
    }

    fn get_input_velocity(&self) -> f32 {
        let (up, down) = match self.is_player_one {
            true => (KeyCode::W, KeyCode::S),
            false => (KeyCode::Up, KeyCode::Down),
        };

        if is_key_down(up) {
            return -5.0;
        }

        if is_key_down(down) {
            return 5.0;
        }

        0.0
    }
}
