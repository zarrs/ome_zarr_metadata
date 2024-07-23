//! "labels" metadata.
//!
//! <https://ngff.openmicroscopy.org/latest/#labels-md>.

#[doc(inline)]
pub use crate::v0_4::{ImageLabelColor, ImageLabelProperties, ImageLabelSource, Labels};

use serde::{Deserialize, Serialize};

/// `image-label` metadata. Stores information about the display colors, source image, and optionally, further arbitrary properties of a label image.
#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct ImageLabel {
    /// The version of the OME-NGFF "image-label" schema.
    pub version: monostate::MustBe!("0.5-dev"),
    /// Describes the color information for the unique label values.
    pub colors: Vec<ImageLabelColor>,
    /// Arbitrary metadata associated with each unique label.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<Vec<ImageLabelProperties>>,
    /// Information about the original image from which the label image derives (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<ImageLabelSource>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn labels_color_properties() {
        let json = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/ome-zarr/latest/examples/label_strict/colors_properties.json"
        ));
        let map: serde_json::Map<String, serde_json::Value> = serde_json::from_str(json).unwrap();
        let image_label = map.get("image-label").unwrap();
        let _image_label: ImageLabel = serde_json::from_value(image_label.clone()).unwrap();
    }
}
