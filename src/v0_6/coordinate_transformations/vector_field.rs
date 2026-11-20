//! Vector field transformations: coordinates and displacements.

use serde::{Deserialize, Serialize};
use validatrix::{Accumulator, Validate};

use super::TransformType;

/// Interpolation method for coordinate and displacement fields.
#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Interpolation {
    /// Nearest-neighbour interpolation.
    #[serde(rename = "nearest")]
    Nearest,
    /// Linear interpolation (default).
    #[default]
    #[serde(rename = "linear")]
    Linear,
    /// Cubic B-spline interpolation.
    #[serde(rename = "bspline-cubic")]
    BsplineCubic,
    /// Other interpolation method.
    #[serde(untagged)]
    Custom(String),
}

/// `coordinates` or `displacements` transformation, stored in a zarr array.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FieldTransform {
    /// Path to the multiscale group containing the field.
    pub path: String,
    /// Interpolation used between field values; defaults to linear.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interpolation: Option<Interpolation>,
}

impl Validate for FieldTransform {
    fn validate_inner(&self, accum: &mut Accumulator) {
        if self.path.is_empty() {
            accum.add_failure_at("path", "empty");
        }
    }
}

macro_rules! field_transform_newtype {
    ($(#[$meta:meta])* $name:ident) => {
        $(#[$meta])*
        #[derive(Serialize, Deserialize, Debug, Clone)]
        #[serde(transparent)]
        pub struct $name(pub FieldTransform);

        impl std::ops::Deref for $name {
            type Target = FieldTransform;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl std::ops::DerefMut for $name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }

        impl From<FieldTransform> for $name {
            fn from(value: FieldTransform) -> Self {
                Self(value)
            }
        }

        impl Validate for $name {
            fn validate_inner(&self, accum: &mut Accumulator) {
                self.0.validate_inner(accum);
            }
        }

        impl TransformType for $name {
            fn input_ndim(&self) -> Option<usize> {
                None
            }
        }
    };
}

field_transform_newtype!(
    /// `coordinates` transformation: a coordinate field stored in a zarr array.
    Coordinates
);
field_transform_newtype!(
    /// `displacements` transformation: a displacement field stored in a zarr array.
    Displacements
);
