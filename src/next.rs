pub use crate::v0_5 as prev;
pub use prev::{
    Axis, AxisType, AxisUnit, AxisUnitSpace, AxisUnitTime, Bioformats2Raw, Channel, Color,
    CoordinateTransform, CoordinateTransformScale, CoordinateTransformTranslation, ImageLabel,
    Labels, MultiscaleImageDataset, MultiscaleImageMetadata, Omero, Plate, PlateAcquisition,
    PlateColumn, PlateRow, PlateWell, Well, WellImage, Window,
};

use serde::{Deserialize, Serialize};
use validatrix::{Accumulator, Validate};

mod multiscales;
pub use multiscales::*;

crate::constrained_version!(ConstrainedVersion, ">=0.6.dev0", "0.6.dev3");

/// OME-Zarr "ome" fields.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct OmeFields {
    /// OME-Zarr version.
    pub version: ConstrainedVersion,
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
    /// Transitional OMERO metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub omero: Option<Omero>,
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

impl TryFrom<prev::OmeFields> for OmeFields {
    type Error = crate::Error;

    fn try_from(value: prev::OmeFields) -> Result<Self, Self::Error> {
        Ok(Self {
            version: Default::default(),
            bioformats2raw: value.bioformats2raw,
            multiscales: value
                .multiscales
                .map(|v| v.into_iter().map(Into::into).collect()),
            labels: value.labels,
            image_label: value.image_label,
            plate: value.plate,
            well: value.well,
            omero: value.omero,
        })
    }
}

impl TryFrom<prev::OmeZarrGroupAttributes> for OmeZarrGroupAttributes {
    type Error = crate::Error;

    fn try_from(value: prev::OmeZarrGroupAttributes) -> Result<Self, Self::Error> {
        Ok(Self {
            ome: OmeFields::try_from(value.ome)?,
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
