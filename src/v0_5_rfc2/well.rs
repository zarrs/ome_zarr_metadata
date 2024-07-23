//! "well" metadata.
//!
//! <https://ngff--242.org.readthedocs.build/latest/index.html#well-md>.

use serde::{Deserialize, Serialize};

use super::WellImage;

/// `well` metadata. Describes all fields of views under a given well.
#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Well {
    /// Specifies the fields of views of the well.
    pub images: Vec<WellImage>,
}

#[cfg(test)]
mod tests {
    use crate::v0_5_rfc2::get_ome_attribute_from_zarr_group_metadata;

    use super::*;

    #[test]
    fn well_0_5_rfc2_spec0() {
        let json = r#"
{
  "zarr_format": 3,
  "node_type": "group",
  "attributes": {
    "ome": {
      "version": "0.5",
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
        ]
      }
    }
  }
}
"#;
        let group_metadata: serde_json::Map<String, serde_json::Value> =
            serde_json::from_str(json).unwrap();
        let ome_metadata = get_ome_attribute_from_zarr_group_metadata(&group_metadata).unwrap();
        let well = ome_metadata.get("well").unwrap();
        let _well: Well = serde_json::from_value(well.clone()).unwrap();
    }

    #[test]
    fn well_0_5_rfc2_spec1() {
        let json = r#"
{
  "zarr_format": 3,
  "node_type": "group",
  "attributes": {
    "ome": {
      "version": "0.5",
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
        ]
      }
    }
  }
}
"#;
        let group_metadata: serde_json::Map<String, serde_json::Value> =
            serde_json::from_str(json).unwrap();
        let ome_metadata = get_ome_attribute_from_zarr_group_metadata(&group_metadata).unwrap();
        let well = ome_metadata.get("well").unwrap();
        let _well: Well = serde_json::from_value(well.clone()).unwrap();
    }
}
