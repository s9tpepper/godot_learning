use godot::classes::base_material_3d::Feature;
use godot::classes::notify::Node3DNotification;
use godot::classes::{
    Camera3D, CharacterBody3D, CsgMesh3D, ICharacterBody3D, InputEvent,
    PhysicsRayQueryParameters3D, StandardMaterial3D,
};
use godot::obj::WithBaseField;
use godot::prelude::*;

use crate::finite_state_machine::FiniteStateMachine;
use crate::some_state_machine::SomeStateMachine;

pub type StateContext = Gd<MovementContext>;

#[derive(GodotClass)]
#[class(base=CharacterBody3D, init)]
#[allow(unused)]
pub struct Player3D {
    #[export]
    context: Option<StateContext>,
    base: Base<CharacterBody3D>,
    state_machine: Option<SomeStateMachine>,

    selected_item: Option<Gd<StandardMaterial3D>>,
}

#[derive(Default, Debug, GodotClass)]
#[class(base=Resource, init)]
pub struct MovementContext {
    #[export]
    pub player: NodePath,

    #[export]
    pub player_scene: NodePath,

    #[export]
    pub pivot: NodePath,

    #[export]
    pub camera: NodePath,

    #[export]
    pub animation_player: GString,

    #[export]
    pub walking_animation_name: GString,

    #[export(range=(0.01, 400.0))]
    pub movement_speed: f32,
}

impl Player3D {
    fn check_collisions_by_dot_product(&mut self) {
        let base = self.base().clone();
        let Some(mut tree) = base.get_tree() else {
            return;
        };

        let player_scene = self
            .get_context()
            .expect("context")
            .bind()
            .get_player_scene();

        let player_skin: Option<Gd<Node3D>> = base.try_get_node_as(&player_scene);
        let Some(player_skin) = player_skin else {
            godot_print!("Couldn't get player skin");
            return;
        };

        let items = tree.get_nodes_in_group("items");
        items.iter_shared().for_each(|item| {
            let node_3d: Result<Gd<Node3D>, _> = item.clone().try_cast();
            let Ok(node_3d) = node_3d else {
                return;
            };

            let basis = player_skin.get_global_transform().basis;
            let looking = basis.col_c().normalized();
            let direction = node_3d.get_global_position() - player_skin.get_global_position();

            let angle_to_item = looking.angle_to(direction.normalized());
            // godot_print!(
            //     "angle_to_item: {}, item: {}, distance: {}, class: {}",
            //     angle_to_item.to_degrees(),
            //     item.get_name(),
            //     direction.length(),
            //     item.get_class().to_string()
            // );

            let gd_mesh3d: Result<Gd<CsgMesh3D>, _> = node_3d.try_cast();
            let degrees = angle_to_item.to_degrees();

            if direction.length() < 5. && (-35.0f32..35.0f32).contains(&degrees)
                || direction.length() < 0.4
            {
                // godot_print!("Close enough");

                godot_print!(
                    "angle_to_item: {}, item: {}, distance: {}",
                    angle_to_item.to_degrees(),
                    item.get_name(),
                    direction.length(),
                );

                if let Ok(mesh3d) = gd_mesh3d {
                    if let Some(material) = mesh3d.get_material() {
                        let standard_material: Result<Gd<StandardMaterial3D>, _> =
                            material.try_cast();
                        if let Ok(mut standard_mat) = standard_material {
                            self.selected_item = Some(standard_mat.clone());
                            standard_mat.set_feature(Feature::EMISSION, true);
                            godot_print!("Turned on shader");
                        } else {
                            godot_print!("Could not cast to standard material");
                        }
                    } else {
                        godot_print!("Could not get material");
                    }
                } else {
                    godot_print!("Could not cast to mesh3d");
                }
            } else if let Ok(mesh3d) = gd_mesh3d {
                if let Some(material) = mesh3d.get_material() {
                    let standard_material: Result<Gd<StandardMaterial3D>, _> = material.try_cast();
                    if let Ok(mut standard_mat) = standard_material {
                        self.selected_item = Some(standard_mat.clone());
                        standard_mat.set_feature(Feature::EMISSION, false);
                    }
                }
            }
        });
    }

    fn _check_collisions_by_mouse_position(&mut self) {
        let base = self.base().clone();

        let mut world_3d = base.get_world_3d().expect("world 3d");
        let mut space_state = world_3d.get_direct_space_state().expect("space state");

        let window = base.get_window().expect("there should be a window");

        let position = window.get_mouse_position();

        let context = self.get_context().expect("Context should exist");
        let camera_path = context.bind().get_camera();
        let camera: Option<Gd<Camera3D>> = base.try_get_node_as(&camera_path);
        if let Some(cam) = camera {
            let from = cam.project_ray_origin(position);
            let to = from + cam.project_ray_normal(position) * 10.;
            let mut query_params =
                PhysicsRayQueryParameters3D::create(from, to).expect("query params");

            let mut excludes: Array<Rid> = Array::new();
            excludes.push(base.get_rid());
            query_params.set_exclude(&excludes);

            let result = space_state.intersect_ray(&query_params);
            let Some(collider) = result.get("collider") else {
                return;
            };

            let gd_mesh3d: Result<Gd<CsgMesh3D>, ConvertError> = collider.try_to();

            if let Ok(mesh3d) = gd_mesh3d {
                if let Some(material) = mesh3d.get_material() {
                    let standard_material: Result<Gd<StandardMaterial3D>, _> = material.try_cast();
                    if let Ok(mut standard_mat) = standard_material {
                        self.selected_item = Some(standard_mat.clone());
                        standard_mat.set_feature(Feature::EMISSION, true);
                    }
                }
            } else if let Some(ref mut material) = self.selected_item {
                material.set_feature(Feature::EMISSION, false);
                self.selected_item = None;
            }

            // NOTE: iterate through results of collision check
            // result.iter_shared().for_each(|(key, value)| {
            //     if key.to_string() == "collider" {
            //         // let gd = value.clone_from
            //
            //         godot_print!("key: {key:?}, value: {value:?}");
            //
            //         // CSGMesh3D
            //     }
            // });

            // godot_print!("raycast result: {result:?}");

            // func _physics_process(delta):
            // 	var space_state = get_world_2d().direct_space_state
            // 	# use global coordinates, not local to node
            // 	var query = PhysicsRayQueryParameters2D.create(Vector2(0, 0), Vector2(50, 100))
            // 	var result = space_state.intersect_ray(query)
            //var to = from + camera3d.project_ray_normal(event.position) * RAY_LENGTH
        }
    }
}

#[godot_api]
impl ICharacterBody3D for Player3D {
    // Called when the node is ready in the scene tree.
    fn ready(&mut self) {
        let base = self.base().clone();

        if let Some(context) = &self.context {
            godot_print!("[Player3D::process()] Starting state machine...");

            let mut state_machine = SomeStateMachine::new(context.clone(), base.clone());
            state_machine.ready();

            self.state_machine = Some(state_machine);
            godot_print!(
                "[Player3D::process()] Set self.state_machine to {:?}",
                self.state_machine
            );
        } else {
            godot_print!("tree: {base:?}, context: {:?}", self.context);
        }
    }

    // Called every frame.
    fn process(&mut self, delta: f64) {
        let Some(ref mut machine) = self.state_machine else {
            godot_print!("[Player3D::process()] Unable to get state machine reference");
            return;
        };

        machine.process(delta);
    }

    // Called every physics frame.
    fn physics_process(&mut self, delta: f64) {
        let Some(ref mut machine) = self.state_machine else {
            godot_print!("[Player3D::physics_process()] Unable to get state machine reference");
            return;
        };

        machine.process_physics(delta);

        // self.check_collisions_by_mouse_position();
        self.check_collisions_by_dot_product();
    }

    // String representation of the object.
    fn to_string(&self) -> GString {
        GString::from("Player3D")
    }

    // Handle user input.
    fn input(&mut self, event: Gd<InputEvent>) {
        let Some(ref mut machine) = self.state_machine else {
            godot_print!("[Player3D::input()] Unable to get state machine reference");
            return;
        };

        machine.input(event);
    }

    // Handle lifecycle notifications.
    fn on_notification(&mut self, _what: Node3DNotification) {}
}
