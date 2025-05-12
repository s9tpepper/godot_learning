pub mod idle;
pub mod walking;

pub trait State {
    // fn set_sender(&mut self, sender: Sender<StateMachineEvents>);
    fn get_state_name(&self) -> String;
}

#[macro_export]
macro_rules! impl_state {
    ($t:ty) => {
        impl $crate::states::State for $t {
            // fn set_sender(&mut self, sender: Sender<StateMachineEvents>) {
            //     self.sender = Some(sender);
            // }

            // fn set_context(&mut self, context: Gd<Node3D>) {
            //     self.context = context;
            // }

            fn get_state_name(&self) -> String {
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
