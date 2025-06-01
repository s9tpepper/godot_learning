use godot::{
    classes::{IRigidBody3D, InputEvent, PanelContainer, RigidBody3D},
    obj::{Base, Gd},
    prelude::{GodotClass, godot_api},
};

#[derive(Debug, GodotClass)]
#[class(init, base = RigidBody3D)]
pub struct TestNpc {
    #[base]
    base: Base<RigidBody3D>,

    #[export]
    loot_options: Option<Gd<PanelContainer>>,
}

#[godot_api]
impl IRigidBody3D for TestNpc {
    fn ready(&mut self) {}

    fn input(&mut self, _event: Gd<InputEvent>) {}
}
