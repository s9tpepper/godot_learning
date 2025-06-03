pub mod lootable;

use godot::{classes::InputEvent, obj::Gd};

pub trait State: std::fmt::Debug {
    type StatesEnum;
    type Context;

    // TODO: Fix the second argument to a generic type, not a concrete type
    fn new(context: Self::Context) -> Self
    where
        Self: Sized;

    fn get_state_name(&self) -> Self::StatesEnum;
    fn set_next_state(&mut self, state: Self::StatesEnum);
    fn get_next_state(&mut self) -> Option<Self::StatesEnum>;

    fn enter(&mut self) {}
    fn exit(&mut self) {}

    // Godot methods
    fn input(&mut self, _event: Gd<InputEvent>) {}
    fn process(&mut self, _delta: f32) {}
    fn physics_process(&mut self, _delta: f32) {}

    fn destroy(&mut self) {}
}
