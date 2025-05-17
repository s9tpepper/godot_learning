#![allow(non_snake_case)]

use std::{collections::HashMap, hash::Hash};

use godot::{classes::InputEvent, obj::Gd};

use crate::states::State;

pub trait FiniteStateMachine: std::fmt::Debug {
    type StatesEnum: Clone + PartialEq + Eq + Hash;
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
    >
    where
        Self: Sized;

    fn get_state(
        &mut self,
        state: Self::StatesEnum,
    ) -> Option<&mut Box<dyn State<Context = Self::Context, StatesEnum = Self::StatesEnum>>>
    where
        Self: Sized,
    {
        let state_map = self.get_states_map();
        state_map.get_mut(&state)
    }

    fn input(&mut self, event: Gd<InputEvent>)
    where
        Self: Sized,
    {
        let state = self.get_current_state();
        let Some(current_state) = self.get_state(state) else {
            return;
        };

        current_state.input(event);
    }

    fn process(&mut self, delta: f64)
    where
        Self: Sized,
    {
        let state = self.get_current_state();
        let transitioning = self.get_transitioning();
        let Some(current_state) = self.get_state(state) else {
            return;
        };

        let next_state = current_state.next();
        match next_state {
            Some(new_state) if !transitioning => self.transition_to_state(new_state),
            Some(_) | None => current_state.process(delta as f32),
        }
    }

    fn process_physics(&mut self, delta: f64)
    where
        Self: Sized,
    {
        let state = self.get_current_state();
        let transitioning = self.get_transitioning();
        let Some(current_state) = self.get_state(state) else {
            return;
        };

        let next_state = current_state.next();
        match next_state {
            Some(new_state) if !transitioning => self.transition_to_state(new_state),
            Some(_) | None => current_state.process_physics(delta as f32),
        }
    }

    fn transition_to_state(&mut self, next_state: Self::StatesEnum)
    where
        Self: Sized,
    {
        self.set_transitioning(true);

        let state = self.get_current_state();
        let Some(current_state) = self.get_state(state) else {
            return;
        };

        current_state.exit();
        self.set_current_state(next_state);

        let state = self.get_current_state();
        let Some(current_state) = self.get_state(state) else {
            return;
        };
        current_state.enter();

        self.set_transitioning(false);
    }
}
