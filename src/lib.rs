#![doc = include_str!("../README.md")]
#![warn(unused_variables)]
#![warn(dead_code)]
#![deny(missing_docs)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

use serde::{Deserialize, Serialize};

/// Version `0.4` OME-NGFF metadata.
///
/// <https://ngff.openmicroscopy.org/0.4/>.
pub mod v0_4;

/// Version `0.5` OME-Zarr metadata.
///
/// <https://ngff.openmicroscopy.org/0.5/>.
pub mod v0_5;

/// Version `0.6` OME-Zarr metadata.
pub mod v0_6;

/// A marker trait for `"ome"` fields.
pub trait OmeFieldsTraits {}

/// OME-Zarr `"ome"` fields (versioned).
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum OmeFields {
    /// Version 0.6 ([`v0_6::OmeFields`]).
    V0_6(v0_6::OmeFields),
    /// Version 0.5 ([`v0_5::OmeFields`]).
    V0_5(v0_5::OmeFields),
}

impl OmeFieldsTraits for OmeFields {}

/// Zarr group `"attributes"` (OME-Zarr).
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OmeZarrGroupAttributes<TOmeFields: OmeFieldsTraits> {
    /// OME-Zarr `"ome"` fields.
    pub ome: TOmeFields,
}

/// Zarr group metadata (OME-Zarr).
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OmeZarrGroupMetadata<TOmeFields = OmeFields>
where
    TOmeFields: OmeFieldsTraits,
{
    /// Zarr attributes with "ome" metadata.
    pub attributes: OmeZarrGroupAttributes<TOmeFields>,
}

/// Zarr group `"attributes"` (OME-NGFF).
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct OmeNgffGroupAttributes {
    /// Transitional `bioformats2raw.layout` metadata.
    #[serde(
        flatten,
        skip_serializing_if = "Option::is_none",
        rename = "bioformats2raw.layout"
    )]
    pub bioformats2raw_layout: Option<v0_4::Bioformats2rawLayout>,
    /// Multiscales image metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiscales: Option<Vec<v0_4::MultiscaleImage>>,
    /// Labels metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<v0_4::Labels>,
    /// Image label metadata.
    #[serde(skip_serializing_if = "Option::is_none", rename = "image-label")]
    pub image_label: Option<v0_4::ImageLabel>,
    /// Plate metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plate: Option<v0_4::Plate>,
    /// Well metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub well: Option<v0_4::Well>,
}
