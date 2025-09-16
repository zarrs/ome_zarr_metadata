//! "labels" metadata.
//!
//! <https://ngff.openmicroscopy.org/0.5/#labels-md>.

use crate::v0_4::validate_unique_labels;
#[doc(inline)]
pub use crate::v0_4::{ImageLabelColor, ImageLabelProperties, ImageLabelSource, Labels};

use serde::{Deserialize, Serialize};
use validatrix::{ Validate};

/// `image-label` metadata. Stores information about the display colors, source image, and optionally, further arbitrary properties of a label image.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct ImageLabel {
    /// Describes the color information for the unique label values.
    pub colors: Option<Vec<ImageLabelColor>>,
    /// Arbitrary metadata associated with each unique label (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<Vec<ImageLabelProperties>>,
    /// Information about the original image from which the label image derives (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<ImageLabelSource>,
}

impl Validate for ImageLabel {
    fn validate_inner(&self, accum: &mut validatrix::Accumulator) -> usize {
        let mut total = 0;
        if let Some(c) = self.colors.as_ref() {
            accum.prefix.push("colors".into());
            total += validate_unique_labels(accum, c.iter());
            accum.prefix.pop();
        }

        if let Some(p) = self.properties.as_ref() {
            accum.prefix.push("properties".into());
            total += validate_unique_labels(accum, p.iter());
            accum.prefix.pop();
        }
        total
    }
}

impl From<crate::v0_4::ImageLabel> for ImageLabel {
    fn from(value: crate::v0_4::ImageLabel) -> Self {
        Self {
            colors: value.colors,
            properties: value.properties,
            source: value.source,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::v0_5::OmeZarrGroupMetadata;

    use super::*;

    #[test]
    fn labels_color_properties() {
        let json = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/ome-zarr/0.5/examples/label_strict/colors_properties.json"
        ));
        let ome_metadata: OmeZarrGroupMetadata = serde_json::from_str(json).unwrap();
        let _image_label: ImageLabel = ome_metadata.attributes.ome.image_label.unwrap();
    }
}
