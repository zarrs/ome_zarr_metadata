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

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Ome {
    #[serde(
        flatten,
        skip_serializing_if = "Option::is_none",
        rename = "bioformats2raw.layout"
    )]
    bioformats2raw_layout: Option<Bioformats2rawLayout>,
    #[serde(skip_serializing_if = "Option::is_none")]
    multiscales: Option<Vec<MultiscaleImage>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<Vec<Labels>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "image-label")]
    image_label: Option<ImageLabel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    plate: Option<Plate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    well: Option<Well>,
}
