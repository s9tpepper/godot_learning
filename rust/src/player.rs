use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use godot::classes::notify::Node3DNotification;
use godot::classes::{CharacterBody3D, ICharacterBody3D, InputEvent};
use godot::prelude::*;

use crate::finite_state_machine::FiniteStateMachine;
use crate::some_state_machine::{SomeStateMachine, SomeStates};

#[derive(Debug, GodotClass)]
#[class(base=CharacterBody3D, init)]
#[allow(unused)]
pub struct Player3D {
    base: Base<CharacterBody3D>,

    state_machine: Option<Fsm>,
}

pub type Fsm =
    FsmHelper<SomeStates<Gd<Player3D>>, HashMap<String, SomeStates<Gd<Player3D>>>, Gd<Player3D>>;

pub type FsmHelper<E, S, C> =
    Rc<RefCell<Box<dyn FiniteStateMachine<Enum = E, States = S, Context = C>>>>;

#[godot_api]
impl ICharacterBody3D for Player3D {
    // Called when the node is ready in the scene tree.
    fn ready(&mut self) {
        let state_machine = SomeStateMachine::new(self.to_gd());

        let machine: Fsm = Rc::new(RefCell::new(Box::new(state_machine)));
        let machine_rc = machine.clone();
        machine.borrow_mut().ready(machine_rc);

        self.state_machine = Some(machine.clone());
    }

    // Called every frame.
    fn process(&mut self, _delta: f64) {}

    // Called every physics frame.
    fn physics_process(&mut self, _delta: f64) {}

    // String representation of the object.
    fn to_string(&self) -> GString {
        GString::from("Player3D")
    }

    // Handle user input.
    fn input(&mut self, _event: Gd<InputEvent>) {}

    // Handle lifecycle notifications.
    fn on_notification(&mut self, _what: Node3DNotification) {}
}
