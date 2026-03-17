use godot::classes::{Button, CheckButton, Control, IControl, Label, Timer};
use godot::prelude::*;

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

    #[init(node = "StatusTimeout")]
    status_timeout: OnReady<Gd<Timer>>,

    #[init(node = "Status")]
    status_label: OnReady<Gd<Label>>,
}

#[godot_api]
impl IControl for Player {
    fn ready(&mut self) {
        godot_print!("kILL YOURSELF !!! ");

        self.rewind
            .signals()
            .pressed()
            .connect_other(self, Self::rewind_pressed);

        self.play_pause
            .signals()
            .toggled()
            .connect_other(self, Self::play_pause_toggled);

        self.skip
            .signals()
            .pressed()
            .connect_other(self, Self::skip_pressed);

        self.status_timeout
            .signals()
            .timeout()
            .connect_other(self, Self::status_timeout);
    }
}

#[godot_api]
impl Player {
    #[func]
    fn rewind_pressed(&mut self) {
        self.status_label.set_text("status: rewinded!!!!!!!j");
        self.status_timeout.start();
    }

    #[func]
    fn play_pause_toggled(&mut self, toggled_on: bool) {
        match toggled_on {
            true => self.status_label.set_text("status: resumed !!>"),
            false => self.status_label.set_text("status: paused !!>"),
        };
        self.status_timeout.start();
    }

    #[func]
    fn skip_pressed(&mut self) {
        self.status_label
            .set_text("status: SKIppeer!!!!!1111111111");
        self.status_timeout.start();
    }

    #[func]
    fn status_timeout(&mut self) {
        self.status_label.set_text("status: default");
    }
}
