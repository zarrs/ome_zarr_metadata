#![doc = include_str!("../README.md")]
#![warn(unused_variables)]
#![warn(dead_code)]
#![deny(missing_docs)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

/// Version `0.4` (OME-NGFF) metadata.
///
/// <https://ngff.openmicroscopy.org/0.4/>.
pub mod v0_4;

/// Version `0.5` (OME-Zarr) metadata.
///
/// <https://ngff.openmicroscopy.org/0.5/>.
pub mod v0_5;

mod errors;
pub use errors::{Error, Result};

mod ndim;
pub use ndim::{MaybeNDim, NDim};

pub use validatrix::{Valid, Validate};

mod any;
pub use any::AnyOmeFields;

/// Future version of OME-Zarr metadata.
///
/// May not be up to date with the latest development specification.
/// May implement not-yet-stabilised RFCs.
/// Breaking API changes may be made at any time.
#[cfg(feature = "next")]
pub mod next;
