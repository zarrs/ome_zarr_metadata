pub use crate::v0_4::axes::*;
pub use crate::v0_4::bioformats2raw_layout::*;
pub use crate::v0_4::coordinate_transformations::*;
pub use crate::v0_4::multiscales::{MultiscaleImageDataset, MultiscaleImageMetadata};
pub use crate::v0_4::plate::{PlateAcquisition, PlateColumn, PlateRow, PlateWell};
pub use crate::v0_4::well::WellImage;
pub use crate::v0_5::labels::*;
pub use crate::v0_5::multiscales::*;
pub use crate::v0_5::plate::*;
pub use crate::v0_5::well::*;

use serde::Deserialize;
use serde::Serialize;

/// OME-Zarr "ome" fields.
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
    pub multiscale: Option<MultiscaleImage>,
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

/// OME-Zarr top-level group attributes.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OmeZarrGroupAttributes {
    /// OME-Zarr "ome" fields.
    pub ome: OmeFields,
}

/// OME-Zarr top-level group metadata.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OmeZarrGroupMetadata {
    /// Zarr attributes with "ome" metadata.
    pub attributes: OmeZarrGroupAttributes,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rfc_6() {
        let json = r#"{
  "zarr_format": 3,
  "node_type": "group",
  "attributes": {
    "ome": {
      "version": "0.5",
      "multiscale": {
        "name": "example",
        "axes": [
          { "name": "t", "type": "time", "unit": "millisecond" },
          { "name": "c", "type": "channel" },
          { "name": "z", "type": "space", "unit": "micrometer" },
          { "name": "y", "type": "space", "unit": "micrometer" },
          { "name": "x", "type": "space", "unit": "micrometer" }
        ],
        "datasets": [
          {
            "path": "0",
            "coordinateTransformations": [
              {
                "type": "scale",
                "scale": [1.0, 1.0, 0.5, 0.5, 0.5]
              }
            ]
          },
          {
            "path": "1",
            "coordinateTransformations": [
              {
                "type": "scale",
                "scale": [1.0, 1.0, 1.0, 1.0, 1.0]
              }
            ]
          }
        ],
        "coordinateTransformations": [
          {
            "type": "scale",
            "scale": [0.1, 1.0, 1.0, 1.0, 1.0]
          }
        ]
      }
    }
  }
}"#;

        serde_json::from_str::<OmeZarrGroupMetadata>(&json).unwrap();
    }
}
