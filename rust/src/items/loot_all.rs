use crate::common::inventory::{InventoryItem, ItemCategory};

#[derive(Debug)]
pub struct LootAll {}

impl LootAll {
    pub fn new() -> Self {
        LootAll {}
    }
}

impl InventoryItem for LootAll {
    fn get_name(&self) -> String {
        "Loot All".into()
    }

    fn get_category(&self) -> ItemCategory {
        ItemCategory::Food
    }

    fn get_max_stack_size(&self) -> i32 {
        1
    }

    fn get_icon(&self) -> String {
        "".into()
    }

    fn get_boxed(&self) -> Box<dyn InventoryItem> {
        Box::new(LootAll {})
    }
}
