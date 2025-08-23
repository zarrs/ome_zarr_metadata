//! "bioformats2raw.layout" metadata (transitional).
//!
//! <https://ngff.openmicroscopy.org/0.4/#bf2raw>.

use serde::{Deserialize, Serialize};

/// Top level group metadata indicating with the transitional `bioformats2raw.layout` metadata.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Bioformats2Raw {
    /// The top-level identifier metadata added by bioformats2raw
    #[serde(rename = "bioformats2raw.layout")]
    pub layout: monostate::MustBe!(3u64),
    /// Paths to image groups
    pub series: Option<Vec<String>>,
}

#[cfg(test)]
mod tests {
    use crate::v0_4::OmeNgffGroupAttributes;

    use super::*;

    #[test]
    fn bioformats2raw_layout_image() {
        let json = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/ome-zarr/0.4/examples/bf2raw/image.json"
        ));
        let ome_metadata: OmeNgffGroupAttributes = serde_json::from_str(json).unwrap();
        let _bioformats2raw: Bioformats2Raw = ome_metadata.bioformats2raw_layout.unwrap();
    }

    #[test]
    fn bioformats2raw_layout_plate() {
        let json = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/ome-zarr/0.4/examples/bf2raw/plate.json"
        ));
        let ome_metadata: OmeNgffGroupAttributes = serde_json::from_str(json).unwrap();
        let _bioformats2raw: Bioformats2Raw = ome_metadata.bioformats2raw_layout.unwrap();
    }
}
