use crate::audio_singleton::AudioSingleton;

use godot::classes::{Button, CheckButton, Control, IControl, Label, Timer};
use godot::prelude::*;

#[derive(PartialEq, PartialOrd)]
enum Status {
    Default,
    Paused,
    Resumed,
    Rewind,
    Skip,
}

/// The music player scene
#[derive(GodotClass)]
#[class(init, base = Control)]
struct Player {
    base: Base<Control>,

    #[init(node = "Container/Rewind")]
    rewind: OnReady<Gd<Button>>,

    #[init(node = "Container/PlayPause")]
    play_pause: OnReady<Gd<CheckButton>>,

    #[init(node = "Container/Skip")]
    skip: OnReady<Gd<Button>>,

    #[init(node = "AddSong")]
    add_song: OnReady<Gd<Button>>,

    #[init(node = "StatusTimeout")]
    status_timeout: OnReady<Gd<Timer>>,

    #[init(node = "Status")]
    status_label: OnReady<Gd<Label>>,

    #[init(val = 0)]
    status_streak: i32,

    #[init(val = Status::Default)]
    last_status: Status,

    #[init(node = "/root/GlobalAudio")]
    audio_singleton: OnReady<Gd<AudioSingleton>>,
}

#[godot_api]
impl IControl for Player {
    fn ready(&mut self) {
        godot_print!("kILL YOURSELF !!! ");

        self.rewind
            .signals()
            .pressed()
            .connect_other(self, Self::_rewind_pressed);

        self.play_pause
            .signals()
            .toggled()
            .connect_other(self, Self::_play_pause_toggled);

        self.skip
            .signals()
            .pressed()
            .connect_other(self, Self::_skip_pressed);

        self.add_song
            .signals()
            .pressed()
            .connect_other(self, Self::_add_song_to_queue_pressed);

        self.status_timeout
            .signals()
            .timeout()
            .connect_other(self, Self::_status_timeout);
    }
}

#[godot_api]
impl Player {
    #[func]
    fn _rewind_pressed(&mut self) {
        if self.last_status == Status::Rewind {
            self.status_streak += 1;
        }
        else {
            self.status_streak = 0;
        }
        self.last_status = Status::Rewind;

        if self.status_streak == 0 {
            self.status_label.set_text("status: rewinded!!!!!!!j");
        }
        else {
            self.status_label.set_text(&format!(
                "status: rewinded!!!!!!!j x{}",
                self.status_streak + 1
            ));
        }
        self.status_timeout.start();
    }

    #[func]
    fn _play_pause_toggled(&mut self, toggled_on: bool) {
        self.status_streak = 0;
        if toggled_on {
            self.last_status = Status::Resumed;
            self.status_label.set_text("status: resumed !!>");
            self.audio_singleton.bind().audio_player.play();
        }
        else {
            self.last_status = Status::Paused;
            self.status_label.set_text("status: paused !!>");
            self.audio_singleton.bind().audio_player.pause();
        }

        self.status_timeout.start();
    }

    #[func]
    fn _skip_pressed(&mut self) {
        if self.last_status == Status::Skip {
            self.status_streak += 1;
        }
        else {
            self.status_streak = 0;
        }
        self.last_status = Status::Skip;

        if self.status_streak == 0 {
            self.status_label
                .set_text("status: SKIppeer!!!!!1111111111");
        }
        else {
            self.status_label.set_text(&format!(
                "status: SKIppeer!!!!!1111111111 x{}",
                self.status_streak + 1
            ));
        }
        self.status_timeout.start();
    }

    #[func]
    fn _add_song_to_queue_pressed(&mut self) {
        self.status_label.set_text("status: ade dsong !!!l");

        self.audio_singleton
            .bind()
            .add_file_path_to_queue("res://assets/audio/Toby Fox - Dogsong.flac");
    }

    #[func]
    fn _status_timeout(&mut self) {
        self.status_label.set_text("status: default");
        self.status_streak = 0;
    }
}
