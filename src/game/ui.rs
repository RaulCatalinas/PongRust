use macroquad::{
    color::{BLACK, DARKGRAY, GRAY, WHITE},
    input::{MouseButton, is_mouse_button_pressed, mouse_position},
    shapes::draw_rectangle,
    text::{TextDimensions, draw_text, measure_text},
    window::screen_width,
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

pub fn draw_button(
    text: &str,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    font_size: f32,
    on_click: impl FnOnce(),
) {
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

    if hovered && is_mouse_button_pressed(MouseButton::Left) {
        on_click();
    }
}

pub fn draw_center_line(window_width: f32, window_height: f32) {
    let dash_height = 20.0;
    let gap = 10.0;
    let x = window_width / 2.0;
    let margin = 5.0;
    let mut y = constants::SCORE_FONT_SIZE + gap + margin;

    while y < window_height {
        draw_rectangle(x - 2.0, y, 4.0, dash_height, GRAY);
        y += dash_height + gap;
    }
}
