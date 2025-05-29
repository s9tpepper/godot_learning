use std::sync::LazyLock;

use godot::{
    builtin::Vector2,
    classes::{AnimationPlayer, Input},
    obj::Gd,
};

use crate::{actions::Actions, common::states::State};

use super::{context::MovementContext, movement_states::MovementStates};

// TODO: Figure out a better way to do this so that I don't have to
// duplicate this object in both Idle and Walking states
static ACTIONS: LazyLock<Actions> = LazyLock::new(Actions::default);

#[derive(Debug)]
pub struct Idle {
    #[allow(unused)]
    context: Gd<MovementContext>,
    next_state: Option<MovementStates>,
}

struct IdleNodes {
    animator: Gd<AnimationPlayer>,
}

impl State for Idle {
    type StatesEnum = MovementStates;
    type Context = Gd<MovementContext>;

    fn new(context: Self::Context) -> Self {
        Idle {
            context,
            next_state: None,
        }
    }

    fn get_state_name(&self) -> Self::StatesEnum {
        MovementStates::Idle
    }

    fn set_next_state(&mut self, state: Self::StatesEnum) {
        self.next_state = Some(state);
    }

    fn get_next_state(&mut self) -> Option<Self::StatesEnum> {
        self.next_state.clone()
    }

    fn enter(&mut self) {
        let gd_context = self.context.clone();
        let context = gd_context.bind();
        let mut animator = context.get_node(context.animator.clone());

        self.set_next_state(MovementStates::Idle);

        animator.stop();
    }

    fn input(&mut self, _event: Gd<godot::classes::InputEvent>) {
        let input = Input::singleton();

        let movement_vector = input.get_vector(
            ACTIONS.right,
            ACTIONS.left,
            ACTIONS.backward,
            ACTIONS.forward,
        );

        if movement_vector != Vector2::ZERO {
            self.set_next_state(MovementStates::Walking);
        }
    }

    fn process(&mut self, _delta: f32) {}

    fn physics_process(&mut self, _delta: f32) {}

    fn exit(&mut self) {
        self.set_next_state(MovementStates::Idle);
    }
}
