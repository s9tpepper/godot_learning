use std::{cell::RefCell, rc::Rc};

use godot::{
    classes::InputEvent,
    global::godot_error,
    obj::{Gd, NewAlloc},
};
use thiserror::Error;

use crate::common::states::State;

use super::{LootMachineContext, hover_listener::HoverListener, loot_state::LootState};

#[derive(Debug, Error)]
pub enum LootMenuHoverStateError {
    #[error("The next_state could not be borrowed")]
    NextState,

    #[error("The active flag could not be borrowed")]
    ActiveFlag,

    #[error("The context could not be borrowed")]
    Context,

    #[error("The collision object was None, it should not be missing")]
    CollisionObjectMissing,
}

#[derive(Debug)]
pub struct Hover {
    context: LootMachineContext,
    next_state: Rc<RefCell<Option<LootState>>>,
    active: Rc<RefCell<bool>>,
    connected: bool,
}

impl Hover {
    fn set_active(&mut self, is_active: bool) {
        let borrow = self
            .active
            .try_borrow_mut()
            .map_err(|_| LootMenuHoverStateError::ActiveFlag);

        match borrow {
            Ok(mut active) => {
                *active = is_active;
            }
            Err(error) => godot_error!("{error}"),
        }
    }

    fn get_listener(&self) -> Gd<HoverListener> {
        let mut listener = HoverListener::new_alloc();
        listener.bind_mut().next_state = self.next_state.clone();
        listener.bind_mut().active = self.active.clone();

        listener
    }
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
        let next_state_borrow = self
            .next_state
            .try_borrow_mut()
            .map_err(|_| LootMenuHoverStateError::NextState);

        match next_state_borrow {
            Ok(mut next_state) => {
                *next_state = Some(state);
            }
            Err(error) => godot_error!("{error}"),
        }
    }

    fn get_next_state(&mut self) -> Option<Self::StatesEnum> {
        let next_state_borrow = self
            .next_state
            .try_borrow_mut()
            .map_err(|_| LootMenuHoverStateError::NextState);

        match next_state_borrow {
            Ok(next_state) => next_state.clone(),
            Err(error) => {
                godot_error!("{error}");
                None
            }
        }
    }

    fn exit(&mut self) {
        self.set_next_state(LootState::Hover);
        self.set_active(false);
    }

    fn enter(&mut self) {
        self.set_active(true);

        if self.connected {
            return;
        }

        let context_result = self
            .context
            .try_borrow_mut()
            .map_err(|_| LootMenuHoverStateError::Context);

        match context_result {
            Ok(mut context) => {
                let collision_object_result = context
                    .collision_object
                    .as_mut()
                    .ok_or(LootMenuHoverStateError::CollisionObjectMissing);

                match collision_object_result {
                    Ok(collision_object) => {
                        collision_object.signals().input_event().connect_obj(
                            &self.get_listener(),
                            |this: &mut HoverListener, _, event: Gd<InputEvent>, _, _, _| {
                                let _ = this
                                    .input_event(event)
                                    .map_err(|error| godot_error!("{error}"));
                            },
                        );

                        collision_object.signals().mouse_exited().connect_obj(
                            &self.get_listener(),
                            |this: &mut HoverListener| {
                                let _ =
                                    this.mouse_exited().map_err(|error| godot_error!("{error}"));
                            },
                        );

                        self.connected = true;
                    }
                    Err(error) => godot_error!("{error}"),
                }
            }
            Err(error) => godot_error!("{error}"),
        }
    }
}
