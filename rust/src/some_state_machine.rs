use std::{collections::HashMap, rc::Rc, sync::Mutex};

use godot::{global::godot_print, obj::Gd};

use crate::{
    finite_state_machine::{FiniteStateMachine, StateMap},
    player::{Fsm, Player3D},
    states::{State, StateUpdates, idle::Idle},
};

#[derive(Debug, Default)]
pub enum SomeStates<T> {
    #[default]
    Noop,
    Idle(Rc<Mutex<Idle<T>>>),
    // Walking(Walking<Player3D>),
}

#[derive(Debug, Default)]
pub struct SomeStateMachine<C: std::fmt::Debug> {
    context: Option<C>,

    states: StateMap,

    #[allow(unused)]
    current_state: String,
}

impl Default for &mut SomeStates<Gd<Player3D>> {
    fn default() -> Self {
        panic!("Yeet")
    }
}

impl<C: std::fmt::Debug + Clone> FiniteStateMachine for SomeStateMachine<C>
where
    C: 'static,
{
    type Enum = SomeStates<C>;
    type Context = C;

    fn ready(&mut self, state_machine: Fsm<C>) {
        godot_print!("[SomeStateMachine::ready()]");

        let Some(context) = &self.context else {
            godot_print!("[SomeStateMachine::ready()] - No context found");
            return;
        };

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

    fn setup_states(&mut self, context: Self::Context, state_machine: Fsm<C>) -> StateMap {
        godot_print!("[FiniteStateMachine::setup_states()]");

        let mut states: StateMap = HashMap::new();

        // TODO: Make this macro to facilitate registering states
        // register_states!(Idle, Walking, sender);

        let mut idle = Idle::<Self::Context>::new(context);
        idle.set_state_machine(state_machine);

        let state_name = idle.get_state_name();

        let boxed = Box::new(idle) as Box<dyn StateUpdates>;
        states.insert(state_name, boxed);

        states
    }
}

impl<C: std::fmt::Debug> SomeStateMachine<C> {
    pub fn new(context: C) -> Self {
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
