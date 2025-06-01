use std::{cell::RefCell, rc::Rc};

use godot::{
    classes::{INode3D, Node3D},
    global::godot_print,
    obj::{Base, NewAlloc},
    prelude::{GodotClass, godot_api},
};

use crate::common::states::State;

use super::{LootMachineContext, loot_state::LootState};

#[derive(Debug)]
pub struct Idle {
    context: LootMachineContext,
    next_state: Rc<RefCell<Option<LootState>>>,
    connected: bool,
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
            connected: false,
        }
    }

    fn destroy(&mut self) {
        let _ = self.next_state.take();
        let _ = self.active.take();
        let _ = self.context.take();
    }

    fn get_state_name(&self) -> Self::StatesEnum {
        LootState::Idle
    }

    fn set_next_state(&mut self, state: Self::StatesEnum) {
        let mut borrow = self.next_state.try_borrow_mut();
        if let Ok(next_state) = &mut borrow {
            **next_state = Some(state);
        }
    }

    fn get_next_state(&mut self) -> Option<Self::StatesEnum> {
        self.next_state.try_borrow().unwrap().clone()
    }

    fn exit(&mut self) {
        self.set_next_state(LootState::Idle);

        let mut borrow = self.active.try_borrow_mut();
        if let Ok(active) = &mut borrow {
            **active = false;
        }

        godot_print!("disabled idle state");
    }

    // TODO: Get rid of all of these unwraps everywhere
    fn enter(&mut self) {
        {
            let mut borrow = self.active.try_borrow_mut();
            if let Ok(active) = &mut borrow {
                **active = true;
            }

            godot_print!("enabled idle state");
        }

        if self.connected {
            return;
        }

        let context = self.context.clone();
        if let Ok(mut context) = context.try_borrow_mut() {
            if let Some(ref mut collision_object) = context.collision_object {
                let mut idle_listener = IdleListener::new_alloc();

                // TODO: Switch the active/connected logic to
                // toggle the process_mode of the collider object
                // collision_object.set_process_mode(mode);

                idle_listener.bind_mut().next_state = self.next_state.clone();
                idle_listener.bind_mut().active = self.active.clone();

                collision_object.signals().mouse_entered().connect_obj(
                    &idle_listener,
                    |this: &mut IdleListener| {
                        // NOTE: Using this active bool to stop the state changes
                        // because godot-rust does not yet have a disconnect()
                        // function for the input_event() signal implemented
                        let active = {
                            let mut borrow = this.active.try_borrow_mut();
                            if let Ok(active) = &mut borrow {
                                **active
                            } else {
                                false
                            }
                        };

                        if !active {
                            return;
                        }

                        let mut borrow = this.next_state.try_borrow_mut();
                        if let Ok(next_state) = &mut borrow {
                            **next_state = Some(LootState::Hover);
                        }
                    },
                );

                self.connected = true;
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
