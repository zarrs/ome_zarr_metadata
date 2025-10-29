use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::{next::TransformationType, MaybeNDim};

/// [`CoordinateTransform`] `scale` type metadata.
#[allow(missing_docs)]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum Scale {
    /// A list of floats.
    Scale(Vec<f32>),
    /// A path to binary data at a location in this container.
    Path(PathBuf),
}

impl MaybeNDim for Scale {
    fn maybe_ndim(&self) -> Option<usize> {
        if let Scale::Scale(scale) = self {
            Some(scale.len())
        } else {
            None
        }
    }
}

impl From<Vec<f32>> for Scale {
    fn from(scale: Vec<f32>) -> Self {
        Scale::Scale(scale)
    }
}

impl From<PathBuf> for Scale {
    fn from(path: PathBuf) -> Self {
        Scale::Path(path)
    }
}

impl validatrix::Validate for Scale {
    fn validate_inner(&self, accum: &mut validatrix::Accumulator) {
        if let Self::Scale(values) = self {
            accum.with_key("scale", |acc| {
                for (idx, val) in values.iter().enumerate() {
                    acc.with_key(idx, |a| {
                        if val.is_nan() {
                            a.add_failure("scale value is NaN");
                        }
                        if val.is_infinite() {
                            a.add_failure("scale value is infinite");
                        }
                        if val < &0.0 {
                            a.add_failure("scale value is negative");
                        }
                    });
                }
            });
        }
    }
}

impl TransformationType for Scale {
    fn invertible(&self) -> Option<bool> {
        match self {
            Scale::Scale(items) => Some(items.iter().all(|s| s > &0.0)),
            Scale::Path(_) => None,
        }
    }

    fn input_ndim(&self) -> Option<usize> {
        self.maybe_ndim()
    }

    fn output_ndim(&self) -> Option<usize> {
        self.maybe_ndim()
    }
}

impl From<Scale> for super::CoordinateTransformInner {
    fn from(value: Scale) -> Self {
        Self::Scale(value)
    }
}
