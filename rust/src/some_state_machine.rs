use std::{collections::HashMap, rc::Rc, sync::Mutex};

use godot::{global::godot_print, obj::Gd};

use crate::{
    finite_state_machine::FiniteStateMachine,
    player::Player3D,
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
pub struct SomeStateMachine {
    context: Option<Gd<Player3D>>,

    states: HashMap<String, SomeStates<Gd<Player3D>>>,
    current_state: String,
    // current_state_node: SomeStates,
}

impl SomeStates<Gd<Player3D>> {
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

impl FiniteStateMachine for SomeStateMachine {
    type Enum = SomeStates<Gd<Player3D>>;
    type States = HashMap<String, SomeStates<Gd<Player3D>>>;
    type Context = Gd<Player3D>;

    fn get_state(&mut self, state: &str) -> &mut dyn StateUpdates {
        let state = self.states.get_mut(state).expect("State is not registered");

        state.as_state_mut()
    }

    fn setup_states(&mut self, context: Self::Context) -> Self::States {
        godot_print!("[FiniteStateMachine::setup_states()]");

        let mut states: Self::States = HashMap::new();

        // TODO: Make this macro to facilitate registering states
        // register_states!(Idle, Walking, sender);

        let mut state_machine = Rc::new(Mutex::new(self));

        let idle = Idle::<Self::Context>::new(context);

        let state_name = idle.get_state_name();
        let idle_rc = Rc::new(Mutex::new(idle));
        states.insert(state_name, SomeStates::Idle(idle_rc));

        states
    }
}

impl SomeStateMachine {
    pub fn new(context: Gd<Player3D>) -> Self {
        SomeStateMachine {
            context: Some(context),
            states: HashMap::default(),
            current_state: "".to_string(),
            // TODO: Fix reference to current state, rename and remove node
            // since it is no longer in the node tree
            // current_state_node: SomeStates::default(),
        }
    }

    pub fn physics_process(&mut self, delta: f64) {
        let Some(mut state_node) = self.states.get(&self.current_state.to_string()) else {
            return;
        };

        // let tmp = state_node as &'node dyn Any;
        // if let Some(current_state) = tmp.downcast_ref::<Box<dyn StateUpdates>>() {
        //     current_state.update(delta as f32);
        // }
    }

    pub fn ready(&mut self) {
        godot_print!("[SomeStateMachine::ready()]");
        let Some(context) = &self.context else {
            godot_print!("[SomeStateMachine::ready()] - No context found");
            return;
        };

        godot_print!("[SomeStateMachine::ready()] - Started channel.");

        self.states = self.setup_states(context.clone());

        godot_print!("[SomeStateMachine::ready()] - Set up states.");
        // godot_print!("states: {:?}", self.states);

        self.switch("Idle");
        godot_print!("[SomeStateMachine::ready()] - Switched to Idle");
    }

    fn process(&mut self, _delta: f64) {
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
