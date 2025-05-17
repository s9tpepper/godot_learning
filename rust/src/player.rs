use godot::classes::notify::Node3DNotification;
use godot::classes::{CharacterBody3D, ICharacterBody3D, InputEvent};
use godot::prelude::*;

use crate::finite_state_machine::FiniteStateMachine;
use crate::some_state_machine::SomeStateMachine;

pub type StateContext = Gd<MovementContext>;

#[derive(GodotClass)]
#[class(base=CharacterBody3D, init)]
#[allow(unused)]
pub struct Player3D {
    #[export]
    context: Option<StateContext>,
    base: Base<CharacterBody3D>,
    state_machine: Option<SomeStateMachine>,
}

#[derive(Default, Debug, GodotClass)]
#[class(base=Resource, init)]
pub struct MovementContext {
    #[export]
    pub player: NodePath,

    #[export]
    pub player_scene: NodePath,

    #[export]
    pub walking_animation_name: GString,
}

#[godot_api]
impl ICharacterBody3D for Player3D {
    // Called when the node is ready in the scene tree.
    fn ready(&mut self) {
        if let Some(context) = &self.context {
            let mut state_machine = SomeStateMachine::new(context.clone());
            state_machine.ready();

            self.state_machine = Some(state_machine);
        }
    }

    // Called every frame.
    fn process(&mut self, delta: f64) {
        let Some(ref mut machine) = self.state_machine else {
            godot_print!("Unable to get state machine reference");
            return;
        };

        machine.process(delta);
    }

    // Called every physics frame.
    fn physics_process(&mut self, delta: f64) {
        let Some(ref mut machine) = self.state_machine else {
            godot_print!("Unable to get state machine reference");
            return;
        };

        machine.process_physics(delta);
    }

    // String representation of the object.
    fn to_string(&self) -> GString {
        GString::from("Player3D")
    }

    // Handle user input.
    fn input(&mut self, event: Gd<InputEvent>) {
        let Some(ref mut machine) = self.state_machine else {
            godot_print!("Unable to get state machine reference");
            return;
        };

        machine.input(event);
    }

    // Handle lifecycle notifications.
    fn on_notification(&mut self, _what: Node3DNotification) {}
}
