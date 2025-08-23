//! "multiscales" metadata.
//!
//! <https://ngff.openmicroscopy.org/0.4/#multiscale-md>.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

use crate::{
    v0_4::AxisType,
    validation::{new_validation_err, validate_ndims, ValidationResult, DUPLICATE_AXES},
    MaybeNDim, NDim,
};

use super::{Axis, CoordinateTransform};

/// `multiscales` element metadata. Describes a multiscale image.
#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
#[validate(schema(function = "valid_multiscale"))]
pub struct MultiscaleImage {
    /// The version of the multiscale metadata of the image.
    pub version: monostate::MustBe!("0.4"),
    /// The name of the multiscale image (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The axes of the multiscale image.
    #[validate(nested, length(min = 2, max = 5), custom(function = "valid_axes"))]
    pub axes: Vec<Axis>,
    #[validate(nested, custom(function = "valid_datasets"))]
    /// The datasets describe the arrays storing the individual resolution levels.
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

fn unique_axis_names(axes: &[Axis]) -> ValidationResult {
    let mut names = BTreeSet::default();
    for a in axes {
        if !names.insert(a.name.as_str()) {
            return new_validation_err(
                DUPLICATE_AXES,
                format!("axis name '{}' is duplicated", a.name),
            );
        }
    }
    Ok(())
}

/// Checking for number of axes should happen outside of this function.
pub(crate) fn valid_axes(axes: &[Axis]) -> ValidationResult {
    if axes.len() < 2 || axes.len() > 5 {
        return new_validation_err("axis_count", format!("got {} axes", axes.len()));
    }
    unique_axis_names(axes)?;
    let mut ax2: Vec<_> = axes.iter().collect();
    for _ in 0..2 {
        let ax = ax2.pop().expect("already check number of axes");
        if ax.r#type != Some(AxisType::Space) {
            return new_validation_err("axis_type", "not enough trailing space axes");
        }
    }
    let Some(last) = ax2.last() else {
        return Ok(());
    };
    if last.r#type == Some(AxisType::Space) {
        ax2.pop();
    }
    // should be max 1 time, 1 channel/custom
    if ax2.len() == 1 {
        return match ax2.first().unwrap().r#type {
            Some(AxisType::Space) => new_validation_err("axis_type", "too many space axes"),
            _ => Ok(()),
        };
    }
    let mut it = ax2.into_iter();
    let t = it.next().unwrap();
    if t.r#type != Some(AxisType::Time) {
        return new_validation_err(
            "axis_type",
            format!("expected time axis in first position, got {:?}", t.r#type),
        );
    }
    let c = it.next().unwrap();
    match c.r#type {
        Some(AxisType::Space | AxisType::Time) => new_validation_err(
            "axis_type",
            format!("expected channel/custom/null axis, got {:?}", c.r#type),
        ),
        _ => Ok(()),
    }
}

pub(crate) fn valid_datasets(dss: &[MultiscaleImageDataset]) -> Result<(), ValidationError> {
    if dss.is_empty() {
        return new_validation_err("dataset_count", "zero datasets found");
    }
    let mut ndim_iter = dss.iter().filter_map(|d| d.maybe_ndim());
    let Some(first) = ndim_iter.next() else {
        return Ok(());
    };
    for other in ndim_iter {
        if other != first {
            return Err(ValidationError::new(
                "dimensionality conflict within multiscale datasets",
            ));
        }
    }
    Ok(())
}

pub(crate) fn valid_transforms(ct: &[CoordinateTransform]) -> Result<(), ValidationError> {
    validate_ndims(ct, "dimensionality conflict within coordinate transforms")?;
    Ok(())
}

/// Check that all dimensionalities are consistent.
fn valid_multiscale(img: &MultiscaleImage) -> Result<(), ValidationError> {
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

impl NDim for &MultiscaleImage {
    fn ndim(&self) -> usize {
        self.axes.len()
    }
}

/// [`MultiscaleImage`] `datasets` element metadata. Describes an individual resolution level.
#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct MultiscaleImageDataset {
    /// The path to the array for this resolution relative to the current zarr group.
    pub path: String,
    /// A list of transformations that map the data coordinates to the physical coordinates (as specified by "axes") for this resolution level.
    #[validate(nested, custom(function = "valid_transforms"))]
    pub coordinate_transformations: Vec<CoordinateTransform>,
}

impl MaybeNDim for MultiscaleImageDataset {
    fn maybe_ndim(&self) -> Option<usize> {
        self.coordinate_transformations
            .iter()
            .filter_map(|c| c.maybe_ndim())
            .next()
    }
}

/// [`MultiscaleImage`] `metadata` metadata. Information about the downscaling method.
///
/// E.g. fields: `description`, `method`, `version`, `args`, `kwargs`.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MultiscaleImageMetadata(pub serde_json::Map<String, serde_json::Value>);

#[cfg(test)]
mod tests {
    use crate::v0_4::OmeNgffGroupAttributes;

    use super::*;

    #[test]
    fn multiscales_example() {
        let json = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/ome-zarr/0.4/examples/multiscales_strict/multiscales_example.json"
        ))
        .lines()
        .filter(|line| !line.contains("//")) // Remove comments
        .collect::<String>();
        let ome_metadata: OmeNgffGroupAttributes = serde_json::from_str(&json).unwrap();
        let _multiscales: Vec<MultiscaleImage> = ome_metadata.multiscales.unwrap();
    }

    #[test]
    fn multiscales_transformations() {
        let json = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/ome-zarr/0.4/examples/multiscales_strict/multiscales_transformations.json"
        ));
        let ome_metadata: OmeNgffGroupAttributes = serde_json::from_str(json).unwrap();
        let _multiscales: Vec<MultiscaleImage> = ome_metadata.multiscales.unwrap();
    }
}
