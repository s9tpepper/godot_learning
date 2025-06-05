pub mod context;
pub mod idle;
pub mod movement_states;
pub mod walking;

use std::collections::HashMap;

use context::MovementContext;
use godot::{
    classes::{AnimationPlayer, CharacterBody3D, Node, Node3D},
    global::godot_print,
    obj::{Base, Gd},
    prelude::{GodotClass, godot_api},
};

use crate::{
    common::{finite_state_machine::FiniteStateMachine, states::State},
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
    pub fn start(&mut self, context: Gd<MovementContext>, scene_tree: Gd<Node>) {
        self.context = context;
        self.context.bind_mut().set_scene_tree(scene_tree);
        self.get_nodes();

        self.states = self.setup_states(self.context.clone());
        self.set_current_state(MovementStates::Idle);
    }

    fn get_nodes(&mut self) {
        let mut context = self.context.bind_mut();
        let scene_tree = context
            .get_scene_tree()
            .expect("Need to set_scene_tree() on MovementContext first");

        let pivot = scene_tree.try_get_node_as::<Node3D>(&context.get_pivot());
        let player = scene_tree.try_get_node_as::<CharacterBody3D>(&context.get_player());
        let player_scene = scene_tree.try_get_node_as::<Node3D>(&context.get_player_scene());

        let (Some(pivot), Some(player), Some(player_scene)) =
            (pivot.clone(), player.clone(), player_scene.clone())
        else {
            godot_print!("pivot: {pivot:?}");
            godot_print!("player: {player:?}");
            godot_print!("player_scene: {player_scene:?}");

            panic!("Could not get nodes");
        };

        let Some(animator) =
            player_scene.try_get_node_as::<AnimationPlayer>(context.get_animation_player().arg())
        else {
            godot_print!("Couldn't get animator");
            panic!("Could not get animator");
        };

        context.player_node = Some(player);
        context.pivot_node = Some(pivot);
        context.player_scene_node = Some(player_scene);
        context.animator = Some(animator);
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
