use std::collections::HashMap;

use godot::{global::godot_print, obj::Gd};

use crate::{
    finite_state_machine::{FiniteStateMachine, StateMap},
    player::{Fsm, MovementContext},
    states::{State, StateUpdates, idle::Idle, movement_states::MovementStates, walking::Walking},
};

#[derive(Debug, Default)]
pub struct SomeStateMachine {
    context: Option<Gd<MovementContext>>,

    states: StateMap<Self>,

    #[allow(unused)]
    current_state: MovementStates,
}

impl FiniteStateMachine for SomeStateMachine {
    type StatesEnum = MovementStates;
    type Context = Gd<MovementContext>;

    fn ready(&mut self, state_machine: Fsm<Self::Context, Self::StatesEnum>) {
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

        self.switch(MovementStates::Idle);

        godot_print!("[SomeStateMachine::ready()] - Switched to Idle");
    }

    fn set_current_state(&mut self, state: Self::StatesEnum) {
        self.current_state = state;
    }

    fn get_state(
        &mut self,
        state: Self::StatesEnum,
    ) -> Option<&mut Box<dyn StateUpdates<StatesEnum = Self::StatesEnum>>> {
        let state = self.states.get_mut(&state);
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
        state_machine: Fsm<Self::Context, Self::StatesEnum>,
    ) -> StateMap<Self> {
        godot_print!("[FiniteStateMachine::setup_states()]");

        let mut states: StateMap<Self> = HashMap::new();

        // TODO: Make this macro to facilitate registering states
        // register_states!(Idle, Walking);
        // OR: make this a function in FiniteStateMachine to avoid
        // the repetition

        let mut idle = Idle::new(context.clone());
        idle.set_state_machine(state_machine.clone());
        let state_name = idle.get_state_name();
        let boxed = Box::new(idle) as Box<dyn StateUpdates<StatesEnum = Self::StatesEnum>>;
        states.insert(state_name, boxed);

        let mut walking = Walking::new(context.clone());
        walking.set_state_machine(state_machine.clone());
        let state_name = walking.get_state_name();
        let boxed = Box::new(walking) as Box<dyn StateUpdates<StatesEnum = Self::StatesEnum>>;
        states.insert(state_name, boxed);

        states
    }

    fn process(&mut self, delta: f64) -> Option<Self::StatesEnum> {
        let Some(state) = self.states.get_mut(&self.current_state) else {
            godot_print!(
                "[some_state_machine::process()] - could not get state: {}",
                self.current_state
            );
            return None;
        };

        state.process(delta as f32)
    }

    fn get_current_state(&self) -> Self::StatesEnum {
        self.current_state.clone()
    }
}

impl SomeStateMachine {
    pub fn new(context: Gd<MovementContext>) -> Self {
        SomeStateMachine {
            context: Some(context),
            states: HashMap::default(),
            current_state: MovementStates::Idle,
        }
    }

    pub fn _physics_process(&mut self, _delta: f64) {
        let Some(mut _state_node) = self.states.get(&self.current_state) else {
            return;
        };
    }
}
