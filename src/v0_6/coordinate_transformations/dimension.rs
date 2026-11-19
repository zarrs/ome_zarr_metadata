//! Dimension-manipulating transformations: mapAxis and projectAxis.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};
use validatrix::{Accumulator, Validate};

use super::TransformType;

pub(super) fn unique_indices(
    accum: &mut Accumulator,
    key: &'static str,
    indices: &[usize],
) -> BTreeSet<usize> {
    let mut seen = BTreeSet::default();
    for (idx, i) in indices.iter().enumerate() {
        if !seen.insert(*i) {
            accum.with_keys(&[key.into(), idx.into()], |a| {
                a.add_failure(format!("duplicate index {i}"))
            });
        }
    }
    seen
}

/// `mapAxis` transformation: an axis permutation.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MapAxis {
    /// The value at position `i` is the index of the input axis which becomes the `i`th output axis.
    pub map_axis: Vec<usize>,
}

impl Validate for MapAxis {
    fn validate_inner(&self, accum: &mut Accumulator) {
        let n = self.map_axis.len();
        if n == 0 {
            accum.add_failure_at("mapAxis", "empty");
        }
        unique_indices(accum, "mapAxis", &self.map_axis);
        for (idx, i) in self.map_axis.iter().enumerate() {
            if *i >= n {
                accum.with_keys(&["mapAxis".into(), idx.into()], |a| {
                    a.add_failure(format!("index {i} out of range for {n} axes"))
                });
            }
        }
    }
}

impl TransformType for MapAxis {
    fn input_ndim(&self) -> Option<usize> {
        Some(self.map_axis.len())
    }
}

/// `projectAxis` transformation: add and/or drop dimensions.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct ProjectAxis {
    /// Indices of input dimensions to drop.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dropped_inputs: Vec<usize>,
    /// Indices of output dimensions which are newly created (with value 0).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub created_outputs: Vec<usize>,
}

impl Validate for ProjectAxis {
    fn validate_inner(&self, accum: &mut Accumulator) {
        if self.dropped_inputs.is_empty() && self.created_outputs.is_empty() {
            accum.add_failure("neither droppedInputs nor createdOutputs given");
        }
        unique_indices(accum, "droppedInputs", &self.dropped_inputs);
        unique_indices(accum, "createdOutputs", &self.created_outputs);
    }
}

impl TransformType for ProjectAxis {
    fn input_ndim(&self) -> Option<usize> {
        None
    }

    fn output_ndim(&self) -> Option<usize> {
        None
    }
}
