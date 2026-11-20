//! "scene" metadata.
//!
//! <https://ngff.openmicroscopy.org/specifications/0.6/#scene-md>.

use serde::{Deserialize, Serialize};
use validatrix::{Accumulator, Validate};

use crate::v0_6::validate_local_transform_graph;

use super::coordinate_systems::{find_system, validate_coordinate_systems};
use super::coordinate_transformations::require_named_io;
use super::{CoordinateSystem, CoordinateTransform};

/// `scene` metadata. Describes spatial relationships between images.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Scene {
    /// Transformations between coordinate systems.
    pub coordinate_transformations: Vec<CoordinateTransform>,
    /// Coordinate systems applicable to multiple images (optional).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub coordinate_systems: Vec<CoordinateSystem>,
}

impl Scene {
    /// Get a coordinate system defined in this scene by name.
    pub fn get_coordinate_system(&self, name: &str) -> Option<&CoordinateSystem> {
        find_system(&self.coordinate_systems, name)
    }
}

impl Validate for Scene {
    fn validate_inner(&self, accum: &mut Accumulator) {
        accum.with_key("coordinateSystems", |a| {
            validate_coordinate_systems(a, &self.coordinate_systems)
        });
        accum.with_key("coordinateTransformations", |a| {
            if self.coordinate_transformations.is_empty() {
                a.add_failure("empty");
            }
            a.validate_iter(&self.coordinate_transformations);
            for (idx, ct) in self.coordinate_transformations.iter().enumerate() {
                a.with_key(idx, |a2| {
                    require_named_io(a2, ct);
                    for (key, r) in [("input", &ct.input), ("output", &ct.output)] {
                        // without a path, the name refers to a system in this scene
                        if let (None, Some(name)) = (r.path.as_ref(), r.name.as_deref()) {
                            if self.get_coordinate_system(name).is_none() {
                                a2.with_keys(&[key.into(), "name".into()], |a3| {
                                    a3.add_failure(format!(
                                        "unknown scene coordinate system '{name}'"
                                    ))
                                });
                            }
                        }
                    }
                });
            }
            validate_local_transform_graph(
                a,
                &self.coordinate_systems,
                &self.coordinate_transformations,
            );
        });
    }
}
