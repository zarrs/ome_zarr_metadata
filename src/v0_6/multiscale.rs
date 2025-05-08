//! "multiscales" metadata.
//!
//! <https://ngff.openmicroscopy.org/0.6/#multiscale-md>.

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

// #[cfg(test)]
// mod tests {
//     use crate::v0_6::OmeZarrGroupMetadata;

//     use super::*;

//     #[test]
//     fn multiscale_example() {
//         let json = include_str!(concat!(
//             env!("CARGO_MANIFEST_DIR"),
//             "/ome-zarr/0.6/examples/multiscales_strict/multiscales_example.json"
//         ))
//         .lines()
//         .filter(|line| !line.contains("//")) // Remove comments
//         .collect::<String>();
//         let ome_metadata: OmeZarrGroupMetadata = serde_json::from_str(&json).unwrap();
//         let _multiscales: MultiscaleImage = ome_metadata.attributes.ome.multiscale.unwrap();
//     }

//     #[test]
//     fn multiscale_transformations() {
//         let json = include_str!(concat!(
//             env!("CARGO_MANIFEST_DIR"),
//             "/ome-zarr/0.6/examples/multiscales_strict/multiscales_transformations.json"
//         ));
//         let ome_metadata: OmeZarrGroupMetadata = serde_json::from_str(json).unwrap();
//         let _multiscale: MultiscaleImage = ome_metadata.attributes.ome.multiscale.unwrap();
//     }
// }
