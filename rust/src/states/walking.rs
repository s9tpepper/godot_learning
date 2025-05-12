use godot::{
    classes::{INode, Node, Node3D},
    global::godot_print,
    obj::{Base, Gd},
    prelude::{GodotClass, godot_api},
};

use crate::impl_state;

use super::StateUpdates;

#[derive(GodotClass)]
#[class(base=Node, init)]
pub struct Walking {
    #[base]
    base: Base<Node>,

    context: Option<Gd<Node3D>>,
    // state_machine:
}

#[godot_api]
impl INode for Walking {
    fn ready(&mut self) {}
    fn physics_process(&mut self, _delta: f64) {}
}

impl_state!(Walking);

impl StateUpdates for Gd<Walking> {
    fn enter(&self) {
        godot_print!("Implement the enter logic for Walking state")
    }

    fn update(&self, _delta: f32) {
        todo!()
    }

    fn exit(&self) {
        todo!()
    }
}
