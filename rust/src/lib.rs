use godot::prelude::*;

mod audio_playback;
mod audio_singleton;
mod player;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}
