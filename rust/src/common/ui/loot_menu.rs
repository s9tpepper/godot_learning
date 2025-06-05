use std::{
    cell::{Ref, RefCell},
    rc::Rc,
};

use godot::{
    builtin::Color,
    classes::{
        CollisionObject3D, IPanelContainer, InputEvent, PackedScene, PanelContainer, VBoxContainer,
        control::SizeFlags,
    },
    global::{godot_error, godot_print},
    obj::{Base, NewAlloc, WithBaseField, WithUserSignals},
    prelude::{Gd, GodotClass, IoError, godot_api},
    tools::try_load,
};
use thiserror::Error;

use crate::{
    common::inventory::{Inventory, InventorySlot},
    items::loot_all::LootAll,
};

use super::{
    loot_option::{LootOption, LootOptionError},
    loot_option_listener::LootOptionListener,
    utils::is_inbounds,
};

const LOOT_OPTION_SCENE: &str = "res://ui/loot_option.tscn";

#[derive(Error, Debug)]
pub enum LootMenuError {
    #[error("The loot option scene could not be loaded")]
    OptionSceneLoad(#[from] IoError),
    #[error("The loot option scene could not be instantiated")]
    OptionSceneInstantiate,
    #[error("The loot slot could not be borrowed")]
    SlotBorrow,
    #[error("The loot slots could not be borrowed")]
    SlotsBorrow,
    #[error("Error casting Gd<Node> to LootOption")]
    OptionCast,
    #[error("Error setting item on loot option")]
    SetItem(#[from] LootOptionError),
}

#[derive(Debug, GodotClass)]
#[class(init, base = PanelContainer)]
pub struct LootMenu {
    #[base]
    base: Base<PanelContainer>,

    pub mouse_hovering: Rc<RefCell<bool>>,
    pub option_selected: Rc<RefCell<bool>>,
    pub menu_container: Option<Gd<VBoxContainer>>,
    pub inventory_slots: Rc<RefCell<Vec<Rc<RefCell<InventorySlot>>>>>,
}

#[godot_api]
impl IPanelContainer for LootMenu {
    fn input(&mut self, event: Gd<InputEvent>) {
        let global_rect = self.base().get_global_rect();
        let is_hovering = is_inbounds(global_rect, event);

        self.update_hovering(is_hovering);
    }

    fn process(&mut self, _delta: f64) {
        let empty = self.base().get_children().len() == 1;

        let trigger = self.option_selected.clone();
        let mut option_clicked = self.signals().option_clicked();

        // TODO: Clean these ifs up
        let selected_borrow = trigger.try_borrow_mut();
        if let Ok(mut selected) = selected_borrow {
            if *selected && empty {
                option_clicked.emit();
                self.base_mut().queue_free();
            } else {
                *selected = false;
            }
        }
    }
}

#[godot_api]
impl LootMenu {
    #[signal]
    pub fn option_clicked();

    pub fn len(&self) -> usize {
        match &self.menu_container {
            Some(container) => container.get_children().len(),
            None => 0,
        }
    }

    fn get_listener(
        &self,
        inventory: Rc<RefCell<Inventory>>,
        slot_ref: Rc<RefCell<InventorySlot>>,
        collision_object: Gd<CollisionObject3D>,
    ) -> Gd<LootOptionListener> {
        let mut listener = LootOptionListener::new_alloc();
        listener.bind_mut().inventory = inventory.clone();
        listener.bind_mut().slot = slot_ref.clone();
        listener.bind_mut().collision_object = Some(collision_object.clone());
        listener.bind_mut().option_selected = self.option_selected.clone();
        listener.bind_mut().menu_container = self.menu_container.clone();
        listener.bind_mut().loot_slots = self.inventory_slots.clone();

        listener
    }

    fn add_option_click_listener(
        &self,
        inventory: Rc<RefCell<Inventory>>,
        slot_ref: Rc<RefCell<InventorySlot>>,
        collision_object: Gd<CollisionObject3D>,
        mut loot_option: Gd<LootOption>,
    ) {
        let listener = self.get_listener(
            inventory.clone(),
            slot_ref.clone(),
            collision_object.clone(),
        );

        if self.menu_container.is_none() {
            return;
        }

        loot_option.signals().option_clicked().connect_obj(
            &listener,
            |this: &mut LootOptionListener| {
                let _ = this
                    .handle_loot_option_click()
                    .map_err(|error| godot_error!("{error}"));
            },
        );
    }

    fn add_options_to_menu(
        &self,
        vbox: Gd<VBoxContainer>,
        loot_slots_refcell: Rc<RefCell<Vec<Rc<RefCell<InventorySlot>>>>>,
        inventory: Rc<RefCell<Inventory>>,
        collision_object: Gd<CollisionObject3D>,
    ) -> Result<(), LootMenuError> {
        let menu_option_scene =
            try_load::<PackedScene>(LOOT_OPTION_SCENE).map_err(LootMenuError::OptionSceneLoad)?;

        let option_scene = menu_option_scene.clone();
        let options_container = vbox.clone();

        let loot_slots = loot_slots_refcell
            .try_borrow()
            .map_err(|_| LootMenuError::SlotsBorrow)?;

        for slot_ref in loot_slots.iter() {
            self.add_option_to_menu(
                slot_ref,
                option_scene.clone(),
                inventory.clone(),
                collision_object.clone(),
                options_container.clone(),
            )?;
        }

        self.add_loot_all_option(loot_slots, menu_option_scene, vbox)?;

        Ok(())
    }

    fn add_option_to_menu(
        &self,
        slot_ref: &Rc<RefCell<InventorySlot>>,
        option_scene: Gd<PackedScene>,
        inventory: Rc<RefCell<Inventory>>,
        collision_object: Gd<CollisionObject3D>,
        mut options_container: Gd<VBoxContainer>,
    ) -> Result<(), LootMenuError> {
        let option_node = option_scene
            .instantiate()
            .ok_or(LootMenuError::OptionSceneInstantiate)?;

        let mut loot_option = option_node
            .try_cast::<LootOption>()
            .map_err(|_| LootMenuError::OptionCast)?;

        let slot_borrow = slot_ref.try_borrow_mut();
        match slot_borrow {
            Ok(ref slot) => {
                loot_option
                    .bind_mut()
                    .set_item(slot)
                    .map_err(LootMenuError::SetItem)?;
                drop(slot_borrow);

                Ok(())
            }

            Err(_) => {
                drop(slot_borrow);

                Err(LootMenuError::SlotBorrow)
            }
        }?;

        self.add_option_click_listener(
            inventory.clone(),
            slot_ref.clone(),
            collision_object.clone(),
            loot_option.clone(),
        );

        options_container.add_child(&loot_option);

        Ok(())
    }

    fn add_loot_all_option(
        &self,
        loot_slots: Ref<'_, Vec<Rc<RefCell<InventorySlot>>>>,
        menu_option_scene: Gd<PackedScene>,
        mut vbox: Gd<VBoxContainer>,
    ) -> Result<(), LootMenuError> {
        if loot_slots.len() > 1 {
            // Add Loot All option
            let option_node = menu_option_scene.instantiate().unwrap();
            let loot_option = option_node.try_cast::<LootOption>();
            if let Ok(mut loot_option) = loot_option {
                let loot_all = InventorySlot::new(Some(Box::new(LootAll::new())), 1);
                loot_option.bind_mut().set_item(&loot_all)?;
                loot_option.bind_mut().enable_amount(false)?;
                vbox.add_child(&loot_option);
            }
        }

        Ok(())
    }

    pub fn set_options(
        &mut self,
        slots: Rc<RefCell<Vec<Rc<RefCell<InventorySlot>>>>>,
        inventory: Rc<RefCell<Inventory>>,
        collision_object: Gd<CollisionObject3D>,
    ) -> Result<(), LootMenuError> {
        self.base_mut().set_v_size_flags(SizeFlags::EXPAND_FILL);
        self.base_mut()
            .add_theme_color_override("black", Color::BLACK);

        let mut vbox = VBoxContainer::new_alloc();
        self.menu_container = Some(vbox.clone());

        vbox.set_v_size_flags(SizeFlags::EXPAND_FILL);
        self.base_mut().add_child(&vbox);

        self.inventory_slots = slots.clone();
        self.add_options_to_menu(vbox, slots, inventory, collision_object)?;

        godot_print!("loot options added");

        Ok(())
    }

    fn update_hovering(&mut self, hovering: bool) {
        let mouse_hovering_borrow = self.mouse_hovering.try_borrow_mut();
        if let Ok(mut mouse_hovering) = mouse_hovering_borrow {
            *mouse_hovering = hovering;
        }
    }
}
