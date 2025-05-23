use std::sync::LazyLock;

use godot::{
    builtin::Vector2,
    classes::{AnimationPlayer, CharacterBody3D, Input, Node3D},
    global::godot_print,
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
    animator: Gd<AnimationPlayer>,
}

struct IdleNodes {
    animator: Gd<AnimationPlayer>,
}

impl Idle {
    fn get_nodes(
        context: Gd<MovementContext>,
        player_3_d: Gd<CharacterBody3D>,
    ) -> anyhow::Result<IdleNodes> {
        let context = context.bind();

        let player_scene = player_3_d.try_get_node_as::<Node3D>(&context.get_player_scene());

        let Some(player_scene) = player_scene.clone() else {
            godot_print!("player: {player_scene:?}");
            panic!("Could not get nodes");
        };

        let Some(animator) =
            player_scene.try_get_node_as::<AnimationPlayer>(context.get_animation_player().arg())
        else {
            godot_print!("Couldn't get animator");
            panic!("Could not get animator");
        };

        Ok(IdleNodes {
            animator: animator.clone(),
        })
    }
}

impl State for Idle {
    type StatesEnum = MovementStates;
    type Context = Gd<MovementContext>;

    fn new(context: Self::Context, character_body: Gd<CharacterBody3D>) -> Self {
        let Ok(IdleNodes { animator }) = Idle::get_nodes(context.clone(), character_body) else {
            panic!("Could not get idle state nodes");
        };

        Idle {
            context,
            animator,
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
        godot_print!("Entering Idle state...");
        self.set_next_state(MovementStates::Idle);
        self.animator.stop();
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

    fn process_physics(&mut self, _delta: f32) {}

    fn exit(&mut self) {
        godot_print!("Exiting Idle state...");
        self.set_next_state(MovementStates::Idle);
    }
}
