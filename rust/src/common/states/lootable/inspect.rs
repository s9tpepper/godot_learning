use godot::{classes::InputEvent, obj::Gd};

use crate::common::{inventory::Inventory, states::State};

use super::{LootContext, loot_state::LootState};

#[derive(Debug)]
pub struct Inspect {}

impl State for Inspect {
    type StatesEnum = LootState;
    type Context = LootContext;
    type Subject = Inventory;

    fn new(context: Self::Context, subject: Self::Subject) -> Self
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

    fn process_physics(&mut self, delta: f32) {
        todo!()
    }

    fn exit(&mut self) {
        todo!()
    }
}
