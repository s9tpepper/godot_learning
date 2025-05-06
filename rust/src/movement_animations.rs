use godot::{
    builtin::{NodePath, StringName},
    classes::{AnimationPlayer, INode3D, Node3D},
    global::godot_print,
    obj::{Base, Gd},
    prelude::{GodotClass, godot_api},
};

use crate::motion_signals::MotionSignals;

#[derive(GodotClass)]
#[class(base=Node3D, init)]
pub struct MovementAnimations {
    base: Base<Node3D>,

    #[export]
    target_node: Option<Gd<Node3D>>,

    #[export]
    animation_player_path: StringName,

    #[export]
    walking_animation_name: StringName,

    #[export]
    motion_signals: Option<Gd<MotionSignals>>,
}

#[godot_api]
impl MovementAnimations {
    fn walking(&mut self) {
        if let Some(mut animation_player) = self.get_animation_player() {
            animation_player
                .play_ex()
                .name(&self.walking_animation_name)
                .done();
        }
    }

    fn idle(&mut self) {
        if let Some(mut animation_player) = self.get_animation_player() {
            animation_player.stop();
        }
    }

    fn get_animation_player(&mut self) -> Option<Gd<AnimationPlayer>> {
        let Some(target_node) = &mut self.target_node else {
            godot_print!("No target node found");
            return None;
        };

        // TODO: Figure out how to get rid of this clone()
        let node_path: NodePath = self.animation_player_path.clone().into();
        if !target_node.has_node(&node_path) {
            godot_print!("Could not find animation player instance");
            return None;
        }

        if self.walking_animation_name.is_empty() {
            godot_print!("No walking animation name detected");
            return None;
        }

        Some(target_node.get_node_as::<AnimationPlayer>(&node_path))
    }
}

#[godot_api]
impl INode3D for MovementAnimations {
    fn ready(&mut self) {
        let Some(mut motion_signals) = self.get_motion_signals() else {
            return;
        };

        motion_signals
            .signals()
            .walking()
            .connect_obj(self, |s: &mut Self, is_walking: bool| {
                if is_walking {
                    s.walking();
                } else {
                    s.idle();
                }
            });
    }
}
