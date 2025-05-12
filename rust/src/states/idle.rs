use std::{rc::Rc, sync::Mutex};

use godot::{global::godot_print, obj::Gd};

use crate::{impl_state, player::Player3D};

use super::StateUpdates;

#[derive(Debug)]
pub struct Idle<T> {
    context: T,
}

impl<T> Idle<T> {
    pub fn new(context: T) -> Self {
        Idle { context }
    }
}

impl_state!(Idle<Gd<Player3D>>);

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
