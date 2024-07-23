//! "well" metadata.
//!
//! <https://ngff.openmicroscopy.org/0.4/#well-md>.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// `well` metadata. Describes all fields of views under a given well.
#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Well {
    /// The version of the "well" schema.
    pub version: monostate::MustBe!("0.4"),
    /// Specifies the fields of views of the well.
    pub images: Vec<WellImage>,
}

/// [`Well`] `images` element metadata. Specifies a field of view for a given well.
#[derive(Serialize, Deserialize, Debug)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn well_2fields() {
        let json = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/ome-zarr/0.4/examples/well_strict/well_2fields.json"
        ));
        let map: serde_json::Map<String, serde_json::Value> = serde_json::from_str(json).unwrap();
        let well = map.get("well").unwrap();
        let _well: Well = serde_json::from_value(well.clone()).unwrap();
    }

    #[test]
    fn well_4fields() {
        let json = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/ome-zarr/0.4/examples/well_strict/well_4fields.json"
        ));
        let map: serde_json::Map<String, serde_json::Value> = serde_json::from_str(json).unwrap();
        let well = map.get("well").unwrap();
        let _well: Well = serde_json::from_value(well.clone()).unwrap();
    }
}
