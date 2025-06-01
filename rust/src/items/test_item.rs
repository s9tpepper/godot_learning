use serde::{Deserialize, Serialize};

use crate::common::inventory::{InventoryItem, ItemCategory};

#[derive(Debug, Serialize, Deserialize)]
pub struct TestItem {}

impl TestItem {
    pub fn new() -> Self {
        TestItem {}
    }
}

impl InventoryItem for TestItem {
    fn get_name(&self) -> String {
        "This is the Test Item".into()
    }

    fn get_category(&self) -> ItemCategory {
        ItemCategory::Food
    }

    fn get_max_stack_size(&self) -> i32 {
        10
    }

    fn get_icon(&self) -> String {
        "".into()
    }

    fn get_boxed(&self) -> Box<dyn InventoryItem> {
        Box::new(TestItem {})
    }
}
