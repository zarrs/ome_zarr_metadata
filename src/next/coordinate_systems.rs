use crate::v0_4::AxisUnit;
use serde::{Deserialize, Serialize};

/// A named set of axes representing a known space.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CoordinateSystem {
    /// Name of the coordinate system.
    pub name: String,
    /// Ordered axes of the coordinate system.
    pub axes: Vec<Axis>,
}

/// [`Axis`] `type` metadata. Represents the type of an axis.
#[non_exhaustive]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum AxisType {
    /// The `array` axis type.
    Array,
    /// The `space` axis type.
    Space,
    /// The `time` axis type.
    Time,
    /// The `channel` axis type.
    Channel,
    /// The `coordinate` axis type.
    Coordinate,
    /// The `displacement` axis type.
    Displacement,
    #[serde(untagged)]
    /// A custom axis type.
    Custom(String),
}

/// `axis` element metadata. Represents a dimension (axis) of a physical coordinate space.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Axis {
    /// The name for this dimension.
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A longer, more descriptive name for the axis.
    pub long_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether this axis is discrete (i.e. may not be interpolated).
    pub discrete: Option<bool>,
    /// The optional type of this dimension.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<AxisType>,
    /// The optional physical unit of this dimension.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<AxisUnit>,
}
