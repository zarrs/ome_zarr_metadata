pub(crate) mod labels;
pub(crate) mod multiscales;
pub(crate) mod plate;
pub(crate) mod well;

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
pub use well::*;

use serde::de::Error;

/// Zarr group metadata (OME-Zarr 0.5).
pub type OmeZarrGroupMetadata = crate::OmeZarrGroupMetadata<OmeFields>;

/// Zarr group `"attributes"` (OME-Zarr 0.5).
pub type OmeZarrGroupAttributes = crate::OmeZarrGroupAttributes<OmeFields>;

/// OME-Zarr `"ome"` fields.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(deny_unknown_fields)]
pub struct OmeFields {
    /// OME-Zarr version.
    pub version: monostate::MustBe!("0.5"),
    /// Transitional `bioformats2raw.layout` metadata.
    #[serde(
        flatten,
        skip_serializing_if = "Option::is_none",
        rename = "bioformats2raw.layout"
    )]
    pub bioformats2raw_layout: Option<Bioformats2rawLayout>,
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
}

impl crate::OmeFieldsTraits for OmeFields {}

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
