use std::{cell::RefCell, rc::Rc, str::FromStr};

use godot::{
    builtin::{NodePath, Vector3},
    classes::{
        CollisionObject3D, INode3D, IRigidBody3D, InputEvent, Node, Node3D, PackedScene,
        RigidBody3D,
    },
    global::godot_print,
    meta::ToGodot,
    obj::{Base, Gd, NewAlloc, WithBaseField},
    prelude::{GodotClass, godot_api},
    tools::load,
};

use crate::{
    common::{
        inventory::{Inventory, InventoryItem, InventorySlot, ItemCategory},
        states::lootable::{LootContext, LootMachine},
    },
    player::Player3D,
    states::movement::MovementMachine,
};

#[derive(GodotClass)]
#[class(base=Node3D, init)]
struct Shell {
    base: Base<Node3D>,
    level: Option<Gd<Node>>,
    inventory: Option<Rc<RefCell<Inventory>>>,
    test_loot_machines: Vec<LootMachine>,
}

#[derive(Debug, GodotClass)]
#[class(init, base = RigidBody3D)]
pub struct TestItem {
    #[base]
    base: Base<RigidBody3D>,
}

#[godot_api]
impl IRigidBody3D for TestItem {
    fn ready(&mut self) {}

    fn input(&mut self, _event: Gd<InputEvent>) {}
}

impl InventoryItem for Gd<TestItem> {
    fn get_name(&self) -> String {
        format!("TestItem {}", self.instance_id())
    }

    fn get_category(&self) -> ItemCategory {
        ItemCategory::Food
    }

    fn get_max_stack_size(&self) -> i32 {
        10
    }
}

#[godot_api]
impl INode3D for Shell {
    fn ready(&mut self) {
        // NOTE: This will move eventually to some kind of top level systems
        // manager of some kind

        let inventory = Inventory::new();
        // let test_item = TestItem {};
        // let mut slot = InventorySlot::new(Some(Box::new(test_item)), 5);
        // inventory.add(&mut slot);

        // inventory.add(&mut slot);

        let inventory_rc = Rc::new(RefCell::new(inventory));

        self.test_loot_machines = vec![];

        self.inventory = Some(inventory_rc.clone());

        // self.inventory = Inventory::new();

        // TODO: Later, we can load inventory from some persisted data
        // self.inventory.load(); <-- Something like this

        let mut level = load::<PackedScene>("res://scenes/level.tscn")
            .instantiate()
            .unwrap();

        let player = load::<PackedScene>("res://scenes/player/player.tscn")
            .instantiate()
            .unwrap();
        godot_print!("[shell.rs] player: {player:?}");

        #[allow(clippy::option_map_unit_fn)]
        self.base_mut()
            .get_tree()
            .and_then(|tree| tree.get_root())
            .map(|mut root| {
                level.call_deferred("add_child", &[player.to_variant()]);
                root.call_deferred("add_child", &[level.to_variant()]);
            });

        self.level = Some(level);

        let player3d = player
            .clone()
            .try_cast::<Player3D>()
            .expect("Player3D should exist");

        // NOTE: Move this machine creation to some State Machine manager level
        let movement_machine = self.base_mut().get_node_as::<MovementMachine>(
            &NodePath::from_str("MovementMachine").expect("node path"),
        );
        let context = player3d
            .bind()
            .get_context()
            .expect("context to exist")
            .clone();
        movement_machine.clone().bind_mut().start(context, player);

        godot_print!("Movement machine: {movement_machine}");

        // Add spheres for testing
        let test_sphere = load::<PackedScene>("res://test_sphere.tscn");
        for i in 1..=2 {
            let sphere = test_sphere.instantiate().unwrap();
            let mut sphere = sphere.clone().try_cast::<CollisionObject3D>().unwrap();
            let inventory_item = sphere.clone().try_cast::<TestItem>().unwrap();

            sphere.set_position(Vector3::UP * i as f32 * 10.);
            self.level.clone().expect("xx").add_child(&sphere.clone());

            godot_print!("Creating inventory slot...");
            let slot = InventorySlot::new(Some(Box::new(inventory_item.clone())), 13);
            let loot_context_rc = Rc::new(RefCell::new(LootContext::new(
                slot,
                inventory_rc.clone(),
                sphere.clone(),
            )));

            // TODO: Re-do start() for LootMachine
            let mut item_loot_machine = LootMachine::new_alloc();
            item_loot_machine.bind_mut().start(loot_context_rc);
            self.base_mut().add_child(&item_loot_machine);

            godot_print!("Finished creating inventory slot.");
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
