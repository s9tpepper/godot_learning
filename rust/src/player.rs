use godot::classes::InputEvent;
use godot::classes::notify::Node3DNotification;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node3D, init)]
#[allow(unused)]
struct Player3D {
    base: Base<Node3D>,
}

#[godot_api]
impl INode3D for Player3D {
    // Called when the node is ready in the scene tree.
    fn ready(&mut self) {}

    // Called every frame.
    fn process(&mut self, _delta: f64) {}

    // Called every physics frame.
    fn physics_process(&mut self, _delta: f64) {
        println!("HELLO");
        // In GDScript, this would be:
        // rotation += angular_speed * delta

        //let radians = (self.angular_speed * delta * 2.) as f32;
        //self.base_mut().rotate(radians);

        // The 'rotate' method requires a f32,
        // therefore we convert 'self.angular_speed * delta' which is a f64 to a f32
    }

    // String representation of the object.
    fn to_string(&self) -> GString {
        GString::from("Player3D")
    }

    // Handle user input.
    fn input(&mut self, _event: Gd<InputEvent>) {
        godot_print!("input()");

        // self.input_event = event;

        // event.is_action_pressed("move_forward");
        // event.is_action_released("move_forward");

        // TODO: Refactor this to something better than this
        // match event.as_text() {
        //     val if val == self.actions.w => self.move_forward(),
        //     val if val == self.actions.a => self.move_left(),
        //     val if val == self.actions.s => self.move_backward(),
        //     val if val == self.actions.d => self.move_right(),
        //
        //     // "a" => godot_print!("Moving forward!"),
        //     _ => {}
        // }

        // godot_print!("event_text is: {event_text}");
    }

    // Handle lifecycle notifications.
    fn on_notification(&mut self, _what: Node3DNotification) {}
}
