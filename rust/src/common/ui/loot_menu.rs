use std::{cell::RefCell, rc::Rc};

use godot::{
    builtin::Color,
    classes::{
        CollisionObject3D, IPanelContainer, InputEvent, Node, PackedScene, PanelContainer,
        VBoxContainer, control::SizeFlags,
    },
    global::godot_print,
    obj::{Base, NewAlloc, WithBaseField, WithUserSignals},
    prelude::{Gd, GodotClass, godot_api},
    tools::load,
};

use crate::{
    common::inventory::{Inventory, InventorySlot},
    items::loot_all::LootAll,
};

use super::{loot_option::LootOption, utils::is_inbounds};

const LOOT_OPTION_SCENE: &str = "res://ui/loot_option.tscn";

#[derive(Debug, GodotClass)]
#[class(init, base = PanelContainer)]
pub struct LootMenu {
    #[base]
    base: Base<PanelContainer>,

    pub mouse_hovering: Rc<RefCell<bool>>,
    pub option_selected: Rc<RefCell<bool>>,
}

#[godot_api]
impl IPanelContainer for LootMenu {
    fn input(&mut self, event: Gd<InputEvent>) {
        let global_rect = self.base().get_global_rect();
        let is_hovering = is_inbounds(global_rect, event);

        self.update_hovering(is_hovering);
    }

    fn process(&mut self, _delta: f64) {
        let trigger = self.option_selected.clone();
        let mut option_clicked = self.signals().option_clicked();

        let selected_borrow = trigger.try_borrow();
        if let Ok(selected) = selected_borrow {
            if *selected {
                option_clicked.emit();
                self.base_mut().queue_free();
            }
        }
    }
}

#[godot_api]
impl LootMenu {
    #[signal]
    pub fn option_clicked();

    pub fn set_options(
        &mut self,
        slots: Rc<RefCell<Vec<Rc<RefCell<InventorySlot>>>>>,
        inventory: Rc<RefCell<Inventory>>,
        collision_object: Gd<CollisionObject3D>,
    ) {
        self.base_mut().set_v_size_flags(SizeFlags::EXPAND_FILL);
        self.base_mut()
            .add_theme_color_override("black", Color::BLACK);

        let mut vbox = VBoxContainer::new_alloc();
        vbox.set_v_size_flags(SizeFlags::EXPAND_FILL);
        self.base_mut().add_child(&vbox);

        let menu_option_scene = load::<PackedScene>(LOOT_OPTION_SCENE);

        let option_scene = menu_option_scene.clone();
        let mut options_container = vbox.clone();

        let slots_borrow = slots.try_borrow();
        if let Ok(slots) = slots_borrow {
            slots.iter().for_each(move |slot_ref| {
                let option_node = option_scene.instantiate().unwrap();
                let loot_option = option_node.try_cast::<LootOption>();
                if let Ok(mut loot_option) = loot_option {
                    {
                        let slot_borrow = slot_ref.try_borrow_mut();
                        let Ok(slot) = slot_borrow else { return };
                        loot_option.bind_mut().set_item(&slot);
                    }

                    let mut listener = LootOptionListener::new_alloc();
                    listener.bind_mut().inventory = inventory.clone();
                    listener.bind_mut().slot = slot_ref.clone();
                    listener.bind_mut().collision_object = Some(collision_object.clone());
                    listener.bind_mut().option_selected = self.option_selected.clone();

                    loot_option.signals().option_clicked().connect_obj(
                        &listener,
                        |this: &mut LootOptionListener| {
                            let inventory_borrow = this.inventory.try_borrow_mut();
                            if let Ok(mut inventory) = inventory_borrow {
                                let slot_borrow = this.slot.try_borrow_mut();
                                let Ok(mut slot) = slot_borrow else { return };

                                match inventory.add(&mut slot) {
                                    Some(_unlooted_slot) => {
                                        // TODO: Message user that the inventory is full
                                        // Do an sound, or animation, or something
                                    }
                                    None => {
                                        // remove the lootable from the world
                                        let collision_obj = this.collision_object.as_mut().unwrap();

                                        if collision_obj.is_instance_valid() {
                                            collision_obj.queue_free();
                                        }
                                    }
                                }

                                let selected_borrow = this.option_selected.try_borrow_mut();
                                if let Ok(mut selected) = selected_borrow {
                                    *selected = true;
                                }
                            }
                        },
                    );

                    options_container.add_child(&loot_option);
                }
            });
        }

        let option_node = menu_option_scene.instantiate().unwrap();
        let loot_option = option_node.try_cast::<LootOption>();
        if let Ok(mut loot_option) = loot_option {
            let loot_all = InventorySlot::new(Some(Box::new(LootAll::new())), 1);
            loot_option.bind_mut().set_item(&loot_all);
            loot_option.bind_mut().enable_amount(false);
            vbox.add_child(&loot_option);
        }

        godot_print!("loot options added");
    }

    fn update_hovering(&mut self, hovering: bool) {
        let mouse_hovering_borrow = self.mouse_hovering.try_borrow_mut();
        if let Ok(mut mouse_hovering) = mouse_hovering_borrow {
            *mouse_hovering = hovering;
        }
    }
}

#[derive(Debug, GodotClass)]
#[class(init, base = Node)]
struct LootOptionListener {
    base: Base<Node>,
    pub slot: Rc<RefCell<InventorySlot>>,
    pub inventory: Rc<RefCell<Inventory>>,
    pub collision_object: Option<Gd<CollisionObject3D>>,
    pub option_selected: Rc<RefCell<bool>>,
}

#[godot_api]
impl LootOptionListener {
    #[signal]
    fn dummy();
}
