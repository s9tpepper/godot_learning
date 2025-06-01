use std::{cell::RefCell, rc::Rc, str::FromStr};

use godot::{
    builtin::{NodePath, Vector3},
    classes::{CollisionObject3D, INode3D, Node, Node3D, PackedScene},
    obj::{Base, Gd, NewAlloc, WithBaseField},
    prelude::{GodotClass, godot_api},
    tools::load,
};

use crate::{
    common::{
        inventory::{Inventory, InventorySlot},
        states::lootable::{LootContext, LootMachine},
    },
    items::test_item::TestItem,
    npc::test_npc::TestNpc,
    player::Player3D,
    states::movement::MovementMachine,
};

#[derive(GodotClass)]
#[class(base=Node3D, init)]
struct Shell {
    base: Base<Node3D>,
    level: Option<Gd<Node>>,
    inventory: Option<Rc<RefCell<Inventory>>>,
}

#[godot_api]
impl INode3D for Shell {
    fn ready(&mut self) {
        // NOTE: This will move eventually to some kind of top level systems
        // manager of some kind

        self.test_inventory();
        self.load_test_level();
        self.setup_player();
        self.add_test_npc();
    }
}

impl Shell {
    fn test_inventory(&mut self) {
        let inventory = Inventory::new();
        let inventory_rc = Rc::new(RefCell::new(inventory));
        self.inventory = Some(inventory_rc.clone());

        // TODO: Later, we can load inventory from some persisted data
        // self.inventory.load(); <--  Something like this
    }

    fn add_to_scene(&mut self, node: Gd<Node>) {
        #[allow(clippy::option_map_unit_fn)]
        self.base_mut().add_child(&node);

        // self.base_mut(
        //     .get_tree()
        //     .and_then(|tree| tree.get_root())
        //     .map(|mut root| {
        //         root.call_deferred("add_child", &[node.to_variant()]);
        //     });
    }

    fn load_test_level(&mut self) {
        let level = load::<PackedScene>("res://scenes/level.tscn")
            .instantiate()
            .unwrap();

        self.add_to_scene(level.clone());

        self.level = Some(level);
    }

    fn setup_player(&mut self) {
        let player = load::<PackedScene>("res://scenes/player/player.tscn")
            .instantiate()
            .unwrap();

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
        movement_machine
            .clone()
            .bind_mut()
            .start(context, player.clone());

        self.add_to_scene(player);
    }

    fn add_test_npc(&mut self) {
        let test_npc = load::<PackedScene>("res://test_npc.tscn");
        for i in 1..=2 {
            let test_npc_scene = test_npc.instantiate().unwrap();
            let mut test_npc = test_npc_scene.clone().try_cast::<TestNpc>().unwrap();
            test_npc.set_position(Vector3::UP * i as f32 * 10.);
            self.base_mut().add_child(&test_npc);

            self.add_loot_item_to_npc(test_npc_scene.clone());
        }
    }

    fn add_loot_item_to_npc(&mut self, npc: Gd<Node>) {
        let test_npc_collider = npc.clone().try_cast::<CollisionObject3D>().unwrap();

        let test_item = TestItem::new();
        let slot = InventorySlot::new(Some(Box::new(test_item)), 2);

        let test_item2 = TestItem::new();
        let slot2 = InventorySlot::new(Some(Box::new(test_item2)), 8);

        let loot_context = LootContext::new(
            Rc::new(RefCell::new(vec![
                Rc::new(RefCell::new(slot)),
                Rc::new(RefCell::new(slot2)),
            ])),
            self.inventory.clone().expect("inventory").clone(),
            test_npc_collider.clone(),
        );

        let loot_context_rc = Rc::new(RefCell::new(loot_context));

        // TODO: Re-do start() for LootMachine
        let mut item_loot_machine = LootMachine::new_alloc();
        item_loot_machine.bind_mut().start(loot_context_rc);
        self.base_mut().add_child(&item_loot_machine);
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
