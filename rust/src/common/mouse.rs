use std::sync::LazyLock;

use godot::{
    classes::{INode3D, Input, InputEvent, input::MouseMode},
    obj::Gd,
    prelude::{GodotClass, godot_api},
};

use crate::actions::Actions;

static ACTIONS: LazyLock<Actions> = LazyLock::new(Actions::default);

#[derive(GodotClass)]
#[class(base=Node3D, init)]
#[allow(unused)]
/// Controls mouse going from Confined to Captured state
struct GameMouse {}

#[godot_api]
impl INode3D for GameMouse {
    fn ready(&mut self) {
        let mut input = Input::singleton();
        input.set_mouse_mode(MouseMode::CAPTURED);
    }

    fn input(&mut self, _event: Gd<InputEvent>) {
        let mut input = Input::singleton();
        if input.is_action_just_pressed(ACTIONS.mouse_mode) {
            if input.get_mouse_mode() == MouseMode::CAPTURED {
                input.set_mouse_mode(MouseMode::VISIBLE);
            } else {
                input.set_mouse_mode(MouseMode::CAPTURED);
            }
        }
    }
}
