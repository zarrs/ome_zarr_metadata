//! "multiscales" metadata.
//!
//! <https://ngff.openmicroscopy.org/0.5/#multiscale-md>.

use serde::{Deserialize, Serialize};

use super::{Axis, CoordinateTransform, MultiscaleImageDataset, MultiscaleImageMetadata};

/// `multiscales` element metadata. Describes a multiscale image.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct MultiscaleImage {
    /// The name of the multiscale image (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The axes of the multiscale image.
    pub axes: Vec<Axis>,
    /// The datasets describe the arrays storing the individual resolution levels.
    pub datasets: Vec<MultiscaleImageDataset>,
    /// Describes transformations that are applied to all resolution levels in the same manner (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coordinate_transformations: Option<Vec<CoordinateTransform>>,
    /// The type of downscaling method used to generate the multiscale image pyramid (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// A dictionary with additional information about the downscaling method (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<MultiscaleImageMetadata>,
}

#[cfg(test)]
mod tests {
    use crate::v0_5::{get_ome_attribute_from_zarr_group_metadata, Ome};

    use super::*;

    #[test]
    fn multiscales_example() {
        let json = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/ome-zarr/0.5/examples/multiscales_strict/multiscales_example.json"
        ))
        .lines()
        .filter(|line| !line.contains("//")) // Remove comments
        .collect::<String>();

        let group_metadata: serde_json::Map<String, serde_json::Value> =
            serde_json::from_str(&json).unwrap();
        let ome_metadata = get_ome_attribute_from_zarr_group_metadata(&group_metadata).unwrap();
        let ome_metadata: Ome = serde_json::from_value(ome_metadata.clone()).unwrap();
        let _multiscales: Vec<MultiscaleImage> = ome_metadata.multiscales.unwrap();
    }

    #[test]
    fn multiscales_transformations() {
        let json = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/ome-zarr/0.5/examples/multiscales_strict/multiscales_transformations.json"
        ));
        let group_metadata: serde_json::Map<String, serde_json::Value> =
            serde_json::from_str(json).unwrap();
        let ome_metadata = get_ome_attribute_from_zarr_group_metadata(&group_metadata).unwrap();
        let ome_metadata: Ome = serde_json::from_value(ome_metadata.clone()).unwrap();
        let _multiscales: Vec<MultiscaleImage> = ome_metadata.multiscales.unwrap();
    }
}
