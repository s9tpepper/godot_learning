use godot::{
    builtin::Rect2,
    classes::{InputEvent, InputEventMouseButton, InputEventMouseMotion},
    obj::Gd,
};

pub fn is_inbounds(global_rect: Rect2, event: Gd<InputEvent>) -> bool {
    let event_motion = event.clone().try_cast::<InputEventMouseMotion>();
    let event_button = event.clone().try_cast::<InputEventMouseButton>();

    if let Ok(mouse_event) = event_motion {
        let click_position = mouse_event.get_global_position();
        return global_rect.contains_point(click_position);
    }

    if let Ok(button_event) = event_button {
        let click_position = button_event.get_global_position();
        return global_rect.contains_point(click_position);
    }

    false
}
