use std::{cell::RefCell, collections::HashMap, rc::Rc};

use godot::{
    builtin::Vector3,
    classes::{CollisionObject3D, InputEvent, Node},
    global::godot_print,
    obj::Gd,
    prelude::{GodotClass, godot_api},
};

use idle::Idle;
use loot_state::LootState;

use crate::{
    common::{
        finite_state_machine::FiniteStateMachine,
        inventory::{Inventory, InventorySlot},
    },
    impl_inode3d_for_fsm,
};

use super::State;

pub mod chosen;
pub mod hover;
pub mod idle;
pub mod inspect;
pub mod loot_state;

pub type LootMachineContext = Rc<RefCell<LootContext>>;

type DynState = Box<dyn State<Context = LootMachineContext, StatesEnum = LootState>>;

type StateMap = HashMap<LootState, DynState>;

#[derive(Default, Debug)]
pub struct LootContext {
    inventory_slot: InventorySlot,
    inventory: Rc<RefCell<Inventory>>,
    collision_object: Option<Gd<CollisionObject3D>>,
}

impl LootContext {
    pub fn new(
        inventory_slot: InventorySlot,
        inventory: Rc<RefCell<Inventory>>,
        collision_object: Gd<CollisionObject3D>,
    ) -> Self {
        LootContext {
            inventory_slot,
            inventory,
            collision_object: Some(collision_object),
        }
    }
}

#[derive(Default, Debug, GodotClass)]
#[class(init, base = Node3D)]
pub struct LootMachine {
    context: LootMachineContext,
    states: StateMap,
    current_state: LootState,
    transitioning: bool,
}

impl_inode3d_for_fsm!(LootMachine);

impl LootMachine {
    pub fn start(&mut self, context: LootMachineContext) {
        self.context = context;
    }

    fn register_state(&mut self, state: DynState, states: &mut StateMap) {
        let state_name = state.get_state_name();
        states.insert(state_name, state);
    }
}

impl FiniteStateMachine for LootMachine {
    type StatesEnum = LootState;
    type Context = LootMachineContext;

    fn ready(&mut self) {
        self.states = self.setup_states(self.context.clone());
        self.set_current_state(LootState::Idle);
        self.transition_to_state(LootState::Idle);
    }

    fn setup_states(
        &mut self,
        context: Self::Context,
    ) -> std::collections::HashMap<
        Self::StatesEnum,
        Box<dyn super::State<Context = Self::Context, StatesEnum = Self::StatesEnum>>,
    > {
        let mut states: StateMap = HashMap::new();

        let idle_state = Idle::new(context);
        self.register_state(Box::new(idle_state), &mut states);

        godot_print!("[LootMachine] Registered Idle state");

        states
    }

    fn get_current_state(&self) -> Self::StatesEnum {
        self.current_state.clone()
    }

    fn set_current_state(&mut self, state: Self::StatesEnum) {
        godot_print!("LootMachine::set_current_state: {state:?}");

        self.current_state = state;
    }

    fn set_transitioning(&mut self, in_transition: bool) {
        self.transitioning = in_transition;
    }

    fn get_transitioning(&self) -> bool {
        self.transitioning
    }

    fn get_states_map(
        &mut self,
    ) -> &mut std::collections::HashMap<
        Self::StatesEnum,
        Box<dyn super::State<Context = Self::Context, StatesEnum = Self::StatesEnum>>,
    > {
        &mut self.states
    }
}
