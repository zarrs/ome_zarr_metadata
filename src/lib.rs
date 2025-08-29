#![doc = include_str!("../README.md")]
#![warn(unused_variables)]
#![warn(dead_code)]
#![deny(missing_docs)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

#[cfg(test)]
pub(crate) mod tests;

/// Version `0.4` (OME-NGFF) metadata.
///
/// <https://ngff.openmicroscopy.org/0.4/>.
pub mod v0_4;

/// Version `0.5` (OME-Zarr) metadata.
///
/// <https://ngff.openmicroscopy.org/0.5/>.
pub mod v0_5;
