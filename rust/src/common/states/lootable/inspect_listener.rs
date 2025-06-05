use std::{cell::RefCell, rc::Rc};

use godot::{
    classes::{INode3D, Node3D},
    obj::{Base, Gd},
    prelude::{GodotClass, godot_api},
};

use crate::common::ui::loot_menu::LootMenu;

use super::{LootContext, inspect::InspectError, loot_state::LootState};

#[derive(GodotClass)]
#[class(init, base = Node3D)]
pub struct InspectListener {
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

    pub fn mouse_entered(&self) -> Result<(), InspectError> {
        let mut hovering = self
            .mouse_hovering
            .try_borrow_mut()
            .map_err(|_| InspectError::HoveringFlag)?;
        *hovering = true;

        let mut trigger = self
            .trigger_menu
            .try_borrow_mut()
            .map_err(|_| InspectError::TriggerMenu)?;
        *trigger = true;

        Ok(())
    }

    pub fn mouse_exited(&self) -> Result<(), InspectError> {
        let mut hovering = self
            .mouse_hovering
            .try_borrow_mut()
            .map_err(|_| InspectError::HoveringFlag)?;
        *hovering = false;

        Ok(())
    }

    pub fn option_clicked(&self) -> Result<(), InspectError> {
        let mut active = self
            .active
            .try_borrow_mut()
            .map_err(|_| InspectError::ActiveFlag)?;

        *active = false;

        let mut loot_menu_opt = self
            .menu
            .try_borrow_mut()
            .map_err(|_| InspectError::LootMenu)?;

        let loot_menu = loot_menu_opt
            .as_ref()
            .ok_or(InspectError::MenuShouldNotBeNone)?;

        let mut next_state = self
            .next_state
            .try_borrow_mut()
            .map_err(|_| InspectError::NextState)?;

        if loot_menu.bind().len() == 1 {
            *next_state = Some(LootState::Destroy);
        } else {
            *next_state = Some(LootState::Idle);
        }

        *loot_menu_opt = None;

        Ok(())
    }
}
