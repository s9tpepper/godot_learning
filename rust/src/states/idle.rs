use godot::{global::godot_print, obj::Gd};

use crate::player::{Fsm, FsmHelper, MovementContext};

use super::{State, StateUpdates};

#[derive(Debug)]
pub struct Idle {
    #[allow(unused)]
    context: Gd<MovementContext>,

    state_machine: Option<Fsm<Gd<MovementContext>>>,
}

impl Idle {
    pub fn new(context: Gd<MovementContext>) -> Self {
        Idle {
            context,
            state_machine: None,
        }
    }
}

impl State for Idle {
    type Context = Gd<MovementContext>;

    fn set_state_machine(&mut self, state_machine: FsmHelper<Self::Context>) {
        self.state_machine = Some(state_machine);
    }

    fn get_state_name(&self) -> String {
        "Idle".to_string()
    }
}

impl StateUpdates for Idle {
    fn enter(&mut self) {
        godot_print!("Implement the enter logic for Idle state");

        godot_print!(
            "animation name {}",
            self.context.bind_mut().get_walking_animation_name()
        );
    }

    fn update(&self, _delta: f32) {
        todo!()
    }

    fn exit(&self) {
        todo!()
    }
}
