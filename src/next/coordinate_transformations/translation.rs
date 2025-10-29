use crate::{next::TransformationType, MaybeNDim};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// [`CoordinateTransform`] `translation` type metadata.
#[allow(missing_docs)]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum Translation {
    /// A list of floats.
    Translation(Vec<f32>),
    /// A path to binary data at a location in this container.
    Path(PathBuf),
}

impl MaybeNDim for Translation {
    fn maybe_ndim(&self) -> Option<usize> {
        if let Translation::Translation(translation) = self {
            Some(translation.len())
        } else {
            None
        }
    }
}

impl From<Vec<f32>> for Translation {
    fn from(translation: Vec<f32>) -> Self {
        Translation::Translation(translation)
    }
}

impl From<PathBuf> for Translation {
    fn from(path: PathBuf) -> Self {
        Translation::Path(path)
    }
}

impl validatrix::Validate for Translation {
    fn validate_inner(&self, _accum: &mut validatrix::Accumulator) {}
}

impl TransformationType for Translation {
    fn invertible(&self) -> Option<bool> {
        Some(true)
    }
    fn input_ndim(&self) -> Option<usize> {
        self.maybe_ndim()
    }
    fn output_ndim(&self) -> Option<usize> {
        self.maybe_ndim()
    }
}

impl From<Translation> for super::CoordinateTransformInner {
    fn from(value: Translation) -> Self {
        Self::Translation(value)
    }
}
