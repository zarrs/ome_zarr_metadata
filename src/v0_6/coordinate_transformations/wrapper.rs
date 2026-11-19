//! Wrapper transformations, which contain other transformations: sequence, byDimension, and bijection.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};
use validatrix::{Accumulator, Validate};

use super::dimension::unique_indices;
use super::{Transform, TransformType};

/// `sequence` transformation.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Sequence {
    /// Transformations to apply in order.
    pub transformations: Vec<Transform>,
}

impl Validate for Sequence {
    fn validate_inner(&self, accum: &mut Accumulator) {
        accum.with_key("transformations", |a| {
            if self.transformations.is_empty() {
                a.add_failure("empty");
            }
            a.validate_iter(&self.transformations);
            for (idx, pair) in self.transformations.windows(2).enumerate() {
                if let (Some(o), Some(i)) = (pair[0].output_ndim(), pair[1].input_ndim()) {
                    if o != i {
                        a.add_failure_at(
                            idx + 1,
                            format!("input dimensionality {i} does not match previous output {o}"),
                        );
                    }
                }
            }
        });
    }
}

impl TransformType for Sequence {
    fn input_ndim(&self) -> Option<usize> {
        self.transformations.first().and_then(|t| t.input_ndim())
    }

    fn output_ndim(&self) -> Option<usize> {
        self.transformations.last().and_then(|t| t.output_ndim())
    }
}

/// `bijection` transformation.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Bijection {
    /// Forward transformation.
    pub forward: Box<Transform>,
    /// Inverse transformation.
    pub inverse: Box<Transform>,
}

fn opts_compatible(a: Option<usize>, b: Option<usize>) -> bool {
    match (a, b) {
        (Some(x), Some(y)) => x == y,
        _ => true,
    }
}

impl Validate for Bijection {
    fn validate_inner(&self, accum: &mut Accumulator) {
        accum.validate_member_at("forward", self.forward.as_ref());
        accum.validate_member_at("inverse", self.inverse.as_ref());
        if !opts_compatible(self.forward.output_ndim(), self.inverse.input_ndim()) {
            accum.add_failure_at(
                "forward",
                format!(
                    "output dimensionality {} does not match inverse input dimensionality {}",
                    self.forward.output_ndim().unwrap_or_default(),
                    self.inverse.input_ndim().unwrap_or_default()
                ),
            );
        }
        if !opts_compatible(self.inverse.output_ndim(), self.forward.input_ndim()) {
            accum.add_failure_at(
                "inverse",
                format!(
                    "output dimensionality {} does not match forward input dimensionality {}",
                    self.inverse.output_ndim().unwrap_or_default(),
                    self.forward.input_ndim().unwrap_or_default()
                ),
            );
        }
    }
}

impl TransformType for Bijection {
    fn input_ndim(&self) -> Option<usize> {
        self.forward.input_ndim().or(self.inverse.output_ndim())
    }

    fn output_ndim(&self) -> Option<usize> {
        self.forward.output_ndim().or(self.inverse.input_ndim())
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
