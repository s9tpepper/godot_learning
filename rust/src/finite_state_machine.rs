#![allow(non_snake_case)]

use std::collections::HashMap;

use godot::global::godot_print;

use crate::{player::FsmHelper, states::StateUpdates};

pub type StateMap<T: FiniteStateMachine> =
    HashMap<T::StatesEnum, Box<dyn StateUpdates<StatesEnum = T::StatesEnum>>>;

pub trait FiniteStateMachine: std::fmt::Debug {
    type StatesEnum: Clone;
    type Context;

    fn ready(&mut self, state_machine: FsmHelper<Self::Context, Self::StatesEnum>);

    fn setup_states(
        &mut self,
        context: Self::Context,
        state_machine: FsmHelper<Self::Context, Self::StatesEnum>,
    ) -> HashMap<Self::StatesEnum, Box<dyn StateUpdates<StatesEnum = Self::StatesEnum>>>;

    fn get_state(
        &mut self,
        state: Self::StatesEnum,
    ) -> Option<&mut Box<dyn StateUpdates<StatesEnum = Self::StatesEnum>>>;

    fn switch(&mut self, state: Self::StatesEnum) {
        //godot_print!("[FiniteStateMachine::switch()] {state}");

        if let Some(to_state) = self.get_state(state.clone()) {
            godot_print!("[FiniteStateMachine::switch() - Got state]");
            to_state.enter();
            godot_print!("[FiniteStateMachine::switch() - Triggered enter() on state]");

            self.set_current_state(state);
        }
    }

    fn set_current_state(&mut self, state: Self::StatesEnum);

    fn process(&mut self, delta: f64) -> Option<Self::StatesEnum>;
}
