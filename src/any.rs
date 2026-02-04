use super::{v0_4, v0_5};
use serde::Deserialize;
use validatrix::Validate;

/// OME-Zarr metadata in any supported version.
#[derive(Debug, Deserialize, Clone)]
#[serde(from = "AnyOmeZarrAttributes")]
pub enum AnyOmeFields {
    /// Version 0.4 metadata
    V0_4(v0_4::OmeNgffGroupAttributes),
    /// Version 0.5 metadata
    V0_5(v0_5::OmeFields),
    #[cfg(feature = "next")]
    /// Future version metadata
    VNext(super::next::OmeFields),
}

impl AnyOmeFields {
    /// Get the version string for the OME-Zarr metadata.
    pub fn version(&self) -> String {
        match self {
            AnyOmeFields::V0_4(m) => m.version(),
            AnyOmeFields::V0_5(m) => m.version.to_string(),
            #[cfg(feature = "next")]
            AnyOmeFields::VNext(m) => m.version.to_string(),
        }
    }
}

impl Validate for AnyOmeFields {
    fn validate_inner(&self, accum: &mut validatrix::Accumulator) {
        match self {
            AnyOmeFields::V0_4(attrs) => attrs.validate_inner(accum),
            AnyOmeFields::V0_5(fields) => accum.validate_member_at("ome", fields),
            #[cfg(feature = "next")]
            AnyOmeFields::VNext(fields) => accum.validate_member_at("ome", fields),
        }
    }
}

/// Extend this enum and related impls when adding support
/// for future versions.
#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
enum NamespacedOmeFields {
    V0_5(v0_5::OmeFields),
    #[cfg(feature = "next")]
    VNext(super::next::OmeFields),
}

impl From<NamespacedOmeFields> for AnyOmeFields {
    fn from(value: NamespacedOmeFields) -> Self {
        match value {
            NamespacedOmeFields::V0_5(ome) => Self::V0_5(ome),
            #[cfg(feature = "next")]
            NamespacedOmeFields::VNext(ome) => Self::VNext(ome),
        }
    }
}

/// Extend this enum and related impls when adding support
/// for pre-0.5 versions.
#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
enum FreeOmeFields {
    V0_4(v0_4::OmeNgffGroupAttributes),
}

impl From<FreeOmeFields> for AnyOmeFields {
    fn from(value: FreeOmeFields) -> Self {
        match value {
            FreeOmeFields::V0_4(ome) => Self::V0_4(ome),
        }
    }
}

/// Intermediate type used for deserialising either pre- or post-0.5
/// OME-Zarr metadata, which stored their fields freely in the zarr attributes,
/// or within the "ome" namespace respectively.
#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
enum AnyOmeZarrAttributes {
    Namespaced { ome: NamespacedOmeFields },
    Free(FreeOmeFields),
}

impl From<AnyOmeZarrAttributes> for AnyOmeFields {
    fn from(value: AnyOmeZarrAttributes) -> Self {
        match value {
            AnyOmeZarrAttributes::Namespaced { ome } => ome.into(),
            AnyOmeZarrAttributes::Free(ome) => ome.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use serde::Serialize;

    use super::*;

    fn can_roundtrip<T: Serialize>(expected_version: &str, group_attrs: &T) {
        let s = serde_json::to_string(group_attrs).unwrap();
        let attrs2: AnyOmeFields = serde_json::from_str(&s).unwrap();
        assert_eq!(attrs2.version(), expected_version);
    }

    #[test]
    fn can_deser_v0_4() {
        let val = v0_4::OmeNgffGroupAttributes {
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
        can_roundtrip("0.4", &val);
    }

    #[test]
    fn can_deser_v0_5() {
        let val = v0_5::OmeZarrGroupAttributes {
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
        can_roundtrip("0.5", &val);
    }
}
