//! "labels" metadata.
//!
//! <https://ngff--249.org.readthedocs.build/0.5-dev1/#labels-md>.

#[doc(inline)]
pub use crate::v0_4::{ImageLabelColor, ImageLabelProperties, ImageLabelSource, Labels};

use serde::{Deserialize, Serialize};

/// `image-label` metadata. Stores information about the display colors, source image, and optionally, further arbitrary properties of a label image.
#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct ImageLabel {
    /// The version of the OME-NGFF "image-label" schema.
    pub version: monostate::MustBe!("0.5-dev1"),
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
    fn labels_0_5_dev1_spec() {
        let json = r#"
{
  "labels": [
    "cell_space_segmentation"
  ]
}
"#;
        let map: serde_json::Map<String, serde_json::Value> = serde_json::from_str(json).unwrap();
        let labels = map.get("labels").unwrap();
        let _labels: Labels = serde_json::from_value(labels.clone()).unwrap();
    }

    #[test]
    fn image_label_0_5_dev1_spec() {
        let json = r#"
{
  "image-label": {
    "version": "0.5-dev1",
    "colors": [
      {
        "label-value": 0,
        "rgba": [0, 0, 128, 128]
      },
      {
        "label-value": 1,
        "rgba": [0, 128, 0, 128]
      }
    ],
    "properties": [
      {
        "label-value": 0,
        "area (pixels)": 1200,
        "class": "intercellular space"
      },
      {
        "label-value": 1,
        "area (pixels)": 1650,
        "class": "cell",
        "cell type": "neuron"
      }
    ],
    "source": {
      "image": "../../"
    }
  }
}
"#;
        let map: serde_json::Map<String, serde_json::Value> = serde_json::from_str(json).unwrap();
        let image_label = map.get("image-label").unwrap();
        let _image_label: ImageLabel = serde_json::from_value(image_label.clone()).unwrap();
    }
    #[test]
    fn image_label_0_5_dev1_minimal() {
        let json = r#"
{
  "image-label": {
    "version": "0.5-dev1",
    "colors": [
      {
        "label-value": 0,
        "rgba": [0, 0, 128, 128]
      },
      {
        "label-value": 1,
        "rgba": [0, 128, 0, 128]
      }
    ]
  }
}
"#;
        let map: serde_json::Map<String, serde_json::Value> = serde_json::from_str(json).unwrap();
        let image_label = map.get("image-label").unwrap();
        let _image_label: ImageLabel = serde_json::from_value(image_label.clone()).unwrap();
    }
}
