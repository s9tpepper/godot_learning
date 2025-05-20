// use godot::{
//     classes::{ICollisionObject3D, InputEvent},
//     obj::Gd,
//     prelude::{GodotClass, godot_api},
// };
//
// #[derive(GodotClass)]
// #[class(base=CollisionObject3D, init)]
// #[allow(unused)]
// struct ClickableObject {}
//
// #[godot_api]
// impl ICollisionObject3D for ClickableObject {
//     // Called when the node is ready in the scene tree.
//     fn ready(&mut self) {
//         // self.input_event(camera, event, event_position, normal, shape_idx);
//     }
//
//     // Called every frame.
//     fn process(&mut self, _delta: f64) {}
//
//     // Called every physics frame.
//     fn physics_process(&mut self, _delta: f64) {}
//
//     // Handle user input.
//     fn input(&mut self, _event: Gd<InputEvent>) {}
// }
