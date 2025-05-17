use godot::{global::godot_print, obj::Gd};

use crate::player::MovementContext;

use super::{State, movement_states::MovementStates};

#[derive(Debug)]
pub struct Idle {
    #[allow(unused)]
    context: Gd<MovementContext>,
    elapsed: f32,
    next_state: Option<MovementStates>,
}

impl State for Idle {
    type StatesEnum = MovementStates;
    type Context = Gd<MovementContext>;

    fn new(context: Self::Context) -> Self {
        Idle {
            context,
            elapsed: 0.,
            next_state: None,
        }
    }

    fn get_state_name(&self) -> Self::StatesEnum {
        MovementStates::Idle
    }

    fn set_next_state(&mut self, state: Self::StatesEnum) {
        self.next_state = Some(state);
    }

    fn get_next_state(&mut self) -> Option<Self::StatesEnum> {
        self.next_state.clone()
    }

    fn enter(&mut self) {
        self.set_next_state(MovementStates::Idle);
    }

    fn input(&mut self, _event: Gd<godot::classes::InputEvent>) {
        todo!("Implement input handling for Idle state");
    }

    fn process(&mut self, delta: f32) {
        godot_print!("idle::process()");

        self.elapsed += delta;

        if self.elapsed < 1. {
            return;
        }

        self.set_next_state(MovementStates::Walking);
    }

    fn process_physics(&mut self, _delta: f32) {}

    fn exit(&mut self) {
        godot_print!("Exiting Idle state...");

        self.elapsed = 0.;
        self.set_next_state(MovementStates::Idle);
    }
}
