//! Matrix transformations: affine and rotation.

use serde::{Deserialize, Serialize};
use validatrix::{Accumulator, Validate};

use crate::linalg::Matrix;

use super::TransformType;

/// `affine` transformation: either an inline matrix or a path to a zarr array.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Affine {
    /// Rows of the upper `M x (N+1)` part of a homogeneous affine matrix.
    Inline {
        /// Matrix rows.
        affine: Matrix,
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
            Affine::Inline { .. } => (),
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
            Affine::Inline { affine } => Some(affine.cols().saturating_sub(1)),
            _ => None,
        }
    }

    fn output_ndim(&self) -> Option<usize> {
        match self {
            Affine::Inline { affine } => Some(affine.rows()),
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
        rotation: Matrix,
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
                if rotation.is_orthogonal(1e-6) {
                    let det = rotation
                        .determinant()
                        .expect("orthogonal matrix must be square");
                    if (det - 1.0).abs() > 1e-6 {
                        a.add_failure(format!("matrix determinant must be 1; got {det}"));
                    }
                } else {
                    a.add_failure("matrix is not orthogonal");
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
            Rotation::Inline { rotation } => Some(rotation.rows()),
            _ => None,
        }
    }
}
