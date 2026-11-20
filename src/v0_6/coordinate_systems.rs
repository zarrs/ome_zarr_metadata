//! "coordinateSystems" and "axes" metadata.
//!
//! <https://ngff.openmicroscopy.org/specifications/0.6/#coordinate-systems-md>.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};
use validatrix::{Accumulator, Validate};

use crate::NDim;
pub use crate::v0_5::{AxisUnit, AxisUnitSpace, AxisUnitTime};

/// [`Axis`] `type` metadata. Represents the type of an axis.
#[non_exhaustive]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum AxisType {
    /// The `array` axis type. Always discrete.
    Array,
    /// The `space` axis type.
    Space,
    /// The `time` axis type.
    Time,
    /// The `channel` axis type.
    Channel,
    /// The `coordinate` axis type, for the vector dimension of a coordinate field.
    Coordinate,
    /// The `displacement` axis type, for the vector dimension of a displacement field.
    Displacement,
    /// A custom axis type.
    #[serde(untagged)]
    Custom(String),
}

impl AxisType {
    /// Whether this axis type is discrete.
    pub fn must_be_discrete(&self) -> bool {
        matches!(
            self,
            Self::Array | Self::Channel | Self::Coordinate | Self::Displacement
        )
    }
}

impl From<crate::v0_5::AxisType> for AxisType {
    fn from(value: crate::v0_5::AxisType) -> Self {
        match value {
            crate::v0_5::AxisType::Space => Self::Space,
            crate::v0_5::AxisType::Time => Self::Time,
            crate::v0_5::AxisType::Channel => Self::Channel,
            crate::v0_5::AxisType::Custom(s) => Self::Custom(s),
        }
    }
}

/// `axis` element metadata. Represents a dimension (axis) of a coordinate system.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Axis {
    /// The name for this dimension. Must be non-empty and unique within the coordinate system.
    pub name: String,
    /// The optional type of this dimension.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<AxisType>,
    /// Whether this axis is discrete (i.e. should not be interpolated).
    ///
    /// Prefer [`Axis::is_discrete`], which accounts for `array` axes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discrete: Option<bool>,
    /// The optional physical unit of this dimension.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<AxisUnit>,
    /// A longer name or description of the axis.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub long_name: Option<String>,
}

impl Axis {
    /// Whether this axis is discrete: either explicitly, or because it is an `array` axis.
    pub fn is_discrete(&self) -> bool {
        matches!(self.r#type, Some(AxisType::Array)) || self.discrete.unwrap_or(false)
    }
}

impl Validate for Axis {
    fn validate_inner(&self, accum: &mut Accumulator) {
        if self.name.is_empty() {
            accum.add_failure_at("name", "empty axis name");
        }
        if self.discrete == Some(false)
            && self.r#type.as_ref().is_some_and(|t| t.must_be_discrete())
        {
            accum.add_failure_at("discrete", "array axes are discrete");
        }
        let (Some(t), Some(u)) = (&self.r#type, &self.unit) else {
            return;
        };
        match u {
            AxisUnit::Space(u) if t != &AxisType::Space => {
                log::warn!(
                    "got space unit {u:?} for {t:?} axis with name '{}'",
                    self.name
                );
            }
            AxisUnit::Time(u) if t != &AxisType::Time => {
                log::warn!(
                    "got time unit {u:?} for {t:?} axis with name '{}'",
                    self.name
                );
            }
            _ => (),
        }
    }
}

impl From<crate::v0_5::Axis> for Axis {
    fn from(value: crate::v0_5::Axis) -> Self {
        Self {
            name: value.name,
            r#type: value.r#type.map(Into::into),
            discrete: None,
            unit: value.unit,
            long_name: None,
        }
    }
}

/// A named coordinate system, described by its axes.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CoordinateSystem {
    /// Name of the coordinate system. Must be non-empty and unique among coordinate systems in a document.
    pub name: String,
    /// Ordered axes of the coordinate system.
    pub axes: Vec<Axis>,
}

impl NDim for CoordinateSystem {
    fn ndim(&self) -> usize {
        self.axes.len()
    }
}

impl Validate for CoordinateSystem {
    fn validate_inner(&self, accum: &mut Accumulator) {
        if self.name.is_empty() {
            accum.add_failure_at("name", "empty coordinate system name");
        }
        accum.with_key("axes", |a| {
            valid_axes(a, &self.axes);

            if self.axes.is_empty() {
                a.add_failure("no axes");
            }
            a.validate_iter(&self.axes);
        });
    }
}

/// Validate the axes of a coordinate system.
///
/// - each axis is internally valid
/// - axis names are unique
/// - axes are in order ?time, ?, ?space, space, space
pub(crate) fn valid_axes(accum: &mut Accumulator, axes: &[Axis]) {
    if axes.len() < 2 || axes.len() > 5 {
        accum.add_failure(format!("got {} axes, expected 2-5", axes.len()));
    }

    let mut done_time = false;
    let mut done_channel_custom = false;
    let mut n_space = 0;
    let mut names: BTreeSet<&str> = BTreeSet::default();

    for (idx, ax) in axes.iter().enumerate() {
        accum.with_key(idx, |a| {
            ax.validate_inner(a);
            if !names.insert(ax.name.as_str()) {
                a.add_failure(format!("duplicate axis name '{}'", ax.name));
            }
        });
        match ax.r#type {
            Some(AxisType::Space) => {
                n_space += 1;
                if n_space > 3 {
                    accum.add_failure_at(
                        idx,
                        format!("at least {n_space} space axes, should be max 3"),
                    );
                }
                done_time = true;
                done_channel_custom = true;
            }
            Some(AxisType::Time) => {
                if done_time || done_channel_custom || n_space > 0 {
                    accum.add_failure_at(idx, "unexpected time axis");
                }
                done_time = true;
            }
            None
            | Some(AxisType::Channel)
            | Some(AxisType::Coordinate)
            | Some(AxisType::Displacement)
            | Some(AxisType::Array)
            | Some(AxisType::Custom(_)) => {
                if done_channel_custom || n_space > 0 {
                    accum.add_failure_at(idx, "unexpected channel/custom/unknown axis");
                }
                done_channel_custom = true;
                done_time = true;
            }
        }
    }
    if n_space < 2 {
        accum.add_failure(format!("got {n_space} space axes, expected 2-3"));
    }
}

/// Validate a list of coordinate systems, including uniqueness of names.
pub(crate) fn validate_coordinate_systems(accum: &mut Accumulator, systems: &[CoordinateSystem]) {
    accum.validate_iter(systems);
    let mut names = BTreeSet::default();
    for (idx, cs) in systems.iter().enumerate() {
        if !names.insert(cs.name.as_str()) {
            accum.with_keys(&[idx.into(), "name".into()], |a| {
                a.add_failure(format!("duplicate coordinate system name '{}'", cs.name))
            });
        }
    }
}

/// Find a coordinate system by name.
pub(crate) fn find_system<'a>(
    systems: &'a [CoordinateSystem],
    name: &str,
) -> Option<&'a CoordinateSystem> {
    systems.iter().find(|cs| cs.name == name)
}
