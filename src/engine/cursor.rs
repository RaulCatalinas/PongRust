use std::cell::RefCell;
use tao::window::CursorIcon;

thread_local! {
    static CURSOR: RefCell<CursorIcon> = RefCell::new(CursorIcon::Default);
}

pub(crate) fn get() -> CursorIcon {
    CURSOR.with(|c| *c.borrow())
}

pub(crate) fn reset() {
    CURSOR.with(|c| *c.borrow_mut() = CursorIcon::Default);
}

pub fn set(icon: CursorIcon) {
    CURSOR.with(|c| *c.borrow_mut() = icon);
}
