use std::sync::mpsc::Sender;

use godot::{
    classes::{INode, Node, Node3D},
    global::godot_print,
    obj::{Base, Gd},
    prelude::{GodotClass, godot_api},
};

use crate::{finite_state_machine::StateMachineEvents, impl_state};

use super::StateUpdates;

#[derive(GodotClass)]
#[class(base=Node, init)]
pub struct Idle {
    #[base]
    base: Base<Node>,

    context: Option<Gd<Node3D>>,
    sender: Option<Sender<StateMachineEvents>>,
}

#[godot_api]
impl INode for Idle {
    fn ready(&mut self) {}
    fn physics_process(&mut self, _delta: f64) {}
}

impl_state!(Idle);

impl StateUpdates for Gd<Idle> {
    fn enter(&self) {
        godot_print!("Implement the enter logic for Idle state")
    }

    fn update(&self, _delta: f32) {
        todo!()
    }

    fn exit(&self) {
        todo!()
    }
}
