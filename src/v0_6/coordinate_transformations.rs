//! "coordinateTransformations" metadata.
//!
//! <https://ngff.openmicroscopy.org/specifications/0.6/#trafo-md>.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};
use validatrix::{Accumulator, Validate};

use crate::Error;

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
}

/// `identity` transformation.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Identity {}

impl Validate for Identity {
    fn validate_inner(&self, _accum: &mut Accumulator) {}
}

impl TransformType for Identity {
    fn input_ndim(&self) -> Option<usize> {
        None
    }
}

impl TransformType for MapAxis {
    fn input_ndim(&self) -> Option<usize> {
        Some(self.map_axis.len())
    }
}

impl TransformType for ProjectAxis {
    fn input_ndim(&self) -> Option<usize> {
        None
    }

    fn output_ndim(&self) -> Option<usize> {
        None
    }
}

impl TransformType for Translation {
    fn input_ndim(&self) -> Option<usize> {
        Some(self.translation.len())
    }
}

impl TransformType for Scale {
    fn input_ndim(&self) -> Option<usize> {
        Some(self.scale.len())
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

impl TransformType for Rotation {
    fn input_ndim(&self) -> Option<usize> {
        match self {
            Rotation::Inline { rotation } => Some(rotation.len()),
            _ => None,
        }
    }
}

impl TransformType for Sequence {
    fn input_ndim(&self) -> Option<usize> {
        self.transformations.first().and_then(|t| t.input_ndim())
    }

    fn output_ndim(&self) -> Option<usize> {
        self.transformations.last().and_then(|t| t.output_ndim())
    }
}

impl TransformType for ByDimension {
    fn input_ndim(&self) -> Option<usize> {
        self.transformations
            .iter()
            .flat_map(|s| s.input_axes.iter())
            .max()
            .map(|m| m + 1)
    }

    fn output_ndim(&self) -> Option<usize> {
        self.transformations
            .iter()
            .flat_map(|s| s.output_axes.iter())
            .max()
            .map(|m| m + 1)
    }
}

impl TransformType for Bijection {
    fn input_ndim(&self) -> Option<usize> {
        self.forward.input_ndim().or(self.inverse.output_ndim())
    }

    fn output_ndim(&self) -> Option<usize> {
        self.forward.output_ndim().or(self.inverse.input_ndim())
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
}

impl Validate for Transform {
    fn validate_inner(&self, accum: &mut Accumulator) {
        dispatch_transform_type!(self, t => Validate::validate_inner(t, accum));
    }
}

fn unique_indices(
    accum: &mut Accumulator,
    key: &'static str,
    indices: &[usize],
) -> BTreeSet<usize> {
    let mut seen = BTreeSet::default();
    for (idx, i) in indices.iter().enumerate() {
        if !seen.insert(*i) {
            accum.with_keys(&[key.into(), idx.into()], |a| {
                a.add_failure(format!("duplicate index {i}"))
            });
        }
    }
    seen
}

/// `mapAxis` transformation: an axis permutation.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MapAxis {
    /// The value at position `i` is the index of the input axis which becomes the `i`th output axis.
    pub map_axis: Vec<usize>,
}

impl Validate for MapAxis {
    fn validate_inner(&self, accum: &mut Accumulator) {
        let n = self.map_axis.len();
        if n == 0 {
            accum.add_failure_at("mapAxis", "empty");
        }
        unique_indices(accum, "mapAxis", &self.map_axis);
        for (idx, i) in self.map_axis.iter().enumerate() {
            if *i >= n {
                accum.with_keys(&["mapAxis".into(), idx.into()], |a| {
                    a.add_failure(format!("index {i} out of range for {n} axes"))
                });
            }
        }
    }
}

/// `projectAxis` transformation: add and/or drop dimensions.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct ProjectAxis {
    /// Indices of input dimensions to drop.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dropped_inputs: Vec<usize>,
    /// Indices of output dimensions which are newly created (with value 0).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub created_outputs: Vec<usize>,
}

impl Validate for ProjectAxis {
    fn validate_inner(&self, accum: &mut Accumulator) {
        if self.dropped_inputs.is_empty() && self.created_outputs.is_empty() {
            accum.add_failure("neither droppedInputs nor createdOutputs given");
        }
        unique_indices(accum, "droppedInputs", &self.dropped_inputs);
        unique_indices(accum, "createdOutputs", &self.created_outputs);
    }
}

/// `translation` transformation.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Translation {
    /// Values to add to each coordinate.
    pub translation: Vec<f64>,
}

impl Validate for Translation {
    fn validate_inner(&self, accum: &mut Accumulator) {
        if self.translation.is_empty() {
            accum.add_failure_at("translation", "empty");
        }
    }
}

impl From<Vec<f64>> for Translation {
    fn from(translation: Vec<f64>) -> Self {
        Self { translation }
    }
}

/// `scale` transformation.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Scale {
    /// Values to multiply each coordinate by.
    pub scale: Vec<f64>,
}

impl Validate for Scale {
    fn validate_inner(&self, accum: &mut Accumulator) {
        if self.scale.is_empty() {
            accum.add_failure_at("scale", "empty");
        }
    }
}

impl From<Vec<f64>> for Scale {
    fn from(scale: Vec<f64>) -> Self {
        Self { scale }
    }
}

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

/// `sequence` transformation.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Sequence {
    /// Transformations to apply in order.
    pub transformations: Vec<Transform>,
}

impl Validate for Sequence {
    fn validate_inner(&self, accum: &mut Accumulator) {
        accum.with_key("transformations", |a| {
            if self.transformations.is_empty() {
                a.add_failure("empty");
            }
            a.validate_iter(&self.transformations);
            for (idx, pair) in self.transformations.windows(2).enumerate() {
                if let (Some(o), Some(i)) = (pair[0].output_ndim(), pair[1].input_ndim()) {
                    if o != i {
                        a.add_failure_at(
                            idx + 1,
                            format!("input dimensionality {i} does not match previous output {o}"),
                        );
                    }
                }
            }
        });
    }
}

/// Interpolation method for coordinate and displacement fields.
#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Interpolation {
    /// Nearest-neighbour interpolation.
    #[serde(rename = "nearest")]
    Nearest,
    /// Linear interpolation (default).
    #[default]
    #[serde(rename = "linear")]
    Linear,
    /// Cubic B-spline interpolation.
    #[serde(rename = "bspline-cubic")]
    BsplineCubic,
    /// Other interpolation method.
    #[serde(untagged)]
    Custom(String),
}

/// `coordinates` or `displacements` transformation, stored in a zarr array.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FieldTransform {
    /// Path to the multiscale group containing the field.
    pub path: String,
    /// Interpolation used between field values; defaults to linear.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interpolation: Option<Interpolation>,
}

impl Validate for FieldTransform {
    fn validate_inner(&self, accum: &mut Accumulator) {
        if self.path.is_empty() {
            accum.add_failure_at("path", "empty");
        }
    }
}

macro_rules! field_transform_newtype {
    ($(#[$meta:meta])* $name:ident) => {
        $(#[$meta])*
        #[derive(Serialize, Deserialize, Debug, Clone)]
        #[serde(transparent)]
        pub struct $name(pub FieldTransform);

        impl std::ops::Deref for $name {
            type Target = FieldTransform;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl std::ops::DerefMut for $name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }

        impl From<FieldTransform> for $name {
            fn from(value: FieldTransform) -> Self {
                Self(value)
            }
        }

        impl Validate for $name {
            fn validate_inner(&self, accum: &mut Accumulator) {
                self.0.validate_inner(accum);
            }
        }

        impl TransformType for $name {
            fn input_ndim(&self) -> Option<usize> {
                None
            }
        }
    };
}

field_transform_newtype!(
    /// `coordinates` transformation: a coordinate field stored in a zarr array.
    Coordinates
);
field_transform_newtype!(
    /// `displacements` transformation: a displacement field stored in a zarr array.
    Displacements
);

/// `byDimension` transformation.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ByDimension {
    /// Transformations applied to subsets of axes.
    pub transformations: Vec<ByDimensionItem>,
}

impl Validate for ByDimension {
    fn validate_inner(&self, accum: &mut Accumulator) {
        accum.with_key("transformations", |a| {
            if self.transformations.is_empty() {
                a.add_failure("empty");
            }
            a.validate_iter(&self.transformations);
            let mut outputs = BTreeSet::default();
            for (idx, t) in self.transformations.iter().enumerate() {
                for o in t.output_axes.iter() {
                    if !outputs.insert(*o) {
                        a.with_keys(&[idx.into(), "outputAxes".into()], |a2| {
                            a2.add_failure(format!("output axis {o} already produced"))
                        });
                    }
                }
            }
            if let Some(max) = outputs.last() {
                if *max + 1 != outputs.len() {
                    a.add_failure("output axes are not contiguous from 0");
                }
            }
        });
    }
}

/// Item of a [`ByDimension`] transformation.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ByDimensionItem {
    /// Indices of the input axes this transformation consumes.
    pub input_axes: Vec<usize>,
    /// Indices of the output axes this transformation produces.
    pub output_axes: Vec<usize>,
    /// Transformation to apply.
    pub transformation: Transform,
}

impl Validate for ByDimensionItem {
    fn validate_inner(&self, accum: &mut Accumulator) {
        accum.validate_member_at("transformation", &self.transformation);
        unique_indices(accum, "inputAxes", &self.input_axes);
        unique_indices(accum, "outputAxes", &self.output_axes);
        if let Some(n) = self.transformation.input_ndim() {
            if n != self.input_axes.len() {
                accum.add_failure_at(
                    "inputAxes",
                    format!("length does not match transformation input dimensionality {n}"),
                );
            }
        }
        if let Some(n) = self.transformation.output_ndim() {
            if n != self.output_axes.len() {
                accum.add_failure_at(
                    "outputAxes",
                    format!("length does not match transformation output dimensionality {n}"),
                );
            }
        }
    }
}

/// `bijection` transformation.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Bijection {
    /// Forward transformation.
    pub forward: Box<Transform>,
    /// Inverse transformation.
    pub inverse: Box<Transform>,
}

impl Validate for Bijection {
    fn validate_inner(&self, accum: &mut Accumulator) {
        accum.validate_member_at("forward", self.forward.as_ref());
        accum.validate_member_at("inverse", self.inverse.as_ref());
        let dims = [
            self.forward.input_ndim(),
            self.forward.output_ndim(),
            self.inverse.input_ndim(),
            self.inverse.output_ndim(),
        ];
        let known: BTreeSet<_> = dims.into_iter().flatten().collect();
        if known.len() > 1 {
            accum.add_failure(format!(
                "inconsistent dimensionality between forward and inverse: {known:?}"
            ));
        }
    }
}

/// Convert a list of v0.4/v0.5 transformations into a single v0.6 transformation.
pub(crate) fn convert_prev_transforms(
    cts: Vec<crate::v0_5::CoordinateTransform>,
) -> Result<Transform, Error> {
    use crate::v0_5::{
        CoordinateTransform as Prev, CoordinateTransformScale as PS,
        CoordinateTransformTranslation as PT,
    };
    let mut out = Vec::with_capacity(cts.len());
    for ct in cts {
        out.push(match ct {
            Prev::Identity => continue,
            Prev::Translation(PT::List { translation }) => Translation::from(translation).into(),
            Prev::Scale(PS::List { scale }) => {
                Scale::from(scale).into()
            }
            Prev::Translation(PT::Path { .. }) | Prev::Scale(PS::Path { .. }) => {
                return Err(Error::general(
                    "path-based scale/translation transformations are not supported in 0.6",
                ));
            }
        });
    }
    Ok(match out.len() {
        0 => Identity::default().into(),
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
