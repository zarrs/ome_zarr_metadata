use serde::{Deserialize, Serialize};

use crate::next::CoordinateTransformOuter;

/// Explicitly defined forward and reverse transformations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bijection {
    /// Forward transformation.
    pub forward: Box<CoordinateTransformOuter>,
    /// Inverse transformation.
    pub inverse: Box<CoordinateTransformOuter>,
}

impl validatrix::Validate for Bijection {
    fn validate_inner(&self, accum: &mut validatrix::Accumulator) {
        accum.with_key("inverse", |acc| {
            if let (Some(fwd), Some(inv)) =
                (self.forward.input_system(), self.inverse.output_system())
            {
                if fwd != inv {
                    acc.add_failure(
                        "forward input and inverse output coordinate systems do not match",
                    );
                }
            }
            if let (Some(fwd), Some(inv)) =
                (self.forward.output_system(), self.inverse.input_system())
            {
                if fwd != inv {
                    acc.add_failure(
                        "forward output and inverse input coordinate systems do not match",
                    );
                }
            }
        });
    }
}

impl super::TransformationType for Bijection {
    fn invertible(&self) -> Option<bool> {
        Some(true)
    }

    fn input_ndim(&self) -> Option<usize> {
        self.forward.input_ndim()
    }

    fn output_ndim(&self) -> Option<usize> {
        self.forward.output_ndim()
    }

    fn input_system(&self) -> Option<&str> {
        // todo: check that it is the output of the inverse
        self.forward
            .input_system()
            .or_else(|| self.inverse.output_system())
    }

    fn output_system(&self) -> Option<&str> {
        // todo: check that it is the input of the inverse
        self.forward
            .output_system()
            .or_else(|| self.inverse.input_system())
    }
}

impl From<Bijection> for super::CoordinateTransform {
    fn from(value: Bijection) -> Self {
        Self::Bijection(value)
    }
}
