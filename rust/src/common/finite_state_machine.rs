#![allow(non_snake_case, unused)]

use std::{collections::HashMap, fmt::Debug, hash::Hash};

use godot::{
    classes::{INode3D, InputEvent},
    global::godot_print,
    obj::Gd,
};

use super::states::State;

const STATE_ERROR: &str = "should_transition should always return state";

#[macro_export]
macro_rules! impl_inode3d_for_fsm {
    ($machine: ident) => {
        #[godot_api]
        impl godot::classes::INode3D for $machine {
            fn ready(&mut self) {
                common::finite_state_machine::FiniteStateMachine::ready(self);
            }

            fn input(&mut self, event: godot::obj::Gd<godot::classes::InputEvent>) {
                common::finite_state_machine::FiniteStateMachine::input(self, event);
            }

            fn process(&mut self, delta: f64) {
                common::finite_state_machine::FiniteStateMachine::process(self, delta);
            }

            fn physics_process(&mut self, delta: f64) {
                common::finite_state_machine::FiniteStateMachine::physics_process(self, delta);
            }
        }
    };
}

pub trait FiniteStateMachine: Debug + Sized {
    type StatesEnum: PartialEq + Eq + Hash + Debug;
    type Context;

    fn ready(&mut self);
    #[allow(clippy::type_complexity)]
    fn setup_states(
        &mut self,
        context: Self::Context,
    ) -> HashMap<
        Self::StatesEnum,
        Box<dyn State<Context = Self::Context, StatesEnum = Self::StatesEnum>>,
    >;
    fn get_current_state(&self) -> Self::StatesEnum;
    fn set_current_state(&mut self, state: Self::StatesEnum);
    fn set_transitioning(&mut self, in_transition: bool);
    fn get_transitioning(&self) -> bool;
    #[allow(clippy::type_complexity)]
    fn get_states_map(
        &mut self,
    ) -> &mut HashMap<
        Self::StatesEnum,
        Box<dyn State<Context = Self::Context, StatesEnum = Self::StatesEnum>>,
    >;

    #[allow(clippy::type_complexity)]
    fn get_state(
        &mut self,
        state: &Self::StatesEnum,
    ) -> Option<&mut Box<dyn State<Context = Self::Context, StatesEnum = Self::StatesEnum>>> {
        let state_map = self.get_states_map();
        state_map.get_mut(state)
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        let state = self.get_current_state();
        let Some(current_state) = self.get_state(&state) else {
            return;
        };

        current_state.input(event);
    }

    fn process(&mut self, delta: f64) {
        match self.should_transition() {
            (true, next_state, _) => self.transition_to_state(next_state.expect(STATE_ERROR)),
            (false, _, Some(current_state)) => current_state.process(delta as f32),
            (false, _, None) => {}
        }
    }

    fn physics_process(&mut self, delta: f64) {
        match self.should_transition() {
            (true, next_state, _) => self.transition_to_state(next_state.expect(STATE_ERROR)),
            (false, _, Some(current_state)) => current_state.physics_process(delta as f32),
            (false, _, None) => {}
        }
    }

    #[allow(clippy::type_complexity)]
    fn should_transition(
        &mut self,
    ) -> (
        bool,
        Option<Self::StatesEnum>,
        Option<
            &mut Box<
                dyn State<
                        StatesEnum = <Self as FiniteStateMachine>::StatesEnum,
                        Context = <Self as FiniteStateMachine>::Context,
                    >,
            >,
        >,
    ) {
        let state = self.get_current_state();
        let transitioning = self.get_transitioning();
        let Some(current_state) = self.get_state(&state) else {
            return (false, None, None);
        };

        let next_state = current_state.get_next_state();
        match &next_state {
            Some(new_state) if !transitioning && state != *new_state => (true, next_state, None),
            Some(_) | None => (false, None, Some(current_state)),
        }
    }

    fn transition_to_state(&mut self, next_state: Self::StatesEnum) {
        self.set_transitioning(true);
        let state = self.get_current_state();

        let Some(current_state) = self.get_state(&state) else {
            godot_print!(
                "FiniteStateMachine::transition_to_state():: Unable to get state: {:?}",
                state
            );
            return;
        };

        current_state.exit();
        self.set_current_state(next_state);

        let state = self.get_current_state();
        let Some(current_state) = self.get_state(&state) else {
            godot_print!(
                "FiniteStateMachine::transition_to_state():: Unable to get state: {:?}",
                state
            );
            return;
        };
        current_state.enter();

        self.set_transitioning(false);
    }
}
