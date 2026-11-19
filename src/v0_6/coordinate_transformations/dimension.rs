//! Dimension-manipulating transformations: mapAxis, projectAxis, and byDimension.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};
use validatrix::{Accumulator, Validate};

use super::{Transform, TransformType};

fn unique_indices(
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

/// `byDimension` transformation.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ByDimension {
    /// Transformations applied to subsets of axes.
    pub transformations: Vec<ByDimensionItem>,
}

impl Validate for ByDimension {
    fn validate_inner(&self, accum: &mut Accumulator) {
        accum.with_key("transformations", |a| {
            if self.transformations.is_empty() {
                a.add_failure("empty");
            }
            a.validate_iter(&self.transformations);
            let mut outputs = BTreeSet::default();
            for (idx, t) in self.transformations.iter().enumerate() {
                for o in t.output_axes.iter() {
                    if !outputs.insert(*o) {
                        a.with_keys(&[idx.into(), "outputAxes".into()], |a2| {
                            a2.add_failure(format!("output axis {o} already produced"))
                        });
                    }
                }
            }
            if let Some(max) = outputs.last() {
                if *max + 1 != outputs.len() {
                    a.add_failure("output axes are not contiguous from 0");
                }
            }
        });
    }
}

impl TransformType for ByDimension {
    fn input_ndim(&self) -> Option<usize> {
        self.transformations
            .iter()
            .flat_map(|s| s.input_axes.iter())
            .max()
            .map(|m| m + 1)
    }

    fn output_ndim(&self) -> Option<usize> {
        self.transformations
            .iter()
            .flat_map(|s| s.output_axes.iter())
            .max()
            .map(|m| m + 1)
    }
}

/// Item of a [`ByDimension`] transformation.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ByDimensionItem {
    /// Indices of the input axes this transformation consumes.
    pub input_axes: Vec<usize>,
    /// Indices of the output axes this transformation produces.
    pub output_axes: Vec<usize>,
    /// Transformation to apply.
    pub transformation: Transform,
}

impl Validate for ByDimensionItem {
    fn validate_inner(&self, accum: &mut Accumulator) {
        accum.validate_member_at("transformation", &self.transformation);
        unique_indices(accum, "inputAxes", &self.input_axes);
        unique_indices(accum, "outputAxes", &self.output_axes);
        if let Some(n) = self.transformation.input_ndim() {
            if n != self.input_axes.len() {
                accum.add_failure_at(
                    "inputAxes",
                    format!("length does not match transformation input dimensionality {n}"),
                );
            }
        }
        if let Some(n) = self.transformation.output_ndim() {
            if n != self.output_axes.len() {
                accum.add_failure_at(
                    "outputAxes",
                    format!("length does not match transformation output dimensionality {n}"),
                );
            }
        }
    }
}
