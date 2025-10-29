use serde::{Deserialize, Serialize};

/// Rotation matrix.
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct Rotation;

impl validatrix::Validate for Rotation {
    fn validate_inner(&self, _accum: &mut validatrix::Accumulator) {
        todo!()
    }
}

impl super::TransformationType for Rotation {
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

impl From<Rotation> for super::CoordinateTransform {
    fn from(value: Rotation) -> Self {
        Self::Rotation(value)
    }
}
