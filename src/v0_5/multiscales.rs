//! "multiscales" metadata.
//!
//! <https://ngff.openmicroscopy.org/0.5/#multiscale-md>.

use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

use crate::{validation::ValidationResult, MaybeNDim, NDim};

use super::{Axis, CoordinateTransform, MultiscaleImageDataset, MultiscaleImageMetadata};
use crate::v0_4::multiscales::{valid_axes, valid_datasets, valid_transforms};

/// Check that all dimensionalities are consistent.
fn valid_multiscale(img: &MultiscaleImage) -> ValidationResult {
    for ds in img.datasets.iter() {
        if img.ndim_conflicts(ds).is_some() {
            return Err(ValidationError::new(
                "dimensionality conflict between multiscale axes and dataset",
            ));
        }
    }
    for ct in img.coordinate_transformations.iter().flatten() {
        if img.ndim_conflicts(ct).is_some() {
            return Err(ValidationError::new(
                "dimensionality conflict between multiscale axes and coordinate transform",
            ));
        }
    }
    Ok(())
}

/// `multiscales` element metadata. Describes a multiscale image.
#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
#[serde(rename_all = "camelCase")]
#[validate(schema(function = "valid_multiscale"))]
pub struct MultiscaleImage {
    /// The name of the multiscale image (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The axes of the multiscale image.
    #[validate(length(min = 2, max = 5), custom(function = "valid_axes"))]
    // #[validate(nested)]
    pub axes: Vec<Axis>,
    /// The datasets describe the arrays storing the individual resolution levels.
    #[validate(nested, length(min = 1), custom(function = "valid_datasets"))]
    pub datasets: Vec<MultiscaleImageDataset>,
    /// Describes transformations that are applied to all resolution levels in the same manner (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested, custom(function = "valid_transforms"))]
    pub coordinate_transformations: Option<Vec<CoordinateTransform>>,
    /// The type of downscaling method used to generate the multiscale image pyramid (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// A dictionary with additional information about the downscaling method (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<MultiscaleImageMetadata>,
}

impl NDim for MultiscaleImage {
    fn ndim(&self) -> usize {
        self.axes.len()
    }
}

impl From<crate::v0_4::MultiscaleImage> for MultiscaleImage {
    fn from(value: crate::v0_4::MultiscaleImage) -> Self {
        Self {
            name: value.name,
            axes: value.axes,
            datasets: value.datasets,
            coordinate_transformations: value.coordinate_transformations,
            r#type: value.r#type,
            metadata: value.metadata,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::v0_5::OmeZarrGroupMetadata;

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
        let ome_metadata: OmeZarrGroupMetadata = serde_json::from_str(&json).unwrap();
        let _multiscales: Vec<MultiscaleImage> = ome_metadata.attributes.ome.multiscales.unwrap();
    }

    #[test]
    fn multiscales_transformations() {
        let json = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/ome-zarr/0.5/examples/multiscales_strict/multiscales_transformations.json"
        ));
        let ome_metadata: OmeZarrGroupMetadata = serde_json::from_str(json).unwrap();
        let _multiscales: Vec<MultiscaleImage> = ome_metadata.attributes.ome.multiscales.unwrap();
    }
}
