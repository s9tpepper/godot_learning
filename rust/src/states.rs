use godot::{classes::InputEvent, obj::Gd};
use movement_states::MovementStates;

pub mod idle;
pub mod movement_states;
pub mod walking;

pub trait State {
    type StatesEnum;
    type Context;

    fn get_state_name(&self) -> Self::StatesEnum;
}

pub trait StateUpdates: std::fmt::Debug {
    type StatesEnum;

    fn next(&mut self) -> Option<Self::StatesEnum>;
    fn enter(&mut self);
    fn input(&mut self, event: Gd<InputEvent>);
    fn process(&mut self, delta: f32);
    fn process_physics(&mut self, delta: f32);
    fn exit(&mut self);
}

impl Default for Box<dyn StateUpdates<StatesEnum = MovementStates>> {
    fn default() -> Self {
        Box::new(())
    }
}

impl StateUpdates for () {
    type StatesEnum = MovementStates;

    fn next(&mut self) -> Option<Self::StatesEnum> {
        todo!()
    }

    fn enter(&mut self) {
        todo!()
    }

    fn input(&mut self, _event: Gd<InputEvent>) {
        todo!()
    }

    fn process(&mut self, _delta: f32) {
        todo!()
    }

    fn process_physics(&mut self, _delta: f32) {
        todo!()
    }

    fn exit(&mut self) {
        todo!()
    }
}
