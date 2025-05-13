use std::{collections::HashMap, rc::Rc, sync::Mutex};

use godot::{global::godot_print, obj::Gd};

use crate::{
    impl_state,
    player::{Fsm, Player3D},
    some_state_machine::SomeStates,
};

use super::StateUpdates;

#[derive(Debug)]
pub struct Idle<T> {
    #[allow(unused)]
    context: T,

    state_machine: Option<Fsm>,
}

impl<T> Idle<T> {
    pub fn new(context: T) -> Self {
        Idle {
            context,
            state_machine: None,
        }
    }
}

impl_state!(
    Idle<Gd<Player3D>>,
    SomeStates<Gd<Player3D>>,
    HashMap <String, SomeStates<Gd<Player3D>>>,
    Gd<Player3D>
);

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
