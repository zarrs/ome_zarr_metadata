use std::fmt::Display;

use serde::{Deserialize, Serialize};
use validatrix::Validate;

/// Transitional information specific to the channels of an image and how to render it.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Omero {
    /// Description of channels of the image.
    pub channels: Vec<Channel>,
    /// Catch-all field for any OMERO fields not specified in OME-Zarr.
    #[serde(flatten)]
    pub other: serde_json::Map<String, serde_json::Value>,
}

impl Validate for Omero {
    fn validate_inner(&self, accum: &mut validatrix::Accumulator) {
        accum.validate_iter_at("channels", &self.channels);
    }
}

/// Describes the channels of an image in OMERGO format.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Channel {
    /// Channel color as a hex RGB string.
    pub color: HexColor,
    /// Windowing of the channel.
    pub window: Window,
    /// Catch-all field for any OMERO channel fields not specified in OME-Zarr.
    #[serde(flatten)]
    pub other: serde_json::Map<String, serde_json::Value>,
}

impl Validate for Channel {
    fn validate_inner(&self, accum: &mut validatrix::Accumulator) {
        accum.validate_member_at("window", &self.window);
    }
}

/// Color defined as a hexadecimal RGB string.
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct HexColor(String);

impl AsRef<str> for HexColor {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl From<HexColor> for String {
    fn from(value: HexColor) -> Self {
        value.0
    }
}

impl Display for HexColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl TryFrom<String> for HexColor {
    type Error = crate::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() != 6 || value.chars().any(|c| !c.is_ascii_hexdigit()) {
            return Err(Self::Error::InvalidColor(value));
        }
        Ok(Self(value))
    }
}

impl Serialize for HexColor {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HexColor {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        HexColor::try_from(s).map_err(serde::de::Error::custom)
    }
}

/// Describes the windowing of a channel in OMERO format.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Window {
    /// Minimum value of the window.
    pub min: f64,
    /// Maximum value of the window.
    pub max: f64,
    /// Start value of the window.
    pub start: f64,
    /// End value of the window.
    pub end: f64,
}

impl Validate for Window {
    fn validate_inner(&self, accum: &mut validatrix::Accumulator) {
        if self.max < self.min {
            accum.add_failure_at("max", "less than min");
        }
        if self.end < self.start {
            accum.add_failure_at("end", "before start");
        }
    }
}
