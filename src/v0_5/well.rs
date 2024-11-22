//! "well" metadata.
//!
//! <https://ngff.openmicroscopy.org/0.5/#well-md>.

use serde::{Deserialize, Serialize};

use super::WellImage;

/// `well` metadata. Describes all fields of views under a given well.
#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Well {
    /// Specifies the fields of views of the well.
    pub images: Vec<WellImage>,
}

#[cfg(test)]
mod tests {
    use crate::v0_5::get_ome_attribute_from_zarr_group_metadata;

    use super::*;

    #[test]
    fn well_2fields() {
        let json = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/ome-zarr/0.5/examples/well_strict/well_2fields.json"
        ));
        let group_metadata: serde_json::Map<String, serde_json::Value> = serde_json::from_str(json).unwrap();
        let ome_metadata = get_ome_attribute_from_zarr_group_metadata(&group_metadata).unwrap();
        let well = ome_metadata.get("well").unwrap();
        let _well: Well = serde_json::from_value(well.clone()).unwrap();
    }

    #[test]
    fn well_4fields() {
        let json = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/ome-zarr/0.5/examples/well_strict/well_4fields.json"
        ));
        let group_metadata: serde_json::Map<String, serde_json::Value> = serde_json::from_str(json).unwrap();
        let ome_metadata = get_ome_attribute_from_zarr_group_metadata(&group_metadata).unwrap();
        let well = ome_metadata.get("well").unwrap();
        let _well: Well = serde_json::from_value(well.clone()).unwrap();
    }
}
