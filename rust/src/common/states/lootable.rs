use std::{cell::RefCell, collections::HashMap, rc::Rc};

use godot::{
    classes::{CollisionObject3D, Node3D},
    global::godot_error,
    obj::{Base, Gd, WithBaseField},
    prelude::{GodotClass, godot_api},
};

use hover::Hover;
use idle::Idle;
use inspect::Inspect;
use loot_state::LootState;
use thiserror::Error;

use crate::{
    common::{
        finite_state_machine::FiniteStateMachine,
        inventory::{Inventory, InventorySlot},
    },
    impl_inode3d_for_fsm,
};

use super::State;

pub mod hover;
pub mod hover_listener;
pub mod idle;
pub mod idle_listener;
pub mod inspect;
pub mod inspect_listener;
pub mod loot_state;

pub type LootMachineContext = Rc<RefCell<LootContext>>;

type DynState = Box<dyn State<Context = LootMachineContext, StatesEnum = LootState>>;

type StateMap = HashMap<LootState, DynState>;

#[derive(Default, Debug)]
pub struct LootContext {
    inventory_slots: Rc<RefCell<Vec<Rc<RefCell<InventorySlot>>>>>,
    inventory: Rc<RefCell<Inventory>>,
    collision_object: Option<Gd<CollisionObject3D>>,
}

impl LootContext {
    pub fn new(
        inventory_slots: Rc<RefCell<Vec<Rc<RefCell<InventorySlot>>>>>,
        inventory: Rc<RefCell<Inventory>>,
        collision_object: Gd<CollisionObject3D>,
    ) -> Self {
        LootContext {
            inventory,
            inventory_slots,
            collision_object: Some(collision_object),
        }
    }

    pub fn destroy(&mut self) {
        let _ = self.inventory.take();

        let slots_borrow = self.inventory_slots.try_borrow_mut();
        if let Ok(mut slots) = slots_borrow {
            slots.iter_mut().for_each(|slot| {
                let _ = slot.take();
            });

            slots.clear();
        }

        let _ = self.inventory_slots.take();

        if let Some(ref mut collision_obj) = self.collision_object {
            if collision_obj.is_instance_valid() {
                collision_obj.queue_free();
            }
            self.collision_object = None;
        }
    }
}

#[derive(Debug, GodotClass)]
#[class(init, base = Node3D)]
pub struct LootMachine {
    #[base]
    base: Base<Node3D>,

    context: LootMachineContext,
    states: StateMap,
    current_state: LootState,
    transitioning: bool,
}

impl_inode3d_for_fsm!(LootMachine);

#[derive(Debug, Error)]
pub enum LootMachineError {
    #[error("The context could not be borrowed")]
    Context,
}

impl LootMachine {
    pub fn start(&mut self, context: LootMachineContext) {
        self.context = context;
    }

    fn register_state(&mut self, state: DynState, states: &mut StateMap) {
        let state_name = state.get_state_name();
        states.insert(state_name, state);
    }

    fn destroy(&mut self) {
        self.states.iter_mut().for_each(|(_, state)| {
            state.destroy();
        });
        self.states.clear();

        let mut context_borrow = self
            .context
            .try_borrow_mut()
            .map_err(|_| LootMachineError::Context);

        match context_borrow {
            Ok(ref mut context) => context.destroy(),
            Err(ref error) => godot_error!("{error}"),
        }
        drop(context_borrow);

        let _ = self.context.take();

        self.base_mut().queue_free();
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

        let idle_state = Idle::new(context.clone());
        self.register_state(Box::new(idle_state), &mut states);

        let hover_state = Hover::new(context.clone());
        self.register_state(Box::new(hover_state), &mut states);

        let inspect_state = Inspect::new(context);
        self.register_state(Box::new(inspect_state), &mut states);

        states
    }

    fn get_current_state(&self) -> Self::StatesEnum {
        self.current_state.clone()
    }

    fn set_current_state(&mut self, state: Self::StatesEnum) {
        if state == LootState::Destroy {
            return self.destroy();
        }

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
