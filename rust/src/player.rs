use godot::classes::notify::Node3DNotification;
use godot::classes::{CharacterBody3D, ICharacterBody3D, InputEvent};
use godot::prelude::*;

use crate::some_state_machine::SomeStateMachine;

#[derive(Debug, GodotClass)]
#[class(base=CharacterBody3D, init)]
#[allow(unused)]
pub struct Player3D {
    base: Base<CharacterBody3D>,

    state_machine: SomeStateMachine,
}

#[godot_api]
impl ICharacterBody3D for Player3D {
    // Called when the node is ready in the scene tree.
    fn ready(&mut self) {
        self.state_machine = SomeStateMachine::new(self.to_gd());
        self.state_machine.ready();
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
