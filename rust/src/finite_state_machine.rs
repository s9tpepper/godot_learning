#![allow(non_snake_case)]

use std::collections::HashMap;

use godot::global::godot_print;

use crate::{player::FsmHelper, states::StateUpdates};

pub type StateMap = HashMap<String, Box<dyn StateUpdates>>;

pub trait FiniteStateMachine: std::fmt::Debug {
    type Enum;
    type Context;

    fn ready(&mut self, state_machine: FsmHelper<Self::Enum, Self::Context>);

    fn setup_states(
        &mut self,
        context: Self::Context,
        state_machine: FsmHelper<Self::Enum, Self::Context>,
    ) -> StateMap;

    fn get_state(&mut self, state: &str) -> Option<&mut Box<dyn StateUpdates>>;

    fn switch(&mut self, state: &str) {
        godot_print!("[FiniteStateMachine::switch()] {state}");

        if let Some(to_state) = self.get_state(state) {
            godot_print!("[FiniteStateMachine::switch() - Got state]");
            to_state.enter();
            godot_print!("[FiniteStateMachine::switch() - Triggered enter() on state]");
        }
    }
}
