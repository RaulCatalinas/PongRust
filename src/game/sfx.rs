use macroquad::audio::{PlaySoundParams, Sound, load_sound_from_bytes, play_sound};

pub async fn load_sound_from_embedded(bytes: &[u8]) -> Sound {
    load_sound_from_bytes(bytes)
        .await
        .expect("Failed to load sound effect")
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
