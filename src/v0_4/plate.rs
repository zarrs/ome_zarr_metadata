//! "plate" metadata
//!
//! <https://ngff.openmicroscopy.org/0.4/#plate-md>.

use std::{num::NonZeroU64, path::PathBuf};

use serde::{Deserialize, Serialize};

/// `plate` metadata. For high-content screening datasets.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct Plate {
    /// The version of the multiscale metadata of the image.
    pub version: monostate::MustBe!("0.4"),
    /// A list of JSON objects defining the acquisitions for a given plate to which wells can refer to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acquisitions: Option<Vec<PlateAcquisition>>,
    /// A list of JSON objects defining the columns of the plate
    pub columns: Vec<PlateColumn>,
    /// The field count defining the maximum number of fields per view across all wells (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_count: Option<NonZeroU64>,
    /// The name of the plate (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Defines the rows of the plate.
    pub rows: Vec<PlateRow>,
    /// Defines the wells of the plate.
    pub wells: Vec<PlateWell>,
}

/// [`Plate`] `acquisitions` element metadata. Defines a plate acquisition.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct PlateAcquisition {
    /// A unique integer identifier that fields of view can refer to.
    pub id: u64,
    /// A string identifying the name of the acquisition (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The maximum number of fields of view for the acquisition (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximumfieldcount: Option<NonZeroU64>,
    /// A string specifying a description for the acquisition (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// An epoch timestamp specifying the start timestamp of the acquisition (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starttime: Option<u64>,
    /// An epoch timestamp specifying the end timestamp of the acquisition (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endtime: Option<u64>,
}

/// [`Plate`] `columns` element metadata. Defines a plate column.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct PlateColumn {
    /// Specifies the unique column mame.
    pub name: String,
}

/// [`Plate`] `rows` element metadata. Defines a plate row.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct PlateRow {
    /// Specifies the unique row mame.
    pub name: String,
}

/// [`Plate`] `wells` element metadata. Defines a plate well.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct PlateWell {
    /// A string specifying the path to the well subgroup.
    pub path: PathBuf,
    /// Specifies the row index.
    #[serde(rename = "rowIndex")]
    pub row_index: u64,
    /// Specifies the column index.
    #[serde(rename = "columnIndex")]
    pub column_index: u64,
}

#[cfg(test)]
mod tests {
    use crate::v0_4::OmeNgffGroupAttributes;

    use super::*;

    #[test]
    fn plate_2wells() {
        let json = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/ome-zarr/0.4/examples/plate_strict/plate_2wells.json"
        ));
        let ome_metadata: OmeNgffGroupAttributes = serde_json::from_str(json).unwrap();
        let _plate: Plate = ome_metadata.plate.unwrap();
    }

    #[test]
    fn plate_6wells() {
        let json = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/ome-zarr/0.4/examples/plate_strict/plate_6wells.json"
        ));
        let ome_metadata: OmeNgffGroupAttributes = serde_json::from_str(json).unwrap();
        let _plate: Plate = ome_metadata.plate.unwrap();
    }
}
