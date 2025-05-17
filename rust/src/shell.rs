//uid://bbynqotbuicfn // test_scene.tscn
use godot::{
    classes::{INode3D, Node3D, PackedScene},
    global::godot_print,
    meta::ToGodot,
    obj::{Base, WithBaseField},
    prelude::{GodotClass, godot_api},
    tools::load,
};

#[derive(GodotClass)]
#[class(base=Node3D, init)]
struct Shell {
    base: Base<Node3D>,
}

#[godot_api]
impl INode3D for Shell {
    fn ready(&mut self) {
        let scene = load::<PackedScene>("res://player.tscn");

        self.base_mut()
            .get_tree()
            .and_then(|tree| tree.get_root())
            .and_then(|mut root| {
                scene
                    .instantiate()
                    .map(|player_node| root.call_deferred("add_child", &[player_node.to_variant()]))
            });

        godot_print!("Finish start up");
    }
}
// Treated as an enum with two values: "One" and "Two"
// Displayed in the editor
// Treated as read-only by the editor
// #[var(
//     usage_flags = [EDITOR, GROUP]
// )]
// my_group_of_things: i8,
//
// #[export]
// my_export: i32,
//
// #[export]
// my_other_thing: i32,
//
// #[var(
//     usage_flags = [EDITOR, GROUP]
// )]
// SecondGroup: i8,
// #[export]
// my_other_thingie: i32,
// #[export(flags_3d_navigation)]
// collision_layers: i16,
