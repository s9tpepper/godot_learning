use godot::{classes::InputEvent, obj::Gd};
use movement_states::MovementStates;

use crate::player::MovementContext;

pub mod idle;
pub mod movement_states;
pub mod walking;

pub trait State: std::fmt::Debug {
    type StatesEnum;
    type Context;

    fn get_state_name(&self) -> Self::StatesEnum;
    fn next(&mut self) -> Option<Self::StatesEnum>;
    fn enter(&mut self);
    fn input(&mut self, event: Gd<InputEvent>);
    fn process(&mut self, delta: f32);
    fn process_physics(&mut self, delta: f32);
    fn exit(&mut self);
}

impl Default for Box<dyn State<Context = Gd<MovementContext>, StatesEnum = MovementStates>> {
    fn default() -> Self {
        Box::new(())
    }
}

impl State for () {
    type Context = Gd<MovementContext>;
    type StatesEnum = MovementStates;

    fn get_state_name(&self) -> Self::StatesEnum {
        MovementStates::Idle
    }

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
