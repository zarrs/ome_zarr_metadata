//! "axes" metadata.
//!
//! <https://ngff.openmicroscopy.org/0.4/#axes-md>.

use serde::{Deserialize, Serialize};

/// `axis` element metadata. Represents a dimension (axis) of a physical coordinate space.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields, rename_all = "lowercase")]
#[serde(tag = "type")]
pub enum Axis {
    /// A `space` axis.
    Space(AxisSpace),
    /// A `time` axis.
    Time(AxisTime),
    /// A `channel` axis.
    Channel(AxisChannel),
    /// A custom axis.
    Custom(AxisCustom),
}

/// A `space` axis.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct AxisSpace {
    /// The name for this dimension.
    pub name: String,
    /// The optional physical unit of this dimension.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<AxisUnitSpace>,
    /// The anatomical orientation of the dimension ([RFC 4](https://ngff.openmicroscopy.org/rfc/4/index.html)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orientation: Option<Orientation>,
}

/// A `time` axis.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct AxisTime {
    /// The name for this dimension.
    pub name: String,
    /// The optional physical unit of this dimension.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<AxisUnitTime>,
}

/// A `channel` axis.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct AxisChannel {
    /// The name for this dimension.
    pub name: String,
}

/// A `custom` axis.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct AxisCustom {
    /// The name for this dimension.
    pub name: String,
    /// The optional physical unit of this dimension.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
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

/// Orientation object that can represent different types of orientation information.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", content = "value")]
#[non_exhaustive]
pub enum Orientation {
    /// An `anatomical` orientation.
    #[serde(rename = "anatomical")]
    Anatomical(AnatomicalOrientation),
}

/// `orientation` metadata of the `anatomical` type. Represents an anotomical orientation.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "kebab-case")]
#[allow(missing_docs)]
pub enum AnatomicalOrientation {
    LeftToRight,
    RightToLeft,
    AnteriorToPosterior,
    PosteriorToAnterior,
    InferiorToSuperior,
    SuperiorToInferior,
    DorsalToVentral,
    VentralToDorsal,
    DorsalToPalmar,
    PalmarToDorsal,
    DorsalToPlantar,
    PlantarToDorsal,
    RostralToCaudal,
    CaudalToRostral,
    CranialToCaudal,
    CaudalToCranial,
    ProximalToDistal,
    DistalToProximal,
}
