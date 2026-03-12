use godot::classes::{CheckButton, HBoxContainer, IHBoxContainer};
use godot::prelude::*;

/// The music player scene
#[derive(GodotClass)]
#[class(init, base = HBoxContainer)]
struct Player {
    base: Base<HBoxContainer>,

    #[init(node = "PlayPause")]
    play_pause: OnReady<Gd<CheckButton>>,
}

#[godot_api]
impl IHBoxContainer for Player {}
