use std::{cell::RefCell, rc::Rc};

use godot::{
    classes::{INode3D, InputEvent, InputEventMouseButton, Node3D},
    global::godot_print,
    obj::{Base, Gd, NewAlloc},
    prelude::{GodotClass, godot_api},
};

use crate::common::states::State;

use super::{LootMachineContext, loot_state::LootState};

#[derive(Debug)]
pub struct Hover {
    context: LootMachineContext,
    next_state: Rc<RefCell<Option<LootState>>>,
    active: Rc<RefCell<bool>>,
    connected: bool,
}

impl State for Hover {
    type StatesEnum = LootState;
    type Context = LootMachineContext;

    fn new(context: Self::Context) -> Self {
        Hover {
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
        LootState::Hover
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
        self.set_next_state(LootState::Hover);

        let mut borrow = self.active.try_borrow_mut();
        if let Ok(active) = &mut borrow {
            **active = false;

            godot_print!("disabled hover state");
        }
    }

    // TODO: Get rid of all of these unwraps everywhere
    fn enter(&mut self) {
        {
            let mut borrow = self.active.try_borrow_mut();
            if let Ok(active) = &mut borrow {
                **active = true;
            }

            godot_print!("enabled hover state");
        }

        if self.connected {
            return;
        }

        let context = self.context.clone();
        if let Ok(mut context) = context.try_borrow_mut() {
            if let Some(ref mut collision_object) = context.collision_object {
                let mut click_listener = HoverListener::new_alloc();

                click_listener.bind_mut().next_state = self.next_state.clone();
                click_listener.bind_mut().active = self.active.clone();

                collision_object.signals().input_event().connect_obj(
                    &click_listener,
                    |this: &mut HoverListener, _, event: Gd<InputEvent>, _, _, _| {
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

                        let event = event.try_cast::<InputEventMouseButton>();
                        if let Ok(event) = event {
                            if event.is_released() {
                                let mut borrow = this.next_state.try_borrow_mut();
                                if let Ok(next_state) = &mut borrow {
                                    **next_state = Some(LootState::Inspect);
                                }
                            }
                        }
                    },
                );

                let mut leave_listener = HoverListener::new_alloc();

                leave_listener.bind_mut().next_state = self.next_state.clone();
                leave_listener.bind_mut().active = self.active.clone();

                collision_object.signals().mouse_exited().connect_obj(
                    &leave_listener,
                    |this: &mut HoverListener| {
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
                            **next_state = Some(LootState::Idle);
                        }
                    },
                );

                self.connected = true;
            }
        } else {
            godot_print!("Could not borrow context in LootState::Hover");
        }
    }
}

// Mouse signal listener
#[derive(GodotClass)]
#[class(init, base = Node3D)]
struct HoverListener {
    pub next_state: Rc<RefCell<Option<LootState>>>,
    pub active: Rc<RefCell<bool>>,
    base: Base<Node3D>,
}

#[godot_api]
impl INode3D for HoverListener {}

#[godot_api]
impl HoverListener {
    #[signal]
    fn dummy();
}
