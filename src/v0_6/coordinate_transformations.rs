//! "coordinateTransformations" metadata.
//!
//! <https://ngff.openmicroscopy.org/specifications/0.6/#trafo-md>.

use serde::{Deserialize, Serialize};
use validatrix::{Accumulator, Validate};

use crate::Error;
use crate::v0_5 as prev;

mod core;
mod dimension;
mod matrix;
mod vector_field;
mod wrapper;

pub use self::core::{Identity, Scale, Translation};
pub use dimension::{MapAxis, ProjectAxis};
pub use matrix::{Affine, Rotation};
pub use vector_field::{Coordinates, Displacements, Interpolation};
pub use wrapper::{Bijection, ByDimension, Sequence};

/// Reference to a coordinate system, used for the `input` and `output` of a transformation.
///
/// If `path` is omitted, the coordinate system is in the same metadata document.
#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct CoordinateSystemRef {
    /// Name of the coordinate system.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Path to the zarr node (array or group) where the coordinate system is defined.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}

impl CoordinateSystemRef {
    /// Reference a named coordinate system in the same document.
    pub fn from_name(name: impl Into<String>) -> Self {
        Self {
            name: Some(name.into()),
            path: None,
        }
    }

    /// Reference the coordinate system of a zarr array.
    pub fn from_path(path: impl Into<String>) -> Self {
        Self {
            name: None,
            path: Some(path.into()),
        }
    }

    /// Reference a named coordinate system defined in a descendant zarr group.
    pub fn from_name_path(name: impl Into<String>, path: impl Into<String>) -> Self {
        Self {
            name: Some(name.into()),
            path: Some(path.into()),
        }
    }

    /// Classify this reference according to which fields are present.
    pub fn kind(&self) -> CoordinateSystemRefKind<'_> {
        match (self.name.as_deref(), self.path.as_deref()) {
            (Some(name), None) => CoordinateSystemRefKind::Local { name },
            (Some(name), Some(path)) => CoordinateSystemRefKind::Descendant { name, path },
            (None, Some(path)) => CoordinateSystemRefKind::Array { path },
            (None, None) => CoordinateSystemRefKind::Empty,
        }
    }

    /// Name of a coordinate system in the same document (`name` without `path`).
    pub fn as_local(&self) -> Option<&str> {
        match self.kind() {
            CoordinateSystemRefKind::Local { name } => Some(name),
            _ => None,
        }
    }

    /// `(name, path)` of a coordinate system in another zarr group.
    pub fn as_descendant(&self) -> Option<(&str, &str)> {
        match self.kind() {
            CoordinateSystemRefKind::Descendant { name, path } => Some((name, path)),
            _ => None,
        }
    }

    /// Path of an array whose coordinate system is referenced (`path` without `name`).
    pub fn as_array(&self) -> Option<&str> {
        match self.kind() {
            CoordinateSystemRefKind::Array { path } => Some(path),
            _ => None,
        }
    }
}

/// Borrowed view of a [`CoordinateSystemRef`], classified by which fields are present.
///
/// See <https://ngff.openmicroscopy.org/specifications/0.6/#coord-trafo-constraints>.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CoordinateSystemRefKind<'a> {
    /// Named coordinate system in the same metadata document.
    Local {
        /// Coordinate system name.
        name: &'a str,
    },
    /// Named coordinate system defined in another zarr group.
    Descendant {
        /// Coordinate system name.
        name: &'a str,
        /// Path to the zarr group.
        path: &'a str,
    },
    /// Array coordinate system of the zarr array at `path`.
    Array {
        /// Path to the zarr array.
        path: &'a str,
    },
    /// Neither `name` nor `path` given (invalid).
    Empty,
}

impl Validate for CoordinateSystemRef {
    fn validate_inner(&self, accum: &mut Accumulator) {
        if self.name.is_none() && self.path.is_none() {
            accum.add_failure("reference has neither name nor path");
        }
    }
}

/// A coordinate transformation, mapping points from its `input` to its `output` coordinate system.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CoordinateTransform {
    /// Optional name of the transformation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Input coordinate system.
    pub input: CoordinateSystemRef,
    /// Output coordinate system.
    pub output: CoordinateSystemRef,
    /// Type-specific parameters.
    #[serde(flatten)]
    pub transform: Transform,
}

impl CoordinateTransform {
    /// Input dimensionality, if it can be determined from the parameters.
    pub fn input_ndim(&self) -> Option<usize> {
        self.transform.input_ndim()
    }

    /// Output dimensionality, if it can be determined from the parameters.
    pub fn output_ndim(&self) -> Option<usize> {
        self.transform.output_ndim()
    }

    /// Create a new, unnamed transformation.
    pub fn new(
        input: CoordinateSystemRef,
        output: CoordinateSystemRef,
        transform: impl Into<Transform>,
    ) -> Self {
        Self {
            name: None,
            input,
            output,
            transform: transform.into(),
        }
    }
}

impl Validate for CoordinateTransform {
    fn validate_inner(&self, accum: &mut Accumulator) {
        accum.validate_member_at("input", &self.input);
        accum.validate_member_at("output", &self.output);
        self.transform.validate_inner(accum);
    }
}

/// Validate that a transformation's `input` and `output` both have `name`s.
pub(crate) fn require_named_io(accum: &mut Accumulator, ct: &CoordinateTransform) {
    for (key, r) in [("input", &ct.input), ("output", &ct.output)] {
        if r.name.is_none() {
            accum.with_keys(&[key.into(), "name".into()], |a| a.add_failure("missing"))
        }
    }
}

/// Validate references from transformations to coordinate systems defined in the same document.
///
/// For each transformation whose `input` or `output` refers to a coordinate system
/// by `name` only (i.e. without `path`), check that the system exists in `systems`.
///
/// If both systems are resolved, check the transformation's dimensionality against them
/// with [`TransformType::check_ndim`].
/// If only one is resolved, compare it with the transformation's dimensionality on that side,
/// where that can be determined.
///
/// References with a `path` are not resolved, as they refer to other zarr nodes.
/// Keys are relative to the transformations array.
pub(crate) fn validate_local_transform_graph(
    accum: &mut Accumulator,
    systems: &[super::CoordinateSystem],
    transforms: &[CoordinateTransform],
) {
    use crate::NDim;

    // TODO: validate graph connectedness

    for (idx, ct) in transforms.iter().enumerate() {
        let mut resolve = |terminal_key: &'static str, system_ref: &CoordinateSystemRef| {
            let name = system_ref.as_local()?;
            let cs = systems.iter().find(|cs| cs.name == name);
            if cs.is_none() {
                accum.with_keys(&[idx.into(), terminal_key.into(), "name".into()], |a| {
                    a.add_failure(format!("unknown coordinate system '{name}'"))
                });
            }
            cs
        };
        let input_cs = resolve("input", &ct.input);
        let output_cs = resolve("output", &ct.output);

        match (input_cs, output_cs) {
            (Some(i), Some(o)) => {
                if ct.transform.check_ndim(i.ndim(), o.ndim()) == Some(false) {
                    accum.add_failure_at(
                        idx,
                        format!(
                            "transformation is not compatible with input coordinate system \
                             '{}' ({}D) and output coordinate system '{}' ({}D)",
                            i.name,
                            i.ndim(),
                            o.name,
                            o.ndim()
                        ),
                    );
                }
            }
            (Some(cs), None) => {
                check_one_side(accum, idx, "input", ct.input_ndim(), cs);
            }
            (None, Some(cs)) => {
                check_one_side(accum, idx, "output", ct.output_ndim(), cs);
            }
            (None, None) => (),
        }
    }
}

/// Compare a transformation's dimensionality on one side with that of a coordinate system.
fn check_one_side(
    accum: &mut Accumulator,
    idx: usize,
    terminal_key: &'static str,
    transform_ndim: Option<usize>,
    cs: &super::CoordinateSystem,
) {
    use crate::NDim;

    let Some(n) = transform_ndim else {
        return;
    };
    let expected = cs.ndim();
    if n != expected {
        accum.with_keys(&[idx.into(), terminal_key.into()], |a| {
            a.add_failure(format!(
                "transformation {terminal_key} dimensionality {n} does not match \
                 coordinate system '{}' dimensionality {expected}",
                cs.name
            ))
        });
    }
}

/// Type-specific parameters of a coordinate transformation, discriminated by the `type` field.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "camelCase")]
#[non_exhaustive]
pub enum Transform {
    /// Do-nothing transformation.
    Identity(Identity),
    /// Axis permutation.
    MapAxis(MapAxis),
    /// Add or drop dimensions.
    ProjectAxis(ProjectAxis),
    /// Translation vector.
    Translation(Translation),
    /// Scale vector.
    Scale(Scale),
    /// Affine matrix.
    Affine(Affine),
    /// Rotation matrix.
    Rotation(Rotation),
    /// Sequence of transformations, applied in order.
    Sequence(Sequence),
    /// Coordinate field stored in a zarr array.
    Coordinates(Coordinates),
    /// Displacement field stored in a zarr array.
    Displacements(Displacements),
    /// Lower-dimensional transformations applied to subsets of axes.
    ByDimension(ByDimension),
    /// Explicit forward and inverse transformations.
    Bijection(Bijection),
}

macro_rules! dispatch_transform_type {
    ($value:expr, $t:ident => $body:expr) => {
        match $value {
            Transform::Identity($t) => $body,
            Transform::MapAxis($t) => $body,
            Transform::ProjectAxis($t) => $body,
            Transform::Translation($t) => $body,
            Transform::Scale($t) => $body,
            Transform::Affine($t) => $body,
            Transform::Rotation($t) => $body,
            Transform::Sequence($t) => $body,
            Transform::Coordinates($t) => $body,
            Transform::Displacements($t) => $body,
            Transform::ByDimension($t) => $body,
            Transform::Bijection($t) => $body,
        }
    };
}

macro_rules! impl_from_variant {
    ($($variant:ident($ty:ty)),* $(,)?) => {
        $(
            impl From<$ty> for Transform {
                fn from(value: $ty) -> Self {
                    Self::$variant(value)
                }
            }
        )*
    };
}

/// Common behaviour of the types wrapped by [`Transform`].
pub trait TransformType: Into<Transform> + Validate {
    /// Input dimensionality, if it can be determined from the parameters.
    fn input_ndim(&self) -> Option<usize>;

    /// Output dimensionality, if it can be determined from the parameters.
    ///
    /// Defaults to [`TransformType::input_ndim`].
    fn output_ndim(&self) -> Option<usize> {
        self.input_ndim()
    }

    /// Check whether the transformation is valid for the given input and output dimensionalities.
    fn check_ndim(&self, input: usize, output: usize) -> Option<bool> {
        Some(self.input_ndim()? == input && self.output_ndim()? == output)
    }
}

impl_from_variant!(
    Identity(Identity),
    Coordinates(Coordinates),
    Displacements(Displacements),
    MapAxis(MapAxis),
    ProjectAxis(ProjectAxis),
    Translation(Translation),
    Scale(Scale),
    Affine(Affine),
    Rotation(Rotation),
    Sequence(Sequence),
    ByDimension(ByDimension),
    Bijection(Bijection),
);

impl TransformType for Transform {
    fn input_ndim(&self) -> Option<usize> {
        dispatch_transform_type!(self, t => TransformType::input_ndim(t))
    }

    fn output_ndim(&self) -> Option<usize> {
        dispatch_transform_type!(self, t => TransformType::output_ndim(t))
    }

    fn check_ndim(&self, input: usize, output: usize) -> Option<bool> {
        dispatch_transform_type!(self, t => TransformType::check_ndim(t, input, output))
    }
}

impl Validate for Transform {
    fn validate_inner(&self, accum: &mut Accumulator) {
        dispatch_transform_type!(self, t => Validate::validate_inner(t, accum));
    }
}

impl TryFrom<prev::CoordinateTransform> for Transform {
    type Error = crate::Error;

    fn try_from(value: prev::CoordinateTransform) -> Result<Self, Self::Error> {
        match value {
            prev::CoordinateTransform::Identity => Ok(Self::Identity(Identity {})),
            prev::CoordinateTransform::Translation(t) => Ok(Self::Translation(t.try_into()?)),
            prev::CoordinateTransform::Scale(s) => Ok(Self::Scale(s.try_into()?)),
        }
    }
}

impl FromIterator<Transform> for Transform {
    fn from_iter<T: IntoIterator<Item = Transform>>(iter: T) -> Self {
        let mut out = Vec::new();
        for t in iter {
            match t {
                Transform::Identity(_) => continue,
                _ => out.push(t),
            }
        }
        match out.len() {
            0 => Identity.into(),
            1 => out.pop().unwrap(),
            _ => Transform::Sequence(Sequence {
                transformations: out,
            }),
        }
    }
}

/// Convert a list of v0.4/v0.5 transformations into a single v0.6 transformation.
pub(crate) fn convert_prev_transforms(
    cts: Vec<crate::v0_5::CoordinateTransform>,
) -> Result<Transform, Error> {
    let mut out = Vec::with_capacity(cts.len());
    for ct in cts {
        out.push(ct.try_into()?);
    }
    Ok(match out.len() {
        0 => Identity.into(),
        1 => out.pop().unwrap(),
        _ => Transform::Sequence(Sequence {
            transformations: out,
        }),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_affine_path() {
        let s = r#"{"type":"affine","path":"a","input":{"name":"x"},"output":{"name":"y"}}"#;
        let ct: CoordinateTransform = serde_json::from_str(s).unwrap();
        assert!(matches!(
            ct.transform,
            Transform::Affine(Affine::Path { .. })
        ));
        let s2 = serde_json::to_string(&ct).unwrap();
        let _: CoordinateTransform = serde_json::from_str(&s2).unwrap();
    }

    #[test]
    fn nested_sequence() {
        let s = r#"{"type":"sequence","transformations":[{"type":"scale","scale":[1,2]},{"type":"translation","translation":[3,4]}]}"#;
        let ct: Transform = serde_json::from_str(s).unwrap();
        ct.validate().unwrap();
        assert_eq!(ct.input_ndim(), Some(2));
    }

    #[test]
    fn bad_map_axis() {
        let s = r#"{"type":"mapAxis","mapAxis":[0,0]}"#;
        let ct: Transform = serde_json::from_str(s).unwrap();
        assert!(ct.validate().is_err());
    }

    #[test]
    fn interpolation() {
        let i: Interpolation = serde_json::from_str("\"bspline-cubic\"").unwrap();
        assert_eq!(i, Interpolation::BsplineCubic);
        let i: Interpolation = serde_json::from_str("\"cubic\"").unwrap();
        assert_eq!(i, Interpolation::Custom("cubic".into()));
    }
}
