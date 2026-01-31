use std::{fmt::Display, str::FromStr};

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
    /// Channel color, stored as a hex RGB string.
    pub color: Color,
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
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Color {
    /// Red value `[0,255]`
    pub r: u8,
    /// Green value `[0,255]`
    pub g: u8,
    /// Blue value `[0,255]`
    pub b: u8,
}

impl From<Color> for String {
    fn from(value: Color) -> Self {
        value.to_string()
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }
}

impl FromStr for Color {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 6 {
            return Err(Self::Err::InvalidColor);
        }
        let r = u8::from_str_radix(&s[0..2], 16).map_err(|_| Self::Err::InvalidColor)?;
        let g = u8::from_str_radix(&s[2..4], 16).map_err(|_| Self::Err::InvalidColor)?;
        let b = u8::from_str_radix(&s[4..6], 16).map_err(|_| Self::Err::InvalidColor)?;
        Ok(Self { r, g, b })
    }
}

impl Serialize for Color {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.to_string().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Color {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Color::from_str(&s).map_err(serde::de::Error::custom)
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
