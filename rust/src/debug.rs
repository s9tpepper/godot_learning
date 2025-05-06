use godot::{
    classes::{ILabel, Input, Label},
    obj::{Base, EngineEnum, WithBaseField},
    prelude::{GodotClass, godot_api},
};

#[derive(GodotClass)]
#[class(base=Label, init)]
pub struct Debug {
    base: Base<Label>,
}

#[godot_api]
impl Debug {}

#[godot_api]
impl ILabel for Debug {
    // Called every frame.
    fn process(&mut self, _delta: f64) {
        let input = Input::singleton();
        let mode = input.get_mouse_mode();
        self.base_mut().set_text(mode.as_str());
    }
}
