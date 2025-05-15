use std::cell::RefCell;
use std::rc::Rc;

use godot::classes::notify::Node3DNotification;
use godot::classes::{CharacterBody3D, ICharacterBody3D, InputEvent};
use godot::prelude::*;

use crate::finite_state_machine::FiniteStateMachine;
use crate::some_state_machine::SomeStateMachine;
use crate::states::movement_states::MovementStates;

#[derive(GodotClass)]
#[class(base=CharacterBody3D, init)]
#[allow(unused)]
pub struct Player3D {
    #[export]
    context: Option<Gd<MovementContext>>,

    base: Base<CharacterBody3D>,

    state_machine: Option<Fsm<Gd<MovementContext>, MovementStates>>,
}

pub type Fsm<C, E> = FsmHelper<C, E>;

pub type FsmHelper<C, E> = Rc<RefCell<Box<dyn FiniteStateMachine<StatesEnum = E, Context = C>>>>;

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
        godot_print!("PLAYER READY ONE");

        if let Some(context) = &self.context {
            let state_machine = SomeStateMachine::new(context.clone());

            let machine: Fsm<Gd<MovementContext>, MovementStates> =
                Rc::new(RefCell::new(Box::new(state_machine)));
            let machine_rc = machine.clone();

            machine.borrow_mut().ready(machine_rc);

            self.state_machine = Some(machine.clone());
        }
    }

    // Called every frame.
    fn process(&mut self, delta: f64) {
        let Some(machine) = &self.state_machine else {
            godot_print!("Unable to get state machine reference");
            return;
        };

        let mut machine = machine.borrow_mut();
        if let Some(state_change) = machine.process(delta) {
            machine.switch(state_change);
        }
    }

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
