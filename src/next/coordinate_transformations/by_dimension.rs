use serde::{Deserialize, Serialize};

/// Build a high dimensional transformation using lower dimensional transformations on subsets of dimensions
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct ByDimension;

impl validatrix::Validate for ByDimension {
    fn validate_inner(&self, _accum: &mut validatrix::Accumulator) {
        todo!()
    }
}

impl super::TransformationType for ByDimension {
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

impl From<ByDimension> for super::CoordinateTransform {
    fn from(value: ByDimension) -> Self {
        Self::ByDimension(value)
    }
}
