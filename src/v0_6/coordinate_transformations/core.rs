//! Core transformations: identity, scale, translation, sequence, and bijection.

use serde::{Deserialize, Serialize};
use validatrix::{Accumulator, Validate};

use super::{Transform, TransformType};
use crate::v0_5 as prev;

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

/// `sequence` transformation.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Sequence {
    /// Transformations to apply in order.
    pub transformations: Vec<Transform>,
}

impl Validate for Sequence {
    fn validate_inner(&self, accum: &mut Accumulator) {
        accum.with_key("transformations", |a| {
            if self.transformations.is_empty() {
                a.add_failure("empty");
            }
            a.validate_iter(&self.transformations);
            for (idx, pair) in self.transformations.windows(2).enumerate() {
                if let (Some(o), Some(i)) = (pair[0].output_ndim(), pair[1].input_ndim()) {
                    if o != i {
                        a.add_failure_at(
                            idx + 1,
                            format!("input dimensionality {i} does not match previous output {o}"),
                        );
                    }
                }
            }
        });
    }
}

impl TransformType for Sequence {
    fn input_ndim(&self) -> Option<usize> {
        self.transformations.first().and_then(|t| t.input_ndim())
    }

    fn output_ndim(&self) -> Option<usize> {
        self.transformations.last().and_then(|t| t.output_ndim())
    }
}

/// `bijection` transformation.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Bijection {
    /// Forward transformation.
    pub forward: Box<Transform>,
    /// Inverse transformation.
    pub inverse: Box<Transform>,
}

fn opts_compatible(a: Option<usize>, b: Option<usize>) -> bool {
    match (a, b) {
        (Some(x), Some(y)) => x == y,
        _ => true,
    }
}

impl Validate for Bijection {
    fn validate_inner(&self, accum: &mut Accumulator) {
        accum.validate_member_at("forward", self.forward.as_ref());
        accum.validate_member_at("inverse", self.inverse.as_ref());
        if !opts_compatible(self.forward.output_ndim(), self.inverse.input_ndim()) {
            accum.add_failure_at(
                "forward",
                format!(
                    "output dimensionality {} does not match inverse input dimensionality {}",
                    self.forward.output_ndim().unwrap_or_default(),
                    self.inverse.input_ndim().unwrap_or_default()
                ),
            );
        }
        if !opts_compatible(self.inverse.output_ndim(), self.forward.input_ndim()) {
            accum.add_failure_at(
                "inverse",
                format!(
                    "output dimensionality {} does not match forward input dimensionality {}",
                    self.inverse.output_ndim().unwrap_or_default(),
                    self.forward.input_ndim().unwrap_or_default()
                ),
            );
        }
    }
}

impl TransformType for Bijection {
    fn input_ndim(&self) -> Option<usize> {
        self.forward.input_ndim().or(self.inverse.output_ndim())
    }

    fn output_ndim(&self) -> Option<usize> {
        self.forward.output_ndim().or(self.inverse.input_ndim())
    }
}
