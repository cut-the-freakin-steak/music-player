use godot::prelude::*;
use rodio::{DeviceSinkError, MixerDeviceSink, mixer::Mixer};

#[derive(GodotClass)]
#[class(singleton, init, base = Node)]
pub struct AudioSingleton {
    base: Base<Node>,

    #[init(val = OnReady::manual())]
    pub stream_handle: OnReady<MixerDeviceSink>,

    #[init(val = OnReady::manual())]
    pub mixer: OnReady<Mixer>,

    #[init(val = OnReady::manual())]
    pub player: OnReady<rodio::Player>,
}

#[godot_api]
impl INode for AudioSingleton {
    fn ready(&mut self) {
        godot_print!("asl;dfjlasjdl;fjasdfgh jkvafskd fgvbawlekfbjn");

        let sink_result = Self::init_audio();
        match sink_result {
            Ok((stream_handle, mixer)) => {
                self.stream_handle.init(stream_handle);
                self.mixer.init(mixer);
                godot_print!("nothing wrong yet");
            }
            Err(err) => godot_error!("something's gone wrong while initializing audio: {}", err),
        }

        self.player.init(rodio::Player::connect_new(&self.mixer));
        godot_print!("nothing wrong yet x2");
    }
}

impl AudioSingleton {
    /// if you dont do this then i hate you :c
    pub fn init_audio() -> Result<(MixerDeviceSink, Mixer), DeviceSinkError> {
        let stream_handle = rodio::DeviceSinkBuilder::open_default_sink()?;
        let mixer = stream_handle.mixer().to_owned();

        godot_print!("nothing wrong yet x3");
        return Ok((stream_handle, mixer));
    }
}
