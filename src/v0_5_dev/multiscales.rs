//! "multiscales" metadata.
//!
//! <https://ngff.openmicroscopy.org/latest/#multiscale-md>.

use serde::{Deserialize, Serialize};

use super::{Axis, CoordinateTransform, MultiscaleImageDataset, MultiscaleImageMetadata};

/// `multiscales` element metadata. Describes a multiscale image.
#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct MultiscaleImage {
    /// The version of the multiscale metadata of the image.
    pub version: monostate::MustBe!("0.5-dev"),
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
    use super::*;

    #[test]
    fn multiscales_example() {
        let json = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/ome-zarr/latest/examples/multiscales_strict/multiscales_example.json"
        ))
        .lines()
        .filter(|line| !line.contains("//")) // Remove comments
        .collect::<String>();

        let map: serde_json::Map<String, serde_json::Value> = serde_json::from_str(&json).unwrap();
        let multiscales = map.get("multiscales").unwrap();
        let _multiscales: Vec<MultiscaleImage> =
            serde_json::from_value(multiscales.clone()).unwrap();
    }

    #[test]
    fn multiscales_transformations() {
        let json = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/ome-zarr/latest/examples/multiscales_strict/multiscales_transformations.json"
        ));
        let map: serde_json::Map<String, serde_json::Value> = serde_json::from_str(json).unwrap();
        let multiscales = map.get("multiscales").unwrap();
        let _multiscales: Vec<MultiscaleImage> =
            serde_json::from_value(multiscales.clone()).unwrap();
    }
}
