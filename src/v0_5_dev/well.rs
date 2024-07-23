//! "well" metadata.
//!
//! <https://ngff.openmicroscopy.org/latest/#well-md>.

use serde::{Deserialize, Serialize};

use super::WellImage;

/// `well` metadata. Describes all fields of views under a given well.
#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Well {
    /// The version of the "well" schema.
    pub version: monostate::MustBe!("0.5-dev"),
    /// Specifies the fields of views of the well.
    pub images: Vec<WellImage>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn well_2fields() {
        let json = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/ome-zarr/latest/examples/well_strict/well_2fields.json"
        ));
        let map: serde_json::Map<String, serde_json::Value> = serde_json::from_str(json).unwrap();
        let well = map.get("well").unwrap();
        let _well: Well = serde_json::from_value(well.clone()).unwrap();
    }

    #[test]
    fn well_4fields() {
        let json = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/ome-zarr/latest/examples/well_strict/well_4fields.json"
        ));
        let map: serde_json::Map<String, serde_json::Value> = serde_json::from_str(json).unwrap();
        let well = map.get("well").unwrap();
        let _well: Well = serde_json::from_value(well.clone()).unwrap();
    }
}
