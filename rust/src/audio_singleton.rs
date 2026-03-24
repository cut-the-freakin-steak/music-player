use godot::classes::ProjectSettings;
use godot::obj::Singleton;
use godot::prelude::*;

use std::fs::File;

use rodio::{DeviceSinkError, MixerDeviceSink, decoder::DecoderError};

/// this will always be a child of /root, and if its not then something is wrong
#[derive(GodotClass)]
#[class(init, base = Node)]
pub struct AudioSingleton {
    base: Base<Node>,

    #[init(val = OnReady::manual())]
    pub stream_handle: OnReady<MixerDeviceSink>,

    #[init(val = OnReady::manual())]
    pub audio_player: OnReady<rodio::Player>,

    #[init(val = vec![])]
    pub previous_songs: Vec<String>,

    #[init(val = vec![])]
    pub last_10_songs: Vec<File>,
}

#[godot_api]
impl INode for AudioSingleton {
    fn ready(&mut self) {
        godot_print!("asl;dfjlasjdl;fjasdfgh jkvafskd fgvbawlekfbjn");

        let sink_result = Self::init_audio();
        match sink_result {
            Ok((stream_handle, audio_player)) => {
                self.stream_handle.init(stream_handle);
                self.audio_player.init(audio_player);
                godot_print!("nothing wrong yet");
            }
            Err(err) => godot_error!("something's gone wrong while initializing audio: {}", err),
        }
        godot_print!("nothing wrong yet x2");
    }
}

impl AudioSingleton {
    /// if you dont do this then i hate you :c
    /// *CRUCIAL STEP!* initializes audio sink to allow audio to be played through [rodio](https://github.com/RustAudio/rodio?tab=readme-ov-file)
    fn init_audio() -> Result<(MixerDeviceSink, rodio::Player), DeviceSinkError> {
        let stream_handle = rodio::DeviceSinkBuilder::open_default_sink()?;
        let mixer = stream_handle.mixer();
        let audio_player = rodio::Player::connect_new(mixer);
        audio_player.pause();

        godot_print!("nothing wrong yet x3");
        return Ok((stream_handle, audio_player));
    }

    /// wrapper around res:// and user:// file paths to load files
    /// to go from a file path directly to player queue, see `AudioSingleton::add_file_path_to_queue`
    pub fn load_file(&mut self, file_path: &str) -> Result<File, std::io::Error> {
        let project_settings = ProjectSettings::singleton();

        let mut file_path = file_path.to_string();

        if file_path.starts_with("res://") || file_path.starts_with("user://") {
            file_path = project_settings.globalize_path(&file_path).to_string();
        }

        if self.previous_songs.len() >= u8::MAX as usize {
            self.previous_songs.remove(0);
        }

        self.previous_songs.push(file_path.clone());

        let file = File::open(file_path)?;

        return Ok(file);
    }

    /// add a rust file to the queue of the audio player and add file to vec of last 10 songs loaded.
    /// to go from a file path directly to player queue, see `AudioSingleton::add_file_path_to_queue`
    pub fn add_file_to_queue(&mut self, file: File) -> Result<(), DecoderError> {
        let other_file = file.try_clone().expect("couldn't clone file");
        self.audio_player.append(rodio::Decoder::try_from(file)?);

        if self.last_10_songs.len() >= 10 {
            self.last_10_songs.remove(0);
        }

        self.last_10_songs.push(other_file);

        return Ok(());
    }

    /// add a valid file path to an audio file into the player's queue
    /// also adds file to vec of last 10 songs loaded
    pub fn add_file_path_to_queue(&mut self, file_path: &str) {
        let song_file_result = self.load_file(file_path);

        if let Ok(song_file) = song_file_result {
            if let Err(err) = self.add_file_to_queue(song_file) {
                godot_error!("decoding error! {}", err);
            }
        }
        else if let Err(err) = song_file_result {
            godot_error!("there was an error retrieving the song file: {}", err);
        }
    }
}
