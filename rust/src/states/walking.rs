use godot::global::godot_print;

use crate::{
    player::{Fsm, FsmHelper},
    some_state_machine::SomeStates,
};

use super::{State, StateUpdates};

#[derive(Debug)]
pub struct Walking<T> {
    #[allow(unused)]
    context: T,

    state_machine: Option<Fsm<T>>,
}

impl<T> Walking<T> {
    pub fn new(context: T) -> Self {
        Walking {
            context,
            state_machine: None,
        }
    }
}

impl<C> State for Walking<C> {
    type Enum = SomeStates<C>;
    type Context = C;

    fn set_state_machine(&mut self, state_machine: FsmHelper<Self::Enum, Self::Context>) {
        self.state_machine = Some(state_machine);
    }

    fn get_state_name(&self) -> String {
        "Walking".to_string()
    }
}

impl<T: std::fmt::Debug> StateUpdates for Walking<T>
where
    T: 'static,
{
    fn enter(&self) {
        godot_print!("Implement the enter logic for Walking state")
    }

    fn update(&self, _delta: f32) {
        todo!()
    }

    fn exit(&self) {
        todo!()
    }
}
