//uid://bbynqotbuicfn // test_scene.tscn
use godot::{
    builtin::Vector3,
    classes::{INode3D, Node, Node3D, PackedScene},
    meta::ToGodot,
    obj::{Base, Gd, WithBaseField},
    prelude::{GodotClass, godot_api},
    tools::load,
};

#[derive(GodotClass)]
#[class(base=Node3D, init)]
struct Shell {
    base: Base<Node3D>,
    level: Option<Gd<Node>>,
}

#[godot_api]
impl INode3D for Shell {
    fn ready(&mut self) {
        let mut level = load::<PackedScene>("res://scenes/level.tscn")
            .instantiate()
            .unwrap();

        let player = load::<PackedScene>("res://scenes/player/player.tscn")
            .instantiate()
            .unwrap();

        #[allow(clippy::option_map_unit_fn)]
        self.base_mut()
            .get_tree()
            .and_then(|tree| tree.get_root())
            .map(|mut root| {
                level.call_deferred("add_child", &[player.to_variant()]);
                root.call_deferred("add_child", &[level.to_variant()]);
            });

        self.level = Some(level);

        // Add spheres for testing
        let test_sphere = load::<PackedScene>("res://test_sphere.tscn");
        for i in 1..10 {
            let sphere = test_sphere.instantiate().unwrap();
            let mut sphere: Gd<Node3D> = sphere.try_cast().unwrap();

            sphere.set_position(Vector3::UP * i as f32 * 10.);
            self.level.clone().expect("xx").add_child(&sphere);
        }
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
