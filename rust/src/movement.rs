use std::sync::LazyLock;

use godot::classes::{AnimationPlayer, CharacterBody3D, Input, ProjectSettings};
use godot::prelude::*;

use crate::actions::Actions;
use crate::motion_signals::MotionSignals;

const DEFAULT_GRAVITY: f32 = 9.8;
const DEFAULT_GRAVITY_VECTOR: Vector3 = Vector3::new(0., -1., 0.);
const GRAVITY_VECTOR_SETTINGS_PATH: &str = "physics/3d/default_gravity_vector";
const GRAVITY_SETTINGS_PATH: &str = "physics/3d/default_gravity";

static ACTIONS: LazyLock<Actions> = LazyLock::new(Actions::default);

#[derive(GodotClass)]
#[class(base=Node3D, init)]
#[allow(unused)]
struct Movement {
    instant_velocity: Vector3,
    base: Base<Node3D>,

    #[export]
    pivot: Option<Gd<Node3D>>,

    #[export]
    target: Option<Gd<CharacterBody3D>>,

    #[export]
    target_node: Option<Gd<Node3D>>,

    #[export]
    debug_ball: Option<Gd<Node3D>>,

    #[export]
    animation_player_path: StringName,

    #[export]
    walking_animation_name: StringName,

    #[export(range=(0.01, 400.0))]
    movement_speed: f32,

    #[export(range=(1., 100.))]
    jump_height: f32,

    #[export(range=(1., 200.))]
    fall_speed: f32,

    #[export(range=(1., 200.))]
    jump_force: f32,

    #[export]
    motion_signals: Option<Gd<MotionSignals>>,

    // Whether the player is currently jumping
    jumping: bool,

    // The current y position while jumping.
    jump_position: f32,

    // The target jump height starting from where the player
    // initiates the jump. The target jump height is different
    // if the player jumps while standing on the ground as opposed
    // to starting a jump while standing on a box or obstacle.
    target_jump_height: f32,
}

impl Movement {
    fn apply_jump(&mut self, input: &Gd<Input>, node: &mut Gd<CharacterBody3D>, delta: f64) {
        let settings = ProjectSettings::singleton();
        let g = settings.get_setting(GRAVITY_VECTOR_SETTINGS_PATH);
        let gravity_vector: Vector3 = g.try_to().unwrap_or(DEFAULT_GRAVITY_VECTOR);

        let g = settings.get_setting(GRAVITY_SETTINGS_PATH);
        let gravity: f32 = g.try_to().unwrap_or(DEFAULT_GRAVITY);

        let jump_impulse = (gravity_vector * gravity * self.jump_force) * -1.;

        if input.is_action_just_pressed(ACTIONS.jump) && node.is_on_floor() {
            self.instant_velocity.y = jump_impulse.y * delta as f32;
            self.jump_position = node.get_transform().origin.y;
            self.jumping = true;
            self.target_jump_height = self.jump_position + self.jump_height;
        } else if self.jumping && self.jump_position < self.target_jump_height {
            self.instant_velocity.y = jump_impulse.y * delta as f32;
            self.jump_position = node.get_transform().origin.y;
        }

        if self.jump_position >= self.target_jump_height {
            self.jumping = false;
        }

        if !node.is_on_floor() && !self.jumping {
            self.instant_velocity.y -= self.fall_speed * gravity * delta as f32;
            self.jump_position = node.get_transform().origin.y;
        }
    }

    fn _apply_animations(&mut self) {
        let Some(target_node) = &mut self.target_node else {
            return;
        };

        // TODO: Figure out how to get rid of this clone()
        let node_path: NodePath = self.animation_player_path.clone().into();
        if !target_node.has_node(&node_path) {
            return;
        }

        if self.walking_animation_name.is_empty() {
            return;
        }

        let mut animation_player = target_node.get_node_as::<AnimationPlayer>(&node_path);
        if self.instant_velocity != Vector3::ZERO {
            animation_player
                .play_ex()
                .name(&self.walking_animation_name)
                .done();
        } else {
            animation_player.stop();
        }
    }
}

#[godot_api]
impl INode3D for Movement {
    // Called every frame.
    fn physics_process(&mut self, delta: f64) {
        let input = Input::singleton();

        let Some(ref mut node) = self.get_target() else {
            return;
        };

        self.instant_velocity = Vector3::ZERO;

        // self.apply_ground_movement(&input, delta);
        self.apply_jump(&input, node, delta);
        // self.apply_animations();

        node.set_velocity(self.instant_velocity);
        node.move_and_slide();
    }
}
