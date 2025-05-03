use std::sync::LazyLock;

use godot::classes::{CharacterBody3D, ProjectSettings};
use godot::prelude::*;

use crate::actions::Actions;

const DEFAULT_GRAVITY: f32 = 9.8;
const DEFAULT_GRAVITY_VECTOR: Vector3 = Vector3::new(0., -1., 0.);
const GRAVITY_VECTOR_SETTINGS_PATH: &str = "physics/3d/default_gravity_vector";
const GRAVITY_SETTINGS_PATH: &str = "physics/3d/default_gravity";

static ACTIONS: LazyLock<Actions> = LazyLock::new(Actions::default);

#[derive(GodotClass)]
#[class(base=Node3D, init)]
#[allow(unused)]
struct Movement {
    current_velocity: Vector3,
    base: Base<Node3D>,

    #[export]
    target: Option<Gd<CharacterBody3D>>,

    #[export(range=(0.01, 400.0))]
    movement_speed: f32,

    #[export(range=(1., 100.))]
    jump_height: f32,

    #[export(range=(1., 200.))]
    fall_speed: f32,

    #[export(range=(1., 200.))]
    jump_force: f32,

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
    fn apply_ground_movement(&mut self, input: &Gd<Input>) {
        if input.is_action_pressed(ACTIONS.forward) {
            self.current_velocity += Vector3::FORWARD;
        }

        if input.is_action_pressed(ACTIONS.left) {
            self.current_velocity += Vector3::LEFT;
        }

        if input.is_action_pressed(ACTIONS.right) {
            self.current_velocity += Vector3::RIGHT;
        }

        if input.is_action_pressed(ACTIONS.backward) {
            self.current_velocity += Vector3::BACK;
        }
    }

    fn apply_jump(&mut self, input: &Gd<Input>, node: &mut Gd<CharacterBody3D>, delta: f64) {
        let settings = ProjectSettings::singleton();
        let g = settings.get_setting(GRAVITY_VECTOR_SETTINGS_PATH);
        let gravity_vector: Vector3 = g.try_to().unwrap_or(DEFAULT_GRAVITY_VECTOR);

        let g = settings.get_setting(GRAVITY_SETTINGS_PATH);
        let gravity: f32 = g.try_to().unwrap_or(DEFAULT_GRAVITY);

        if input.is_action_just_pressed(ACTIONS.jump) && node.is_on_floor() {
            let jump_impulse = (gravity_vector * gravity * self.jump_force) * -1.;

            self.current_velocity.y = jump_impulse.y * delta as f32;
            self.jump_position = node.get_transform().origin.y;
            self.jumping = true;
            self.target_jump_height = self.jump_position + self.jump_height;
        } else if self.jumping && self.jump_position < self.target_jump_height {
            let jump_impulse = (gravity_vector * gravity * self.jump_force) * -1.;

            self.current_velocity.y = jump_impulse.y * delta as f32;
            self.jump_position = node.get_transform().origin.y;

            if self.jump_position >= self.target_jump_height {
                self.jumping = false;
            }
        }

        if !node.is_on_floor() && !self.jumping {
            self.current_velocity.y -= self.fall_speed * gravity * delta as f32;
            self.jump_position = node.get_transform().origin.y;
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

        self.current_velocity = Vector3::ZERO;

        self.apply_ground_movement(&input);
        self.current_velocity *= self.movement_speed;

        self.apply_jump(&input, node, delta);

        node.set_velocity(self.current_velocity);
        node.move_and_slide();
    }
}
