use godot::classes::{Control, IControl};
use godot::prelude::*;

/// The music player scene
#[derive(GodotClass)]
#[class(init, base = Control)]
struct Player {
    base: Base<Control>,
}

#[godot_api]
impl IControl for Player {}
