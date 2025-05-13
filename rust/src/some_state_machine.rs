use std::{collections::HashMap, rc::Rc, sync::Mutex};

use godot::{global::godot_print, obj::Gd};

use crate::{
    finite_state_machine::FiniteStateMachine,
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

impl<T> SomeStates<T> {
    fn get_default_state_name() -> String {
        Idle::<T>::state_name()
    }
}

#[derive(Debug, Default)]
pub struct SomeStateMachine<C: std::fmt::Debug> {
    context: Option<C>,

    states: HashMap<String, SomeStates<C>>,

    #[allow(unused)]
    current_state: String,
    // current_state_node: SomeStates,
}

impl<T> SomeStates<T> {
    pub fn as_state_mut(&mut self) -> &mut dyn StateUpdates {
        match self {
            SomeStates::Noop => panic!(),
            SomeStates::Idle(gd) => gd,
            // SomeStates::Walking(gd) => gd,
        }
    }
}

impl Default for &mut SomeStates<Gd<Player3D>> {
    fn default() -> Self {
        panic!("Yeet")
    }
}

impl<C: std::fmt::Debug + Clone> FiniteStateMachine for SomeStateMachine<C> {
    type Enum = SomeStates<C>;
    type States = HashMap<String, SomeStates<C>>;
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

        self.switch(&SomeStates::<C>::get_default_state_name());

        godot_print!("[SomeStateMachine::ready()] - Switched to Idle");
    }

    fn get_state(&mut self, state: &str) -> Option<&mut dyn StateUpdates> {
        let state = self.states.get_mut(state);
        match state {
            Some(state) => Some(state.as_state_mut()),
            None => {
                godot_print!("Could not retrieve requested state: {state:?}");
                None
            }
        }
    }

    fn setup_states(&mut self, context: Self::Context, state_machine: Fsm<C>) -> Self::States {
        godot_print!("[FiniteStateMachine::setup_states()]");

        let mut states: Self::States = HashMap::new();

        // TODO: Make this macro to facilitate registering states
        // register_states!(Idle, Walking, sender);

        let mut idle = Idle::<Self::Context>::new(context);
        idle.set_state_machine(state_machine);

        let state_name = idle.get_state_name();

        let idle_rc = Rc::new(Mutex::new(idle));
        states.insert(state_name, SomeStates::Idle(idle_rc));

        states
    }
}

impl<C: std::fmt::Debug> SomeStateMachine<C> {
    pub fn new(context: C) -> Self {
        SomeStateMachine {
            context: Some(context),
            states: HashMap::default(),
            current_state: "".to_string(),
            // TODO: Fix reference to current state, rename and remove node
            // since it is no longer in the node tree
            // current_state_node: SomeStates::default(),
        }
    }

    pub fn _physics_process(&mut self, _delta: f64) {
        let Some(mut _state_node) = self.states.get(&self.current_state.to_string()) else {
            return;
        };

        // let tmp = state_node as &'node dyn Any;
        // if let Some(current_state) = tmp.downcast_ref::<Box<dyn StateUpdates>>() {
        //     current_state.update(delta as f32);
        // }
    }

    fn _process(&mut self, _delta: f64) {
        // let Some(receiver) = &self.receiver else {
        //     return;
        // };
        //
        // let Ok(message) = receiver.try_recv() else {
        //     return;
        // };

        // TODO: Change this to not use mpsc::Sender
        // #[allow(clippy::single_match)]
        // match message {
        //     StateMachineEvents::Switch(new_state) => {
        //         self.switch(&new_state);
        //     }
        //
        //     _ => {}
        // }
    }
}
