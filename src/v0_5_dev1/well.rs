//! "well" metadata.
//!
//! <https://ngff--249.org.readthedocs.build/0.5-dev1/#well-md>.

use serde::{Deserialize, Serialize};

use super::WellImage;

/// `well` metadata. Describes all fields of views under a given well.
#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Well {
    /// The version of the "well" schema.
    pub version: monostate::MustBe!("0.5-dev1"),
    /// Specifies the fields of views of the well.
    pub images: Vec<WellImage>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn well_0_5_dev1_spec0() {
        let json = r#"
{
    "well": {
        "images": [
            {
                "acquisition": 1,
                "path": "0"
            },
            {
                "acquisition": 1,
                "path": "1"
            },
            {
                "acquisition": 2,
                "path": "2"
            },
            {
                "acquisition": 2,
                "path": "3"
            }
        ],
        "version": "0.5-dev1"
    }
}
"#;
        let map: serde_json::Map<String, serde_json::Value> = serde_json::from_str(json).unwrap();
        let well = map.get("well").unwrap();
        let _well: Well = serde_json::from_value(well.clone()).unwrap();
    }

    #[test]
    fn well_0_5_dev1_spec1() {
        let json = r#"
{
    "well": {
        "images": [
            {
                "acquisition": 0,
                "path": "0"
            },
            {
                "acquisition": 3,
                "path": "1"
            }
        ],
        "version": "0.5-dev1"
    }
}
"#;
        let map: serde_json::Map<String, serde_json::Value> = serde_json::from_str(json).unwrap();
        let well = map.get("well").unwrap();
        let _well: Well = serde_json::from_value(well.clone()).unwrap();
    }
}
