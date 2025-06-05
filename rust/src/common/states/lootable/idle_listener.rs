use std::{cell::RefCell, rc::Rc};

use godot::{
    classes::{INode3D, Node3D},
    global::godot_error,
    obj::Base,
    prelude::{GodotClass, godot_api},
};

use super::{idle::LootMenuIdleStateError, loot_state::LootState};

#[derive(GodotClass)]
#[class(init, base = Node3D)]
pub struct IdleListener {
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

    pub fn mouse_entered(&mut self) -> Result<(), LootMenuIdleStateError> {
        // NOTE: Using self active bool to stop the state changes
        // because godot-rust does not yet have a disconnect()
        // function for the input_event() signal implemented

        let borrow = self
            .active
            .try_borrow_mut()
            .map_err(|_| LootMenuIdleStateError::ActiveFlag);

        let active = match borrow {
            Ok(active) => *active,
            Err(error) => {
                godot_error!("{error}");

                false
            }
        };

        if !active {
            return Ok(());
        }

        let borrow = self
            .next_state
            .try_borrow_mut()
            .map_err(|_| LootMenuIdleStateError::NextState);

        match borrow {
            Ok(mut next_state) => {
                *next_state = Some(LootState::Hover);
            }

            Err(error) => godot_error!("{error}"),
        }

        Ok(())
    }
}
