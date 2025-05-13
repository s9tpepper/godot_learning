use std::collections::HashMap;

use godot::{global::godot_print, obj::Gd};

use crate::{
    finite_state_machine::{FiniteStateMachine, StateMap},
    player::{Fsm, MovementContext},
    states::{State, StateUpdates, idle::Idle, walking::Walking},
};

#[derive(Debug, Default)]
pub struct SomeStateMachine {
    context: Option<Gd<MovementContext>>,

    states: StateMap,

    #[allow(unused)]
    current_state: String,
}

impl FiniteStateMachine for SomeStateMachine {
    type Context = Gd<MovementContext>;

    fn ready(&mut self, state_machine: Fsm<Self::Context>) {
        godot_print!("[SomeStateMachine::ready()]");

        let Some(context) = &self.context else {
            godot_print!("[SomeStateMachine::ready()] - No context found");
            return;
        };

        godot_print!("context: {context:?}");

        // context

        self.states = self.setup_states(context.clone(), state_machine.clone());

        godot_print!(
            "[SomeStateMachine::ready()] - Set up states. {:?}",
            self.states
        );

        self.switch("Idle");

        godot_print!("[SomeStateMachine::ready()] - Switched to Idle");
    }

    fn get_state(&mut self, state: &str) -> Option<&mut Box<dyn StateUpdates>> {
        let state = self.states.get_mut(state);
        match state {
            Some(state) => Some(state),
            None => {
                godot_print!("Could not retrieve requested state: {state:?}");
                None
            }
        }
    }

    fn setup_states(
        &mut self,
        context: Self::Context,
        state_machine: Fsm<Self::Context>,
    ) -> StateMap {
        godot_print!("[FiniteStateMachine::setup_states()]");

        let mut states: StateMap = HashMap::new();

        // TODO: Make this macro to facilitate registering states
        // register_states!(Idle, Walking);
        // OR: make this a function in FiniteStateMachine to avoid
        // the repetition

        let mut idle = Idle::new(context.clone());
        idle.set_state_machine(state_machine.clone());
        let state_name = idle.get_state_name();
        let boxed = Box::new(idle) as Box<dyn StateUpdates>;
        states.insert(state_name, boxed);

        let mut walking = Walking::new(context.clone());
        walking.set_state_machine(state_machine.clone());
        let state_name = walking.get_state_name();
        let boxed = Box::new(walking) as Box<dyn StateUpdates>;
        states.insert(state_name, boxed);

        states
    }
}

impl SomeStateMachine {
    pub fn new(context: Gd<MovementContext>) -> Self {
        SomeStateMachine {
            context: Some(context),
            states: HashMap::default(),
            current_state: "".to_string(),
        }
    }

    pub fn _physics_process(&mut self, _delta: f64) {
        let Some(mut _state_node) = self.states.get(&self.current_state.to_string()) else {
            return;
        };
    }

    fn _process(&mut self, _delta: f64) {}
}
