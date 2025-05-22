use godot::init::gdextension;

mod actions;
mod debug;
mod motion_signals;
mod movement;
mod movement_animations;
mod player;
mod shell;
mod some_state_machine;
mod states;

mod common;

#[allow(unused)]
fn main() {
    use godot::prelude::*;

    struct MyExtension;

    #[gdextension]
    unsafe impl ExtensionLibrary for MyExtension {}
}
