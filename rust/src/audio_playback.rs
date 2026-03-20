use crate::audio_singleton::AudioSingleton;
use godot::obj::Singleton;
use rodio::mixer::Mixer;
use rodio::{DeviceSinkError, MixerDeviceSink};
use std::error::Error;
use std::fs::File;

/// if you dont do this then i hate you :c
pub fn init_audio() -> Result<(MixerDeviceSink, Mixer), DeviceSinkError> {
    let stream_handle = rodio::DeviceSinkBuilder::open_default_sink()?;
    let mixer = stream_handle.mixer().to_owned();

    return Ok((stream_handle, mixer));
}

/// play a given file
pub fn play_audio(file_path: &str) -> Result<(), Box<dyn Error>> {
    let playback = AudioSingleton::singleton();

    let file = File::open(file_path)?;

    playback
        .bind()
        .player
        .append(rodio::Decoder::try_from(file)?);

    playback.bind().player.sleep_until_end();

    return Ok(());
}
