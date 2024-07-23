//! "multiscales" metadata.
//!
//! <https://ngff--249.org.readthedocs.build/0.5-dev1/#multiscale-md>.

use serde::{Deserialize, Serialize};

use super::{Axis, CoordinateTransform, MultiscaleImageDataset, MultiscaleImageMetadata};

/// `multiscales` element metadata. Describes a multiscale image.
#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct MultiscaleImage {
    /// The version of the multiscale metadata of the image.
    pub version: monostate::MustBe!("0.5-dev1"),
    /// The name of the multiscale image (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The axes of the multiscale image.
    pub axes: Vec<Axis>,
    /// The datasets describe the arrays storing the individual resolution levels.
    pub datasets: Vec<MultiscaleImageDataset>,
    /// Describes transformations that are applied to all resolution levels in the same manner (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coordinate_transformations: Option<Vec<CoordinateTransform>>,
    /// The type of downscaling method used to generate the multiscale image pyramid (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// A dictionary with additional information about the downscaling method (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<MultiscaleImageMetadata>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multiscales_0_5_dev1_spec() {
        let json = r#"
{
    "multiscales": [
        {
            "version": "0.5-dev1",
            "name": "example",
            "axes": [
                {"name": "t", "type": "time", "unit": "millisecond"},
                {"name": "c", "type": "channel"},
                {"name": "z", "type": "space", "unit": "micrometer"},
                {"name": "y", "type": "space", "unit": "micrometer"},
                {"name": "x", "type": "space", "unit": "micrometer"}
            ],
            "datasets": [
                {
                    "path": "0",
                    "coordinateTransformations": [{
                        "type": "scale",
                        "scale": [1.0, 1.0, 0.5, 0.5, 0.5]
                    }]
                },
                {
                    "path": "1",
                    "coordinateTransformations": [{
                        "type": "scale",
                        "scale": [1.0, 1.0, 1.0, 1.0, 1.0]
                    }]
                },
                {
                    "path": "2",
                    "coordinateTransformations": [{
                        "type": "scale",
                        "scale": [1.0, 1.0, 2.0, 2.0, 2.0]
                    }]
                }
            ],
            "coordinateTransformations": [{
                "type": "scale",
                "scale": [0.1, 1.0, 1.0, 1.0, 1.0]
            }],
            "type": "gaussian",
            "metadata": {
                "description": "the fields in metadata depend on the downscaling implementation. Here, the parameters passed to the skimage function are given",
                "method": "skimage.transform.pyramid_gaussian",
                "version": "0.16.1",
                "args": "[true]",
                "kwargs": {"multichannel": true}
            }
        }
    ]
}
"#;
        let map: serde_json::Map<String, serde_json::Value> = serde_json::from_str(json).unwrap();
        let multiscales = map.get("multiscales").unwrap();
        let _multiscales: Vec<MultiscaleImage> =
            serde_json::from_value(multiscales.clone()).unwrap();
    }

    #[test]
    fn multiscales_0_5_dev1_minimal() {
        let json = r#"
{
    "multiscales": [
        {
            "version": "0.5-dev1",
            "name": "example",
            "axes": [
                {"name": "t"},
                {"name": "c"},
                {"name": "z"},
                {"name": "y"},
                {"name": "x"}
            ],
            "datasets": [
                {
                    "path": "0",
                    "coordinateTransformations": [{
                        "type": "scale",
                        "scale": [1.0, 1.0, 0.5, 0.5, 0.5]
                    }]
                },
                {
                    "path": "1",
                    "coordinateTransformations": [{
                        "type": "scale",
                        "scale": [1.0, 1.0, 1.0, 1.0, 1.0]
                    }]
                },
                {
                    "path": "2",
                    "coordinateTransformations": [{
                        "type": "scale",
                        "scale": [1.0, 1.0, 2.0, 2.0, 2.0]
                    }]
                }
            ]
        }
    ]
}
"#;
        let map: serde_json::Map<String, serde_json::Value> = serde_json::from_str(json).unwrap();
        let multiscales = map.get("multiscales").unwrap();
        let _multiscales: Vec<MultiscaleImage> =
            serde_json::from_value(multiscales.clone()).unwrap();
    }
}
