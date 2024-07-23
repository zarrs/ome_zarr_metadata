#![doc = include_str!("../README.md")]
#![warn(unused_variables)]
#![warn(dead_code)]
#![deny(missing_docs)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

/// Version `0.4` (OME-NGFF) metadata.
///
/// <https://ngff.openmicroscopy.org/0.4/>.
pub mod v0_4;

/// Version `0.5-dev` metadata. Editor's Draft, 17 July 2024.
///
/// <https://ngff.openmicroscopy.org/latest/>.
pub mod v0_5_dev;

/// Version `0.5-dev1` metadata. Editor's Draft, 27 June 2024.
///
/// <https://ngff--249.org.readthedocs.build/0.5-dev1/>.
pub mod v0_5_dev1;

/// Version `0.5+RFC-2` metadata. Editor's Draft, 4 July 2024.
///
/// <https://ngff--242.org.readthedocs.build/latest/>.
#[allow(dead_code, unused_imports)]
pub mod v0_5_rfc2;
