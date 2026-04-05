use macroquad::{
    color::{BLACK, DARKGRAY, WHITE},
    input::{MouseButton, is_mouse_button_pressed, mouse_position},
    shapes::draw_rectangle,
    text::{TextDimensions, draw_text, measure_text},
    window::{screen_height, screen_width},
};

use crate::constants;

pub fn render_game_scoreboard(current_score_player1: u8, current_score_player2: u8) {
    let text = format!("{} | {}", current_score_player1, current_score_player2);

    let TextDimensions { width, .. } =
        measure_text(&text, None, constants::SCORE_FONT_SIZE as u16, 1.0);

    draw_text(
        &text,
        screen_width() / 2.0 - width / 2.0,
        constants::SCORE_FONT_SIZE,
        constants::SCORE_FONT_SIZE,
        WHITE,
    );
}

pub fn render_win_screen(win_player1: bool) -> bool {
    let winner = if win_player1 {
        "Player 1 Wins!"
    } else {
        "Player 2 Wins!"
    };

    let TextDimensions { width, height, .. } =
        measure_text(winner, None, constants::WIN_FONT_SIZE as u16, 1.0);

    draw_text(
        winner,
        screen_width() / 2.0 - width / 2.0,
        screen_height() / 2.0 - height / 2.0,
        constants::WIN_FONT_SIZE,
        WHITE,
    );

    draw_button(
        "Restart",
        screen_width() / 2.0 - 200.0 / 2.0,
        screen_height() / 2.0 + 50.0,
        200.0,
        50.0,
        50.0,
    )
}

fn draw_button(text: &str, x: f32, y: f32, width: f32, height: f32, font_size: f32) -> bool {
    let (mouse_x, mouse_y) = mouse_position();

    let hovered = mouse_x >= x && mouse_x <= x + width && mouse_y >= y && mouse_y <= y + height;

    let TextDimensions {
        width: text_width,
        height: text_height,
        ..
    } = measure_text(text, None, font_size as u16, 1.0);

    let button_color = if hovered { WHITE } else { DARKGRAY };
    let text_color = if hovered { BLACK } else { WHITE };

    draw_rectangle(x, y, width, height, button_color);

    draw_text(
        text,
        x + width / 2.0 - text_width / 2.0,
        y + height / 2.0 + text_height / 2.0,
        font_size,
        text_color,
    );

    hovered && is_mouse_button_pressed(MouseButton::Left)
}
