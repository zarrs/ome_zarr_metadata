//! "plate" metadata
//!
//! <https://ngff.openmicroscopy.org/0.5/#plate-md>.

use std::num::NonZeroU64;

use serde::{Deserialize, Serialize};

use super::{PlateAcquisition, PlateColumn, PlateRow, PlateWell};

/// `plate` metadata. For high-content screening datasets.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct Plate {
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
    use crate::{v0_5::OmeFields, OmeZarrGroupMetadata};

    use super::*;

    #[test]
    fn plate_2wells() {
        let json = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/ome-zarr/0.5/examples/plate_strict/plate_2wells.json"
        ));
        let ome_metadata: OmeZarrGroupMetadata<OmeFields> = serde_json::from_str(json).unwrap();
        let _plate: Plate = ome_metadata.attributes.ome.plate.unwrap();
    }

    #[test]
    fn plate_6wells() {
        let json = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/ome-zarr/0.5/examples/plate_strict/plate_6wells.json"
        ));
        let ome_metadata: OmeZarrGroupMetadata<OmeFields> = serde_json::from_str(json).unwrap();
        let _plate: Plate = ome_metadata.attributes.ome.plate.unwrap();
    }
}
