use godot::{
    classes::{INode3D, Input, InputEvent, InputEventMouseMotion, Node3D, input::MouseMode},
    obj::Gd,
    prelude::{GodotClass, godot_api},
};

use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node3D, init)]
#[allow(unused)]
struct Camera {
    base: Base<Node3D>,

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
    // Handle user input.
    fn input(&mut self, event: Gd<InputEvent>) {
        let input = Input::singleton();
        if input.get_mouse_mode() == MouseMode::CONFINED {
            return;
        }

        // input.set_mouse_mode(MouseMode::CAPTURED);

        #[allow(clippy::single_match)]
        match event.try_cast::<InputEventMouseMotion>() {
            Ok(event) => {
                let relative = event.get_relative();
                // godot_print!("relative: {relative}");

                self.accumulated_rotation += relative * self.rotation_speed;

                self.base_mut().set_basis(Basis::default());

                // let y = self
                //     .accumulated_rotation
                //     .x
                //     .clamp(self.min_y_angle, self.max_y_angle);

                let y = self.accumulated_rotation.x;
                let x = self
                    .accumulated_rotation
                    .y
                    .clamp(self.min_x_angle, self.max_x_angle);

                // godot_print!(
                //     "setting rotation: x: {x}, y: {y}, accumulated_rotation: {}",
                //     self.accumulated_rotation
                // );

                self.base_mut().rotate_object_local(Vector3::UP, y);
                self.base_mut().rotate_object_local(Vector3::LEFT, x);
            }

            _ => {}
        }
    }
}
