//! "well" metadata.
//!
//! <https://ngff.openmicroscopy.org/0.5/#well-md>.

use std::{collections::BTreeSet, path::PathBuf};

use serde::{Deserialize, Serialize};
use validatrix::Validate;

use crate::v0_5 as prev;

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

/// [`Well`] `images` element metadata. Specifies a field of view for a given well.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct WellImage {
    /// A string specifying the path to the field of view.
    ///
    /// The path MUST contain only alphanumeric characters, MUST be case-sensitive, and MUST NOT be a duplicate of any other path in the images list.
    pub path: PathBuf,
    /// An integer identifying the acquisition (optional).
    ///
    /// If multiple acquisitions were performed in the plate, it MUST contain an acquisition key whose value MUST be an integer identifying the acquisition which MUST match one of the acquisition JSON objects defined in the plate metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acquisition: Option<u64>,
}

impl Validate for WellImage {
    fn validate_inner(&self, accum: &mut validatrix::Accumulator) {
        let Some(s) = self.path.as_os_str().to_str() else {
            accum.add_failure_at("path", "path is not valid UTF-8");
            return;
        };
        if s.is_empty() {
            accum.add_failure_at("path", "path is empty");
        }
        if s == "." || s == ".." {
            accum.add_failure_at("path", "path cannot be '.' or '..'");
            return;
        }
        if s.starts_with("__") {
            accum.add_failure_at("path", "path cannot start with '__'");
        }
        for c in s.chars() {
            if !c.is_alphanumeric() && c != '-' && c != '_' && c != '.' {
                accum.add_failure_at("path", format!("path contains invalid character '{c}'"));
                break;
            }
        }
    }
}

impl From<prev::WellImage> for WellImage {
    fn from(value: prev::WellImage) -> Self {
        Self {
            path: value.path,
            acquisition: value.acquisition,
        }
    }
}

impl From<prev::Well> for Well {
    fn from(value: prev::Well) -> Self {
        Self {
            images: value.images.into_iter().map(WellImage::from).collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::v0_6::OmeZarrGroupMetadata;

    use super::*;

    #[test]
    fn well_2fields() {
        let json = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/ome-zarr/specifications/0.6/examples/well_strict/well_2fields.json"
        ));
        let ome_metadata: OmeZarrGroupMetadata = serde_json::from_str(json).unwrap();
        let _well: Well = ome_metadata.attributes.ome.well.unwrap();
    }

    #[test]
    fn well_4fields() {
        let json = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/ome-zarr/specifications/0.6/examples/well_strict/well_4fields.json"
        ));
        let ome_metadata: OmeZarrGroupMetadata = serde_json::from_str(json).unwrap();
        let _well: Well = ome_metadata.attributes.ome.well.unwrap();
    }
}
