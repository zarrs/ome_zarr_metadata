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

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
struct Ome {
    version: monostate::MustBe!("0.5"),
    #[serde(
        skip_serializing_if = "Option::is_none",
        rename = "bioformats2raw.layout"
    )]
    bioformats2raw_layout: Option<monostate::MustBe!(3u8)>,
    #[serde(skip_serializing_if = "Option::is_none")]
    multiscales: Option<Vec<MultiscaleImage>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<Vec<Labels>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "image-label")]
    image_label: Option<ImageLabel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    plate: Option<Plate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    well: Option<Well>,
}

/// Return the `ome` attribute from Zarr group metadata.
///
/// # Errors
/// Returns an error if:
///  - the `attributes`, `attributes.ome`, or `attributes.ome.version` keys do not exist, or
///  - the `attributes.ome.version` key is not equal to `"0.5"`.
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
