//! "axes" metadata.
//!
//! <https://ngff.openmicroscopy.org/0.4/#axes-md>.

use serde::{Deserialize, Serialize};
use validatrix::{Accumulator, Validate};

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

impl Validate for Axis {
    fn validate_inner(&self, accum: &mut Accumulator) {

        let (Some(t), Some(u)) = (&self.r#type, &self.unit) else {
            return;
        };
        match u {
            AxisUnit::Space(_) => {
                if t != &AxisType::Space {
                    accum.add_failure("got space unit for non-space axis");
                }
            }
            AxisUnit::Time(_) => {
                if t != &AxisType::Time {
                    accum.add_failure("got time unit for non-time axis");
                }
            }
            AxisUnit::Custom(_) => (),
        }
    }
}

/// [`Axis`] `type` metadata. Represents the type of an axis.
#[non_exhaustive]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum AxisType {
    /// The `space` axis type.
    Space,
    /// The `time` axis type.
    Time,
    /// The `channel` axis type.
    Channel,
    #[serde(untagged)]
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

impl From<AxisUnitSpace> for AxisUnit {
    fn from(value: AxisUnitSpace) -> Self {
        Self::Space(value)
    }
}

impl From<AxisUnitTime> for AxisUnit {
    fn from(value: AxisUnitTime) -> Self {
        Self::Time(value)
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn custom_axis() {
        let space: AxisType = serde_json::from_str("\"space\"").unwrap();
        assert_eq!(space, AxisType::Space);
        let potato: AxisType = serde_json::from_str("\"potato\"").unwrap();
        assert_eq!(potato, AxisType::Custom("potato".to_string()));
        let custom: AxisType = serde_json::from_str("\"custom\"").unwrap();
        assert_eq!(custom, AxisType::Custom("custom".to_string()));
    }
}
