use godot::{global::godot_print, obj::Gd};

use crate::{
    player::{Fsm, FsmHelper, MovementContext},
    some_state_machine::SomeStates,
};

use super::{State, StateUpdates};

#[derive(Debug)]
pub struct Walking {
    #[allow(unused)]
    context: Gd<MovementContext>,

    state_machine: Option<Fsm<Gd<MovementContext>>>,
}

impl Walking {
    pub fn new(context: Gd<MovementContext>) -> Self {
        Walking {
            context,
            state_machine: None,
        }
    }
}

impl State for Walking {
    type Context = Gd<MovementContext>;
    type Enum = SomeStates<Self::Context>;

    fn set_state_machine(&mut self, state_machine: FsmHelper<Self::Enum, Self::Context>) {
        self.state_machine = Some(state_machine);
    }

    fn get_state_name(&self) -> String {
        "Walking".to_string()
    }
}

impl StateUpdates for Walking {
    fn enter(&mut self) {
        godot_print!("Implement the enter logic for Walking state");
        godot_print!("Context: {:?}", self.context);

        godot_print!(
            "walking animation name: {}",
            self.context.bind_mut().get_walking_animation_name()
        )
    }

    fn update(&self, _delta: f32) {
        todo!()
    }

    fn exit(&self) {
        todo!()
    }
}
