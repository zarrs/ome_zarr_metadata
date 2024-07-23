//! "bioformats2raw.layout" metadata (transitional).
//!
//! <https://ngff.openmicroscopy.org/0.4/#bf2raw>.

use serde::{Deserialize, Serialize};

/// Top level group metadata indicating with the transitional `bioformats2raw.layout` metadata.
#[derive(Serialize, Deserialize, Debug)]
pub struct Bioformats2rawLayout {
    /// The top-level identifier metadata added by bioformats2raw
    #[serde(rename = "bioformats2raw.layout")]
    pub bioformats2raw_layout: monostate::MustBe!(3u64),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bioformats2raw_layout_image() {
        let json = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/ome-zarr/0.4/examples/bf2raw/image.json"
        ));
        let _bioformats2raw: Bioformats2rawLayout = serde_json::from_str(json).unwrap();
    }

    #[test]
    fn bioformats2raw_layout_plate() {
        let json = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/ome-zarr/0.4/examples/bf2raw/plate.json"
        ));
        let _bioformats2raw: Bioformats2rawLayout = serde_json::from_str(json).unwrap();
    }
}
