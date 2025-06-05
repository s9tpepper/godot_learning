use std::{cell::RefCell, rc::Rc};

use godot::{
    classes::{CollisionObject3D, Node, VBoxContainer},
    obj::{Base, Gd},
    prelude::{GodotClass, godot_api},
};
use thiserror::Error;

use crate::common::inventory::{Inventory, InventorySlot};

#[derive(Error, Debug)]
pub enum LootOptionListenerError {
    #[error("Error borrowing the inventory")]
    InventoryBorrow,
    #[error("Error adding item to inventory")]
    AddToInventory,
    #[error("Error borrowing selected loot option")]
    SelectedOptionBorrow,
    #[error("Error borrowing loot slots")]
    LootSlotsBorrow,
    #[error("Error borrowing looted item's slot")]
    LootSlotBorrow,
    #[error("Loot slots was missing an item")]
    MissingLootItem,
    #[error("Looted item was not found in loot slots, wtf?")]
    MissingLootedIndex,
    #[error("CollisionObject was None, it should not be missing")]
    CollisionObjectNone,
}

#[derive(Debug, GodotClass)]
#[class(init, base = Node)]
pub struct LootOptionListener {
    base: Base<Node>,
    pub slot: Rc<RefCell<InventorySlot>>,
    pub inventory: Rc<RefCell<Inventory>>,
    pub collision_object: Option<Gd<CollisionObject3D>>,
    pub option_selected: Rc<RefCell<bool>>,
    pub menu_container: Option<Gd<VBoxContainer>>,
    pub loot_slots: Rc<RefCell<Vec<Rc<RefCell<InventorySlot>>>>>,
}

#[godot_api]
impl LootOptionListener {
    #[signal]
    fn dummy();

    fn add_to_inventory(&mut self) -> Result<bool, LootOptionListenerError> {
        let mut inventory = self
            .inventory
            .try_borrow_mut()
            .map_err(|_| LootOptionListenerError::InventoryBorrow)?;

        let mut slot = self
            .slot
            .try_borrow_mut()
            .map_err(|_| LootOptionListenerError::LootSlotBorrow)?;

        inventory
            .add(&mut slot)
            .map_err(|_| LootOptionListenerError::AddToInventory)
    }

    pub fn handle_loot_option_click(&mut self) -> Result<(), LootOptionListenerError> {
        match self.add_to_inventory()? {
            false => {
                // TODO: fire off a signal or something,
                // the user's inventory is full
            }

            true => {
                let slot = self
                    .slot
                    .try_borrow()
                    .map_err(|_| LootOptionListenerError::LootSlotBorrow)?;

                if slot.item.is_some() {
                    let item = slot
                        .item
                        .as_ref()
                        .ok_or(LootOptionListenerError::MissingLootItem)?;
                    let uuid = item.get_uuid().to_string();

                    remove_item_uuid_from_menu(&uuid, self.loot_slots.clone())?;
                }

                // Check if all items have been looted
                let inventory_slots = self
                    .loot_slots
                    .try_borrow()
                    .map_err(|_| LootOptionListenerError::LootSlotsBorrow)?;

                if inventory_slots.is_empty() {
                    let collision_obj = self
                        .collision_object
                        .as_mut()
                        .ok_or(LootOptionListenerError::CollisionObjectNone)?;

                    if collision_obj.is_instance_valid() {
                        collision_obj.queue_free();
                    }
                }
            }
        }

        let mut selected = self
            .option_selected
            .try_borrow_mut()
            .map_err(|_| LootOptionListenerError::SelectedOptionBorrow)?;

        *selected = true;

        Ok(())
    }
}

fn remove_item_uuid_from_menu(
    uuid: &str,
    loot_slots: Rc<RefCell<Vec<Rc<RefCell<InventorySlot>>>>>,
) -> Result<(), LootOptionListenerError> {
    let mut slots = loot_slots
        .try_borrow_mut()
        .map_err(|_| LootOptionListenerError::LootSlotsBorrow)?;

    let mut item_index: Option<usize> = None;
    for (index, loot_slot) in slots.iter().enumerate() {
        let slot = loot_slot
            .try_borrow()
            .map_err(|_| LootOptionListenerError::LootSlotBorrow)?;

        let item = slot
            .item
            .as_ref()
            .ok_or(LootOptionListenerError::MissingLootItem)?;

        if item.get_uuid() == uuid {
            item_index = Some(index);
            break;
        }
    }

    let looted_index = item_index.ok_or(LootOptionListenerError::MissingLootedIndex)?;
    slots.remove(looted_index);

    Ok(())
}
