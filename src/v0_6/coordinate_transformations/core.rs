//! Core transformations: identity, scale, and translation.

use serde::{Deserialize, Serialize};
use validatrix::{Accumulator, Validate};

use super::TransformType;
use crate::{util::ValidNum, v0_5 as prev};

/// `identity` transformation.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Identity;

impl Validate for Identity {
    fn validate_inner(&self, _accum: &mut Accumulator) {}
}

impl TransformType for Identity {
    fn input_ndim(&self) -> Option<usize> {
        None
    }
}

/// `translation` transformation.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Translation {
    /// Values to add to each coordinate.
    pub translation: Vec<f64>,
}

impl TryFrom<prev::CoordinateTransformTranslation> for Translation {
    type Error = crate::Error;

    fn try_from(value: prev::CoordinateTransformTranslation) -> Result<Self, Self::Error> {
        match value {
            prev::CoordinateTransformTranslation::List { translation } => Ok(Self { translation }),
            prev::CoordinateTransformTranslation::Path { .. } => Err(Self::Error::requires_zarr(
                "v0.6 does not support path-form translations",
            )),
        }
    }
}

impl Validate for Translation {
    fn validate_inner(&self, accum: &mut Accumulator) {
        if self.translation.is_empty() {
            accum.add_failure_at("translation", "empty");
        }
        accum.with_key("translation", |a| {
            for (i, &v) in self.translation.iter().enumerate() {
                a.validate_member_at(i, &ValidNum(v));
            }
        });
    }
}

impl From<Vec<f64>> for Translation {
    fn from(translation: Vec<f64>) -> Self {
        Self { translation }
    }
}

impl TransformType for Translation {
    fn input_ndim(&self) -> Option<usize> {
        Some(self.translation.len())
    }
}

/// `scale` transformation.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Scale {
    /// Values to multiply each coordinate by.
    pub scale: Vec<f64>,
}

impl TryFrom<prev::CoordinateTransformScale> for Scale {
    type Error = crate::Error;

    fn try_from(value: prev::CoordinateTransformScale) -> Result<Self, Self::Error> {
        match value {
            prev::CoordinateTransformScale::List { scale } => Ok(Self { scale }),
            prev::CoordinateTransformScale::Path { .. } => Err(Self::Error::requires_zarr(
                "v0.6 does not support path-form scales",
            )),
        }
    }
}

impl Validate for Scale {
    fn validate_inner(&self, accum: &mut Accumulator) {
        if self.scale.is_empty() {
            accum.add_failure_at("scale", "empty");
        }
        accum.with_key("scale", |a| {
            for (i, &v) in self.scale.iter().enumerate() {
                a.validate_member_at(i, &ValidNum(v));
            }
        });
    }
}

impl From<Vec<f64>> for Scale {
    fn from(scale: Vec<f64>) -> Self {
        Self { scale }
    }
}

impl TransformType for Scale {
    fn input_ndim(&self) -> Option<usize> {
        Some(self.scale.len())
    }
}
