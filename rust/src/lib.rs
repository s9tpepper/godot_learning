#![feature(lazy_type_alias)]

use godot::init::gdextension;

mod actions;
mod camera;
mod debug;
mod finite_state_machine;
mod motion_signals;
mod mouse;
mod movement;
mod movement_animations;
mod player;
mod some_state_machine;
mod states;

#[allow(unused)]
fn main() {
    use godot::prelude::*;

    struct MyExtension;

    #[gdextension]
    unsafe impl ExtensionLibrary for MyExtension {}
}
