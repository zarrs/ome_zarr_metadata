pub(crate) mod axes;
pub(crate) mod bioformats2raw_layout;
pub(crate) mod coordinate_transformations;
pub(crate) mod labels;
pub(crate) mod multiscales;
pub(crate) mod plate;
pub(crate) mod well;

pub use axes::*;
pub use bioformats2raw_layout::*;
pub use coordinate_transformations::*;
pub use labels::*;
pub use multiscales::*;
pub use plate::*;
use serde::{Deserialize, Serialize};
pub use well::*;

/// OME-NGFF top-level group attributes.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct OmeNgffGroupAttributes {
    /// Transitional `bioformats2raw.layout` metadata.
    #[serde(
        flatten,
        skip_serializing_if = "Option::is_none",
        rename = "bioformats2raw.layout"
    )]
    pub bioformats2raw_layout: Option<Bioformats2rawLayout>,
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::*;
    
    const VERSION: (u64, u64) = (0, 4);

    #[test]
    fn parse_examples() {
        let mut msg = String::default();
        let mut failed = 0;
        let mut total = 0;
        for (dname, map) in get_examples(VERSION) {
            for (fname, content) in map {
                total += 1;

                let Err(e) = serde_json::from_str::<OmeNgffGroupAttributes>(content) else {
                    continue;
                };
                failed += 1;
                msg.push_str(&format!(
                    "dir {dname}, example {fname}: failed with error {e}\n"
                ));
            }
        }
        if failed > 0 {
            panic!("Failed {failed} of {total}:\n{}", msg.trim_end());
        }
    }
}
