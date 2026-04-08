pub mod keyboard;
pub mod mouse;

pub(crate) fn flush() {
    mouse::flush();
    keyboard::flush();
}
