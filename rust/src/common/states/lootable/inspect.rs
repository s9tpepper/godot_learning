use std::{
    cell::{Ref, RefCell, RefMut},
    rc::Rc,
};

use godot::{
    builtin::Vector2,
    classes::{InputEvent, InputEventMouseButton},
    global::{godot_error, godot_print},
    obj::{Gd, NewAlloc},
};
use thiserror::Error;

use crate::common::{states::State, ui::loot_menu::LootMenu};

use super::{
    LootContext, LootMachineContext, inspect_listener::InspectListener, loot_state::LootState,
};

#[derive(Error, Debug)]
pub enum InspectError<'a> {
    #[error("The Viewport is missing a camera, get_camera_3d() returned None")]
    CameraMissing,
    #[error("The CollisionObject3D is missing a Viewport, get_viewport() returned None")]
    ViewportMissing,
    #[error("The CollisionObject3D is missing from the LootContext")]
    CollisionObjectMissing,
    #[error("The `{0}` was already borrowed")]
    AlreadyBorrowed(&'a str),
    #[error("The menu was None when as_mut() was invoked")]
    MenuShouldNotBeNone,
    #[error("The collider instance has been freed and is no longer valid")]
    ColliderInstanceInvalid,
    #[error("There was an error adding options to loot menu")]
    LootMenu,
    #[error("Borrow error borrowing state active flag")]
    ActiveFlag,
    #[error("Borrow error borrowing next_state pointer")]
    NextState,
    #[error("Borrow error borrowing mouse_hovering flag")]
    HoveringFlag,
    #[error("Borrow error borrowing trigger_menu flag")]
    TriggerMenu,
}

#[derive(Debug)]
pub struct Inspect {
    context: LootMachineContext,
    next_state: Rc<RefCell<Option<LootState>>>,
    active: Rc<RefCell<bool>>,
    menu: Rc<RefCell<Option<Gd<LootMenu>>>>,
    mouse_hovering: Rc<RefCell<bool>>,
    trigger_menu: Rc<RefCell<bool>>,
    connected: bool,
    destroyed: bool,
}

impl Inspect {
    fn add_menu_listener(&self, menu: &mut Gd<LootMenu>) {
        let listener = self.get_signal_listener();
        menu.signals()
            .option_clicked()
            .connect_obj(&listener, |this: &mut InspectListener| {
                let _ = this
                    .option_clicked()
                    .map_err(|error| godot_error!("{error}"));
            });
    }

    fn create_menu(&mut self) -> Result<(), InspectError> {
        let context = self.context.clone();

        let menu_refcell = self.menu.clone();
        let mut menu_option = menu_refcell
            .try_borrow_mut()
            .map_err(|_| InspectError::AlreadyBorrowed("Menu"))?;

        let context = context
            .try_borrow_mut()
            .map_err(|_| InspectError::AlreadyBorrowed("Context"))?;

        let inventory = context.inventory.clone();
        let slots = context.inventory_slots.clone();
        let mut collider = context
            .collision_object
            .clone()
            .ok_or(InspectError::CollisionObjectMissing)?;

        if !collider.is_instance_valid() {
            return Err(InspectError::ColliderInstanceInvalid);
        }

        let mut menu = LootMenu::new_alloc();
        *menu_option = Some(menu.clone());
        drop(menu_option);
        drop(context);

        menu.set_visible(false);

        // NOTE: put this off screen so it doesn't flicker from top
        // left corner into it's actual position first
        menu.set_position(Vector2::new(-10000., -10000.));

        menu.bind_mut()
            .set_options(slots, inventory, collider.clone())
            .map_err(|_| InspectError::LootMenu)?;

        menu.bind_mut().mouse_hovering = self.mouse_hovering.clone();

        collider.add_sibling(&menu);

        self.add_menu_listener(&mut menu);
        self.update_menu_position()?;

        menu.set_visible(true);

        Ok(())
    }

    fn update_menu_position(&mut self) -> Result<(), InspectError> {
        if !self.is_active() {
            return Ok(());
        }

        let mut menu_option = self
            .menu
            .try_borrow_mut()
            .map_err(|_| InspectError::AlreadyBorrowed("Menu"))?;

        let menu = menu_option
            .as_mut()
            .ok_or(InspectError::MenuShouldNotBeNone)?;

        let context = self
            .context
            .try_borrow_mut()
            .map_err(|_| InspectError::AlreadyBorrowed("Context"))?;

        let collider = match &context.collision_object {
            Some(collision_object) => {
                if collision_object.is_instance_valid() {
                    collision_object
                } else {
                    return Ok(());
                }
            }

            None => return Ok(()),
        };

        let viewport = collider
            .get_viewport()
            .ok_or(InspectError::ViewportMissing)?;

        let camera = viewport
            .get_camera_3d()
            .ok_or(InspectError::CameraMissing)?;

        let menu_position = camera.unproject_position(collider.get_position());
        menu.set_position(menu_position);
        menu.queue_redraw();

        Ok(())
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

    fn set_active(&mut self) {
        let borrow = self.active.try_borrow_mut();
        match borrow {
            Ok(mut active) => {
                *active = true;
            }
            Err(error) => godot_error!("{error}"),
        }
    }

    fn set_trigger(&mut self, trigger: bool) {
        let trigger_borrow = self.trigger_menu.try_borrow_mut();

        match trigger_borrow {
            Ok(mut trigger_ref) => {
                *trigger_ref = trigger;
                godot_print!("Updated Inspect.rs trigger: {trigger:?}");
            }
            Err(error) => godot_error!("Unable to set trigger to: {trigger:?}, Error: {error:?}"),
        }
    }

    fn add_mouse_entered_listener(&mut self, context: &mut RefMut<LootContext>) {
        let listener = self.get_signal_listener();

        match &mut context.collision_object {
            Some(collision_object) => {
                collision_object.signals().mouse_entered().connect_obj(
                    &listener,
                    |this: &mut InspectListener| {
                        let _ = this
                            .mouse_entered()
                            .map_err(|error| godot_error!("{error}"));
                    },
                );
            }

            None => godot_error!(
                "CollisionObject3D is missing, can not add mouse entered listener for menu"
            ),
        }
    }

    fn add_mouse_exited_listener(&mut self, context: &mut RefMut<LootContext>) {
        let listener = self.get_signal_listener();

        match &mut context.collision_object {
            Some(collision_object) => {
                collision_object.signals().mouse_entered().connect_obj(
                    &listener,
                    |this: &mut InspectListener| {
                        let _ = this.mouse_exited().map_err(|error| godot_error!("{error}"));
                    },
                );
            }

            None => godot_error!(
                "CollisionObject3D is missing, can not add mouse entered listener for menu"
            ),
        }
    }

    fn check_out_of_bounds_click(
        &mut self,
        mouse_event: Gd<InputEventMouseButton>,
        mouse_hovering: Ref<bool>,
        mut menu: RefMut<Option<Gd<LootMenu>>>,
    ) {
        if mouse_event.is_released() && !*mouse_hovering && menu.is_some() {
            match *menu {
                Some(ref mut loot_menu) => {
                    loot_menu.queue_free();
                    *menu = None;
                }
                None => godot_error!("Loot menu is None, could not close menu"),
            }

            self.set_next_state(LootState::Idle);
        }
    }

    fn is_active(&self) -> bool {
        let active_refcell = self.active.clone();

        match active_refcell.try_borrow() {
            Ok(active) => *active,
            Err(_) => false,
        }
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
        let borrow = self
            .next_state
            .try_borrow_mut()
            .map_err(|_| InspectError::NextState);

        match borrow {
            Ok(mut next_state) => *next_state = Some(state),
            Err(error) => godot_error!("{error}"),
        }
    }

    fn get_next_state(&mut self) -> Option<Self::StatesEnum> {
        let borrow = self
            .next_state
            .try_borrow_mut()
            .map_err(|_| InspectError::NextState);

        match borrow {
            Ok(next_state) => next_state.clone(),
            Err(error) => {
                godot_error!("{error}");
                None
            }
        }
    }

    fn exit(&mut self) {
        let active_refcell = self.active.clone();
        let mut borrow = active_refcell.try_borrow_mut();
        if let Ok(active) = &mut borrow {
            **active = false;
        }

        self.set_next_state(LootState::Inspect);

        godot_print!("disabled idle state");
    }

    fn enter(&mut self) {
        godot_print!("Inspect:: enter()");

        self.set_active();
        self.set_trigger(true);

        if self.connected {
            return;
        }

        let context_refcell = self.context.clone();
        match context_refcell.try_borrow_mut() {
            Ok(mut ctx) => {
                self.add_mouse_entered_listener(&mut ctx);
                self.add_mouse_exited_listener(&mut ctx);
            }
            Err(error) => godot_error!("{error}"),
        };
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        if !self.is_active() {
            return;
        }

        let mouse_button_event = event.try_cast::<InputEventMouseButton>();
        if mouse_button_event.is_err() {
            return;
        }

        let mouse_event = mouse_button_event.unwrap();
        let mouse_hovering_refcell = self.mouse_hovering.clone();
        let menu_refcell = self.menu.clone();

        let mouse_hovering_borrow = mouse_hovering_refcell
            .try_borrow()
            .map_err(|_| InspectError::HoveringFlag);

        match mouse_hovering_borrow {
            Ok(mouse_hovering) => {
                let menu_borrow = menu_refcell
                    .try_borrow_mut()
                    .map_err(|_| InspectError::LootMenu);
                match menu_borrow {
                    Ok(menu) => self.check_out_of_bounds_click(mouse_event, mouse_hovering, menu),
                    Err(error) => godot_error!("{error}"),
                }
            }
            Err(error) => godot_error!("{error}"),
        }
    }

    fn process(&mut self, _delta: f32) {
        if !self.is_active() || self.destroyed {
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

        if !is_none {
            return;
        }

        let trigger_menu = self.trigger_menu.clone();
        let trigger_borrow = trigger_menu.try_borrow_mut();
        match trigger_borrow {
            Ok(mut trigger) => {
                if *trigger && is_none {
                    *trigger = false;

                    if let Err(error) = self.create_menu() {
                        godot_error!("{error}");
                    }
                } else {
                    godot_error!(
                        "Not ready to create menu: trigger: {trigger:?}, is_none: {is_none:?}"
                    );
                }
            }

            Err(_) => godot_error!("Could not borrow trigger to check for menu creation"),
        }
    }

    fn physics_process(&mut self, _delta: f32) {
        if !self.is_active() {
            return;
        }

        if let Err(error) = self.update_menu_position() {
            godot_error!("{error}");
        }
    }
}
