// use godot::{global::godot_print, obj::Gd};
//
// use crate::player::{Fsm, FsmHelper, MovementContext};
//
// use super::{State, StateUpdates, movement_states::MovementStates};
//
// #[derive(Debug)]
// pub struct Walking {
//     #[allow(unused)]
//     context: Gd<MovementContext>,
//
//     state_machine: Option<Fsm<Gd<MovementContext>, MovementStates>>,
// }
//
// impl Walking {
//     pub fn new(context: Gd<MovementContext>) -> Self {
//         Walking {
//             context,
//             state_machine: None,
//         }
//     }
// }
//
// impl State for Walking {
//     type StatesEnum = MovementStates;
//     type Context = Gd<MovementContext>;
//
//     fn set_state_machine(&mut self, state_machine: FsmHelper<Self::Context, Self::StatesEnum>) {
//         self.state_machine = Some(state_machine);
//     }
//
//     fn get_state_name(&self) -> Self::StatesEnum {
//         MovementStates::Walking
//     }
// }
//
// impl StateUpdates for Walking {
//     type StatesEnum = MovementStates;
//
//     fn enter(&mut self) {
//         godot_print!("Implement the enter logic for Walking state");
//         godot_print!("Context: {:?}", self.context);
//
//         godot_print!(
//             "walking animation name: {}",
//             self.context.bind_mut().get_walking_animation_name()
//         )
//     }
//
//     fn process(&mut self, _delta: f32) -> Option<Self::StatesEnum> {
//         None
//     }
//
//     fn process_physics(&mut self, _delta: f32) -> Option<Self::StatesEnum> {
//         None
//     }
//
//     fn exit(&mut self) {}
// }
