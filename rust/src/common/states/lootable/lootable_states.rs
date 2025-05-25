#[derive(Debug, Hash, PartialEq, Eq)]
pub enum LootableStates {
    Idle,
    Hover,
    Inspect,
    Chosen,
}
