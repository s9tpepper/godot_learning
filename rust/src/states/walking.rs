// use godot::{
//     classes::{INode, Node, Node3D},
//     obj::{Base, Gd},
//     prelude::{GodotClass, godot_api},
// };
//
// use crate::{
//     finite_state_machine::{FiniteStateMachine, StateUpdates},
//     impl_state,
// };
//
// #[derive(GodotClass)]
// #[class(base=Node, init)]
// struct Walking {
//     base: Base<Node>,
//     context: Option<Gd<Node3D>>,
//     pub fsm: Option<Gd<FiniteStateMachine>>,
// }
//
// impl Walking {
//     // fn some_func(&self) {
//     //     if let Some(fsm) = &self.fsm {
//     //         fsm.bind().switch("SomeState");
//     //     }
//     // }
// }
//
// impl_state!(Walking);
// #[godot_api]
// impl INode for Walking {
//     fn ready(&mut self) {}
//
//     fn physics_process(&mut self, _delta: f64) {}
// }
//
// impl StateUpdates for Walking {
//     fn enter(&self) {
//         todo!()
//     }
//
//     fn update(&self, _delta: f32) {
//         todo!()
//     }
//
//     fn exit(&self) {
//         todo!()
//     }
// }
