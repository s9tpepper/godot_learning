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
    builtin::{GString, Signal},
    classes::{INode, INode3D, Node, Node3D, Object, RefCounted, class_macros::sys::Global},
    global::godot_print,
    meta::ToGodot,
    obj::{Base, Bounds, DynGd, Gd, Inherits, NewAlloc, WithBaseField, WithUserSignals},
    prelude::{Array, Export, GodotClass, godot_api, godot_dyn},
};

use crate::states::{self, idle::Idle};

#[derive(Default)]
pub enum StateMachineEvents {
    #[default]
    Noop,

    Switch(String),
}

pub trait State {
    fn set_context(&mut self, node: Gd<Node3D>);
    fn set_sender(&mut self, sender: Sender<StateMachineEvents>);
    fn get_state_name(&self) -> String;
}

#[macro_export]
macro_rules! impl_state {
    ($t:ty) => {
        impl $crate::finite_state_machine::State for $t {
            fn set_context(&mut self, context: Gd<Node3D>) {
                self.context = Some(context);
                godot::global::godot_print!("State set context success");
            }

            fn set_sender(
                &mut self,
                sender: std::sync::mpsc::Sender<finite_state_machine::StateMachineEvents>,
            ) {
                self.sender = Some(sender);
            }

            fn get_state_name(&self) -> String {
                stringify!($t).to_string()
            }
        }
    };
}

pub trait StateUpdates {
    fn enter(&self);
    fn update(&self, delta: f32);
    fn exit(&self);
}

#[derive(GodotClass)]
#[class(base=Node3D, init)]
pub struct SomeStateMachine {
    #[export]
    context: Option<Gd<Node3D>>,

    base: Base<Node3D>,
    states: HashMap<String, Gd<Node>>,
    current_state: String,

    receiver: Option<Receiver<StateMachineEvents>>,
    sender: Option<Sender<StateMachineEvents>>,

    // NOTE: Could not figure out how to store the actual current state
    // closest was &Box<dyn StateUpdates>
    current_state_node: Box<dyn StateUpdates>,
}

impl FiniteStateMachine for SomeStateMachine {
    fn get_states(&mut self) -> &mut HashMap<String, Gd<Node>> {
        &mut self.states
    }

    fn setup_states(
        &self,
        mut context: Gd<Node3D>,
        sender: Sender<StateMachineEvents>,
    ) -> HashMap<String, Gd<Node>> {
        godot_print!("[FiniteStateMachine::setup_states()]");
        let mut states = HashMap::new();

        self.base()
            .get_children()
            .iter_shared()
            .for_each(|mut state| {
                godot_print!("[FiniteStateMachine::setup_states() - Setting up state]");
                godot_print!("[FiniteStateMachine::setup_states() - {state}]");

                // NOTE: These 4 lines work to set the context
                // let mut idle = state.clone().cast::<Idle>();
                // godot_print!("got Idle");
                // idle.bind_mut().set_context(context.clone());
                // godot_print!("got Idle and set context");

                let tmp = &mut state as &mut dyn Any;
                let Some(st) = tmp.downcast_mut::<Box<dyn State>>() else {
                    godot_print!("Could not downcast to Box dyn State");
                    return;
                };

                //let dyn_state = idle.into_dyn::<dyn State>();

                // let variant = state.to_variant();
                // godot_print!("Got variant");
                //
                // let mut dyn_state: DynGd<RefCounted, dyn State> = match variant.try_to() {
                //     Ok(x) => x,
                //     Err(err) => {
                //         godot_print!("Error converting: {err}");
                //         panic!("oops");
                //     }
                // };
                //
                // // let mut dyn_state: DynGd<RefCounted, dyn State> = variant.try_to();
                // godot_print!("Got dyn_state");
                //
                // dyn_state.dyn_bind_mut().set_context(context.clone());
                // godot_print!("Got set the context");

                // let dyn_state = state.into_dyn::<dyn State>();

                //let node = state.try_cast::<DynGd<Node, Idle>>();

                // let mut node = &mut state as &mut dyn Any;
                // let node = node.deref_mut();

                // node.deref_mut()

                // let Some(mut node) = node.downcast_mut::<Box<dyn State>>() else {
                //     return;
                // };

                // <Gd<Idle> as State>::set_context(node, context.clone());

                //node.base_mut().set_context(context.clone());

                //
                // let node = node.deref_mut();
                //
                // let tmp = node as &mut dyn Any;
                // let Some(a_state) = tmp.downcast_mut::<Idle>() else {
                //     godot_print!("Could not downcast to fucking State");
                //     return;
                // };
                //
                // a_state.set_context(context.clone());

                // TODO: Figure out why this is not casting correctly to call the methods from
                // State trait
                // if let Ok(mut current_state) = state.clone().try_cast::<Idle>() {
                //     current_state.bind_mut().set_context(context.clone());
                //     current_state.bind_mut().set_sender(sender.clone());
                //
                //     states.insert(current_state.bind().get_state_name(), state);
                // } else {
                //     godot_print!(
                //         "[FiniteStateMachine::setup_states() - Error downcasting to dyn State]"
                //     );
                // }
            });

        states
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

        let (sender, receiver) = mpsc::channel::<StateMachineEvents>();
        self.sender = Some(sender.clone());
        self.receiver = Some(receiver);
        godot_print!("[SomeStateMachine::ready()] - Started channel.");

        self.states = self.setup_states(context.clone(), sender);
        godot_print!("[SomeStateMachine::ready()] - Set up states.");
        godot_print!("states: {:?}", self.states);

        self.switch("Idle");
        godot_print!("[SomeStateMachine::ready()] - Switched to Idle");
    }

    fn process(&mut self, _delta: f64) {
        let Some(receiver) = &self.receiver else {
            return;
        };

        let Ok(message) = receiver.try_recv() else {
            return;
        };

        #[allow(clippy::single_match)]
        match message {
            StateMachineEvents::Switch(new_state) => {
                self.switch(&new_state);
            }

            _ => {}
        }
    }
}

pub trait FiniteStateMachine: INode3D + WithBaseField + Inherits<Node> {
    fn get_states(&mut self) -> &mut HashMap<String, Gd<Node>>;
    fn setup_states(
        &self,
        context: Gd<Node3D>,
        sender: Sender<StateMachineEvents>,
    ) -> HashMap<String, Gd<Node>>;

    fn switch(&mut self, state: &str) {
        godot_print!("[FiniteStateMachine::switch()]");

        let to_state = self.get_state(state);
        godot_print!("[FiniteStateMachine::switch() - Got state]");

        to_state.enter();
        godot_print!("[FiniteStateMachine::switch() - Triggered enter() on state]");
    }

    fn get_state(&mut self, state: &str) -> &mut Box<dyn StateUpdates> {
        let states = self.get_states();
        let Some(node) = states.get_mut(state) else {
            godot_print!("The state {state} is missing.");

            panic!("The state {state} is missing.");
        };

        let tmp = node as &mut dyn Any;
        let current_state = tmp
            .downcast_mut::<Box<dyn StateUpdates>>()
            .expect("State must implement StateUpdates");

        current_state
    }
}

impl Default for Box<dyn StateUpdates> {
    fn default() -> Self {
        Box::new(())
    }
}

impl StateUpdates for () {
    fn enter(&self) {
        todo!()
    }

    fn update(&self, delta: f32) {
        todo!()
    }

    fn exit(&self) {
        todo!()
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
