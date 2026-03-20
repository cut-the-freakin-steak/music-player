use crate::audio_singleton::AudioSingleton;
use crate::godot_print;

use godot::classes::ProjectSettings;
use godot::obj::Singleton;
use std::error::Error;
use std::fs::File;

/// play a given file
pub fn play_audio(file_path: &str) -> Result<(), Box<dyn Error>> {
    let playback = AudioSingleton::singleton();
    let project_settings = ProjectSettings::singleton();

    let mut file_path = file_path.to_string();

    if file_path.starts_with("res://") || file_path.starts_with("user://") {
        file_path = project_settings.globalize_path(&file_path).to_string();
    }

    let file = File::open(file_path)?;

    // BUG: this line causes a panic and i dont fucking know why
    playback
        .bind()
        .player
        .append(rodio::Decoder::try_from(file)?);

    playback.bind().player.sleep_until_end();

    godot_print!("nothing wrong yet x4");
    return Ok(());
}
