use godot::{
    builtin::{GString, NodePath},
    classes::Node,
    obj::Gd,
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

    scene_tree: Option<Gd<Node>>,
}

impl MovementContext {
    pub fn set_scene_tree(&mut self, scene_tree: Gd<Node>) {
        self.scene_tree = Some(scene_tree);
    }

    pub fn get_scene_tree(&self) -> Option<Gd<Node>> {
        self.scene_tree.clone()
    }
}
