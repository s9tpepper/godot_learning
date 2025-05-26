use godot::{
    classes::{INode3D, InputEvent, Node3D},
    obj::{Base, Gd},
    prelude::{GodotClass, godot_api},
};

#[derive(GodotClass)]
#[class(base=Node3D, init)]
#[allow(unused)]
struct Loot {
    base: Base<Node3D>,
}

#[godot_api]
impl INode3D for Loot {
    // Called when the node is ready in the scene tree.
    fn ready(&mut self) {}

    // Called every frame.
    fn process(&mut self, _delta: f64) {}

    // Called every physics frame.
    fn physics_process(&mut self, _delta: f64) {}

    // Handle user input.
    fn input(&mut self, _event: Gd<InputEvent>) {}
}
