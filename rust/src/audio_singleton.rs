use crate::audio_playback;
use godot::prelude::*;
use rodio::{MixerDeviceSink, mixer::Mixer};

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
        let sink_result = audio_playback::init_audio();
        match sink_result {
            Ok((stream_handle, mixer)) => {
                self.stream_handle.init(stream_handle);
                self.mixer.init(mixer);
            }
            Err(err) => godot_error!("something's gone wrong while initializing audio: {}", err),
        }

        self.player.init(rodio::Player::connect_new(&self.mixer));
    }
}
