use macroquad::{
    color::WHITE,
    text::{TextDimensions, draw_text, measure_text},
    window::{screen_height, screen_width},
};

use crate::{
    constants,
    game::{Game, ui},
};

impl Game {
    pub fn main_menu_scene(&mut self) {
        let title = "Pong in Rust";
        let TextDimensions { width, .. } = measure_text(
            title,
            None,
            constants::MAIN_MENU_GAME_NAME_FONT_SIZE as u16,
            1.0,
        );

        draw_text(
            title,
            screen_width() / 2.0 - width / 2.0,
            screen_height() / 3.0,
            constants::MAIN_MENU_GAME_NAME_FONT_SIZE,
            WHITE,
        );

        let button_width = 200.0;
        let button_height = 50.0;
        let gap = 10.0;
        let start_y = screen_height() / 2.0;

        ui::draw_button(
            "Start Game",
            screen_width() / 2.0 - button_width / 2.0,
            start_y,
            button_width,
            button_height,
            constants::MAIN_MENU_GAME_NAME_FONT_SIZE / 2.0,
            || self.scene = crate::game::scenes::Scene::Game,
        );

        ui::draw_button(
            "Exit",
            screen_width() / 2.0 - button_width / 2.0,
            start_y + button_height + gap,
            button_width,
            button_height,
            constants::MAIN_MENU_BUTTON_FONT_SIZE,
            || std::process::exit(0),
        );
    }
}
