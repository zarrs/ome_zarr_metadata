use serde::{Deserialize, Serialize};

use crate::MaybeNDim;

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct Coordinates;

impl MaybeNDim for Coordinates {
    fn maybe_ndim(&self) -> Option<usize> {
        None
    }
}

impl validatrix::Validate for Coordinates {
    fn validate_inner(&self, _accum: &mut validatrix::Accumulator) {
        todo!()
    }
}

impl super::TransformationType for Coordinates {
    fn invertible(&self) -> Option<bool> {
        todo!()
    }

    fn input_ndim(&self) -> Option<usize> {
        todo!()
    }

    fn output_ndim(&self) -> Option<usize> {
        todo!()
    }
}

impl From<Coordinates> for super::CoordinateTransform {
    fn from(value: Coordinates) -> Self {
        Self::Coordinates(value)
    }
}
