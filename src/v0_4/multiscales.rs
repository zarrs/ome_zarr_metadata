//! "multiscales" metadata.
//!
//! <https://ngff.openmicroscopy.org/0.4/#multiscale-md>.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};
use validatrix::{Accumulator, Validate};

use crate::{
    v0_4::AxisType,
    validation::{validate_ndims},
    MaybeNDim, NDim,
};

use super::{Axis, CoordinateTransform};

/// `multiscales` element metadata. Describes a multiscale image.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct MultiscaleImage {
    /// The version of the multiscale metadata of the image.
    pub version: monostate::MustBe!("0.4"),
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

impl Validate for MultiscaleImage {
    fn validate_inner(&self, accum: &mut Accumulator) -> usize {
        let mut total = 0;
        accum.prefix.push("axes".into());
        total += valid_axes(accum, &self.axes);
        accum.prefix.pop();
        
        accum.prefix.push("datasets".into());
        total += accum.validate_iter(&self.datasets);
        total += valid_datasets(accum, self.maybe_ndim(), &self.datasets);
        accum.prefix.pop();
        
        if let Some(ct) = self.coordinate_transformations.as_ref() {
            accum.prefix.push("coordinateTransformations".into());
            total += accum.validate_iter(ct);
            total += valid_transforms(accum, self.maybe_ndim(), ct);
            accum.prefix.pop();
        }

        total
    }
}

fn unique_axis_names(accum: &mut Accumulator, axes: &[Axis]) -> usize {
    let mut total = 0;
    let mut names = BTreeSet::default();
    for (idx, a) in axes.iter().enumerate() {
        if !names.insert(a.name.as_str()) {
            accum.add_failure(
                format!("duplicate axis name '{}'", a.name).into(),
                &[idx.into()],
            );
            total += 1;
        }
    }
    total
}

/// ?time, ?channel/custom/null, ?space, space, space
pub(crate) fn valid_axes(accum: &mut Accumulator, axes: &[Axis]) -> usize {
    let mut total = accum.validate_iter(axes);
    total += unique_axis_names(accum, axes);
    if axes.len() < 2 || axes.len() > 5 {
        accum.add_failure(format!("got {} axes, expected 2-5", axes.len()).into(), &[]);
        total += 1;
    }

    let mut done_time = false;
    let mut done_channel_custom = false;
    let mut n_space = 0;

    for (idx, ax) in axes.iter().enumerate() {
        match ax.r#type {
            Some(AxisType::Space) => {
                n_space += 1;
                if n_space > 3 {
                    accum.add_failure(
                        format!("at least {n_space} space axes, should be max 3").into(),
                        &[idx.into()],
                    );
                    total += 1;
                }
                done_time |= true;
                done_channel_custom |= true;
            }
            Some(AxisType::Time) => {
                if done_time || done_channel_custom || n_space > 0 {
                    accum.add_failure("unexpected time axis".into(), &[idx.into()]);
                    total += 1;
                }
                done_time |= true;
            }
            None | Some(AxisType::Channel) | Some(AxisType::Custom(_)) => {
                if done_channel_custom || n_space > 0 {
                    accum.add_failure(
                        "unexpected channel/custom/unknown axis".into(),
                        &[idx.into()],
                    );
                    total += 1;
                }
                done_channel_custom |= true;
                done_time |= true;
            }
        }
    }
    if n_space < 2 {
        accum.add_failure(format!("got {n_space} space axes, expected 2-3").into(), &[]);
        total += 1;
    }
    total
}

pub(crate) fn valid_datasets(accum: &mut Accumulator, expected_ndim: Option<usize>, dss: &[MultiscaleImageDataset]) -> usize {
    let total = validate_ndims(accum, expected_ndim, dss.iter());
    total + accum.validate_iter(dss)
}

pub(crate) fn valid_transforms(accum: &mut Accumulator, expected_ndim: Option<usize>, ct: &[CoordinateTransform]) -> usize {
    let total = validate_ndims(accum, expected_ndim, ct.iter());
    total + accum.validate_iter(ct)
}


impl NDim for &MultiscaleImage {
    fn ndim(&self) -> usize {
        self.axes.len()
    }
}

/// [`MultiscaleImage`] `datasets` element metadata. Describes an individual resolution level.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct MultiscaleImageDataset {
    /// The path to the array for this resolution relative to the current zarr group.
    pub path: String,
    /// A list of transformations that map the data coordinates to the physical coordinates (as specified by "axes") for this resolution level.
    pub coordinate_transformations: Vec<CoordinateTransform>,
}

impl Validate for MultiscaleImageDataset {
    fn validate_inner(&self, accum: &mut Accumulator) -> usize {
        let mut total = 0;
        accum.prefix.push("coordinateTransformations".into());
        total += accum.validate_iter(&self.coordinate_transformations);
        accum.prefix.pop();

        total
    }
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
