use std::sync::mpsc::Sender;

use godot::{
    classes::{INode, Node, Node3D},
    obj::{Base, Gd},
    prelude::{GodotClass, godot_api},
};

use crate::{
    finite_state_machine::{self, StateMachineEvents, StateUpdates},
    impl_state,
};

#[derive(GodotClass)]
#[class(base=Node, init)]
pub struct Idle {
    base: Base<Node>,
    context: Option<Gd<Node3D>>,
    sender: Option<Sender<StateMachineEvents>>,
}

#[godot_api]
impl INode for Idle {
    fn ready(&mut self) {}
    fn physics_process(&mut self, _delta: f64) {}
}

// impl State for Gd<Idle> {
//     fn set_context(&mut self, node: Gd<Node3D>) {
//         self.bind_mut().context = Some(node);
//     }
//
//     fn set_sender(&mut self, sender: Sender<StateMachineEvents>) {
//         self.bind_mut().sender = Some(sender);
//     }
//
//     fn get_state_name(&self) -> String {
//         "Idle".to_string()
//     }
// }

impl_state!(Idle);

impl StateUpdates for Idle {
    fn enter(&self) {
        todo!("Implement the enter logic for Idle state")
    }

    fn update(&self, _delta: f32) {
        todo!()
    }

    fn exit(&self) {
        todo!()
    }
}
