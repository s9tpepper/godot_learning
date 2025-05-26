use std::{cell::RefCell, collections::HashMap, rc::Rc};

use godot::{
    classes::INode3D,
    global::godot_print,
    prelude::{GodotClass, godot_api},
};
use idle::Idle;
use loot_state::LootState;

use crate::common::{
    finite_state_machine::FiniteStateMachine,
    inventory::{Inventory, InventorySlot},
};

use super::State;

pub mod chosen;
pub mod hover;
pub mod idle;
pub mod inspect;
pub mod loot_state;

type DynState = Box<dyn State<Context = Rc<LootContext>, StatesEnum = LootState>>;
type StateMap = HashMap<LootState, DynState>;

#[derive(Default, Debug, GodotClass)]
#[class(init, base = Node3D)]
pub struct LootMachine {
    context: Rc<LootContext>,
    states: StateMap,
    current_state: LootState,
    transitioning: bool,
    inventory: Rc<RefCell<Inventory>>,
}

impl LootMachine {
    pub fn new(context: Rc<LootContext>, inventory: Rc<RefCell<Inventory>>) -> Self {
        LootMachine {
            context,
            inventory,
            states: HashMap::default(),
            current_state: LootState::Idle,
            transitioning: false,
        }
    }

    fn register_state(&mut self, state: DynState, states: &mut StateMap) {
        let state_name = state.get_state_name();
        states.insert(state_name, state);
    }
}

#[derive(Default, Debug)]
pub struct LootContext {
    item: InventorySlot,
}

impl LootContext {
    pub fn new(item: InventorySlot) -> Self {
        LootContext { item }
    }
}

impl FiniteStateMachine for LootMachine {
    type StatesEnum = LootState;
    type Context = Rc<LootContext>;

    fn ready(&mut self) {
        godot_print!("[LootMachine] ready()");
        self.states = self.setup_states(self.context.clone());
        self.set_current_state(LootState::Idle);
        godot_print!("[LootMachine] ready() done.");
    }

    fn setup_states(
        &mut self,
        context: Self::Context,
    ) -> std::collections::HashMap<
        Self::StatesEnum,
        Box<dyn super::State<Context = Self::Context, StatesEnum = Self::StatesEnum>>,
    > {
        let mut states: StateMap = HashMap::new();

        self.register_state(Box::new(Idle::new(context.clone())), &mut states);
        godot_print!("[LootMachine] Registered Idle state");

        states
    }

    fn get_current_state(&self) -> Self::StatesEnum {
        self.current_state.clone()
    }

    fn set_current_state(&mut self, state: Self::StatesEnum) {
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
