use std::{cell::RefCell, rc::Rc};

use godot::{
    global::{godot_error, godot_print},
    obj::{Gd, NewAlloc},
};
use thiserror::Error;

use crate::common::states::State;

use super::{LootMachineContext, idle_listener::IdleListener, loot_state::LootState};

#[derive(Debug, Error)]
pub enum LootMenuIdleStateError {
    #[error("Error borrowing next_state pointer")]
    NextState,
    #[error("Error borrowing active flag")]
    ActiveFlag,
}

#[derive(Debug)]
pub struct Idle {
    context: LootMachineContext,
    next_state: Rc<RefCell<Option<LootState>>>,
    connected: bool,
    active: Rc<RefCell<bool>>,
}

impl Idle {
    fn set_active(&mut self, is_active: bool) {
        let borrow = self
            .active
            .try_borrow_mut()
            .map_err(|_| LootMenuIdleStateError::NextState);

        match borrow {
            Ok(mut active) => *active = is_active,
            Err(error) => godot_error!("{error}"),
        }
    }

    fn get_listener(&self) -> Gd<IdleListener> {
        let mut idle_listener = IdleListener::new_alloc();

        // TODO: Switch the active/connected logic to
        // toggle the process_mode of the collider object
        // collision_object.set_process_mode(mode);

        idle_listener.bind_mut().next_state = self.next_state.clone();
        idle_listener.bind_mut().active = self.active.clone();

        idle_listener
    }
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
        let borrow = self
            .next_state
            .try_borrow_mut()
            .map_err(|_| LootMenuIdleStateError::NextState);

        match borrow {
            Ok(mut next_state) => *next_state = Some(state),
            Err(error) => godot_error!("{error}"),
        }
    }

    fn get_next_state(&mut self) -> Option<Self::StatesEnum> {
        let borrow = self
            .next_state
            .try_borrow_mut()
            .map_err(|_| LootMenuIdleStateError::NextState);

        match borrow {
            Ok(next_state) => next_state.clone(),
            Err(error) => {
                godot_error!("{error}");
                None
            }
        }
    }

    fn exit(&mut self) {
        self.set_next_state(LootState::Idle);
        self.set_active(false);

        godot_print!("disabled idle state");
    }

    fn enter(&mut self) {
        self.set_active(true);

        if self.connected {
            return;
        }

        let context = self.context.clone();
        if let Ok(mut context) = context.try_borrow_mut() {
            if let Some(ref mut collision_object) = context.collision_object {
                let idle_listener = self.get_listener();

                collision_object.signals().mouse_entered().connect_obj(
                    &idle_listener,
                    |this: &mut IdleListener| {
                        let _ = this
                            .mouse_entered()
                            .map_err(|error| godot_error!("{error}"));
                    },
                );

                self.connected = true;
            }
        } else {
            godot_print!("Could not borrow context in LootState::Idle");
        }
    }
}
