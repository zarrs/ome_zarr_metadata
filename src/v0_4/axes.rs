//! "axes" metadata.
//!
//! <https://ngff.openmicroscopy.org/0.4/#axes-md>.

use serde::{Deserialize, Serialize};

/// `axis` element metadata. Represents a dimension (axis) of a physical coordinate space.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct Axis {
    /// The name for this dimension.
    pub name: String,
    /// The optional type of this dimension.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<AxisType>,
    /// The optional physical unit of this dimension.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<AxisUnit>,
}

/// [`Axis`] `type` metadata. Represents the type of an axis.
#[non_exhaustive]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]

pub enum AxisType {
    /// The `space` axis type.
    Space,
    /// The `time` axis type.
    Time,
    /// The `channel` axis type.
    Channel,
    /// A custom axis type.
    Custom(String),
}

/// [`Axis`] `unit` metadata. Represents the unit of an axis.
#[non_exhaustive]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum AxisUnit {
    /// A recognised `space` axis unit.
    Space(AxisUnitSpace),
    /// A recognised `time` axis unit.
    Time(AxisUnitTime),
    /// A custom axis unit.
    Custom(String),
}

/// [`AxisUnit`] physical `space` units valid according to UDUNITS-2.
#[non_exhaustive]
#[allow(missing_docs)]
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum AxisUnitSpace {
    Angstrom,
    Attometer,
    Centimeter,
    Decimeter,
    Exameter,
    Femtometer,
    Foot,
    Gigameter,
    Hectometer,
    Inch,
    Kilometer,
    Megameter,
    Meter,
    Micrometer,
    Mile,
    Millimeter,
    Nanometer,
    Parsec,
    Petameter,
    Picometer,
    Terameter,
    Yard,
    Yoctometer,
    Yottameter,
    Zeptometer,
    Zettameter,
}

/// [`AxisUnit`] physical `time` units valid according to UDUNITS-2.
#[non_exhaustive]
#[allow(missing_docs)]
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum AxisUnitTime {
    Attosecond,
    Centisecond,
    Day,
    Decisecond,
    Exasecond,
    Femtosecond,
    Gigasecond,
    Hectosecond,
    Hour,
    Kilosecond,
    Megasecond,
    Microsecond,
    Millisecond,
    Minute,
    Nanosecond,
    Petasecond,
    Picosecond,
    Second,
    Terasecond,
    Yoctosecond,
    Yottasecond,
    Zeptosecond,
    Zettasecond,
}
