mod actions;
mod camera;
mod movement;
mod player;

#[allow(unused)]
fn main() {
    use godot::prelude::*;

    struct MyExtension;

    #[gdextension]
    unsafe impl ExtensionLibrary for MyExtension {}
}
