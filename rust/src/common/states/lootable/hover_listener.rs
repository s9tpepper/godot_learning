use std::{cell::RefCell, rc::Rc};

use godot::{
    classes::{INode3D, InputEvent, InputEventMouseButton, Node3D},
    global::godot_error,
    obj::{Base, Gd},
    prelude::{GodotClass, godot_api},
};

use super::{hover::LootMenuHoverStateError, loot_state::LootState};

#[derive(GodotClass)]
#[class(init, base = Node3D)]
pub struct HoverListener {
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

    fn get_active(&mut self) -> bool {
        let borrow = self
            .active
            .try_borrow_mut()
            .map_err(|_| LootMenuHoverStateError::ActiveFlag);

        match borrow {
            Ok(active) => *active,
            Err(error) => {
                godot_error!("{error}");

                false
            }
        }
    }

    pub fn input_event(&mut self, event: Gd<InputEvent>) -> Result<(), LootMenuHoverStateError> {
        // NOTE: Using self active bool to stop the state changes
        // because godot-rust does not yet have a disconnect()
        // function for the input_event() signal implemented

        let active = self.get_active();
        if !active {
            return Ok(());
        }

        let event = event.try_cast::<InputEventMouseButton>();
        if event.is_err() {
            return Ok(());
        }

        if event.unwrap().is_released() {
            let borrow = self
                .next_state
                .try_borrow_mut()
                .map_err(|_| LootMenuHoverStateError::NextState);

            match borrow {
                Ok(mut next_state) => *next_state = Some(LootState::Inspect),
                Err(error) => godot_error!("{error}"),
            }
        }

        Ok(())
    }

    pub fn mouse_exited(&mut self) -> Result<(), LootMenuHoverStateError> {
        let active = self.get_active();
        if !active {
            return Ok(());
        }

        let borrow = self
            .next_state
            .try_borrow_mut()
            .map_err(|_| LootMenuHoverStateError::NextState);

        match borrow {
            Ok(mut next_state) => *next_state = Some(LootState::Idle),
            Err(error) => godot_error!("{error}"),
        }

        Ok(())
    }
}
