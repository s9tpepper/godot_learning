mod actions;
mod camera;
mod motion_signals;
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
