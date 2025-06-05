use godot::{
    builtin::{Vector2, Vector2i},
    classes::{
        HBoxContainer, IHBoxContainer, ImageTexture, InputEvent, InputEventMouseButton, Label,
        Texture2D, TextureRect,
        texture_rect::{ExpandMode, StretchMode},
    },
    obj::{Base, Gd, WithBaseField, WithUserSignals},
    prelude::{GodotClass, godot_api},
    tools::load,
};
use thiserror::Error;

use crate::common::inventory::InventorySlot;

use super::utils::is_inbounds;

#[derive(Debug, GodotClass)]
#[class(init, base = HBoxContainer)]
pub struct LootOption {
    #[base]
    base: Base<HBoxContainer>,

    #[export]
    icon: Option<Gd<TextureRect>>,

    #[export]
    name: Option<Gd<Label>>,

    #[export]
    count: Option<Gd<Label>>,

    uuid: String,
}

#[derive(Debug, Error)]
pub enum LootOptionError {
    #[error("Loot option name label is missing")]
    Name,

    #[error("Loot option count label is missing")]
    Count,

    #[error("Loot option icon texture rect is missing")]
    Icon,
}

#[godot_api]
impl LootOption {
    #[signal]
    pub fn option_clicked();

    pub fn set_item(&mut self, slot: &InventorySlot) -> Result<(), LootOptionError> {
        let item = slot.item.as_ref().expect("slots to have an item");

        let mut name_label = self.get_name().ok_or(LootOptionError::Name)?;
        name_label.set_text(&item.get_name());

        let mut count_label = self.get_name().ok_or(LootOptionError::Count)?;
        count_label.set_text(&slot.count.to_string());

        // TODO: Fix this to get image specifically for loot item
        let texture = load::<Texture2D>("res://images/test_image.jpeg");
        let image = texture.get_image().unwrap();
        let mut texture = ImageTexture::create_from_image(&image).unwrap();
        texture.set_size_override(Vector2i { x: 32, y: 32 });

        let mut icon = self.get_icon().ok_or(LootOptionError::Icon)?;
        icon.set_texture(&texture);
        icon.set_size(Vector2::new(32., 32.));
        icon.set_stretch_mode(StretchMode::KEEP_ASPECT);
        icon.set_expand_mode(ExpandMode::IGNORE_SIZE);

        self.uuid = item.get_uuid().to_string();

        Ok(())
    }

    pub fn enable_amount(&mut self, enable: bool) -> Result<(), LootOptionError> {
        let mut count_label = self.get_name().ok_or(LootOptionError::Count)?;
        count_label.set_visible(enable);

        Ok(())
    }

    // NOTE: No longer used??
    // pub fn get_uuid(&self) -> &str {
    //     &self.uuid
    // }
}

#[godot_api]
impl IHBoxContainer for LootOption {
    fn input(&mut self, event: Gd<InputEvent>) {
        let input_event_mouse_button = event.clone().try_cast::<InputEventMouseButton>();
        if let Ok(mouse_event) = input_event_mouse_button {
            let global_rect = self.base().get_global_rect();
            let is_hovering = is_inbounds(global_rect, event);
            if is_hovering && mouse_event.is_released() {
                self.signals().option_clicked().emit();
            }
        }
    }
}
