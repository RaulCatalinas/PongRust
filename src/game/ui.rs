use macroquad::{
    color::WHITE,
    text::{TextDimensions, draw_text, measure_text},
    window::screen_width,
};

pub fn render_game_scoreboard(current_score_player1: u8, current_score_player2: u8) {
    let text = format!("{} | {}", current_score_player1, current_score_player2);

    let font_size = 50.0;
    let TextDimensions { width, .. } = measure_text(&text, None, font_size as u16, 1.0);

    draw_text(
        &text,
        screen_width() / 2.0 - width / 2.0,
        font_size,
        font_size,
        WHITE,
    );
}
