#![allow(unused)]
#![allow(non_snake_case)]

use std::{
    any::Any,
    collections::HashMap,
    ops::{Deref, DerefMut},
    rc::Rc,
    sync::mpsc::{self, Receiver, Sender},
    thread::current,
};

use godot::{
    builtin::{GString, Signal, Variant},
    classes::{INode, INode3D, Node, Node3D, Object, RefCounted, class_macros::sys::Global},
    global::godot_print,
    meta::ToGodot,
    obj::{
        Base, Bounds, DynGd, Gd, Inherits, NewAlloc, WithBaseField, WithUserSignals,
        bounds::MemManual,
    },
    prelude::{Array, Export, GodotClass, godot_api, godot_dyn},
};

use crate::states::{State, StateUpdates, idle::Idle, walking::Walking};

#[derive(Default)]
pub enum StateMachineEvents {
    #[default]
    Noop,
    Switch(String),
}

#[derive(GodotClass)]
#[class(base=Node3D, init)]
pub struct SomeStateMachine {
    #[export]
    context: Option<Gd<Node3D>>,

    base: Base<Node3D>,
    states: HashMap<String, SomeStates>,
    current_state: String,

    receiver: Option<Receiver<StateMachineEvents>>,
    sender: Option<Sender<StateMachineEvents>>,

    // NOTE: Could not figure out how to store the actual current state
    // closest was &Box<dyn StateUpdates>
    current_state_node: Box<dyn StateUpdates>,
}

#[derive(Debug, Default)]
pub enum SomeStates {
    #[default]
    Noop,

    Idle(Gd<Idle>),
    Walking(Gd<Walking>),
}

impl SomeStates {
    pub fn as_state_mut(&mut self) -> &mut dyn StateUpdates {
        match self {
            SomeStates::Noop => panic!(),
            SomeStates::Idle(gd) => gd,
            SomeStates::Walking(gd) => gd,
        }
    }
}

impl Default for &mut SomeStates {
    fn default() -> Self {
        panic!("Yeet")
    }
}

impl FiniteStateMachine for SomeStateMachine {
    type States = HashMap<String, SomeStates>;
    type Context = Gd<Node3D>;

    fn get_states(&mut self) -> &mut Self::States {
        &mut self.states
    }

    fn setup_states(
        &self,
        mut context: Gd<Node3D>,
        sender: Sender<StateMachineEvents>,
    ) -> Self::States {
        godot_print!("[FiniteStateMachine::setup_states()]");

        let mut states: Self::States = HashMap::new();

        let sender = self
            .sender
            .clone()
            .expect("A Sender<StateMachineEvents> must be created");

        // TODO: Make this macro to facilitate registering states
        // register_states!(Idle, Walking, sender);

        let mut idle = Idle::new_alloc();
        idle.bind_mut().set_sender(sender.clone());
        states.insert("Idle".to_string(), SomeStates::Idle(idle));

        let mut walking = Walking::new_alloc();
        walking.bind_mut().set_sender(sender.clone());
        states.insert("Walking".to_string(), SomeStates::Walking(walking));

        states
    }

    fn get_state(&mut self, state: &str) -> &mut dyn StateUpdates {
        let states = self.get_states();

        let state = states.get_mut(state).unwrap_or_default();
        state.as_state_mut()
    }
}

#[godot_api]
impl INode3D for SomeStateMachine {
    fn physics_process(&mut self, delta: f64) {
        let Some(mut state_node) = self.states.get(&self.current_state.to_string()) else {
            return;
        };

        let tmp = state_node as &dyn Any;
        if let Some(current_state) = tmp.downcast_ref::<Box<dyn StateUpdates>>() {
            current_state.update(delta as f32);
        }
    }

    fn ready(&mut self) {
        godot_print!("[SomeStateMachine::ready()]");
        let Some(context) = &self.context else {
            godot_print!("[SomeStateMachine::ready()] - No context found");
            return;
        };

        godot_print!("[SomeStateMachine::ready()] - Started channel.");

        let (sender, receiver) = mpsc::channel::<StateMachineEvents>();
        self.sender = Some(sender.clone());
        self.receiver = Some(receiver);
        self.states = self.setup_states(context.clone(), sender);

        godot_print!("[SomeStateMachine::ready()] - Set up states.");
        godot_print!("states: {:?}", self.states);

        self.switch("Idle");
        godot_print!("[SomeStateMachine::ready()] - Switched to Idle");
    }

    fn process(&mut self, _delta: f64) {
        // let Some(receiver) = &self.receiver else {
        //     return;
        // };
        //
        // let Ok(message) = receiver.try_recv() else {
        //     return;
        // };

        // TODO: Change this to not use mpsc::Sender
        // #[allow(clippy::single_match)]
        // match message {
        //     StateMachineEvents::Switch(new_state) => {
        //         self.switch(&new_state);
        //     }
        //
        //     _ => {}
        // }
    }
}

pub trait FiniteStateMachine {
    type States: Default;
    type Context;

    fn get_states(&mut self) -> &mut Self::States;
    fn setup_states(
        &self,
        context: Self::Context,
        sender: Sender<StateMachineEvents>,
    ) -> Self::States;
    // fn get_state(&mut self, state: &str) -> &mut impl StateUpdates;
    fn get_state(&mut self, state: &str) -> &mut dyn StateUpdates;

    fn switch(&mut self, state: &str) {
        godot_print!("[FiniteStateMachine::switch()]");

        let to_state = self.get_state(state);
        godot_print!("[FiniteStateMachine::switch() - Got state]");

        to_state.enter();
        godot_print!("[FiniteStateMachine::switch() - Triggered enter() on state]");
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
