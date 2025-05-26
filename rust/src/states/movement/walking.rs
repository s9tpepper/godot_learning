use std::sync::LazyLock;

use godot::{
    builtin::{Basis, Vector3},
    classes::{Input, InputEvent},
    obj::Gd,
};

use crate::{actions::Actions, common::states::State};

use super::{context::MovementContext, movement_states::MovementStates};

static ACTIONS: LazyLock<Actions> = LazyLock::new(Actions::default);

#[derive(Debug)]
pub struct Walking {
    #[allow(unused)]
    context: Gd<MovementContext>,
    elapsed: f32,
    next_state: Option<MovementStates>,
    instant_velocity: Vector3,
}

impl Walking {
    fn rotate_target_art(&mut self) {
        // Only rotate the model if there is movement
        if self.instant_velocity == Vector3::ZERO {
            return;
        }

        let context = self.context.bind();
        let mut player_scene = context.get_node(context.player_scene_node.clone());

        let current_basis = player_scene.get_basis();
        let target_basis = Basis::looking_at(self.instant_velocity, Vector3::UP, true);
        let interpolated = current_basis.slerp(&target_basis, 0.2);
        player_scene.set_basis(interpolated);
    }

    fn apply_ground_movement(&mut self, input: &Gd<Input>) {
        let gd_context = self.context.clone();
        let context = gd_context.bind();

        let pivot = context.get_node(context.pivot_node.clone());
        let mut player = context.get_node(context.player_node.clone());
        let mut animator = context.get_node(context.animator.clone());

        let context = self.context.clone();
        let context = context.bind();
        let pivot_y = pivot.get_global_rotation().y;

        let movement_vector = input
            .get_vector(
                ACTIONS.right,
                ACTIONS.left,
                ACTIONS.backward,
                ACTIONS.forward,
            )
            .rotated(-pivot_y);

        self.instant_velocity =
            Vector3::new(movement_vector.x, 0., movement_vector.y) * context.movement_speed;

        player.set_velocity(self.instant_velocity);
        player.move_and_slide();

        if self.instant_velocity != Vector3::ZERO {
            animator
                .play_ex()
                .name(context.walking_animation_name.arg())
                .done();
        } else {
            self.set_next_state(MovementStates::Idle);
        }

        self.rotate_target_art();
    }
}

impl State for Walking {
    type StatesEnum = MovementStates;
    type Context = Gd<MovementContext>;

    fn new(context: Self::Context) -> Self {
        Walking {
            context,
            elapsed: 0.,
            next_state: None,
            instant_velocity: Vector3::ZERO,
        }
    }

    fn get_state_name(&self) -> Self::StatesEnum {
        MovementStates::Walking
    }

    fn set_next_state(&mut self, state: Self::StatesEnum) {
        self.next_state = Some(state);
    }

    fn get_next_state(&mut self) -> Option<Self::StatesEnum> {
        self.next_state.clone()
    }

    fn enter(&mut self) {
        self.next_state = Some(MovementStates::Walking);
    }

    fn input(&mut self, _event: Gd<InputEvent>) {}

    fn process(&mut self, _delta: f32) {}

    fn physics_process(&mut self, _delta: f32) {
        let gd_context = self.context.clone();
        let context = gd_context.bind();
        let mut player = context.get_node(context.player_node.clone());

        let input = Input::singleton();

        self.instant_velocity = Vector3::ZERO;

        // TODO: Move this to Jump state
        // self.apply_jump(&input, node, delta);

        self.apply_ground_movement(&input);
        player.set_velocity(self.instant_velocity);
        player.move_and_slide();
    }

    fn exit(&mut self) {
        self.elapsed = 0.;
        self.set_next_state(MovementStates::Walking);
    }
}
