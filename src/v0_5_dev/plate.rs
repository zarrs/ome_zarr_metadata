//! "plate" metadata
//!
//! <https://ngff.openmicroscopy.org/latest/#plate-md>.

use std::num::NonZeroU64;

use serde::{Deserialize, Serialize};

use super::{PlateAcquisition, PlateColumn, PlateRow, PlateWell};

/// `plate` metadata. For high-content screening datasets.
#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Plate {
    /// The version of the multiscale metadata of the image.
    pub version: monostate::MustBe!("0.5-dev"),
    /// A list of JSON objects defining the acquisitions for a given plate to which wells can refer to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acquisitions: Option<Vec<PlateAcquisition>>,
    /// A list of JSON objects defining the columns of the plate
    pub columns: Vec<PlateColumn>,
    /// The field count defining the maximum number of fields per view across all wells (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_count: Option<NonZeroU64>,
    /// The name of the plate (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Defines the rows of the plate.
    pub rows: Vec<PlateRow>,
    /// Defines the wells of the plate.
    pub wells: Vec<PlateWell>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plate_2wells() {
        let json = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/ome-zarr/latest/examples/plate_strict/plate_2wells.json"
        ));
        let map: serde_json::Map<String, serde_json::Value> = serde_json::from_str(json).unwrap();
        let plate = map.get("plate").unwrap();
        let _plate: Plate = serde_json::from_value(plate.clone()).unwrap();
    }

    #[test]
    fn plate_6wells() {
        let json = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/ome-zarr/latest/examples/plate_strict/plate_6wells.json"
        ));
        let map: serde_json::Map<String, serde_json::Value> = serde_json::from_str(json).unwrap();
        let plate = map.get("plate").unwrap();
        let _plate: Plate = serde_json::from_value(plate.clone()).unwrap();
    }
}
