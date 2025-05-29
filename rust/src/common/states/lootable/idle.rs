use std::{cell::RefCell, rc::Rc};

use godot::{
    classes::{INode3D, InputEvent, InputEventMouseMotion, Node3D},
    global::godot_print,
    obj::{Base, Gd, NewAlloc},
    prelude::{GodotClass, godot_api},
};

use crate::common::states::State;

use super::{LootMachineContext, loot_state::LootState};

#[derive(Debug)]
pub struct Idle {
    context: LootMachineContext,
    next_state: Rc<RefCell<Option<LootState>>>,

    active: Rc<RefCell<bool>>,
}

impl State for Idle {
    type StatesEnum = LootState;
    type Context = LootMachineContext;

    fn new(context: Self::Context) -> Self {
        Idle {
            context,
            next_state: Rc::new(RefCell::new(None)),
            active: Rc::new(RefCell::new(false)),
        }
    }

    fn get_state_name(&self) -> Self::StatesEnum {
        LootState::Idle
    }

    fn set_next_state(&mut self, state: Self::StatesEnum) {
        self.next_state = Rc::new(RefCell::new(Some(state)));
    }

    fn get_next_state(&mut self) -> Option<Self::StatesEnum> {
        self.next_state.try_borrow().unwrap().clone()
    }

    fn enter(&mut self) {
        {
            *self.active.try_borrow_mut().unwrap() = true;
        }

        let context = self.context.clone();
        if let Ok(mut context) = context.try_borrow_mut() {
            if let Some(ref mut collision_object) = context.collision_object {
                let mut idle_listener = IdleListener::new_alloc();

                idle_listener.bind_mut().next_state = self.next_state.clone();
                idle_listener.bind_mut().active = self.active.clone();

                collision_object.signals().input_event().connect_obj(
                    &idle_listener,
                    |this: &mut IdleListener, _, event: Gd<InputEvent>, _, _, _| {
                        // NOTE: Using this active bool to stop the state changes
                        // because godot-rust does not yet have a disconnect()
                        // function for the input_event() signal implemented
                        let active = { this.active.try_borrow().unwrap() };
                        if !*active {
                            return;
                        }

                        let event = event.try_cast::<InputEventMouseMotion>();
                        if event.is_ok() {
                            *this.next_state.try_borrow_mut().unwrap() = Some(LootState::Hover);
                            *this.active.try_borrow_mut().unwrap() = false;
                        }
                    },
                );
            }
        } else {
            godot_print!("Could not borrow context in LootState::Idle");
        }
    }
}

// Mouse signal listener
#[derive(GodotClass)]
#[class(init, base = Node3D)]
struct IdleListener {
    pub next_state: Rc<RefCell<Option<LootState>>>,
    pub active: Rc<RefCell<bool>>,
    base: Base<Node3D>,
}

#[godot_api]
impl INode3D for IdleListener {}

#[godot_api]
impl IdleListener {
    #[signal]
    fn dummy();
}
