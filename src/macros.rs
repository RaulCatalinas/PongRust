#[macro_export]
macro_rules! load_sfx {
    ($path:expr) => {
        sfx::load_sound_from_embedded(include_bytes!($path)).await
    };
}
