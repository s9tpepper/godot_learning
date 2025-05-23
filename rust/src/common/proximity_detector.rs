use std::collections::HashMap;

use godot::{
    builtin::{Array, GString, NodePath},
    classes::{INode3D, Node3D},
    meta::{ArrayElement, AsArg},
    obj::{Base, Gd, InstanceId, WithBaseField, WithUserSignals},
    prelude::{Export, GodotClass, GodotConvert, Var, godot_api},
};

/// Detection mode for ProximityDetector, will either detect multiple
/// items within the given parameters to ProximityDetector, or it will
/// detect the single closest item based on the given parameters to
/// the ProximityDetector node.
#[derive(GodotConvert, Var, Export, Default, PartialEq)]
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

#[derive(GodotClass)]
#[class(base=Node3D, init)]
#[allow(unused)]
/// ProximityDetector will detect the proximity of nodes based on
/// the given parameters that are set on the instance in the node tree.
pub struct ProximityDetector {
    #[base]
    base: Base<Node3D>,

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
    // Called every physics frame.
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
        "ProximityDetector".to_string().into()
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
