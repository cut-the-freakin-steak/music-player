use godot::prelude::*;

mod audio_singleton;
mod player;

// set up tokio
use std::sync::OnceLock;
use tokio::runtime::Runtime;

static TOKIO_RT: OnceLock<Runtime> = OnceLock::new();

#[allow(clippy::missing_panics_doc)]
pub fn runtime() -> &'static Runtime {
    TOKIO_RT.get_or_init(|| Runtime::new().expect("Failed to create Tokio runtime"))
}

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}
