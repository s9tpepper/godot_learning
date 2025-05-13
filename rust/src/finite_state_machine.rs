#![allow(non_snake_case)]

use godot::global::godot_print;

use crate::{player::FsmHelper, states::StateUpdates};

pub trait FiniteStateMachine: std::fmt::Debug {
    type Enum;
    type States: Default;
    type Context;

    fn ready(&mut self, state_machine: FsmHelper<Self::Enum, Self::States, Self::Context>);
    fn setup_states(
        &mut self,
        context: Self::Context,
        state_machine: FsmHelper<Self::Enum, Self::States, Self::Context>,
    ) -> Self::States;
    fn get_state(&mut self, state: &str) -> Option<&mut dyn StateUpdates>;

    fn switch(&mut self, state: &str) {
        godot_print!("[FiniteStateMachine::switch()] {state}");

        if let Some(to_state) = self.get_state(state) {
            godot_print!("[FiniteStateMachine::switch() - Got state]");
            to_state.enter();
            godot_print!("[FiniteStateMachine::switch() - Triggered enter() on state]");
        }
    }
}

// Treated as an enum with two values: "One" and "Two"
// Displayed in the editor
// Treated as read-only by the editor
// #[var(
//     usage_flags = [EDITOR, GROUP]
// )]
// my_group_of_things: i8,
//
// #[export]
// my_export: i32,
//
// #[export]
// my_other_thing: i32,
//
// #[var(
//     usage_flags = [EDITOR, GROUP]
// )]
// SecondGroup: i8,
// #[export]
// my_other_thingie: i32,
// #[export(flags_3d_navigation)]
// collision_layers: i16,
