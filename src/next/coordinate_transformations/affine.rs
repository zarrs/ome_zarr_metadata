use std::{collections::BTreeSet, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::{next::TransformationType, MaybeNDim};

/// Matrix transformation from N-dimensional inputs to M-dimensional outputs.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Affine {
    /// Path to Zarr array
    Path(PathBuf),
    /// Length M list of length N+1 lists
    Affine(Vec<Vec<f64>>),
}

impl MaybeNDim for Affine {
    fn maybe_ndim(&self) -> Option<usize> {
        self.input_ndim().and_then(|id| {
            if self.output_ndim()? == id {
                Some(id)
            } else {
                None
            }
        })
    }
}

impl validatrix::Validate for Affine {
    fn validate_inner(&self, accum: &mut validatrix::Accumulator) {
        if let Affine::Affine(items) = self {
            accum.with_key("affine", |acc| {
                if items.is_empty() {
                    acc.add_failure("zero-dimensional output");
                }
                let lengths: BTreeSet<_> = items.iter().map(|item| item.len()).collect();
                if lengths.contains(&0) {
                    acc.add_failure("zero-dimensional input");
                }
                if lengths.len() > 1 {
                    acc.add_failure("ragged matrix");
                }
            });
        }
    }
}

impl TransformationType for Affine {
    fn invertible(&self) -> Option<bool> {
        todo!()
    }

    fn input_ndim(&self) -> Option<usize> {
        match self {
            Affine::Path(_) => None,
            Affine::Affine(items) => Some(items.first()?.len()),
        }
    }

    fn output_ndim(&self) -> Option<usize> {
        match self {
            Affine::Path(_) => None,
            Affine::Affine(items) => Some(items.len()),
        }
    }
}
