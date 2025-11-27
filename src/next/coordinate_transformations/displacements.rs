use serde::{Deserialize, Serialize};

/// A displacement field where a pixel contains a displacement from a location in another array.
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct Displacements;

impl validatrix::Validate for Displacements {
    fn validate_inner(&self, _accum: &mut validatrix::Accumulator) {
        todo!()
    }
}

impl super::TransformationType for Displacements {
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

impl From<Displacements> for super::CoordinateTransformInner {
    fn from(value: Displacements) -> Self {
        Self::Displacements(value)
    }
}
