use super::{v0_4, v0_5};
use serde::Deserialize;
use validatrix::Validate;

/// OME-Zarr metadata in any supported version.
#[derive(Debug, Deserialize, Clone)]
#[serde(untagged, from = "AnyOmeIntermediate")]
pub enum AnyOme {
    /// Version 0.4 metadata
    V0_4(v0_4::OmeNgffGroupAttributes),
    /// Version 0.5 metadata
    V0_5(v0_5::OmeFields),
}

impl AnyOme {
    /// Get the version string for the OME-Zarr metadata.
    pub fn version(&self) -> &'static str {
        match self {
            AnyOme::V0_4(_) => "0.4",
            AnyOme::V0_5(_) => "0.5",
        }
    }
}

impl Validate for AnyOme {
    fn validate_inner(&self, accum: &mut validatrix::Accumulator) {
        match self {
            AnyOme::V0_4(attrs) => attrs.validate_inner(accum),
            AnyOme::V0_5(fields) => accum.validate_member_at("ome", fields),
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
enum AnyContainedOme {
    V0_5(v0_5::OmeFields),
}

impl From<AnyContainedOme> for AnyOme {
    fn from(value: AnyContainedOme) -> Self {
        match value {
            AnyContainedOme::V0_5(ome) => Self::V0_5(ome),
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
enum AnyFreeOme {
    V0_4(v0_4::OmeNgffGroupAttributes),
}

impl From<AnyFreeOme> for AnyOme {
    fn from(value: AnyFreeOme) -> Self {
        match value {
            AnyFreeOme::V0_4(ome) => Self::V0_4(ome),
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
enum AnyOmeIntermediate {
    Contained { ome: AnyContainedOme },
    Free(AnyFreeOme),
}

impl From<AnyOmeIntermediate> for AnyOme {
    fn from(value: AnyOmeIntermediate) -> Self {
        match value {
            AnyOmeIntermediate::Contained { ome } => ome.into(),
            AnyOmeIntermediate::Free(ome) => ome.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_deser_v0_4() {
        let attrs = v0_4::OmeNgffGroupAttributes {
            multiscales: Some(vec![v0_4::MultiscaleImage {
                version: Default::default(),
                name: Some("test".into()),
                axes: vec![],
                datasets: vec![],
                coordinate_transformations: None,
                r#type: None,
                metadata: Default::default(),
            }]),
            ..Default::default()
        };
        let s = serde_json::to_string(&attrs).unwrap();
        let attrs2: AnyOme = serde_json::from_str(&s).unwrap();
        assert_eq!(attrs2.version(), "0.4");
    }

    #[test]
    fn can_deser_v0_5() {
        let attrs = v0_5::OmeZarrGroupAttributes {
            ome: v0_5::OmeFields {
                multiscales: Some(vec![v0_5::MultiscaleImage {
                    name: Some("test".into()),
                    axes: vec![],
                    datasets: vec![],
                    coordinate_transformations: None,
                    r#type: None,
                    metadata: Default::default(),
                }]),
                ..Default::default()
            },
        };
        let s = serde_json::to_string(&attrs).unwrap();
        let attrs2: AnyOme = serde_json::from_str(&s).unwrap();
        assert_eq!(attrs2.version(), "0.5");
    }
}
