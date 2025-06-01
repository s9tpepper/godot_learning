use std::{cell::RefCell, rc::Rc};

use godot::{
    builtin::Vector2,
    classes::{INode3D, InputEvent, InputEventMouseButton, Node3D},
    global::godot_print,
    meta::ToGodot,
    obj::{Base, Gd, NewAlloc},
    prelude::{GodotClass, godot_api},
};

use crate::common::{states::State, ui::loot_menu::LootMenu};

use super::{LootContext, LootMachineContext, loot_state::LootState};

#[derive(Debug)]
pub struct Inspect {
    context: LootMachineContext,
    next_state: Rc<RefCell<Option<LootState>>>,
    connected: bool,
    active: Rc<RefCell<bool>>,
    menu: Rc<RefCell<Option<Gd<LootMenu>>>>,
    mouse_hovering: Rc<RefCell<bool>>,
    trigger_menu: Rc<RefCell<bool>>,
    destroyed: bool,
}

impl Inspect {
    fn create_menu(&mut self) {
        let context = self.context.clone();
        if let Ok(mut context) = context.try_borrow_mut() {
            if context.collision_object.is_none() {
                return;
            }

            let slots = context.inventory_slots.clone();
            let mut menu = LootMenu::new_alloc();
            menu.set_visible(false);

            // NOTE: put this off screen so it doesn't flicker from top
            // left corner into it's actual position first
            menu.set_position(Vector2::new(-10000., -10000.));

            let inventory = context.inventory.clone();
            let collision_object = context.collision_object.as_mut().unwrap();
            if !collision_object.is_instance_valid() {
                return;
            }

            menu.bind_mut()
                .set_options(slots, inventory, collision_object.clone());
            menu.bind_mut().mouse_hovering = self.mouse_hovering.clone();

            let listener = self.get_signal_listener();
            menu.signals()
                .option_clicked()
                .connect_obj(&listener, |this: &mut InspectListener| {
                    let next_state_borrow = this.next_state.try_borrow_mut();
                    if let Ok(mut next_state) = next_state_borrow {
                        *next_state = Some(LootState::Destroy);
                    }
                });

            self.update_menu_position();
            if let Some(ref mut collision_object) = context.collision_object {
                collision_object.add_sibling(&menu);
            } else {
                godot_print!("Could not get collider");
            }

            {
                let menu_borrow = self.menu.try_borrow_mut();
                if let Ok(mut menu_ref) = menu_borrow {
                    *menu_ref = Some(menu.clone());
                } else {
                    godot_print!("Could not borrow menu refcell to update with new menu");
                }
            }

            // TODO: figure out why this still flickers from 0,0, to position
            // even if hidden earlier
            menu.set_visible(true);
        } else {
            godot_print!("Could not get context");
        }
    }

    fn update_menu_position(&mut self) {
        let menu_borrow = self.menu.try_borrow_mut();
        let Ok(mut menu) = menu_borrow else { return };

        if menu.is_none() {
            godot_print!("No menu to update");
            return;
        }

        let context = self.context.clone();

        if let Ok(mut context) = context.try_borrow_mut() {
            if let Some(ref mut collider) = context.collision_object {
                if collider.is_instance_valid() {
                    let camera = collider
                        .clone()
                        .get_viewport()
                        .unwrap()
                        .get_camera_3d()
                        .unwrap();

                    let menu_position = camera.unproject_position(collider.get_position());
                    menu.as_mut().unwrap().set_position(menu_position);
                    menu.as_mut().unwrap().queue_redraw();
                } else {
                    // TODO:  kill this thing if collider isn't valid
                }
            } else {
                godot_print!("Could not get collider");
            }
        } else {
            godot_print!("Could not get context");
        }
    }

    fn get_signal_listener(&self) -> Gd<InspectListener> {
        let mut listener = InspectListener::new_alloc();
        listener.bind_mut().next_state = self.next_state.clone();
        listener.bind_mut().active = self.active.clone();
        listener.bind_mut().context = self.context.clone();
        listener.bind_mut().mouse_hovering = self.mouse_hovering.clone();
        listener.bind_mut().trigger_menu = self.trigger_menu.clone();
        listener.bind_mut().menu = self.menu.clone();

        listener
    }
}

impl State for Inspect {
    type StatesEnum = LootState;
    type Context = LootMachineContext;

    fn new(context: Self::Context) -> Self {
        Inspect {
            context,
            next_state: Rc::new(RefCell::new(None)),
            active: Rc::new(RefCell::new(false)),
            connected: false,
            menu: Rc::new(RefCell::new(None)),
            mouse_hovering: Rc::new(RefCell::new(false)),
            trigger_menu: Rc::new(RefCell::new(false)),
            destroyed: false,
        }
    }

    fn destroy(&mut self) {
        let _ = self.next_state.take();
        let _ = self.active.take();
        let _ = self.context.take();
        let _ = self.menu.take();
        let _ = self.mouse_hovering.take();
        let _ = self.trigger_menu.take();
    }

    fn get_state_name(&self) -> Self::StatesEnum {
        LootState::Inspect
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
        self.set_next_state(LootState::Inspect);

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

            godot_print!("enabled inspect state");
        }

        if self.connected {
            return;
        }

        {
            let trigger_borrow = self.trigger_menu.try_borrow_mut();
            if let Ok(mut trigger) = trigger_borrow {
                *trigger = true;
            }
        }

        let context_borrow = self.context.try_borrow_mut();
        if let Ok(mut context) = context_borrow {
            let listener = self.get_signal_listener();

            if let Some(ref mut collision_object) = context.collision_object {
                collision_object.signals().mouse_entered().connect_obj(
                    &listener,
                    |this: &mut InspectListener| {
                        let mouse_hovering_borrow = this.mouse_hovering.try_borrow_mut();
                        if let Ok(mut mouse_hovering) = mouse_hovering_borrow {
                            *mouse_hovering = true;

                            let trigger_borrow = this.trigger_menu.try_borrow_mut();
                            if let Ok(mut trigger) = trigger_borrow {
                                *trigger = true;
                            }
                        }
                    },
                );

                collision_object.signals().mouse_exited().connect_obj(
                    &listener,
                    |this: &mut InspectListener| {
                        let mouse_hovering_borrow = this.mouse_hovering.try_borrow_mut();
                        if let Ok(mut mouse_hovering) = mouse_hovering_borrow {
                            *mouse_hovering = false;
                        }
                    },
                );
            }
        }
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        let mouse_button_event = event.try_cast::<InputEventMouseButton>();
        if let Ok(mouse_event) = mouse_button_event {
            godot_print!("Mouse button event");

            let mouse_hovering_borrow = self.mouse_hovering.try_borrow();
            if let Ok(mouse_hovering) = mouse_hovering_borrow {
                let menu_borrow = self.menu.try_borrow_mut();
                let Ok(mut menu) = menu_borrow else { return };

                godot_print!("menu: {menu:?}, hovering: {mouse_hovering}");

                if mouse_event.is_released() && !*mouse_hovering && menu.is_some() {
                    godot_print!("Mouse released and mouse not hovering, closing menu...");

                    // close the menu
                    let Some(ref mut loot_menu) = *menu else {
                        return;
                    };
                    loot_menu.queue_free();
                    *menu = None;

                    // move to Idle state
                    let next_state_borrow = self.next_state.try_borrow_mut();
                    let Ok(mut next_state) = next_state_borrow else {
                        return;
                    };
                    *next_state = Some(LootState::Idle);
                }
            }
        }
    }

    fn process(&mut self, _delta: f32) {
        if self.destroyed {
            return;
        }

        let is_none = {
            let menu = self.menu.clone();
            let menu_borrow = menu.try_borrow();
            if let Ok(menu) = menu_borrow {
                menu.is_none()
            } else {
                false
            }
        };

        let trigger_menu = self.trigger_menu.clone();
        let trigger_borrow = trigger_menu.try_borrow_mut();
        if let Ok(mut trigger) = trigger_borrow {
            if *trigger && is_none {
                *trigger = false;

                self.create_menu();
            }
        }
    }

    fn physics_process(&mut self, _delta: f32) {
        self.update_menu_position();
    }
}

// Mouse signal listener
#[derive(GodotClass)]
#[class(init, base = Node3D)]
struct InspectListener {
    pub next_state: Rc<RefCell<Option<LootState>>>,
    pub active: Rc<RefCell<bool>>,
    pub mouse_hovering: Rc<RefCell<bool>>,
    pub trigger_menu: Rc<RefCell<bool>>,
    pub context: Rc<RefCell<LootContext>>,
    pub menu: Rc<RefCell<Option<Gd<LootMenu>>>>,
    base: Base<Node3D>,
}

#[godot_api]
impl INode3D for InspectListener {}

#[godot_api]
impl InspectListener {
    #[signal]
    fn toggle_loot_options();
}
