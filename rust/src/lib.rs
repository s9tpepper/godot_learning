mod actions;
mod camera;
mod debug;
mod motion_signals;
mod mouse;
mod movement;
mod movement_animations;
mod player;

#[allow(unused)]
fn main() {
    use godot::prelude::*;

    struct MyExtension;

    #[gdextension]
    unsafe impl ExtensionLibrary for MyExtension {}
}
