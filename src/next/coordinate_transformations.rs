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

/// Core metadata present in most coordinate transformations.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(tag = "type", rename_all = "camelCase")]
pub struct Common {
    /// Optional name of the transform.
    pub name: Option<String>,
    /// Input coordinate system.
    pub input: Option<String>,
    /// Output coordinate system.
    pub output: Option<String>,
}

/// Transformation between input/output spaces.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CoordinateTransformOuter {
    /// Optional name of the transform.
    pub name: Option<String>,
    /// Input coordinate system. Prefer accessing through [Self::input_system].
    pub input: Option<String>,
    /// Output coordinate system. Prefer accessing through [Self::output_system].
    pub output: Option<String>,
    /// Configuration specific to this transform type.
    #[serde(flatten)]
    pub config: CoordinateTransform,
}

impl CoordinateTransformOuter {
    /// Get the input coordinate system, inferring it from sub-transformations if necessary.
    /// Will always be Some for valid metadata.
    pub fn input_system(&self) -> Option<&str> {
        self.input.as_deref().or(match &self.config {
            CoordinateTransform::Bijection(bij) => bij.input_system(),
            _ => None,
        })
    }

    /// Get the output coordinate system, inferring it from sub-transformations if necessary.
    /// Will always be Some for valid metadata.
    pub fn output_system(&self) -> Option<&str> {
        self.output.as_deref().or(match &self.config {
            CoordinateTransform::Bijection(bij) => bij.output_system(),
            _ => None,
        })
    }
}

impl TransformationType for CoordinateTransformOuter {
    fn invertible(&self) -> Option<bool> {
        self.config.invertible()
    }

    fn input_ndim(&self) -> Option<usize> {
        self.config.input_ndim()
    }

    fn output_ndim(&self) -> Option<usize> {
        self.config.output_ndim()
    }
}

impl Validate for CoordinateTransformOuter {
    fn validate_inner(&self, accum: &mut Accumulator) {
        self.config.validate_inner(accum);
        let (inp, outp) = (self.config.input_system(), self.config.output_system());
        match (self.input.as_deref(), inp) {
            (Some(exp), Some(imp)) if exp != imp => {
                accum.add_failure_at("input", "mismatched coordinate systems");
            }
            // this would fail when validating nested transformations
            // (None, None) => accum.add_failure_at("input", "no coordinate system given"),
            _ => (),
        }
        match (self.output.as_deref(), outp) {
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
pub enum CoordinateTransform {
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

impl Validate for CoordinateTransform {
    fn validate_inner(&self, accum: &mut Accumulator) {
        match self {
            CoordinateTransform::Identity(t) => t.validate_inner(accum),
            CoordinateTransform::MapAxis(t) => t.validate_inner(accum),
            CoordinateTransform::Translation(t) => t.validate_inner(accum),
            CoordinateTransform::Scale(t) => t.validate_inner(accum),
            CoordinateTransform::Affine(t) => t.validate_inner(accum),
            CoordinateTransform::Rotation(t) => t.validate_inner(accum),
            CoordinateTransform::Sequence(t) => t.validate_inner(accum),
            CoordinateTransform::Displacements(t) => t.validate_inner(accum),
            CoordinateTransform::Coordinates(t) => t.validate_inner(accum),
            CoordinateTransform::InverseOf(t) => t.validate_inner(accum),
            CoordinateTransform::Bijection(t) => t.validate_inner(accum),
            CoordinateTransform::ByDimension(t) => t.validate_inner(accum),
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

impl TransformationType for CoordinateTransform {
    fn invertible(&self) -> Option<bool> {
        match self {
            CoordinateTransform::Identity(t) => t.invertible(),
            CoordinateTransform::MapAxis(t) => t.invertible(),
            CoordinateTransform::Translation(t) => t.invertible(),
            CoordinateTransform::Scale(t) => t.invertible(),
            CoordinateTransform::Affine(t) => t.invertible(),
            CoordinateTransform::Rotation(t) => t.invertible(),
            CoordinateTransform::Sequence(t) => t.invertible(),
            CoordinateTransform::Displacements(t) => t.invertible(),
            CoordinateTransform::Coordinates(t) => t.invertible(),
            CoordinateTransform::InverseOf(t) => t.invertible(),
            CoordinateTransform::Bijection(t) => t.invertible(),
            CoordinateTransform::ByDimension(t) => t.invertible(),
        }
    }

    fn input_ndim(&self) -> Option<usize> {
        match self {
            CoordinateTransform::Identity(t) => t.input_ndim(),
            CoordinateTransform::MapAxis(t) => t.input_ndim(),
            CoordinateTransform::Translation(t) => t.input_ndim(),
            CoordinateTransform::Scale(t) => t.input_ndim(),
            CoordinateTransform::Affine(t) => t.input_ndim(),
            CoordinateTransform::Rotation(t) => t.input_ndim(),
            CoordinateTransform::Sequence(t) => t.input_ndim(),
            CoordinateTransform::Displacements(t) => t.input_ndim(),
            CoordinateTransform::Coordinates(t) => t.input_ndim(),
            CoordinateTransform::InverseOf(t) => t.input_ndim(),
            CoordinateTransform::Bijection(t) => t.input_ndim(),
            CoordinateTransform::ByDimension(t) => t.input_ndim(),
        }
    }

    fn output_ndim(&self) -> Option<usize> {
        match self {
            CoordinateTransform::Identity(t) => t.output_ndim(),
            CoordinateTransform::MapAxis(t) => t.output_ndim(),
            CoordinateTransform::Translation(t) => t.output_ndim(),
            CoordinateTransform::Scale(t) => t.output_ndim(),
            CoordinateTransform::Affine(t) => t.output_ndim(),
            CoordinateTransform::Rotation(t) => t.output_ndim(),
            CoordinateTransform::Sequence(t) => t.output_ndim(),
            CoordinateTransform::Displacements(t) => t.output_ndim(),
            CoordinateTransform::Coordinates(t) => t.output_ndim(),
            CoordinateTransform::InverseOf(t) => t.output_ndim(),
            CoordinateTransform::Bijection(t) => t.output_ndim(),
            CoordinateTransform::ByDimension(t) => t.output_ndim(),
        }
    }

    fn input_system(&self) -> Option<&str> {
        match self {
            CoordinateTransform::Identity(t) => t.input_system(),
            CoordinateTransform::MapAxis(t) => t.input_system(),
            CoordinateTransform::Translation(t) => t.input_system(),
            CoordinateTransform::Scale(t) => t.input_system(),
            CoordinateTransform::Affine(t) => t.input_system(),
            CoordinateTransform::Rotation(t) => t.input_system(),
            CoordinateTransform::Sequence(t) => t.input_system(),
            CoordinateTransform::Displacements(t) => t.input_system(),
            CoordinateTransform::Coordinates(t) => t.input_system(),
            CoordinateTransform::InverseOf(t) => t.input_system(),
            CoordinateTransform::Bijection(t) => t.input_system(),
            CoordinateTransform::ByDimension(t) => t.input_system(),
        }
    }

    fn output_system(&self) -> Option<&str> {
        match self {
            CoordinateTransform::Identity(t) => t.output_system(),
            CoordinateTransform::MapAxis(t) => t.output_system(),
            CoordinateTransform::Translation(t) => t.output_system(),
            CoordinateTransform::Scale(t) => t.output_system(),
            CoordinateTransform::Affine(t) => t.output_system(),
            CoordinateTransform::Rotation(t) => t.output_system(),
            CoordinateTransform::Sequence(t) => t.output_system(),
            CoordinateTransform::Displacements(t) => t.output_system(),
            CoordinateTransform::Coordinates(t) => t.output_system(),
            CoordinateTransform::InverseOf(t) => t.output_system(),
            CoordinateTransform::Bijection(t) => t.output_system(),
            CoordinateTransform::ByDimension(t) => t.output_system(),
        }
    }
}

impl Default for CoordinateTransform {
    fn default() -> Self {
        Self::Identity(Default::default())
    }
}
