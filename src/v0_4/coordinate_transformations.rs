//! "coordinateTransformations" metadata.
//!
//! <https://ngff.openmicroscopy.org/0.4/#trafo-md>.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

use crate::{validation::DISALLOWED_TRANSFORM, MaybeNDim};

/// `coordinate_transformations` element metadata. Represents a single coordinate transformation.
///
/// It must contain the field "type".
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum CoordinateTransform {
    /// The identity transformation.
    Identity,
    /// A translation vector.
    Translation(CoordinateTransformTranslation),
    /// A scale vector.
    Scale(CoordinateTransformScale),
}

impl Validate for CoordinateTransform {
    fn validate(&self) -> Result<(), validator::ValidationErrors> {
        match self {
            CoordinateTransform::Identity => {
                let mut e = validator::ValidationErrors::new();
                e.add(
                    "_",
                    ValidationError::new(DISALLOWED_TRANSFORM)
                        .with_message("identity transform disallowed here".into()),
                );
                Err(e)
            }
            _ => Ok(()),
        }
    }
}

impl MaybeNDim for CoordinateTransform {
    fn maybe_ndim(&self) -> Option<usize> {
        match self {
            CoordinateTransform::Identity => None,
            CoordinateTransform::Translation(t) => t.maybe_ndim(),
            CoordinateTransform::Scale(t) => t.maybe_ndim(),
        }
    }
}

/// [`CoordinateTransform`] `translation` type metadata.
#[allow(missing_docs)]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum CoordinateTransformTranslation {
    /// A list of floats.
    List { translation: Vec<f32> },
    /// A path to binary data at a location in this container.
    Path { path: PathBuf },
}

impl MaybeNDim for CoordinateTransformTranslation {
    fn maybe_ndim(&self) -> Option<usize> {
        if let CoordinateTransformTranslation::List { translation } = self {
            Some(translation.len())
        } else {
            None
        }
    }
}

impl From<Vec<f32>> for CoordinateTransformTranslation {
    fn from(translation: Vec<f32>) -> Self {
        CoordinateTransformTranslation::List { translation }
    }
}

impl From<PathBuf> for CoordinateTransformTranslation {
    fn from(path: PathBuf) -> Self {
        CoordinateTransformTranslation::Path { path }
    }
}

/// [`CoordinateTransform`] `scale` type metadata.
#[allow(missing_docs)]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum CoordinateTransformScale {
    /// A list of floats.
    List { scale: Vec<f32> },
    /// A path to binary data at a location in this container.
    Path { path: PathBuf },
}

impl MaybeNDim for CoordinateTransformScale {
    fn maybe_ndim(&self) -> Option<usize> {
        if let CoordinateTransformScale::List { scale } = self {
            Some(scale.len())
        } else {
            None
        }
    }
}

impl From<Vec<f32>> for CoordinateTransformScale {
    fn from(scale: Vec<f32>) -> Self {
        CoordinateTransformScale::List { scale }
    }
}

impl From<PathBuf> for CoordinateTransformScale {
    fn from(path: PathBuf) -> Self {
        CoordinateTransformScale::Path { path }
    }
}
