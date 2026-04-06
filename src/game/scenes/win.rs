use macroquad::{
    color::WHITE,
    text::{TextDimensions, draw_text, measure_text},
    window::{screen_height, screen_width},
};

use crate::{
    constants,
    game::{Game, scenes, ui},
};

impl Game {
    pub fn win_scene(&mut self, win_player1: bool) {
        let winner = if win_player1 {
            "Player 1 Wins!"
        } else {
            "Player 2 Wins!"
        };

        let TextDimensions { width, .. } =
            measure_text(winner, None, constants::WIN_FONT_SIZE as u16, 1.0);

        draw_text(
            winner,
            screen_width() / 2.0 - width / 2.0,
            screen_height() / 3.0,
            constants::WIN_FONT_SIZE,
            WHITE,
        );

        let button_width = 200.0;
        let button_height = 50.0;
        let gap = 10.0;
        let start_y = screen_height() / 2.0 + 20.0;

        ui::draw_button(
            "Restart",
            screen_width() / 2.0 - button_width / 2.0,
            start_y,
            button_width,
            button_height,
            constants::WIN_SCENE_BUTTTON_FONT_SIZE,
            || self.restart_game(),
        );

        let main_menu_button_width = button_width + 20.0;

        ui::draw_button(
            "Main Menu",
            screen_width() / 2.0 - main_menu_button_width / 2.0,
            start_y + button_height + gap,
            main_menu_button_width,
            button_height,
            constants::WIN_SCENE_BUTTTON_FONT_SIZE,
            || self.scene = scenes::Scene::MainMenu,
        );

        ui::draw_button(
            "Exit",
            screen_width() / 2.0 - button_width / 2.0,
            start_y + (button_height + gap) * 2.0,
            button_width,
            button_height,
            constants::MAIN_MENU_BUTTON_FONT_SIZE,
            || std::process::exit(0),
        );
    }
}
