#[derive(Default, Debug, Hash, PartialEq, Eq, Clone)]
pub enum LootState {
    #[default]
    Idle,
    Hover,
    Inspect,
    Chosen,
    // Expired, ??
}
