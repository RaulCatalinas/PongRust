use std::cell::RefCell;
use std::collections::HashSet;
use tao::keyboard::KeyCode;

struct KeyboardState {
    held: HashSet<KeyCode>,
    just_pressed: HashSet<KeyCode>,
    just_released: HashSet<KeyCode>,
}

impl KeyboardState {
    fn new() -> Self {
        Self {
            held: HashSet::new(),
            just_pressed: HashSet::new(),
            just_released: HashSet::new(),
        }
    }

    fn update_key(&mut self, key: KeyCode, pressed: bool) {
        if pressed {
            if !self.held.contains(&key) {
                self.just_pressed.insert(key);
            }
            self.held.insert(key);
            self.just_released.remove(&key);
        } else {
            if self.held.contains(&key) {
                self.just_released.insert(key);
            }
            self.held.remove(&key);
            self.just_pressed.remove(&key);
        }
    }

    fn flush(&mut self) {
        self.just_pressed.clear();
        self.just_released.clear();
    }
}

thread_local! {
    static KEYBOARD: RefCell<KeyboardState> = RefCell::new(KeyboardState::new());
}

pub(crate) fn on_key(key: KeyCode, pressed: bool) {
    KEYBOARD.with(|k| k.borrow_mut().update_key(key, pressed));
}

pub(crate) fn flush() {
    KEYBOARD.with(|k| k.borrow_mut().flush());
}

pub fn just_pressed(key: KeyCode) -> bool {
    KEYBOARD.with(|k| k.borrow().just_pressed.contains(&key))
}

pub fn held(key: KeyCode) -> bool {
    KEYBOARD.with(|k| k.borrow().held.contains(&key))
}

pub fn just_released(key: KeyCode) -> bool {
    KEYBOARD.with(|k| k.borrow().just_released.contains(&key))
}
