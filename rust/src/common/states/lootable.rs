use std::{cell::RefCell, collections::HashMap, rc::Rc};

use idle::Idle;
use lootable_states::LootableStates;

use crate::common::{finite_state_machine::FiniteStateMachine, inventory::Inventory};

use super::State;

pub mod chosen;
pub mod hover;
pub mod idle;
pub mod inspect;
pub mod lootable_states;

type DynState =
    Box<dyn State<Context = LootableContext, StatesEnum = LootableStates, Subject = Inventory>>;
type StateMap = HashMap<LootableStates, DynState>;

#[derive(Debug)]
struct Lootable {
    context: LootableContext,
    states: StateMap,
    current_state: LootableStates,
    transitioning: bool,
    inventory: Rc<RefCell<Inventory>>,
}

impl Lootable {
    pub fn new(context: LootableContext, inventory: Rc<RefCell<Inventory>>) -> Self {
        Lootable {
            context,
            inventory,
            states: HashMap::default(),
            current_state: LootableStates::Idle,
            transitioning: false,
        }
    }

    fn register_state(&mut self, state: DynState, states: &mut StateMap) {
        let state_name = state.get_state_name();
        states.insert(state_name, state);
    }
}

#[derive(Debug, Clone)]
pub struct LootableContext {}

impl FiniteStateMachine for Lootable {
    type StatesEnum = LootableStates;
    type Context = LootableContext;
    type Subject = Inventory;

    fn ready(&mut self) {
        self.states = self.setup_states(self.context.clone());
        self.set_current_state(LootableStates::Idle);
    }

    fn setup_states(
        &mut self,
        context: Self::Context,
    ) -> std::collections::HashMap<
        Self::StatesEnum,
        Box<
            dyn super::State<
                    Context = Self::Context,
                    StatesEnum = Self::StatesEnum,
                    Subject = Inventory,
                >,
        >,
    > {
        let mut states: StateMap = HashMap::new();

        self.register_state(
            Box::new(Idle::new(context.clone(), self.inventory.clone())),
            &mut states,
        );

        // self.register_state(
        //     Box::new(Walking::new(context.clone(), self.player_3_d.clone())),
        //     &mut states,
        // );
        // godot_print!("Created walking state");
        //
        states
    }

    fn get_current_state(&self) -> Self::StatesEnum {
        todo!()
    }

    fn set_current_state(&mut self, state: Self::StatesEnum) {
        todo!()
    }

    fn set_transitioning(&mut self, in_transition: bool) {
        todo!()
    }

    fn get_transitioning(&self) -> bool {
        todo!()
    }

    fn get_states_map(
        &mut self,
    ) -> &mut std::collections::HashMap<
        Self::StatesEnum,
        Box<
            dyn super::State<
                    Context = Self::Context,
                    StatesEnum = Self::StatesEnum,
                    Subject = Self::Subject,
                >,
        >,
    > {
        todo!()
    }
}
