use std::cell::RefCell;

struct MouseState {
    x: f32,
    y: f32,
    held: bool,
    just_pressed: bool,
    just_released: bool,
}

impl MouseState {
    fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            held: false,
            just_pressed: false,
            just_released: false,
        }
    }

    fn update_position(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }

    fn update_button(&mut self, pressed: bool) {
        if pressed {
            self.just_pressed = !self.held;
            self.held = true;
            self.just_released = false;
        } else {
            self.just_released = self.held;
            self.held = false;
            self.just_pressed = false;
        }
    }

    fn flush(&mut self) {
        self.just_pressed = false;
        self.just_released = false;
    }
}

thread_local! {
    static MOUSE: RefCell<MouseState> = RefCell::new(MouseState::new());
}

pub(crate) fn on_moved(x: f32, y: f32) {
    MOUSE.with(|m| m.borrow_mut().update_position(x, y));
}

pub(crate) fn on_button(pressed: bool) {
    MOUSE.with(|m| m.borrow_mut().update_button(pressed));
}

pub(crate) fn flush() {
    MOUSE.with(|m| m.borrow_mut().flush());
}

pub fn position() -> (f32, f32) {
    MOUSE.with(|m| {
        let state = m.borrow();
        (state.x, state.y)
    })
}

pub fn just_pressed() -> bool {
    MOUSE.with(|m| m.borrow().just_pressed)
}

pub fn held() -> bool {
    MOUSE.with(|m| m.borrow().held)
}

pub fn just_released() -> bool {
    MOUSE.with(|m| m.borrow().just_released)
}
