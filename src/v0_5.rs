pub(crate) mod labels;
pub(crate) mod multiscales;
pub(crate) mod plate;
pub(crate) mod well;

use crate::v0_4;
pub use crate::v0_4::axes::*;
pub use crate::v0_4::bioformats2raw_layout::*;
pub use crate::v0_4::coordinate_transformations::*;
pub use crate::v0_4::multiscales::{MultiscaleImageDataset, MultiscaleImageMetadata};
pub use crate::v0_4::plate::{PlateAcquisition, PlateColumn, PlateRow, PlateWell};
pub use crate::v0_4::well::WellImage;

pub use labels::*;
pub use multiscales::*;
pub use plate::*;
use serde::Deserialize;
use serde::Serialize;
use validatrix::{Accumulator, Validate};
pub use well::*;

use serde::de::Error;

/// OME-Zarr "ome" fields.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct OmeFields {
    /// OME-Zarr version.
    pub version: monostate::MustBe!("0.5"),
    /// Transitional `bioformats2raw` metadata.
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub bioformats2raw: Option<Bioformats2Raw>,
    /// Multiscales image metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiscales: Option<Vec<MultiscaleImage>>,
    /// Labels metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Labels>,
    /// Image label metadata.
    #[serde(skip_serializing_if = "Option::is_none", rename = "image-label")]
    pub image_label: Option<ImageLabel>,
    /// Plate metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plate: Option<Plate>,
    /// Well metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub well: Option<Well>,
    /// Transitional data, not fully supported.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub omero: Option<serde_json::Value>,
}

impl Validate for OmeFields {
    fn validate_inner(&self, accum: &mut Accumulator) {
        if let Some(m) = self.multiscales.as_ref() {
            accum.with_key("multiscales", |a| {
                if m.is_empty() {
                    a.add_failure("empty multiscales");
                }
                a.validate_iter(m);
            });
        }

        if let Some(i) = self.image_label.as_ref() {
            accum.validate_member_at("imageLabel", i);
        }
    }
}

/// OME-Zarr top-level group attributes.
///
/// This can be deserialised from a representation of a group's user attributes.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OmeZarrGroupAttributes {
    /// OME-Zarr "ome" fields.
    pub ome: OmeFields,
}

impl Validate for OmeZarrGroupAttributes {
    fn validate_inner(&self, accum: &mut Accumulator) {
        accum.validate_member_at("ome", &self.ome);
    }
}

impl From<v0_4::OmeNgffGroupAttributes> for OmeFields {
    fn from(value: v0_4::OmeNgffGroupAttributes) -> Self {
        Self {
            version: Default::default(),
            bioformats2raw: value.bioformats2raw,
            multiscales: value
                .multiscales
                .map(|v| v.into_iter().map(Into::into).collect()),
            labels: value.labels,
            image_label: value.image_label.map(Into::into),
            plate: value.plate.map(Into::into),
            well: value.well.map(Into::into),
            omero: None,
        }
    }
}

impl From<v0_4::OmeNgffGroupAttributes> for OmeZarrGroupAttributes {
    fn from(value: v0_4::OmeNgffGroupAttributes) -> Self {
        Self { ome: value.into() }
    }
}

/// OME-Zarr top-level group metadata.
///
/// This can be deserialised from a representation of the whole metadata document
/// (i.e. the contents of `zarr.json` in zarr v3, which includes user attributes and core metadata).
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OmeZarrGroupMetadata {
    /// Zarr attributes with "ome" metadata.
    pub attributes: OmeZarrGroupAttributes,
}

impl Validate for OmeZarrGroupMetadata {
    fn validate_inner(&self, accum: &mut Accumulator) {
        accum.validate_member_at("attributes", &self.attributes);
    }
}

/// Return the `ome` attribute from Zarr group metadata.
///
/// # Errors
/// Returns an error if:
///  - the `attributes`, `attributes.ome`, or `attributes.ome.version` keys do not exist, or
///  - the `attributes.ome.version` key is not equal to `"0.5"`.
// TODO: Deprecate this
pub fn get_ome_attribute_from_zarr_group_metadata(
    group_metadata: &serde_json::Map<String, serde_json::Value>,
) -> Result<&serde_json::Value, serde_json::Error> {
    if let Some(attributes) = group_metadata.get("attributes") {
        if let Some(ome) = attributes.get("ome") {
            let version = ome.get("version").ok_or(serde_json::Error::custom(
                "the ome metadata does not contain the version key.".to_string(),
            ))?;
            let _version: monostate::MustBe!("0.5") = serde_json::from_value(version.clone())?;
            Ok(ome)
        } else {
            Err(serde_json::Error::custom(
                "the group attributes do not contain the ome key.".to_string(),
            ))
        }
    } else {
        Err(serde_json::Error::custom(
            "the group does not contain the attributes key.".to_string(),
        ))
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn fields_default() {
        let _ome_fields = OmeFields {
            labels: Some(vec!["x".to_string(), "y".to_string()]),
            ..OmeFields::default()
        };
    }
}
