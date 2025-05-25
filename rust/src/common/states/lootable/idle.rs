use std::{cell::RefCell, rc::Rc};

use godot::{
    classes::{CharacterBody3D, InputEvent},
    obj::Gd,
};

use crate::common::{inventory::Inventory, states::State};

use super::{LootableContext, lootable_states::LootableStates};

#[derive(Debug)]
pub struct Idle {
    context: LootableContext,
    inventory: Rc<RefCell<Inventory>>,
}

impl Idle {
    pub fn new(context: LootableContext, inventory: Rc<RefCell<Inventory>>) -> Self {
        Idle { context, inventory }
    }
}

impl State for Idle {
    type StatesEnum = LootableStates;
    type Context = LootableContext;
    type Subject = Inventory;

    fn new(context: Self::Context, subject: Self::Subject) -> Self
    where
        Self: Sized,
    {
        todo!()
    }

    fn get_state_name(&self) -> Self::StatesEnum {
        LootableStates::Inspect
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

    fn process_physics(&mut self, delta: f32) {
        todo!()
    }

    fn exit(&mut self) {
        todo!()
    }
}
