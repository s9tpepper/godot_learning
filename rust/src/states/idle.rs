use godot::{global::godot_print, obj::Gd};

use crate::player::{Fsm, FsmHelper, MovementContext};

use super::{State, StateUpdates, movement_states::MovementStates};

#[derive(Debug)]
pub struct Idle {
    #[allow(unused)]
    context: Gd<MovementContext>,

    state_machine: Option<Fsm<Gd<MovementContext>, MovementStates>>,

    elapsed: f32,
}

impl Idle {
    pub fn new(context: Gd<MovementContext>) -> Self {
        Idle {
            context,
            state_machine: None,
            elapsed: 0.,
        }
    }
}

impl State for Idle {
    type StatesEnum = MovementStates;
    type Context = Gd<MovementContext>;

    fn set_state_machine(&mut self, state_machine: FsmHelper<Self::Context, Self::StatesEnum>) {
        self.state_machine = Some(state_machine);
    }

    fn get_state_name(&self) -> Self::StatesEnum {
        MovementStates::Idle
    }
}

impl StateUpdates for Idle {
    type StatesEnum = MovementStates;

    fn enter(&mut self) {
        godot_print!("Implement the enter logic for Idle state");

        godot_print!(
            ">>> animation name {}",
            self.context.bind_mut().get_walking_animation_name()
        );
    }

    fn update(&mut self, delta: f32) -> Option<Self::StatesEnum> {
        self.elapsed += delta;
        godot_print!("[Idle::update()] elapsed: {}", self.elapsed);

        if self.elapsed < 1. {
            return Some(MovementStates::Idle);
        }

        Some(MovementStates::Walking)
    }

    fn exit(&mut self) {
        todo!()
    }
}
