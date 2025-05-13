use std::{collections::HashMap, rc::Rc, sync::Mutex};

use godot::{global::godot_print, obj::Gd};

use crate::{
    impl_state,
    player::{Fsm, FsmHelper, Player3D},
    some_state_machine::SomeStates,
};

use super::{State, StateUpdates};

#[derive(Debug)]
pub struct Idle<T> {
    #[allow(unused)]
    context: T,

    state_machine: Option<Fsm<T>>,
}

impl<T> Idle<T> {
    pub fn new(context: T) -> Self {
        Idle {
            context,
            state_machine: None,
        }
    }
}

// impl_state!(
//     Idle<Gd<Player3D>>,
//     SomeStates<Gd<Player3D>>,
//     HashMap <String, SomeStates<Gd<Player3D>>>,
//     Gd<Player3D>
// );

impl<C> State for Idle<C> {
    type Enum = SomeStates<C>;
    type States = HashMap<String, SomeStates<C>>;
    type Context = C;

    fn set_state_machine(
        &mut self,
        state_machine: FsmHelper<Self::Enum, Self::States, Self::Context>,
    ) {
        self.state_machine = Some(state_machine);
    }

    fn get_state_name(&self) -> String {
        stringify!($t).to_string()
    }

    fn state_name() -> String {
        stringify!($t).to_string()
    }
}

impl<T> StateUpdates for Rc<Mutex<Idle<T>>> {
    fn enter(&self) {
        godot_print!("Implement the enter logic for Idle state")
    }

    fn update(&self, _delta: f32) {
        todo!()
    }

    fn exit(&self) {
        todo!()
    }
}
