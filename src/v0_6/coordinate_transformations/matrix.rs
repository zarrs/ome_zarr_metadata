//! Matrix transformations: affine and rotation.

use serde::{Deserialize, Serialize};
use validatrix::{Accumulator, Validate};

use super::TransformType;

/// `affine` transformation: either an inline matrix or a path to a zarr array.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Affine {
    /// Rows of the upper `M x (N+1)` part of a homogeneous affine matrix.
    Inline {
        /// Matrix rows.
        affine: Vec<Vec<f64>>,
    },
    /// Path to a 2D zarr array containing the matrix.
    Path {
        /// Path to the array.
        path: String,
    },
}

impl Validate for Affine {
    fn validate_inner(&self, accum: &mut Accumulator) {
        match self {
            Affine::Inline { affine } => accum.with_key("affine", |a| {
                let Some(first) = affine.first() else {
                    a.add_failure("empty matrix");
                    return;
                };
                if first.len() < 2 {
                    a.add_failure("rows must have at least 2 columns");
                }
                for (idx, row) in affine.iter().enumerate() {
                    if row.len() != first.len() {
                        a.add_failure_at(idx, "inconsistent row length");
                    }
                }
            }),
            Affine::Path { path } => {
                if path.is_empty() {
                    accum.add_failure_at("path", "empty");
                }
            }
        }
    }
}

impl TransformType for Affine {
    fn input_ndim(&self) -> Option<usize> {
        match self {
            Affine::Inline { affine } => affine.first().map(|r| r.len().saturating_sub(1)),
            _ => None,
        }
    }

    fn output_ndim(&self) -> Option<usize> {
        match self {
            Affine::Inline { affine } => Some(affine.len()),
            _ => None,
        }
    }
}

/// `rotation` transformation: either an inline matrix or a path to a zarr array.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Rotation {
    /// Rows of an `N x N` rotation matrix.
    Inline {
        /// Matrix rows.
        rotation: Vec<Vec<f64>>,
    },
    /// Path to a 2D zarr array containing the matrix.
    Path {
        /// Path to the array.
        path: String,
    },
}

impl Validate for Rotation {
    fn validate_inner(&self, accum: &mut Accumulator) {
        match self {
            Rotation::Inline { rotation } => accum.with_key("rotation", |a| {
                let n = rotation.len();
                if n == 0 {
                    a.add_failure("empty matrix");
                }
                for (idx, row) in rotation.iter().enumerate() {
                    if row.len() != n {
                        a.add_failure_at(idx, "matrix is not square");
                    }
                }
            }),
            Rotation::Path { path } => {
                if path.is_empty() {
                    accum.add_failure_at("path", "empty");
                }
            }
        }
    }
}

impl TransformType for Rotation {
    fn input_ndim(&self) -> Option<usize> {
        match self {
            Rotation::Inline { rotation } => Some(rotation.len()),
            _ => None,
        }
    }
}
