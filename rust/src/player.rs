use godot::classes::base_material_3d::Feature;
use godot::classes::{
    AudioStreamPlayer3D, Camera3D, CharacterBody3D, CsgMesh3D, ICharacterBody3D,
    PhysicsRayQueryParameters3D, StandardMaterial3D,
};
use godot::obj::WithBaseField;
use godot::prelude::*;
use rand::Rng;

use crate::common::proximity_detector::ProximityDetector;
use crate::states::movement::context::MovementContext;

pub type StateContext = Gd<MovementContext>;

#[derive(GodotClass)]
#[class(base=CharacterBody3D, init)]
#[allow(unused)]
pub struct Player3D {
    #[export]
    context: Option<StateContext>,
    base: Base<CharacterBody3D>,
    selected_item: Option<Gd<StandardMaterial3D>>,
}

#[godot_api]
impl Player3D {
    #[allow(unused)]
    fn get_player_skin(&self) -> Gd<Node3D> {
        let base = self.base().clone();
        let player_scene_node_path = self
            .get_context()
            .expect("context")
            .bind()
            .get_player_scene();

        let player_skin: Gd<Node3D> = base
            .try_get_node_as(&player_scene_node_path)
            .expect("Player model should be set");

        player_skin
    }

    #[func]
    /// Plays a footstep sound using the footstep node path from the MovementContext
    fn footstep(&self) {
        let base = self.base().clone();
        if !base.is_on_floor() {
            return;
        }

        let Some(context) = self.get_context() else {
            return;
        };

        let Some(ref mut audio_stream_player_3d) =
            base.try_get_node_as::<AudioStreamPlayer3D>(&context.bind().get_footstep())
        else {
            return;
        };

        let mut rng = rand::rng();
        let pitch_scale = rng.random_range(0.8..1.2);
        audio_stream_player_3d.set_pitch_scale(pitch_scale);

        audio_stream_player_3d.play();
    }

    fn _check_collisions_by_mouse_position(&mut self) {
        let base = self.base().clone();

        let mut world_3d = base.get_world_3d().expect("world 3d");
        let mut space_state = world_3d.get_direct_space_state().expect("space state");

        let window = base.get_window().expect("there should be a window");

        let position = window.get_mouse_position();
        // godot_print!("test loot machines: {:?}", self.test_loot_machines);

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

        // NOTE: Test code to test ProximityDetector component
        if let Some(ref mut items_detector) =
            base.try_get_node_as::<ProximityDetector>("ItemsDetector")
        {
            // NOTE: One way to handle detections from ProximityDetector
            items_detector
                .bind_mut()
                .signals()
                .item_entered_proximity()
                .connect(|item: Gd<Node3D>| {
                    let gd_mesh3d: Result<Gd<CsgMesh3D>, _> = item.try_cast();
                    if let Ok(item) = gd_mesh3d {
                        if let Some(material) = item.get_material() {
                            let standard_material: Result<Gd<StandardMaterial3D>, _> =
                                material.try_cast();
                            if let Ok(mut standard_mat) = standard_material {
                                standard_mat.set_feature(Feature::EMISSION, true);
                            } else {
                                godot_print!("Could not cast to standard material");
                            }
                        } else {
                            godot_print!("Could not get material");
                        }
                    }
                });

            // NOTE: One way to handle detections from ProximityDetector
            items_detector
                .bind_mut()
                .signals()
                .item_exited_proximity()
                .connect(|item: Gd<Node3D>| {
                    let gd_mesh3d: Result<Gd<CsgMesh3D>, _> = item.try_cast();
                    if let Ok(item) = gd_mesh3d {
                        if let Some(material) = item.get_material() {
                            let standard_material: Result<Gd<StandardMaterial3D>, _> =
                                material.try_cast();
                            if let Ok(mut standard_mat) = standard_material {
                                standard_mat.set_feature(Feature::EMISSION, false);
                            } else {
                                godot_print!("Could not cast to standard material");
                            }
                        } else {
                            godot_print!("Could not get material");
                        }
                    }
                });

            items_detector
                .bind_mut()
                .signals()
                .items_entered_proximity()
                .connect(|items: Array<Gd<Node3D>>| {
                    items.iter_shared().for_each(|item| {
                        let gd_mesh3d: Result<Gd<CsgMesh3D>, _> = item.try_cast();
                        if let Ok(item) = gd_mesh3d {
                            if let Some(material) = item.get_material() {
                                let standard_material: Result<Gd<StandardMaterial3D>, _> =
                                    material.try_cast();
                                if let Ok(mut standard_mat) = standard_material {
                                    standard_mat.set_feature(Feature::EMISSION, true);
                                } else {
                                    godot_print!("Could not cast to standard material");
                                }
                            } else {
                                godot_print!("Could not get material");
                            }
                        }
                    });
                });

            items_detector
                .bind_mut()
                .signals()
                .items_exited_proximity()
                .connect(|items: Array<Gd<Node3D>>| {
                    items.iter_shared().for_each(|item| {
                        let gd_mesh3d: Result<Gd<CsgMesh3D>, _> = item.try_cast();
                        if let Ok(item) = gd_mesh3d {
                            if let Some(material) = item.get_material() {
                                let standard_material: Result<Gd<StandardMaterial3D>, _> =
                                    material.try_cast();
                                if let Ok(mut standard_mat) = standard_material {
                                    standard_mat.set_feature(Feature::EMISSION, false);
                                } else {
                                    godot_print!("Could not cast to standard material");
                                }
                            } else {
                                godot_print!("Could not get material");
                            }
                        }
                    });
                });
        }
    }

    // String representation of the object.
    fn to_string(&self) -> GString {
        GString::from("Player3D")
    }
}
