use godot::{global::godot_print, obj::Gd};

use crate::player::MovementContext;

use super::{State, StateUpdates, movement_states::MovementStates};

#[derive(Debug)]
pub struct Idle {
    #[allow(unused)]
    context: Gd<MovementContext>,
    elapsed: f32,
    next_state: Option<MovementStates>,
}

impl Idle {
    pub fn new(context: Gd<MovementContext>) -> Self {
        Idle {
            context,
            elapsed: 0.,
            next_state: None,
        }
    }
}

impl State for Idle {
    type StatesEnum = MovementStates;
    type Context = Gd<MovementContext>;

    fn get_state_name(&self) -> Self::StatesEnum {
        MovementStates::Idle
    }
}

impl StateUpdates for Idle {
    type StatesEnum = MovementStates;

    fn next(&mut self) -> Option<Self::StatesEnum> {
        self.next_state.take()
    }

    fn enter(&mut self) {
        godot_print!("Implement the enter logic for Idle state");

        godot_print!(
            ">>> animation name {}",
            self.context.bind_mut().get_walking_animation_name()
        );
    }

    fn input(&mut self, _event: Gd<godot::classes::InputEvent>) {
        todo!("Implement input handling for Idle state");
    }

    fn process(&mut self, delta: f32) {
        self.elapsed += delta;

        if self.elapsed < 1. {
            return;
        }

        self.next_state = Some(MovementStates::Walking);
    }

    fn process_physics(&mut self, _delta: f32) {}

    fn exit(&mut self) {
        todo!()
    }
}
