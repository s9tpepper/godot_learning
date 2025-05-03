use godot::{
    classes::{INode3D, InputEvent, InputEventMouseMotion, Node3D, input::MouseMode},
    obj::Gd,
    prelude::{GodotClass, godot_api},
};

use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node3D, init)]
#[allow(unused)]
struct Camera {
    #[export]
    pivot: Option<Gd<Node3D>>,

    // #[export(range=(0.01, 100.0))]
    // rotation_speed_x: f32,
    #[export(range=(0.001, 1.0))]
    rotation_speed: Vector2,

    #[export]
    min_y_angle: f32,
    #[export]
    max_y_angle: f32,

    #[export]
    min_x_angle: f32,
    #[export]
    max_x_angle: f32,

    accumulated_rotation: Vector2,
}

#[godot_api]
impl INode3D for Camera {
    // Called every frame.
    fn process(&mut self, _delta: f64) {}

    // Handle user input.
    fn input(&mut self, event: Gd<InputEvent>) {
        // #[allow(clippy::single_match)]
        // match event.try_cast::<InputEventMouse>() {
        //     Ok(event) => {
        //     }
        //     _ => {}
        // }

        let mut input = Input::singleton();
        input.set_mouse_mode(MouseMode::CAPTURED);

        #[allow(clippy::single_match)]
        match event.try_cast::<InputEventMouseMotion>() {
            Ok(event) => {
                let Some(pivot) = &mut self.pivot else {
                    return;
                };

                let relative = event.get_relative();
                self.accumulated_rotation += relative * self.rotation_speed;

                pivot.set_basis(Basis::default());

                // let y = self
                //     .accumulated_rotation
                //     .x
                //     .clamp(self.min_y_angle, self.max_y_angle);

                let y = self.accumulated_rotation.x;
                let x = self
                    .accumulated_rotation
                    .y
                    .clamp(self.min_x_angle, self.max_x_angle);

                pivot.rotate_object_local(Vector3::UP, y);
                pivot.rotate_object_local(Vector3::RIGHT, x);
            }

            _ => {}
        }
    }
}
