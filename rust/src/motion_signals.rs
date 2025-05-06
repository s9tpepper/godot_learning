use godot::{
    classes::{INode3D, Node3D},
    obj::Base,
    prelude::{GodotClass, godot_api},
};

#[derive(GodotClass)]
#[class(base=Node3D, init)]
pub struct MotionSignals {
    base: Base<Node3D>,
}

#[godot_api]
impl MotionSignals {
    #[signal]
    pub fn walking(is_walking: bool);
}

#[godot_api]
impl INode3D for MotionSignals {
    fn ready(&mut self) {}
}
