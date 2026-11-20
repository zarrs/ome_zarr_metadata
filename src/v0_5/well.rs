//! "well" metadata.
//!
//! <https://ngff.openmicroscopy.org/0.5/#well-md>.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};
use validatrix::Validate;

use super::WellImage;

/// `well` metadata. Describes all fields of views under a given well.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct Well {
    /// Specifies the fields of views of the well.
    pub images: Vec<WellImage>,
}

impl Validate for Well {
    fn validate_inner(&self, accum: &mut validatrix::Accumulator) {
        accum.validate_iter_at("images", &self.images);
        let mut paths = BTreeSet::new();
        for (idx, p) in self.images.iter().map(|i| &i.path).enumerate() {
            if !paths.insert(p) {
                accum.with_keys(&["images".into(), idx.into(), "path".into()], |a| {
                    a.add_failure(format!("duplicate path '{}'", p.display()));
                });
            }
        }
    }
}

impl From<crate::v0_4::Well> for Well {
    fn from(value: crate::v0_4::Well) -> Self {
        Self {
            images: value.images,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::v0_5::OmeZarrGroupMetadata;

    use super::*;

    #[test]
    fn well_2fields() {
        let json = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/ome-zarr/specifications/0.5/examples/well_strict/well_2fields.json"
        ));
        let ome_metadata: OmeZarrGroupMetadata = serde_json::from_str(json).unwrap();
        let _well: Well = ome_metadata.attributes.ome.well.unwrap();
    }

    #[test]
    fn well_4fields() {
        let json = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/ome-zarr/specifications/0.5/examples/well_strict/well_4fields.json"
        ));
        let ome_metadata: OmeZarrGroupMetadata = serde_json::from_str(json).unwrap();
        let _well: Well = ome_metadata.attributes.ome.well.unwrap();
    }
}
