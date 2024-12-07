//! "labels" and "image-label" metadata.
//!
//! <https://ngff.openmicroscopy.org/0.4/#labels-md>.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// `labels` metadata. A JSON array of paths to the labeled multiscale image(s).
pub type Labels = Vec<String>;

/// `image-label` metadata. Stores information about the display colors, source image, and optionally, further arbitrary properties of a label image.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct ImageLabel {
    /// The version of the OME-NGFF "image-label" schema.
    pub version: monostate::MustBe!("0.4"),
    /// Describes the color information for the unique label values.
    pub colors: Vec<ImageLabelColor>,
    /// Arbitrary metadata associated with each unique label (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<Vec<ImageLabelProperties>>,
    /// Information about the original image from which the label image derives (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<ImageLabelSource>,
}

/// [`ImageLabel`] `colors` element metadata. The colour of a unique image label.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct ImageLabelColor {
    #[serde(alias = "label-value")]
    label_value: u64,
    rgba: [u8; 4],
}

/// [`ImageLabel`] `properties` element metadata. Arbitrary metadata of a unique image label.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ImageLabelProperties {
    #[serde(alias = "label-value")]
    label_value: u64,
    #[serde(flatten)]
    properties: serde_json::Map<String, serde_json::Value>,
}

/// [`ImageLabel`] `source` metadata. Information about the source of a label image.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ImageLabelSource {
    image: Option<PathBuf>,
    #[serde(flatten)]
    source: serde_json::Map<String, serde_json::Value>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn labels_color_properties() {
        let json = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/ome-zarr/0.4/examples/label_strict/colors_properties.json"
        ));
        let map: serde_json::Map<String, serde_json::Value> = serde_json::from_str(json).unwrap();
        let image_label = map.get("image-label").unwrap();
        let _image_label: ImageLabel = serde_json::from_value(image_label.clone()).unwrap();
    }
}
