use std::collections::HashMap;

use godot::{
    classes::{CharacterBody3D, SceneTree},
    global::godot_print,
    obj::Gd,
};

use crate::{
    finite_state_machine::FiniteStateMachine,
    player::StateContext,
    states::{State, idle::Idle, movement_states::MovementStates, walking::Walking},
};

type DynState = Box<dyn State<Context = StateContext, StatesEnum = MovementStates>>;
type StateMap = HashMap<MovementStates, DynState>;

#[derive(Debug)]
pub struct SomeStateMachine {
    context: StateContext,
    player_3_d: Gd<CharacterBody3D>,

    states: StateMap,

    transitioning: bool,

    #[allow(unused)]
    current_state: MovementStates,
}

impl SomeStateMachine {
    pub fn new(context: StateContext, player_3_d: Gd<CharacterBody3D>) -> Self {
        SomeStateMachine {
            context,
            player_3_d,
            states: HashMap::default(),
            current_state: MovementStates::Idle,
            transitioning: false,
        }
    }

    fn register_state(&mut self, state: DynState, states: &mut StateMap) {
        let state_name = state.get_state_name();
        states.insert(state_name, state);
    }
}

impl FiniteStateMachine for SomeStateMachine {
    type StatesEnum = MovementStates;
    type Context = StateContext;

    fn ready(&mut self) {
        godot_print!("[SomeStateMachine::ready()]");

        self.states = self.setup_states(self.context.clone());

        godot_print!(
            "[SomeStateMachine::ready()] - Set up states. {:?}",
            self.states
        );

        self.set_current_state(MovementStates::Idle);

        godot_print!("[SomeStateMachine::ready()] - Switched to Idle");
    }

    fn setup_states(&mut self, context: Self::Context) -> StateMap {
        godot_print!("[FiniteStateMachine::setup_states()]");

        let mut states: StateMap = HashMap::new();

        self.register_state(
            Box::new(Idle::new(context.clone(), self.player_3_d.clone())),
            &mut states,
        );
        godot_print!("Created idle state");

        self.register_state(
            Box::new(Walking::new(context.clone(), self.player_3_d.clone())),
            &mut states,
        );
        // godot_print!("Created walking state");
        //
        states
    }

    fn get_states_map(&mut self) -> &mut StateMap
    where
        Self: Sized,
    {
        &mut self.states
    }

    fn set_current_state(&mut self, state: Self::StatesEnum) {
        self.current_state = state;
    }

    fn get_current_state(&self) -> Self::StatesEnum {
        self.current_state.clone()
    }

    fn set_transitioning(&mut self, in_transition: bool) {
        self.transitioning = in_transition;
    }

    fn get_transitioning(&self) -> bool {
        self.transitioning
    }
}
