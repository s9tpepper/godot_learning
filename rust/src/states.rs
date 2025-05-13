use crate::player::FsmHelper;

pub mod idle;
pub mod walking;

pub trait State {
    type Enum;
    type Context;

    fn get_state_name(&self) -> String;
    fn set_state_machine(&mut self, state_machine: FsmHelper<Self::Enum, Self::Context>);
}

pub trait StateUpdates: std::fmt::Debug {
    fn enter(&mut self);
    fn update(&self, delta: f32);
    fn exit(&self);
}

impl Default for Box<dyn StateUpdates> {
    fn default() -> Self {
        Box::new(())
    }
}

impl StateUpdates for () {
    fn enter(&mut self) {
        todo!()
    }

    fn update(&self, _delta: f32) {
        todo!()
    }

    fn exit(&self) {
        todo!()
    }
}
