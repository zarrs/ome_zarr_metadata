use serde::{Deserialize, Serialize};

use super::CoordinateTransform;
use crate::next::TransformationType;

/// Sequence of other transformations.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Sequence {
    /// List of transformations to be applied.
    pub transformations: Vec<CoordinateTransform>,
}

impl validatrix::Validate for Sequence {
    fn validate_inner(&self, accum: &mut validatrix::Accumulator) {
        accum.with_key("transformations", |acc| {
            if self.transformations.is_empty() {
                acc.add_failure("empty sequence of transformations");
            }
            acc.validate_iter(&self.transformations);
            let mut last_output = None;
            let mut last_output_dim = None;
            for (idx, t) in self.transformations.iter().enumerate() {
                if matches!(t.config, super::CoordinateTransformInner::Sequence(_)) {
                    acc.add_failure_at(idx, "sequences cannot contain sequences");
                }

                if let (Some(last_out), Some(this_inp)) = (last_output, t.input_system()) {
                    if last_out != this_inp {
                        acc.add_failure_at(idx, "input coordinate system does not match previous transformation's output");
                    }
                }
                last_output = t.output_system();

                if let (Some(last_out), Some(this_inp)) = (last_output_dim, t.input_ndim()) {
                    if last_out != this_inp {
                        acc.add_failure_at(idx, "input dimensionality does not match previous transformation's output dimensionality");
                    }
                }
                last_output_dim = t.output_ndim()
            }
        });
    }
}

impl TransformationType for Sequence {
    fn invertible(&self) -> Option<bool> {
        for t in self.transformations.iter() {
            let invertible = t.invertible();
            if !matches!(invertible, Some(true)) {
                return invertible;
            }
        }
        Some(true)
    }

    fn input_ndim(&self) -> Option<usize> {
        self.transformations.first().and_then(|t| t.input_ndim())
    }

    fn input_system(&self) -> Option<&str> {
        self.transformations.first().and_then(|t| t.input_system())
    }

    fn output_ndim(&self) -> Option<usize> {
        self.transformations.last().and_then(|t| t.output_ndim())
    }

    fn output_system(&self) -> Option<&str> {
        self.transformations.last().and_then(|t| t.output_system())
    }
}

impl From<Sequence> for super::CoordinateTransformInner {
    fn from(value: Sequence) -> Self {
        Self::Sequence(value)
    }
}
