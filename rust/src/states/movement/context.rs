use godot::{
    builtin::{GString, NodePath},
    classes::{AnimationPlayer, CharacterBody3D, Node, Node3D},
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

    pub scene_tree: Option<Gd<Node>>,
    pub pivot_node: Option<Gd<Node3D>>,
    pub player_node: Option<Gd<CharacterBody3D>>,
    pub player_scene_node: Option<Gd<Node3D>>,
    pub animator: Option<Gd<AnimationPlayer>>,
}

impl MovementContext {
    pub fn set_scene_tree(&mut self, scene_tree: Gd<Node>) {
        self.scene_tree = Some(scene_tree);
    }

    pub fn get_scene_tree(&self) -> Option<Gd<Node>> {
        self.scene_tree.clone()
    }

    pub fn get_node<T: Clone>(&self, option: Option<T>) -> T {
        option.clone().expect(" exist")
    }
}
