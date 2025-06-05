use std::{cell::RefCell, rc::Rc};

use godot::{
    classes::{INode3D, InputEvent, InputEventMouseButton, Node3D},
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

    pub fn input_event(&mut self, event: Gd<InputEvent>) -> Result<(), LootMenuHoverStateError> {
        // NOTE: Using self active bool to stop the state changes
        // because godot-rust does not yet have a disconnect()
        // function for the input_event() signal implemented
        let active = {
            let mut borrow = self.active.try_borrow_mut();
            if let Ok(active) = &mut borrow {
                **active
            } else {
                false
            }
        };

        if !active {
            return Ok(());
        }

        let event = event.try_cast::<InputEventMouseButton>();
        if let Ok(event) = event {
            if event.is_released() {
                let mut borrow = self.next_state.try_borrow_mut();
                if let Ok(next_state) = &mut borrow {
                    **next_state = Some(LootState::Inspect);
                }
            }
        }

        Ok(())
    }

    pub fn mouse_exited(&mut self) -> Result<(), LootMenuHoverStateError> {
        let active = {
            let mut borrow = self.active.try_borrow_mut();
            if let Ok(active) = &mut borrow {
                **active
            } else {
                false
            }
        };

        if !active {
            return Ok(());
        }

        let mut borrow = self.next_state.try_borrow_mut();
        if let Ok(next_state) = &mut borrow {
            **next_state = Some(LootState::Idle);
        }

        Ok(())
    }
}
