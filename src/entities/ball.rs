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

    fn reset(&mut self) {
        let (random_velocity_x, random_velocity_y) = Self::get_random_velocity();

        self.x = screen_width() / 2.0;
        self.y = screen_height() / 2.0;
        self.velocity_x = random_velocity_x;
        self.velocity_y = random_velocity_y;
    }
}

impl Ball {
    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }

    pub fn radius(&self) -> f32 {
        self.radius
    }

    pub fn new() -> Self {
        let (random_velocity_x, random_velocity_y) = Self::get_random_velocity();

        Self {
            x: screen_width() / 2.0,
            y: screen_height() / 2.0,
            radius: 10.0,
            velocity_x: random_velocity_x,
            velocity_y: random_velocity_y,
        }
    }

    fn get_random_velocity() -> (f32, f32) {
        let mut rng = rand::rng();

        let sign_x = if rng.random_bool(0.5) { 1.0 } else { -1.0 };
        let sign_y = if rng.random_bool(0.5) { 1.0 } else { -1.0 };

        (
            rng.random_range(2.0..=5.0) * sign_x,
            rng.random_range(2.0..=5.0) * sign_y,
        )
    }

    pub fn invert_velocity(&mut self, invert_x: bool, invert_y: bool) {
        if invert_x {
            self.velocity_x = -self.velocity_x;
        }

        if invert_y {
            self.velocity_y = -self.velocity_y;
        }
    }
}
