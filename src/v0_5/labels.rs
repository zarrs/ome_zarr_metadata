//! "labels" metadata.
//!
//! <https://ngff.openmicroscopy.org/0.5/#labels-md>.

#[doc(inline)]
pub use crate::v0_4::{ImageLabelColor, ImageLabelProperties, ImageLabelSource, Labels};

use serde::{Deserialize, Serialize};

/// `image-label` metadata. Stores information about the display colors, source image, and optionally, further arbitrary properties of a label image.
#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct ImageLabel {
    /// Describes the color information for the unique label values.
    pub colors: Vec<ImageLabelColor>,
    /// Arbitrary metadata associated with each unique label (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<Vec<ImageLabelProperties>>,
    /// Information about the original image from which the label image derives (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<ImageLabelSource>,
}

#[cfg(test)]
mod tests {
    use crate::v0_5::get_ome_attribute_from_zarr_group_metadata;

    use super::*;

    #[test]
    fn labels_color_properties() {
        let json = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/ome-zarr/0.5/examples/label_strict/colors_properties.json"
        ));
        let group_metadata: serde_json::Map<String, serde_json::Value> = serde_json::from_str(json).unwrap();
        let ome_metadata = get_ome_attribute_from_zarr_group_metadata(&group_metadata).unwrap();
        let image_label = ome_metadata.get("image-label").unwrap();
        let _image_label: ImageLabel = serde_json::from_value(image_label.clone()).unwrap();
    }
}
