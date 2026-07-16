use std::collections::BTreeSet;

use crate::NDim;

use serde::{Deserialize, Serialize};

/// Map axis transform
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MapAxis {
    /// The value at position i in the array indicates which input axis becomes the i-th output axis.
    pub map_axis: Vec<usize>,
}

impl NDim for MapAxis {
    fn ndim(&self) -> usize {
        self.map_axis.len()
    }
}

impl validatrix::Validate for MapAxis {
    fn validate_inner(&self, accum: &mut validatrix::Accumulator) {
        let mut values = BTreeSet::default();
        let mut expected_vals: BTreeSet<_> = (0..self.map_axis.len()).collect();
        accum.with_key("mapAxis", |accum2| {
            for (idx, val) in self.map_axis.iter().enumerate() {
                if !values.insert(val) {
                    accum2.add_failure_at(idx, format!("repeated input axis {val}"));
                } else if !expected_vals.remove(val) {
                    accum2.add_failure_at(idx, format!("input axis {val} does not exist"));
                }
            }
            if !expected_vals.is_empty() {
                accum2.add_failure(format!("did not map {} axes", expected_vals.len()));
            }
        });
    }
}

impl super::TransformationType for MapAxis {
    fn invertible(&self) -> Option<bool> {
        Some(true)
    }

    fn input_ndim(&self) -> Option<usize> {
        Some(self.ndim())
    }

    fn output_ndim(&self) -> Option<usize> {
        Some(self.ndim())
    }
}

impl From<MapAxis> for super::CoordinateTransformInner {
    fn from(value: MapAxis) -> Self {
        Self::MapAxis(value)
    }
}
