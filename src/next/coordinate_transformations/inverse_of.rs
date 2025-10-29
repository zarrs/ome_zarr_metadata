use serde::{Deserialize, Serialize};

use crate::next::TransformationType;

/// Transform which is the inverse of another transform.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InverseOf {
    /// Transform to invert
    pub transformation: Box<super::CoordinateTransformOuter>,
}

impl validatrix::Validate for InverseOf {
    fn validate_inner(&self, accum: &mut validatrix::Accumulator) {
        accum.with_key("transformation", |acc| {
            let t = self.transformation.as_ref();
            t.validate_inner(acc);
            if t.invertible() == Some(false) {
                acc.add_failure("not invertible");
            }
        });
        accum.validate_member_at("transformation", self.transformation.as_ref());
    }
}

impl TransformationType for InverseOf {
    fn invertible(&self) -> Option<bool> {
        Some(true)
    }

    fn input_ndim(&self) -> Option<usize> {
        self.transformation.output_ndim()
    }

    fn output_ndim(&self) -> Option<usize> {
        self.transformation.input_ndim()
    }

    fn input_system(&self) -> Option<&str> {
        self.transformation.output_system()
    }

    fn output_system(&self) -> Option<&str> {
        self.transformation.input_system()
    }
}

impl From<InverseOf> for super::CoordinateTransform {
    fn from(value: InverseOf) -> Self {
        Self::InverseOf(value)
    }
}
