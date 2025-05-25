#![allow(non_snake_case)]

use std::{collections::HashMap, fmt::Debug, hash::Hash};

use godot::{classes::InputEvent, global::godot_print, obj::Gd};

use super::states::State;

const STATE_ERROR: &str = "should_transition should always return state";

pub trait FiniteStateMachine: Debug + Sized {
    type StatesEnum: PartialEq + Eq + Hash + Debug;
    type Context;
    type Subject;

    fn ready(&mut self);
    #[allow(clippy::type_complexity)]
    fn setup_states(
        &mut self,
        context: Self::Context,
    ) -> HashMap<
        Self::StatesEnum,
        Box<
            dyn State<
                    Context = Self::Context,
                    StatesEnum = Self::StatesEnum,
                    Subject = Self::Subject,
                >,
        >,
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
        Box<
            dyn State<
                    Context = Self::Context,
                    StatesEnum = Self::StatesEnum,
                    Subject = Self::Subject,
                >,
        >,
    >;

    #[allow(clippy::type_complexity)]
    fn get_state(
        &mut self,
        state: &Self::StatesEnum,
    ) -> Option<
        &mut Box<
            dyn State<
                    Context = Self::Context,
                    StatesEnum = Self::StatesEnum,
                    Subject = Self::Subject,
                >,
        >,
    > {
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

    fn process_physics(&mut self, delta: f64) {
        match self.should_transition() {
            (true, next_state, _) => self.transition_to_state(next_state.expect(STATE_ERROR)),
            (false, _, Some(current_state)) => current_state.process_physics(delta as f32),
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
                        Subject = <Self as FiniteStateMachine>::Subject,
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
