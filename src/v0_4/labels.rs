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
    /// Integer label value.
    #[serde(rename = "label-value")]
    pub label_value: u64,
    /// Colour as RGBA array.
    pub rgba: [u8; 4],
}

/// [`ImageLabel`] `properties` element metadata. Arbitrary metadata of a unique image label.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ImageLabelProperties {
    /// Integer label value.
    #[serde(rename = "label-value")]
    pub label_value: u64,
    /// Arbitrary metadata associated with the label.
    #[serde(flatten)]
    pub properties: serde_json::Map<String, serde_json::Value>,
}

/// [`ImageLabel`] `source` metadata. Information about the source of a label image.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ImageLabelSource {
    /// Relative path to the zarr image group which this group labels.
    pub image: Option<PathBuf>,
}

#[cfg(test)]
mod tests {
    use crate::v0_4::OmeNgffGroupAttributes;

    use super::*;

    #[test]
    fn labels_color_properties() {
        let json = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/ome-zarr/0.4/examples/label_strict/colors_properties.json"
        ));
        let ome_metadata: OmeNgffGroupAttributes = serde_json::from_str(json).unwrap();
        let _image_label: ImageLabel = ome_metadata.image_label.unwrap();
    }
}
