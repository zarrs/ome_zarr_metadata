use serde::{Deserialize, Serialize};
use validatrix::Validate;

use crate::MaybeNDim;

/// A no-op transformation.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Identity;

impl MaybeNDim for Identity {
    fn maybe_ndim(&self) -> Option<usize> {
        None
    }
}

impl Validate for Identity {
    fn validate_inner(&self, _accum: &mut validatrix::Accumulator) {}
}

impl super::TransformationType for Identity {
    fn invertible(&self) -> Option<bool> {
        Some(true)
    }

    fn input_ndim(&self) -> Option<usize> {
        None
    }

    fn output_ndim(&self) -> Option<usize> {
        None
    }
}

impl From<Identity> for super::CoordinateTransformInner {
    fn from(value: Identity) -> Self {
        Self::Identity(value)
    }
}
