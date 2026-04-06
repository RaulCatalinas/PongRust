use macroquad::audio::{PlaySoundParams, Sound, load_sound, play_sound};

pub async fn load_sound_effect(path: &str) -> Sound {
    load_sound(path)
        .await
        .expect(&format!("Failed to load sound effect: {}", path))
}

pub fn play_sound_effect(sound: &Sound) {
    play_sound(
        &sound,
        PlaySoundParams {
            looped: false,
            volume: 1.0,
        },
    );
}
