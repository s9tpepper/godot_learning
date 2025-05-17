use godot::{classes::InputEvent, obj::Gd};

pub mod idle;
pub mod movement_states;
pub mod walking;

pub trait State: std::fmt::Debug {
    type StatesEnum;
    type Context;

    fn new(context: Self::Context) -> Self
    where
        Self: Sized;
    fn get_state_name(&self) -> Self::StatesEnum;
    fn next(&mut self) -> Option<Self::StatesEnum>;
    fn enter(&mut self);
    fn input(&mut self, event: Gd<InputEvent>);
    fn process(&mut self, delta: f32);
    fn process_physics(&mut self, delta: f32);
    fn exit(&mut self);
}
