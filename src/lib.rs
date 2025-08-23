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

mod errors;
pub use errors::{Error, Result};

mod validation;
pub use validation::OmeValidate;

/// Trait for a type which has some dimensionality which can always be determined by its metadata.
pub trait NDim {
    /// Number of dimensions according to the metadata.
    fn ndim(&self) -> usize;
}

/// Trait for a type which has some dimensionality which may be determinable by its metadata.
pub trait MaybeNDim {
    /// None if number of dimensions is indeterminate from the metadata.
    fn maybe_ndim(&self) -> Option<usize>;

    /// If both objects have a dimensionality defined, but it's different,
    /// return Some with the two dimensionalities. Otherwise, return None.
    fn ndim_conflicts<T: MaybeNDim>(&self, other: &T) -> Option<(usize, usize)> {
        if let (Some(d1), Some(d2)) = (self.maybe_ndim(), other.maybe_ndim()) {
            if d1 != d2 {
                return Some((d1, d2));
            }
        }
        None
    }
}

impl<T: NDim> MaybeNDim for T {
    fn maybe_ndim(&self) -> Option<usize> {
        Some(NDim::ndim(self))
    }
}
