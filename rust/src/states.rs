use godot::{
    classes::{CharacterBody3D, InputEvent},
    obj::Gd,
};

pub mod idle;
pub mod movement_states;
pub mod walking;

pub trait State: std::fmt::Debug {
    type StatesEnum;
    type Context;

    // TODO: Fix the second argument to a generic type, not a concrete type
    fn new(context: Self::Context, subject: Gd<CharacterBody3D>) -> Self
    where
        Self: Sized;
    fn get_state_name(&self) -> Self::StatesEnum;
    fn set_next_state(&mut self, state: Self::StatesEnum);
    fn get_next_state(&mut self) -> Option<Self::StatesEnum>;
    fn enter(&mut self);
    fn input(&mut self, event: Gd<InputEvent>);
    fn process(&mut self, delta: f32);
    fn process_physics(&mut self, delta: f32);
    fn exit(&mut self);
}
