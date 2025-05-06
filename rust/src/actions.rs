pub struct Actions {
    pub forward: &'static str,
    pub backward: &'static str,
    pub left: &'static str,
    pub right: &'static str,
    pub jump: &'static str,
    pub mouse_mode: &'static str,
}

impl Default for Actions {
    fn default() -> Self {
        Self {
            forward: "move_forward",
            backward: "move_backward",
            left: "move_left",
            right: "move_right",
            jump: "jump",
            mouse_mode: "mouse_mode",
        }
    }
}
