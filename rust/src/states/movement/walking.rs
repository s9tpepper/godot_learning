use std::sync::LazyLock;

use godot::{
    builtin::{Basis, Vector3},
    classes::{AnimationPlayer, CharacterBody3D, Input, InputEvent, Node3D},
    global::godot_print,
    obj::Gd,
};

use crate::{
    actions::Actions,
    common::{camera::Camera, states::State},
};

use super::{context::MovementContext, movement_states::MovementStates};

static ACTIONS: LazyLock<Actions> = LazyLock::new(Actions::default);

#[derive(Debug)]
pub struct Walking {
    #[allow(unused)]
    context: Gd<MovementContext>,
    elapsed: f32,
    next_state: Option<MovementStates>,
    pivot: Gd<Camera>,
    player: Gd<CharacterBody3D>,
    player_scene: Gd<Node3D>,
    instant_velocity: Vector3,
    animator: Gd<AnimationPlayer>,
}

struct WalkingNodes {
    pivot: Gd<Camera>,
    player: Gd<CharacterBody3D>,
    player_scene: Gd<Node3D>,
    animator: Gd<AnimationPlayer>,
}

impl Walking {
    fn get_nodes(
        context: Gd<MovementContext>,
        machine_node: Gd<CharacterBody3D>,
    ) -> anyhow::Result<WalkingNodes> {
        let context = context.bind();

        let pivot = machine_node.try_get_node_as::<Camera>(&context.get_pivot());
        let player = machine_node.try_get_node_as::<CharacterBody3D>(&context.get_player());
        let player_scene = machine_node.try_get_node_as::<Node3D>(&context.get_player_scene());

        let (Some(pivot), Some(player), Some(player_scene)) =
            (pivot.clone(), player.clone(), player_scene.clone())
        else {
            godot_print!("pivot: {pivot:?}");
            godot_print!("player: {player:?}");
            godot_print!("player_scene: {player_scene:?}");

            panic!("Could not get nodes");
        };

        let Some(animator) =
            player_scene.try_get_node_as::<AnimationPlayer>(context.get_animation_player().arg())
        else {
            godot_print!("Couldn't get animator");
            panic!("Could not get animator");
        };

        Ok(WalkingNodes {
            pivot: pivot.clone(),
            player: player.clone(),
            player_scene: player_scene.clone(),
            animator: animator.clone(),
        })
    }

    fn rotate_target_art(&mut self) {
        // Only rotate the model if there is movement
        if self.instant_velocity == Vector3::ZERO {
            return;
        }

        let current_basis = self.player_scene.get_basis();
        let target_basis = Basis::looking_at(self.instant_velocity, Vector3::UP, true);
        let interpolated = current_basis.slerp(&target_basis, 0.2);
        self.player_scene.set_basis(interpolated);
    }

    fn apply_ground_movement(&mut self, input: &Gd<Input>) {
        let context = self.context.clone();
        let context = context.bind();
        let pivot_y = self.pivot.get_global_rotation().y;

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

        self.player.set_velocity(self.instant_velocity);
        self.player.move_and_slide();

        if self.instant_velocity != Vector3::ZERO {
            self.animator
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
    type Subject = Gd<CharacterBody3D>;

    fn new(context: Self::Context, machine_node: Self::Subject) -> Self {
        let Ok(WalkingNodes {
            pivot,
            player,
            player_scene,
            animator,
        }) = Walking::get_nodes(context.clone(), machine_node.clone())
        else {
            godot_print!("Could not get walking nodes");
            panic!("wtf");
        };

        Walking {
            context,
            pivot,
            player,
            player_scene,
            animator,
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
        godot_print!("Entering Walking state...");
        self.next_state = Some(MovementStates::Walking);
    }

    fn input(&mut self, _event: Gd<InputEvent>) {}

    fn process(&mut self, _delta: f32) {}

    fn process_physics(&mut self, _delta: f32) {
        let input = Input::singleton();

        self.instant_velocity = Vector3::ZERO;

        // TODO: Move this to Jump state
        // self.apply_jump(&input, node, delta);

        self.apply_ground_movement(&input);
        self.player.set_velocity(self.instant_velocity);
        self.player.move_and_slide();
    }

    fn exit(&mut self) {
        godot_print!("Exiting Walking state");

        self.elapsed = 0.;
        self.set_next_state(MovementStates::Walking);
    }
}
