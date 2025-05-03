use godot::{
    classes::{Camera3D, INode3D},
    obj::Gd,
    prelude::{GodotClass, godot_api},
};

#[derive(GodotClass)]
#[class(base=Node3D, init)]
#[allow(unused)]
struct Camera {
    #[export]
    camera: Option<Gd<Camera3D>>,
}

#[godot_api]
impl INode3D for Camera {
    // Called when the node is ready in the scene tree.
    fn ready(&mut self) {}

    // Called every frame.
    fn process(&mut self, _delta: f64) {}

    // Called every physics frame.
    fn physics_process(&mut self, _delta: f64) {}

    // String representation of the object.
    fn to_string(&self) -> GString {}

    // Handle user input.
    fn input(&mut self, _event: Gd<InputEvent>) {}

    // Handle lifecycle notifications.
    fn on_notification(&mut self, _what: Node3DNotification) {}
}
