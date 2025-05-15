use std::fmt::Display;

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub enum MovementStates {
    #[default]
    Idle,
    Walking,
}

impl Display for MovementStates {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
