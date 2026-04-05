use macroquad::{
    color::WHITE,
    shapes::draw_circle,
    window::{screen_height, screen_width},
};
use rand::RngExt;

use crate::types;

pub struct Ball {
    x: f32,
    y: f32,
    radius: f32,
    velocity_x: f32,
    velocity_y: f32,
}

impl types::game_object::GameObject for Ball {
    fn update(&mut self) {
        self.x += self.velocity_x;
        self.y += self.velocity_y;
    }

    fn draw(&self) {
        draw_circle(self.x, self.y, self.radius, WHITE);
    }

    fn reset_position(&mut self) {
        self.x = screen_width() / 2.0;
        self.y = screen_height() / 2.0;
    }
}

impl Ball {
    pub fn new() -> Self {
        let mut rng = rand::rng();

        let sign_x = if rng.random_bool(0.5) { 1.0 } else { -1.0 };
        let sign_y = if rng.random_bool(0.5) { 1.0 } else { -1.0 };

        Self {
            x: screen_width() / 2.0,
            y: screen_height() / 2.0,
            radius: 10.0,
            velocity_x: rng.random_range(2.0..=5.0) * sign_x,
            velocity_y: rng.random_range(2.0..=5.0) * sign_y,
        }
    }
}
