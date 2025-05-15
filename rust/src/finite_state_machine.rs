#![allow(non_snake_case)]

use std::collections::HashMap;

use crate::{player::FsmHelper, states::StateUpdates};

pub type StateMap<T: FiniteStateMachine> =
    HashMap<T::StatesEnum, Box<dyn StateUpdates<StatesEnum = T::StatesEnum>>>;

pub trait FiniteStateMachine: std::fmt::Debug {
    type StatesEnum: Clone + PartialEq + Eq;
    type Context;

    fn ready(&mut self, state_machine: FsmHelper<Self::Context, Self::StatesEnum>);
    fn get_current_state(&self) -> Self::StatesEnum;
    fn set_current_state(&mut self, state: Self::StatesEnum);
    fn process(&mut self, delta: f64) -> Option<Self::StatesEnum>;

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
        if self.get_current_state() == state {
            return;
        }

        if let Some(to_state) = self.get_state(state.clone()) {
            to_state.enter();
            self.set_current_state(state);
        }
    }
}
