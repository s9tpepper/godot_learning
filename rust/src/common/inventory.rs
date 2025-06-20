use std::{cmp::min, error::Error};

use godot::global::godot_print;

#[allow(unused)]
#[derive(PartialEq, Eq)]
pub enum ItemCategory {
    Weapon,
    Armor,
    Clothes,
    Food,
    Medicine,
    Loot,
    Powerups,
    Resources,
}

#[derive(Default, Debug)]
pub struct Inventory {
    max_slots: usize,
    slots: Vec<InventorySlot>,
}

#[derive(Default, Debug)]
pub struct InventorySlot {
    pub item: Option<Box<dyn InventoryItem>>,
    pub count: i32,
}

impl InventorySlot {
    pub fn new(item: Option<Box<dyn InventoryItem>>, count: i32) -> Self {
        InventorySlot { item, count }
    }
}

fn add_item_to_slot<'a>(new_slot: &'a mut InventorySlot, slot: &'a mut InventorySlot) {
    godot_print!("add_item_to_slot()");

    let max_stack_size = new_slot
        .item
        .as_ref()
        .expect("Expect slot to always have item")
        .get_max_stack_size();
    godot_print!("max_stack_size: {max_stack_size}");

    let fit_in_slot = min(max_stack_size - slot.count, new_slot.count);
    godot_print!("fit in slot: {fit_in_slot}");

    let remainder = new_slot.count - fit_in_slot;
    godot_print!("remainder: {remainder}");

    slot.count += fit_in_slot;

    godot_print!(
        "Update new_slot.count ({}) to remainder({})",
        new_slot.count,
        remainder
    );

    new_slot.count = remainder;
}

impl Inventory {
    pub fn new() -> Self {
        let max_slots = 50;

        let mut slots = vec![];
        for _ in 0..max_slots {
            slots.push(InventorySlot::default());
        }

        Inventory { max_slots, slots }
    }

    // TODO: Fix unwrap()s and Box dyn Error
    pub fn add(&mut self, new_item: &mut InventorySlot) -> Result<bool, Box<dyn Error>> {
        let item_type = new_item.item.as_ref().expect("item").get_name().clone();
        godot_print!("item_type: {item_type}");

        let mut item_slots: Vec<&mut InventorySlot> = self
            .slots
            .iter_mut()
            .filter(|slot| {
                slot.item.is_some() && slot.item.as_ref().unwrap().get_name() == item_type
            })
            .collect();

        godot_print!("item_slots: {item_slots:?}");

        item_slots.sort_by(|a, b| a.count.cmp(&b.count));

        for slot in item_slots.iter_mut() {
            add_item_to_slot(new_item, slot);

            if new_item.count == 0 {
                break;
            }
        }

        if new_item.count == 0 {
            return Ok(true);
        }

        let mut empty_item_slots: Vec<&mut InventorySlot> = self
            .slots
            .iter_mut()
            .filter(|slot| slot.item.is_none())
            .collect();

        for empty_slot in empty_item_slots.iter_mut() {
            godot_print!("Filling an empty slot");

            let item_clone: Box<dyn InventoryItem> = new_item.item.as_ref().unwrap().get_boxed();
            godot_print!("got boxed clone");

            empty_slot.item = Some(item_clone);
            godot_print!("updated empty slot");

            add_item_to_slot(new_item, empty_slot);

            if new_item.count == 0 {
                break;
            }
        }

        if new_item.count > 0 {
            return Ok(false);
        }

        Ok(true)
    }

    // TODO: Implement remove for when you destroy or use an item
    pub fn remove(&mut self, _item: impl InventoryItem) {}
}

pub trait InventoryItem: std::fmt::Debug {
    fn get_name(&self) -> String;
    fn get_category(&self) -> ItemCategory;
    fn get_max_stack_size(&self) -> i32;
    // TODO: Fix this to be what it should be to retrieve a texture
    fn get_icon(&self) -> String;
    fn get_boxed(&self) -> Box<dyn InventoryItem>;
    fn get_uuid(&self) -> &str;
}
