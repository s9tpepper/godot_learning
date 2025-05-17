use std::collections::HashMap;

use godot::{global::godot_print, obj::Gd};

use crate::{
    finite_state_machine::FiniteStateMachine,
    player::MovementContext,
    states::{State, idle::Idle, movement_states::MovementStates},
};

type DynState = Box<dyn State<Context = Gd<MovementContext>, StatesEnum = MovementStates>>;
type StateMap = HashMap<MovementStates, DynState>;

#[derive(Debug, Default)]
pub struct SomeStateMachine {
    context: Option<Gd<MovementContext>>,

    states: StateMap,

    transitioning: bool,

    #[allow(unused)]
    current_state: MovementStates,
}

impl FiniteStateMachine for SomeStateMachine {
    type StatesEnum = MovementStates;
    type Context = Gd<MovementContext>;

    fn get_states_map(&mut self) -> &mut StateMap
    where
        Self: Sized,
    {
        &mut self.states
    }

    fn ready(&mut self) {
        godot_print!("[SomeStateMachine::ready()]");

        let Some(context) = &self.context else {
            godot_print!("[SomeStateMachine::ready()] - No context found");
            return;
        };

        godot_print!("context: {context:?}");

        self.states = self.setup_states(context.clone());

        godot_print!(
            "[SomeStateMachine::ready()] - Set up states. {:?}",
            self.states
        );

        self.set_current_state(MovementStates::Idle);

        godot_print!("[SomeStateMachine::ready()] - Switched to Idle");
    }

    fn set_current_state(&mut self, state: Self::StatesEnum) {
        self.current_state = state;
    }

    fn setup_states(&mut self, context: Self::Context) -> StateMap {
        godot_print!("[FiniteStateMachine::setup_states()]");

        let mut states: StateMap = HashMap::new();

        // TODO: Make this macro to facilitate registering states
        // register_states!(Idle, Walking);
        // OR: make this a function in FiniteStateMachine to avoid
        // the repetition

        let mut idle = Idle::new(context.clone());
        let state_name = idle.get_state_name();
        let boxed = Box::new(idle) as DynState;
        states.insert(state_name, boxed);

        // TODO: Update Walking state with changes to Idle state
        // let mut walking = Walking::new(context.clone());
        // let state_name = walking.get_state_name();
        // let boxed = Box::new(walking) as Box<dyn StateUpdates<StatesEnum = Self::StatesEnum>>;
        // states.insert(state_name, boxed);

        states
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

impl SomeStateMachine {
    pub fn new(context: Gd<MovementContext>) -> Self {
        SomeStateMachine {
            context: Some(context),
            states: HashMap::default(),
            current_state: MovementStates::Idle,
            transitioning: false,
        }
    }
}
