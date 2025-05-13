use crate::player::FsmHelper;

pub mod idle;
pub mod walking;

pub trait State {
    type Enum;
    type States: Default;
    type Context;

    fn get_state_name(&self) -> String;
    fn state_name() -> String;

    fn set_state_machine(
        &mut self,
        state_machine: FsmHelper<Self::Enum, Self::States, Self::Context>,
    );
}

#[macro_export]
macro_rules! impl_state {
    ($t:ty, $e:ty, $s:ty, $c:ty) => {
        impl $crate::states::State for $t {
            type Enum = $e;
            type States = $s;
            type Context = $c;

            fn set_state_machine(
                &mut self,
                state_machine: $crate::player::FsmHelper<Self::Enum, Self::States, Self::Context>,
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
    };
}

pub trait StateUpdates {
    fn enter(&self);
    fn update(&self, delta: f32);
    fn exit(&self);
}

impl Default for Box<dyn StateUpdates> {
    fn default() -> Self {
        Box::new(())
    }
}

impl StateUpdates for () {
    fn enter(&self) {
        todo!()
    }

    fn update(&self, _delta: f32) {
        todo!()
    }

    fn exit(&self) {
        todo!()
    }
}
