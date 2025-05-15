use movement_states::MovementStates;

use crate::player::FsmHelper;

pub mod idle;
pub mod movement_states;
pub mod walking;

pub trait State {
    type StatesEnum;
    type Context;

    fn get_state_name(&self) -> Self::StatesEnum;
    fn set_state_machine(&mut self, state_machine: FsmHelper<Self::Context, Self::StatesEnum>);
}

pub trait StateUpdates: std::fmt::Debug {
    type StatesEnum;

    fn enter(&mut self);
    fn process(&mut self, delta: f32) -> Option<Self::StatesEnum>;
    fn process_physics(&mut self, delta: f32) -> Option<Self::StatesEnum>;
    fn exit(&mut self);
}

impl Default for Box<dyn StateUpdates<StatesEnum = MovementStates>> {
    fn default() -> Self {
        Box::new(())
    }
}

impl StateUpdates for () {
    type StatesEnum = MovementStates;

    fn enter(&mut self) {
        todo!()
    }

    fn process(&mut self, _delta: f32) -> Option<Self::StatesEnum> {
        todo!()
    }

    fn process_physics(&mut self, _delta: f32) -> Option<Self::StatesEnum> {
        todo!()
    }

    fn exit(&mut self) {
        todo!()
    }
}
