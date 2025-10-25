pub use crate::v0_4::axes::*;
pub use crate::v0_4::bioformats2raw_layout::*;
pub use crate::v0_4::coordinate_transformations::*;
pub use crate::v0_4::multiscales::{MultiscaleImageDataset, MultiscaleImageMetadata};
pub use crate::v0_4::omero::*;
pub use crate::v0_4::plate::{PlateAcquisition, PlateColumn, PlateRow, PlateWell};
pub use crate::v0_4::well::WellImage;
use crate::v0_5;
pub use crate::v0_5::labels::*;
pub use crate::v0_5::multiscales::*;
pub use crate::v0_5::plate::*;
pub use crate::v0_5::well::*;

use serde::Deserialize;
use serde::Serialize;
use validatrix::{Accumulator, Validate};

/// OME-Zarr "ome" fields.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct OmeFields {
    /// OME-Zarr version.
    pub version: monostate::MustBe!("0.6"),
    /// Transitional `bioformats2raw` metadata.
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub bioformats2raw: Option<Bioformats2Raw>,
    /// Multiscales image metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiscales: Option<MultiscaleImage>,
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
    /// Transitional OMERO metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub omero: Option<Omero>,
}

impl Validate for OmeFields {
    fn validate_inner(&self, accum: &mut Accumulator) {
        if let Some(m) = self.multiscales.as_ref() {
            accum.validate_member_at("multiscales", m);
        }

        if let Some(i) = self.image_label.as_ref() {
            accum.validate_member_at("imageLabel", i);
        }

        if let Some(p) = self.plate.as_ref() {
            accum.validate_member_at("plate", p);
        }

        if let Some(o) = self.omero.as_ref() {
            accum.validate_member_at("omero", o);
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

impl TryFrom<v0_5::OmeFields> for OmeFields {
    type Error = crate::Error;

    fn try_from(value: v0_5::OmeFields) -> Result<Self, Self::Error> {
        let multiscales = match value.multiscales {
            Some(v) => {
                if v.len() > 1 {
                    return Err(Self::Error::General(
                        "multiscales must have length 0 or 1".to_string(),
                    ));
                }
                v.into_iter().next()
            }
            None => None,
        };

        Ok(Self {
            version: Default::default(),
            bioformats2raw: value.bioformats2raw,
            multiscales,
            labels: value.labels,
            image_label: value.image_label,
            plate: value.plate,
            well: value.well,
            omero: value.omero,
        })
    }
}

impl TryFrom<v0_5::OmeFields> for OmeZarrGroupAttributes {
    type Error = crate::Error;

    fn try_from(value: v0_5::OmeFields) -> Result<Self, Self::Error> {
        Ok(Self {
            ome: OmeFields::try_from(value)?,
        })
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

    #[test]
    fn rfc_6() {
        let json = r#"{
  "zarr_format": 3,
  "node_type": "group",
  "attributes": {
    "ome": {
      "version": "0.6",
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
