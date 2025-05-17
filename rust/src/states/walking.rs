use godot::{global::godot_print, obj::Gd};

use crate::player::MovementContext;

use super::{State, movement_states::MovementStates};

#[derive(Debug)]
pub struct Walking {
    #[allow(unused)]
    context: Gd<MovementContext>,
    elapsed: f32,
    next_state: Option<MovementStates>,
}

impl State for Walking {
    type StatesEnum = MovementStates;
    type Context = Gd<MovementContext>;

    fn new(context: Self::Context) -> Self {
        Walking {
            context,
            elapsed: 0.,
            next_state: None,
        }
    }

    fn get_state_name(&self) -> Self::StatesEnum {
        MovementStates::Walking
    }

    fn next(&mut self) -> Option<Self::StatesEnum> {
        self.next_state.clone()
    }

    fn enter(&mut self) {
        self.next_state = Some(MovementStates::Walking);
    }

    fn input(&mut self, _event: Gd<godot::classes::InputEvent>) {
        todo!("Implement input handling for Walking state");
    }

    fn process(&mut self, delta: f32) {
        godot_print!("walking::process()");
        self.elapsed += delta;

        if self.elapsed < 1. {
            return;
        }

        self.next_state = Some(MovementStates::Idle);
    }

    fn process_physics(&mut self, _delta: f32) {}

    fn exit(&mut self) {
        godot_print!("Exiting Walking state");

        self.elapsed = 0.;
        self.next_state = Some(MovementStates::Walking);
    }
}
