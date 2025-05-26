use std::rc::Rc;

use godot::{classes::InputEvent, obj::Gd};

use crate::common::states::State;

use super::{LootContext, loot_state::LootState};

#[derive(Debug)]
pub struct Idle {
    context: Rc<LootContext>,
}

impl Idle {
    pub fn new(context: Rc<LootContext>) -> Self {
        Idle { context }
    }
}

impl State for Idle {
    type StatesEnum = LootState;
    type Context = Rc<LootContext>;

    fn new(context: Self::Context) -> Self
    where
        Self: Sized,
    {
        todo!()
    }

    fn get_state_name(&self) -> Self::StatesEnum {
        LootState::Inspect
    }

    fn set_next_state(&mut self, state: Self::StatesEnum) {
        todo!()
    }

    fn get_next_state(&mut self) -> Option<Self::StatesEnum> {
        todo!()
    }

    fn enter(&mut self) {
        todo!()
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        todo!()
    }

    fn process(&mut self, delta: f32) {
        todo!()
    }

    fn physics_process(&mut self, delta: f32) {
        todo!()
    }

    fn exit(&mut self) {
        todo!()
    }
}
