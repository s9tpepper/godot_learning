pub mod context;
pub mod idle;
pub mod movement_states;
pub mod walking;

use std::collections::HashMap;

use context::MovementContext;
use godot::{
    classes::{Node, Node3D},
    global::godot_print,
    obj::{Base, Gd},
    prelude::{GodotClass, godot_api},
};

use crate::{
    common::{self, finite_state_machine::FiniteStateMachine, states::State},
    impl_inode3d_for_fsm,
    player::StateContext,
    states::movement::{idle::Idle, movement_states::MovementStates, walking::Walking},
};

type DynState = Box<dyn State<Context = StateContext, StatesEnum = MovementStates>>;
type StateMap = HashMap<MovementStates, DynState>;

#[derive(Debug, GodotClass)]
#[class(init, base = Node3D)]
pub struct MovementMachine {
    base: Base<Node3D>,

    context: StateContext,
    states: StateMap,
    transitioning: bool,

    #[allow(unused)]
    current_state: MovementStates,
}

impl_inode3d_for_fsm!(MovementMachine);

impl MovementMachine {
    pub fn set_context(&mut self, context: Gd<MovementContext>) -> &mut Self {
        self.context = context;

        self
    }

    pub fn set_scene_tree(&mut self, scene_tree: Gd<Node>) {
        godot_print!("[MovementMachine::ready()] scene_tree: {scene_tree:?}");

        self.context.bind_mut().set_scene_tree(scene_tree);

        self.states = self.setup_states(self.context.clone());

        godot_print!(
            "[MovementMachine::ready()] - Set up states. {:?}",
            self.states
        );

        self.set_current_state(MovementStates::Idle);

        godot_print!("[MovementMachine::ready()] - Switched to Idle");
    }

    fn register_state(&mut self, state: DynState, states: &mut StateMap) {
        let state_name = state.get_state_name();
        states.insert(state_name, state);
    }
}

impl FiniteStateMachine for MovementMachine {
    type StatesEnum = MovementStates;
    type Context = StateContext;

    fn ready(&mut self) {}

    fn setup_states(&mut self, context: Self::Context) -> StateMap {
        godot_print!("[MovementMachine::setup_states()]");

        let mut states: StateMap = HashMap::new();

        self.register_state(Box::new(Idle::new(context.clone())), &mut states);
        godot_print!("Created idle state");

        self.register_state(Box::new(Walking::new(context.clone())), &mut states);
        godot_print!("Created walking state");

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
