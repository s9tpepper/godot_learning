use std::{cell::RefCell, rc::Rc};

use godot::{
    classes::{InputEvent, InputEventMouseButton},
    global::godot_print,
    obj::Gd,
};

use crate::common::{inventory::Inventory, states::State};

use super::{LootContext, loot_state::LootState};

#[derive(Debug)]
pub struct Idle {
    context: Rc<LootContext>,
    inventory: Rc<RefCell<Inventory>>,
    next_state: Option<LootState>,
}

impl Idle {}

impl State for Idle {
    type StatesEnum = LootState;
    type Context = Rc<LootContext>;
    type Subject = Rc<RefCell<Inventory>>;

    fn new(context: Self::Context, inventory: Self::Subject) -> Self
    where
        Self: Sized,
    {
        Idle {
            context,
            inventory,
            next_state: None,
        }
    }

    fn get_state_name(&self) -> Self::StatesEnum {
        LootState::Inspect
    }

    fn set_next_state(&mut self, state: Self::StatesEnum) {
        self.next_state = Some(state);
    }

    fn get_next_state(&mut self) -> Option<Self::StatesEnum> {
        self.next_state.clone()
    }

    fn enter(&mut self) {
        godot_print!("Loot entering idle state");
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        godot_print!("Loot:Idle input()");
        let mouse_button_event = event.try_cast::<InputEventMouseButton>();
        if let Ok(_mouse_button_event) = mouse_button_event {
            godot_print!("sphere was clicked?");
        }
    }

    fn process(&mut self, delta: f32) {}

    fn process_physics(&mut self, delta: f32) {}

    fn exit(&mut self) {}
}
