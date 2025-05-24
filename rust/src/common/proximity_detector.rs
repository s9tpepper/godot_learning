use std::collections::HashMap;

use godot::{
    builtin::{
        Array, Color, GString, NodePath, PackedVector3Array, StringName, Variant, Vector3, real,
    },
    classes::{
        EditorNode3DGizmo, EditorNode3DGizmoPlugin, EditorPlugin, Engine, IEditorNode3DGizmoPlugin,
        IEditorPlugin, INode3D, Node3D,
    },
    global::godot_print,
    meta::{ArrayElement, AsArg},
    obj::{Base, Gd, InstanceId, NewGd, WithBaseField, WithUserSignals},
    prelude::{Export, GodotClass, GodotConvert, Var, godot_api},
};

/// Detection mode for ProximityDetector, will either detect multiple
/// items within the given parameters to ProximityDetector, or it will
/// detect the single closest item based on the given parameters to
/// the ProximityDetector node.
#[derive(GodotConvert, Var, Export, Default, PartialEq, Debug)]
#[godot(via = GString)]
pub enum DetectionMode {
    #[default]
    Single,
    Multiple,
}

impl From<GString> for DetectionMode {
    fn from(value: GString) -> Self {
        match value.to_string().as_str() {
            "Single" => DetectionMode::Single,
            "Multiple" => DetectionMode::Multiple,
            _ => DetectionMode::Multiple,
        }
    }
}

impl From<Variant> for DetectionMode {
    fn from(value: Variant) -> Self {
        match value.to_string().as_str() {
            "Single" => DetectionMode::Single,
            "Multiple" => DetectionMode::Multiple,
            _ => DetectionMode::Multiple,
        }
    }
}

#[derive(GodotClass, Debug)]
#[class(tool, base=Node3D, init)]
/// ProximityDetector will detect the proximity of nodes based on
/// the given parameters that are set on the instance in the node tree.
pub struct ProximityDetector {
    #[base]
    pub base: Base<Node3D>,

    #[export]
    /// NodePath to the node that should be used
    /// to detect proximity from.
    player_scene: NodePath,

    #[export]
    /// Name of the node group to look for nodes in
    detection_group_name: GString,

    #[export]
    /// The max distance to use for node proxmity detection
    detection_distance: f32,

    #[export]
    /// The angle to use for determining that the node is
    /// within the desired view of the player scene
    view_angle_degrees: f32,

    #[export]
    /// The detection mode to use, either Single or Multiple
    /// nodes will be detected based on this setting
    detection_mode: DetectionMode,

    entered_proximity: HashMap<InstanceId, Gd<Node3D>>,
}

impl ProximityDetector {
    fn get_player_skin(&mut self) -> Gd<Node3D> {
        let base = self.base().clone();

        let player_skin: Gd<Node3D> = base
            .try_get_node_as(&self.get_player_scene())
            .expect("Player model should be set");

        player_skin
    }
}

#[godot_api]
impl ProximityDetector {
    #[signal]
    /// A node has entered proximity based on the detector's settings
    pub fn item_entered_proximity(node: Gd<Node3D>);

    #[signal]
    /// A node has exited proximity after having been detected as close
    pub fn item_exited_proximity(node: Gd<Node3D>);

    #[signal]
    /// A list of nodes that have entered proximity based on the detector's settings
    pub fn items_entered_proximity(node: Array<Gd<Node3D>>);

    #[signal]
    /// A list of nodes that have exited proximity after having been detected as close
    pub fn items_exited_proximity(node: Array<Gd<Node3D>>);
}

#[godot_api]
impl INode3D for ProximityDetector {
    fn set_property(&mut self, _property: StringName, _value: Variant) -> bool {
        let engine = Engine::singleton();

        if engine.is_editor_hint() {
            self.base_mut().update_gizmos();
        }

        false
    }

    fn physics_process(&mut self, _delta: f64) {
        let base = self.base().clone();
        let Some(mut tree) = base.get_tree() else {
            return;
        };

        let player_skin = self.get_player_skin();

        let mut entered = vec![];
        let mut exited = vec![];

        let items = tree.get_nodes_in_group(self.get_detection_group_name().arg());
        items.iter_shared().for_each(|item| {
            let node_3d: Result<Gd<Node3D>, _> = item.clone().try_cast();
            let Ok(node_3d) = node_3d else {
                return;
            };

            let basis = player_skin.get_global_transform().basis;
            let looking = basis.col_c().normalized();
            let direction = node_3d.get_global_position() - player_skin.get_global_position();
            let distance = direction.length();

            let angle_to_item = looking.angle_to(direction.normalized());
            let degrees = angle_to_item.to_degrees();
            let viewing_angle = self.get_view_angle_degrees();

            let item_is_close = (distance < self.get_detection_distance()
                && (-viewing_angle..viewing_angle).contains(&degrees))
                || distance < 0.4;

            if item_is_close {
                entered.push((node_3d, distance));
            } else if self.entered_proximity.contains_key(&node_3d.instance_id()) && !item_is_close
            {
                exited.push(node_3d);
            }
        });

        let mode: DetectionMode = self.get_detection_mode().into();
        if mode == DetectionMode::Single {
            entered.sort_by(|(_, a), (_, b)| a.total_cmp(b));

            if !entered.is_empty() {
                let mut first_item: Vec<(Gd<Node3D>, f32)> = vec![];
                for (index, (node, dist)) in entered.iter().enumerate() {
                    if index == 0 {
                        first_item = vec![(node.clone(), *dist)];
                    } else if self.entered_proximity.contains_key(&node.instance_id()) {
                        exited.push(node.clone());
                    }
                }

                entered = first_item;
            }
        }

        exited.iter().for_each(|node| {
            self.entered_proximity.remove(&node.instance_id());
        });

        entered.iter().for_each(|(node, _)| {
            self.entered_proximity
                .insert(node.instance_id(), node.clone());
        });

        match mode {
            DetectionMode::Single => {
                exited.iter().for_each(|node| {
                    self.entered_proximity.remove(&node.instance_id());

                    self.signals().item_exited_proximity().emit(node);
                });

                entered.iter().for_each(|(node, _)| {
                    self.entered_proximity
                        .insert(node.instance_id(), node.clone());

                    self.signals().item_entered_proximity().emit(node);
                });
            }

            DetectionMode::Multiple => {
                self.signals()
                    .items_exited_proximity()
                    .emit(&to_array(&exited));

                let ent: Vec<Gd<Node3D>> = entered.iter().map(|n| n.0.clone()).collect();
                self.signals()
                    .items_entered_proximity()
                    .emit(&to_array(&ent));
            }
        }
    }

    // String representation of the object.
    fn to_string(&self) -> GString {
        "ProximityDetector".into()
    }
}

/// Converts a Vec<T> to an Array<T>
pub fn to_array<'a, T>(vec: &'a [T]) -> Array<T>
where
    T: ArrayElement,
    &'a T: AsArg<T>,
{
    let mut new_array: Array<T> = Array::new();
    vec.iter().for_each(|node| {
        new_array.push(node);
    });

    new_array
}

#[derive(GodotClass)]
#[class(tool, base=EditorNode3DGizmoPlugin)]
struct ProximityGizmo {
    #[base]
    base: Base<EditorNode3DGizmoPlugin>,

    material_created: bool,
}

#[godot_api]
impl IEditorNode3DGizmoPlugin for ProximityGizmo {
    fn init(base: Base<EditorNode3DGizmoPlugin>) -> Self {
        ProximityGizmo {
            base,
            material_created: false,
        }
    }

    fn redraw(&mut self, gizmo: Option<Gd<EditorNode3DGizmo>>) {
        let Some(mut gizmo) = gizmo else { return };

        gizmo.clear();

        if !self.material_created {
            self.base_mut().create_material("main", Color::RED);
        }

        let mut base = self.base_mut();

        let Some(node3d) = gizmo.get_node_3d() else {
            return;
        };

        let mut lines = PackedVector3Array::new();

        let detection_distance: f32 = node3d.get("detection_distance").to();
        let view_angle_degrees: f32 = node3d.get("view_angle_degrees").to();

        let view_angle_radians: real = view_angle_degrees.to_radians();

        let basis_c = node3d.get_basis().col_c();
        let rotated_left = basis_c.rotated(Vector3::UP, view_angle_radians) * detection_distance;
        let rotated_right = basis_c.rotated(Vector3::UP, -view_angle_radians) * detection_distance;

        lines.push(Vector3::new(0., 0., 0.));
        lines.push(rotated_left);

        lines.push(Vector3::new(0., 0., 0.));
        lines.push(rotated_right);

        let Some(mut material) = base.get_material("main") else {
            godot_print!("redraw: Can't get material");
            return;
        };

        let detection_mode: DetectionMode = node3d.get("detection_mode").into();
        match detection_mode {
            DetectionMode::Single => material.set_albedo(Color::CYAN),
            DetectionMode::Multiple => material.set_albedo(Color::MEDIUM_VIOLET_RED),
        }

        gizmo.add_lines(&lines, &material);
    }

    fn get_gizmo_name(&self) -> godot::prelude::GString {
        "ProximityDetector".into()
    }

    fn has_gizmo(&self, for_node_3d: Option<godot::prelude::Gd<Node3D>>) -> bool {
        if for_node_3d.is_none() {
            return false;
        }

        let node = for_node_3d.unwrap();
        let is_proximity_detector = node.try_cast::<ProximityDetector>();

        is_proximity_detector.is_ok()
    }
}

#[derive(GodotClass)]
#[class(tool, init, base = EditorPlugin)]
struct ProximityDetectorPlugin {
    base: Base<EditorPlugin>,

    proximity_gizmo: Option<Gd<ProximityGizmo>>,
}

#[godot_api]
impl IEditorPlugin for ProximityDetectorPlugin {
    fn enter_tree(&mut self) {
        let plugin = ProximityGizmo::new_gd();
        self.proximity_gizmo = Some(plugin.clone());

        self.base_mut().add_node_3d_gizmo_plugin(&plugin);
    }

    fn exit_tree(&mut self) {
        if self.proximity_gizmo.is_none() {
            return;
        }

        let gizmo: Result<Gd<ProximityGizmo>, _> =
            self.proximity_gizmo.as_ref().unwrap().clone().try_cast();

        let Ok(gizmo) = gizmo else { return };
        self.base_mut().remove_node_3d_gizmo_plugin(&gizmo);
    }
}
