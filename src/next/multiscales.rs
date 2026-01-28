use serde::{Deserialize, Serialize};
use validatrix::{Accumulator, Validate};

use crate::{v0_5, MaybeNDim, NDim};

use super::{Axis, CoordinateTransform, MultiscaleImageDataset, MultiscaleImageMetadata};
use crate::v0_4::multiscales::unique_axis_names;
use crate::v0_5::multiscales::{valid_datasets, valid_transforms};

/// `multiscales` element metadata. Describes a multiscale image.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct MultiscaleImage {
    /// The version of the multiscale metadata of the image.
    pub version: super::ConstrainedVersion,
    /// The name of the multiscale image (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The axes of the multiscale image.
    ///
    /// ## Differences from v0.5
    /// - axis count, order, and types are unconstrained (RFC-3)
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
    fn validate_inner(&self, accum: &mut Accumulator) {
        accum.with_key("axes", |a| valid_axes(a, &self.axes));

        accum.with_key("datasets", |a| {
            valid_datasets(a, self.maybe_ndim(), &self.datasets);
        });

        if let Some(ct) = self.coordinate_transformations.as_ref() {
            accum.with_key("coordinateTransformations", |a| {
                valid_transforms(a, self.maybe_ndim(), ct);
            });
        }
    }
}

pub(crate) fn valid_axes(accum: &mut Accumulator, axes: &[Axis]) {
    accum.validate_iter(axes);
    unique_axis_names(accum, axes);
}

impl NDim for &MultiscaleImage {
    fn ndim(&self) -> usize {
        self.axes.len()
    }
}

impl From<v0_5::MultiscaleImage> for MultiscaleImage {
    fn from(value: v0_5::MultiscaleImage) -> Self {
        Self {
            version: super::ConstrainedVersion::default(),
            name: value.name,
            axes: value.axes,
            datasets: value.datasets,
            coordinate_transformations: value.coordinate_transformations,
            r#type: value.r#type,
            metadata: value.metadata,
        }
    }
}
