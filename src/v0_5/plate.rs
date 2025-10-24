//! "plate" metadata
//!
//! <https://ngff.openmicroscopy.org/0.5/#plate-md>.

use std::{collections::HashSet, num::NonZeroU64};

use serde::{Deserialize, Serialize};
use validatrix::Validate;

use super::{PlateAcquisition, PlateColumn, PlateRow, PlateWell};

/// `plate` metadata. For high-content screening datasets.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct Plate {
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

impl Validate for Plate {
    fn validate_inner(&self, accum: &mut validatrix::Accumulator) {
        if let Some(acqs) = self.acquisitions.as_deref() {
            accum.with_key("acquisitions", |acc| {
                let mut visited = HashSet::with_capacity(acqs.len());
                for (idx, a) in acqs.iter().enumerate() {
                    if !visited.insert(a.id) {
                        acc.with_keys(&[idx.into(), "id".into()], |acc_inner| {
                            acc_inner.add_failure(format!("not unique: {}", a.id));
                        });
                    }
                    acc.validate_member_at(idx, a);
                }
            });
        }

        accum.with_key("columns", |acc| {
            let mut col_names = HashSet::with_capacity(self.columns.len());
            for (idx, col) in self.columns.iter().enumerate() {
                if !col_names.insert(&col.name) {
                    acc.with_keys(&[idx.into(), "name".into()], |acc_inner| {
                        acc_inner.add_failure(format!("not unique: {}", col.name));
                    });
                }
                acc.validate_member_at(idx, col);
            }
        });

        accum.with_key("rows", |acc| {
            let mut row_names = HashSet::with_capacity(self.rows.len());

            for (idx, row) in self.rows.iter().enumerate() {
                if !row_names.insert(&row.name) {
                    acc.with_keys(&[idx.into(), "name".into()], |acc_inner| {
                        acc_inner.add_failure(format!("not unique: {}", row.name));
                    });
                }
                acc.validate_member_at(idx, row);
            }
        });

        accum.with_key("wells", |acc| {
            for (idx, well) in self.wells.iter().enumerate() {
                acc.with_key(idx, |w_acc| {
                    let col_name_opt = self.columns.get(well.column_index).map(|c| c.name.as_str());
                    if col_name_opt.is_none() {
                        w_acc.add_failure_at(
                            "columnIndex",
                            format!("column index {} does not exist", well.column_index),
                        );
                    }
                    let row_name_opt = self.rows.get(well.row_index).map(|r| r.name.as_str());
                    if row_name_opt.is_none() {
                        w_acc.add_failure_at(
                            "rowIndex",
                            format!("row index {} does not exist", well.row_index),
                        );
                    }
                    if let (Some(col_name), Some(row_name)) = (col_name_opt, row_name_opt) {
                        let mut comp = well.path.components();
                        let Some(row_component) = comp.next() else {
                            w_acc.add_failure_at("path", "no row name");
                            return;
                        };

                        if row_component
                            .as_os_str()
                            .to_str()
                            .filter(|r| *r == row_name)
                            .is_none()
                        {
                            w_acc.add_failure_at("path", "row name does not match row index");
                        };

                        let Some(col_component) = comp.next() else {
                            w_acc.add_failure_at("path", "no column name");
                            return;
                        };

                        if col_component
                            .as_os_str()
                            .to_str()
                            .filter(|c| *c == col_name)
                            .is_none()
                        {
                            w_acc.add_failure_at("path", "column name does not match column index");
                        };

                        if comp.next().is_some() {
                            w_acc.add_failure_at("path", "too many path components");
                        }
                    }
                });
            }
        });
    }
}

impl From<crate::v0_4::Plate> for Plate {
    fn from(value: crate::v0_4::Plate) -> Self {
        Self {
            acquisitions: value.acquisitions,
            columns: value.columns,
            field_count: value.field_count,
            name: value.name,
            rows: value.rows,
            wells: value.wells,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::v0_5::OmeZarrGroupMetadata;

    use super::*;

    #[test]
    fn plate_2wells() {
        let json = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/ome-zarr/0.5/examples/plate_strict/plate_2wells.json"
        ));
        let ome_metadata: OmeZarrGroupMetadata = serde_json::from_str(json).unwrap();
        let _plate: Plate = ome_metadata.attributes.ome.plate.unwrap();
    }

    #[test]
    fn plate_6wells() {
        let json = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/ome-zarr/0.5/examples/plate_strict/plate_6wells.json"
        ));
        let ome_metadata: OmeZarrGroupMetadata = serde_json::from_str(json).unwrap();
        let _plate: Plate = ome_metadata.attributes.ome.plate.unwrap();
    }
}
