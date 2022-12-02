#![warn(clippy::pedantic)]

use gdnative::prelude::*;
pub mod level;
pub mod tile;

fn init(handle: InitHandle) {
	handle.add_class::<level::Level>();
}

godot_init!(init);
