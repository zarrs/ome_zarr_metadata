//! "coordinateTransformations" metadata.
//!
//! <https://ngff.openmicroscopy.org/0.4/#trafo-md>.

use serde::{Deserialize, Serialize};
use validatrix::{Accumulator, Validate};

mod affine;
pub use affine::*;
mod bijection;
pub use bijection::*;
mod by_dimension;
pub use by_dimension::*;
mod coordinates;
pub use coordinates::*;
mod displacements;
pub use displacements::*;
mod identity;
pub use identity::*;
mod inverse_of;
pub use inverse_of::*;
mod map_axis;
pub use map_axis::*;
mod rotation;
pub use rotation::*;
mod scale;
pub use scale::*;
mod sequence;
pub use sequence::*;
mod translation;
pub use translation::*;

/// Transformation between input/output spaces.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CoordinateTransform {
    /// Optional name of the transform.
    pub name: Option<String>,
    /// Input coordinate system. Prefer accessing through [TransformationType::input_system].
    pub input: Option<String>,
    /// Output coordinate system. Prefer accessing through [TransformationType::output_system].
    pub output: Option<String>,
    /// Configuration specific to this transform type.
    #[serde(flatten)]
    pub inner: CoordinateTransformInner,
}

impl TransformationType for CoordinateTransform {
    fn invertible(&self) -> Option<bool> {
        self.inner.invertible()
    }

    fn input_ndim(&self) -> Option<usize> {
        self.inner.input_ndim()
    }

    fn output_ndim(&self) -> Option<usize> {
        self.inner.output_ndim()
    }

    fn input_system(&self) -> Option<&str> {
        self.input.as_deref().or(match &self.inner {
            CoordinateTransformInner::Bijection(bij) => bij.input_system(),
            _ => None,
        })
    }

    fn output_system(&self) -> Option<&str> {
        self.output.as_deref().or(match &self.inner {
            CoordinateTransformInner::Bijection(bij) => bij.output_system(),
            _ => None,
        })
    }
}

impl Validate for CoordinateTransform {
    fn validate_inner(&self, accum: &mut Accumulator) {
        self.inner.validate_inner(accum);

        match (self.input.as_deref(), self.inner.input_system()) {
            (Some(exp), Some(imp)) if exp != imp => {
                accum.add_failure_at("input", "mismatched coordinate systems");
            }
            // this would fail when validating nested transformations
            // (None, None) => accum.add_failure_at("input", "no coordinate system given"),
            _ => (),
        }

        match (self.output.as_deref(), self.inner.output_system()) {
            (Some(exp), Some(imp)) if exp != imp => {
                accum.add_failure_at("output", "mismatched coordinate systems");
            }
            // this would fail when validating nested transformations
            // (None, None) => accum.add_failure_at("output", "no coordinate system given"),
            _ => (),
        }
    }
}

/// `coordinate_transformations` element metadata. Represents a single coordinate transformation.
///
/// It must contain the field "type".
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "camelCase")]
#[non_exhaustive]
pub enum CoordinateTransformInner {
    /// The identity transformation.
    Identity(Identity),
    /// An axis permutation as a transpose array of integer indices that refer to the ordering of the axes in the respective coordinate system.
    MapAxis(MapAxis),
    /// A translation vector.
    Translation(Translation),
    /// A scale vector.
    Scale(Scale),
    /// Affine transformation matrix.
    Affine(Affine),
    /// Rotation matrix
    Rotation(Rotation),
    /// Sequence of other transformations.
    Sequence(Sequence),
    /// Displacement field
    Displacements(Displacements),
    /// Coordinate transformation
    Coordinates(Coordinates),
    /// Inverse Of transformation
    InverseOf(InverseOf),
    /// Bijection transformation
    Bijection(Bijection),
    /// by_dimension transformation
    ByDimension(ByDimension),
}

impl Validate for CoordinateTransformInner {
    fn validate_inner(&self, accum: &mut Accumulator) {
        match self {
            CoordinateTransformInner::Identity(t) => t.validate_inner(accum),
            CoordinateTransformInner::MapAxis(t) => t.validate_inner(accum),
            CoordinateTransformInner::Translation(t) => t.validate_inner(accum),
            CoordinateTransformInner::Scale(t) => t.validate_inner(accum),
            CoordinateTransformInner::Affine(t) => t.validate_inner(accum),
            CoordinateTransformInner::Rotation(t) => t.validate_inner(accum),
            CoordinateTransformInner::Sequence(t) => t.validate_inner(accum),
            CoordinateTransformInner::Displacements(t) => t.validate_inner(accum),
            CoordinateTransformInner::Coordinates(t) => t.validate_inner(accum),
            CoordinateTransformInner::InverseOf(t) => t.validate_inner(accum),
            CoordinateTransformInner::Bijection(t) => t.validate_inner(accum),
            CoordinateTransformInner::ByDimension(t) => t.validate_inner(accum),
        }
    }
}

/// Useful methods for all transformation types.
pub trait TransformationType {
    /// Whether this type of transformation is invertible.
    fn invertible(&self) -> Option<bool>;

    /// Input coordinate system name, if this transformation type is opinionated.
    fn input_system(&self) -> Option<&str> {
        None
    }

    /// Output coordinate system name, if this transformation type is opinionated.
    fn output_system(&self) -> Option<&str> {
        None
    }

    /// The input dimensionality of the transformation.
    // TODO: this may not be necessary, as most transform containers require explicit input/output
    fn input_ndim(&self) -> Option<usize>;

    /// The output dimensionality of the transformation.
    // TODO: this may not be necessary, as most transform containers require explicit input/output
    fn output_ndim(&self) -> Option<usize>;
}

impl TransformationType for CoordinateTransformInner {
    fn invertible(&self) -> Option<bool> {
        match self {
            CoordinateTransformInner::Identity(t) => t.invertible(),
            CoordinateTransformInner::MapAxis(t) => t.invertible(),
            CoordinateTransformInner::Translation(t) => t.invertible(),
            CoordinateTransformInner::Scale(t) => t.invertible(),
            CoordinateTransformInner::Affine(t) => t.invertible(),
            CoordinateTransformInner::Rotation(t) => t.invertible(),
            CoordinateTransformInner::Sequence(t) => t.invertible(),
            CoordinateTransformInner::Displacements(t) => t.invertible(),
            CoordinateTransformInner::Coordinates(t) => t.invertible(),
            CoordinateTransformInner::InverseOf(t) => t.invertible(),
            CoordinateTransformInner::Bijection(t) => t.invertible(),
            CoordinateTransformInner::ByDimension(t) => t.invertible(),
        }
    }

    fn input_ndim(&self) -> Option<usize> {
        match self {
            CoordinateTransformInner::Identity(t) => t.input_ndim(),
            CoordinateTransformInner::MapAxis(t) => t.input_ndim(),
            CoordinateTransformInner::Translation(t) => t.input_ndim(),
            CoordinateTransformInner::Scale(t) => t.input_ndim(),
            CoordinateTransformInner::Affine(t) => t.input_ndim(),
            CoordinateTransformInner::Rotation(t) => t.input_ndim(),
            CoordinateTransformInner::Sequence(t) => t.input_ndim(),
            CoordinateTransformInner::Displacements(t) => t.input_ndim(),
            CoordinateTransformInner::Coordinates(t) => t.input_ndim(),
            CoordinateTransformInner::InverseOf(t) => t.input_ndim(),
            CoordinateTransformInner::Bijection(t) => t.input_ndim(),
            CoordinateTransformInner::ByDimension(t) => t.input_ndim(),
        }
    }

    fn output_ndim(&self) -> Option<usize> {
        match self {
            CoordinateTransformInner::Identity(t) => t.output_ndim(),
            CoordinateTransformInner::MapAxis(t) => t.output_ndim(),
            CoordinateTransformInner::Translation(t) => t.output_ndim(),
            CoordinateTransformInner::Scale(t) => t.output_ndim(),
            CoordinateTransformInner::Affine(t) => t.output_ndim(),
            CoordinateTransformInner::Rotation(t) => t.output_ndim(),
            CoordinateTransformInner::Sequence(t) => t.output_ndim(),
            CoordinateTransformInner::Displacements(t) => t.output_ndim(),
            CoordinateTransformInner::Coordinates(t) => t.output_ndim(),
            CoordinateTransformInner::InverseOf(t) => t.output_ndim(),
            CoordinateTransformInner::Bijection(t) => t.output_ndim(),
            CoordinateTransformInner::ByDimension(t) => t.output_ndim(),
        }
    }

    fn input_system(&self) -> Option<&str> {
        match self {
            CoordinateTransformInner::Identity(t) => t.input_system(),
            CoordinateTransformInner::MapAxis(t) => t.input_system(),
            CoordinateTransformInner::Translation(t) => t.input_system(),
            CoordinateTransformInner::Scale(t) => t.input_system(),
            CoordinateTransformInner::Affine(t) => t.input_system(),
            CoordinateTransformInner::Rotation(t) => t.input_system(),
            CoordinateTransformInner::Sequence(t) => t.input_system(),
            CoordinateTransformInner::Displacements(t) => t.input_system(),
            CoordinateTransformInner::Coordinates(t) => t.input_system(),
            CoordinateTransformInner::InverseOf(t) => t.input_system(),
            CoordinateTransformInner::Bijection(t) => t.input_system(),
            CoordinateTransformInner::ByDimension(t) => t.input_system(),
        }
    }

    fn output_system(&self) -> Option<&str> {
        match self {
            CoordinateTransformInner::Identity(t) => t.output_system(),
            CoordinateTransformInner::MapAxis(t) => t.output_system(),
            CoordinateTransformInner::Translation(t) => t.output_system(),
            CoordinateTransformInner::Scale(t) => t.output_system(),
            CoordinateTransformInner::Affine(t) => t.output_system(),
            CoordinateTransformInner::Rotation(t) => t.output_system(),
            CoordinateTransformInner::Sequence(t) => t.output_system(),
            CoordinateTransformInner::Displacements(t) => t.output_system(),
            CoordinateTransformInner::Coordinates(t) => t.output_system(),
            CoordinateTransformInner::InverseOf(t) => t.output_system(),
            CoordinateTransformInner::Bijection(t) => t.output_system(),
            CoordinateTransformInner::ByDimension(t) => t.output_system(),
        }
    }
}

impl Default for CoordinateTransformInner {
    fn default() -> Self {
        Self::Identity(Default::default())
    }
}
