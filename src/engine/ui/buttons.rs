use super::text;
use crate::engine::{cursor, input::mouse};

pub struct ButtonConfig<F: Fn()> {
    pub text: &'static str,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub on_click: F,
}

pub fn create_button<F: Fn()>(config: ButtonConfig<F>) {
    let (mx, my) = mouse::position();
    let hovered = mx >= config.x
        && mx <= config.x + config.width
        && my >= config.y
        && my <= config.y + config.height;

    let clicked = hovered && mouse::just_released();

    // Color del fondo según estado
    let color = if hovered {
        glyphon::Color::rgb(80, 80, 80)
    } else {
        glyphon::Color::rgb(50, 50, 50)
    };

    // Dibuja el texto centrado en el botón
    let text_x = config.x + config.width / 2.0;
    let text_y = config.y + config.height / 2.0;

    if hovered {
        cursor::set(tao::window::CursorIcon::Hand);
    }

    text::draw_colored(config.text, text_x, text_y, 20.0, color);

    if clicked {
        (config.on_click)();
    }
}
