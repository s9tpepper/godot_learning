use godot::{
    builtin::{GString, NodePath},
    prelude::GodotClass,
};

#[derive(Default, Debug, GodotClass)]
#[class(base=Resource, init)]
pub struct MovementContext {
    #[export]
    pub player: NodePath,

    #[export]
    pub player_scene: NodePath,

    #[export]
    pub pivot: NodePath,

    #[export]
    pub camera: NodePath,

    #[export]
    pub animation_player: GString,

    #[export]
    pub walking_animation_name: GString,

    #[export(range=(0.01, 400.0))]
    pub movement_speed: f32,

    #[export]
    /// Points to AudioStreamPlayer3D to play a footstep sound
    pub footstep: NodePath,
}
