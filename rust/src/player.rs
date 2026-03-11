use godot::classes::{CheckButton, Control, IControl};
use godot::prelude::*;

/// The music player scene
#[derive(GodotClass)]
#[class(init, base = Control)]
struct Player {
    base: Base<Control>,

    #[init(node = "PlayPause")]
    play_pause: OnReady<Gd<CheckButton>>,
}

#[godot_api]
impl IControl for Player {}
