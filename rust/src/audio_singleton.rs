use godot::classes::ProjectSettings;
use godot::obj::Singleton;
use godot::prelude::*;

use std::error::Error;
use std::fs::File;

use rodio::{DeviceSinkError, MixerDeviceSink};

/// this will always be a child of /root, and if its not then something is wrong
#[derive(GodotClass)]
#[class(init, base = Node)]
pub struct AudioSingleton {
    base: Base<Node>,

    #[init(val = OnReady::manual())]
    pub stream_handle: OnReady<MixerDeviceSink>,

    #[init(val = OnReady::manual())]
    pub audio_player: OnReady<rodio::Player>,
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
    fn init_audio() -> Result<(MixerDeviceSink, rodio::Player), DeviceSinkError> {
        let stream_handle = rodio::DeviceSinkBuilder::open_default_sink()?;
        let mixer = stream_handle.mixer();
        let audio_player = rodio::Player::connect_new(mixer);

        godot_print!("nothing wrong yet x3");
        return Ok((stream_handle, audio_player));
    }

    pub fn play_audio(&self, file_path: &str) -> Result<(), Box<dyn Error>> {
        let project_settings = ProjectSettings::singleton();

        let mut file_path = file_path.to_string();

        if file_path.starts_with("res://") || file_path.starts_with("user://") {
            file_path = project_settings.globalize_path(&file_path).to_string();
        }

        let file = File::open(file_path)?;

        // BUG: this line causes a panic and i dont fucking know why
        self.audio_player.append(rodio::Decoder::try_from(file)?);

        self.audio_player.sleep_until_end();

        godot_print!("nothing wrong yet x4");
        return Ok(());
    }
}
