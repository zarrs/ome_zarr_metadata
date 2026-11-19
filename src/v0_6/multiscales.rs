//! "multiscales" metadata.
//!
//! <https://ngff.openmicroscopy.org/specifications/0.6/#multiscale-md>.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};
use validatrix::{Accumulator, Validate};

use crate::{Error, MaybeNDim, NDim};

pub use crate::v0_5::MultiscaleImageMetadata;

use super::coordinate_systems::{find_system, validate_coordinate_systems};
use super::coordinate_transformations::{convert_prev_transforms, require_named_io};
use super::{CoordinateSystem, CoordinateSystemRef, CoordinateTransform, Transform};

/// `multiscales` element metadata. Describes a multiscale image.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MultiscaleImage {
    /// The name of the multiscale image (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Coordinate systems for this multiscale image.
    ///
    /// One of these is the "intrinsic" coordinate system: the output of every dataset's transformation.
    pub coordinate_systems: Vec<CoordinateSystem>,
    /// The arrays storing the individual resolution levels.
    pub datasets: Vec<MultiscaleImageDataset>,
    /// Additional transformations between the intrinsic coordinate system and others (optional).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub coordinate_transformations: Vec<CoordinateTransform>,
    /// The type of downscaling method used to generate the multiscale image pyramid (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// Additional information about the downscaling method (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<MultiscaleImageMetadata>,
}

impl MultiscaleImage {
    /// Get a coordinate system by name.
    pub fn get_coordinate_system(&self, name: &str) -> Option<&CoordinateSystem> {
        find_system(&self.coordinate_systems, name)
    }

    /// The intrinsic coordinate system, i.e. the output of the first dataset's transformation.
    pub fn intrinsic_coordinate_system(&self) -> Option<&CoordinateSystem> {
        self.coordinate_systems.first()
    }
}

impl MaybeNDim for MultiscaleImage {
    fn maybe_ndim(&self) -> Option<usize> {
        self.intrinsic_coordinate_system().map(|cs| cs.ndim())
    }
}

fn check_ndim(
    accum: &mut Accumulator,
    key: &'static str,
    got: Option<usize>,
    expected: Option<usize>,
) {
    if let (Some(g), Some(e)) = (got, expected) {
        if g != e {
            accum.add_failure_at(
                key,
                format!("dimensionality {g} does not match coordinate system dimensionality {e}"),
            );
        }
    }
}

impl Validate for MultiscaleImage {
    fn validate_inner(&self, accum: &mut Accumulator) {
        accum.with_key("coordinateSystems", |a| {
            if self.coordinate_systems.is_empty() {
                a.add_failure("empty");
            }
            validate_coordinate_systems(a, &self.coordinate_systems);
        });

        let intrinsic = self
            .intrinsic_coordinate_system()
            .map(|cs| cs.name.as_str());
        let intrinsic_ndim = self.maybe_ndim();

        accum.with_key("datasets", |a| {
            if self.datasets.is_empty() {
                a.add_failure("empty");
            }
            a.validate_iter(&self.datasets);
            let mut paths = BTreeSet::default();
            for (idx, ds) in self.datasets.iter().enumerate() {
                if !paths.insert(ds.path.as_str()) {
                    a.add_failure_at(idx, format!("duplicate path '{}'", ds.path));
                }
                a.with_keys(&[idx.into(), "coordinateTransformations".into()], |a2| {
                    let Some(ct) = ds.transform() else {
                        return;
                    };
                    let Some(out) = ds.output_name() else {
                        return;
                    };
                    if Some(out) != intrinsic {
                        a2.add_failure_at(
                            0,
                            "output coordinate system differs from other datasets",
                        );
                    } else if self.get_coordinate_system(out).is_none() {
                        a2.add_failure_at(0, format!("unknown coordinate system '{out}'"));
                    }
                    a2.with_key(0, |a3| {
                        check_ndim(a3, "input", ct.input_ndim(), intrinsic_ndim);
                        check_ndim(a3, "output", ct.output_ndim(), intrinsic_ndim);
                    });
                });
            }
        });

        let cts = &self.coordinate_transformations;
        accum.with_key("coordinateTransformations", |a| {
            a.validate_iter(cts);
            for (idx, ct) in cts.iter().enumerate() {
                a.with_key(idx, |a2| {
                    self.validate_additional_transform(a2, ct, intrinsic)
                });
            }
        });
    }
}

impl MultiscaleImage {
    fn validate_additional_transform(
        &self,
        accum: &mut Accumulator,
        ct: &CoordinateTransform,
        intrinsic: Option<&str>,
    ) {
        require_named_io(accum, ct);
        let (input, output) = (&ct.input, &ct.output);
        let is_intrinsic =
            |r: &CoordinateSystemRef| r.path.is_none() && r.name.as_deref() == intrinsic;

        let other = if is_intrinsic(input) {
            ("output", output)
        } else if is_intrinsic(output) {
            ("input", input)
        } else {
            accum.add_failure("neither input nor output is the intrinsic coordinate system");
            return;
        };

        match (other.1.path.as_ref(), other.1.name.as_deref()) {
            // coordinate system in a child labels group
            (Some(_), _) => {
                if !matches!(
                    ct.transform,
                    Transform::Identity(_) | Transform::Scale(_) | Transform::Translation(_)
                ) {
                    accum.add_failure_at(
                        "type",
                        "transformations to labels must be identity, scale, or translation",
                    );
                }
            }
            (None, Some(name)) => match self.get_coordinate_system(name) {
                None => accum.with_keys(&[other.0.into(), "name".into()], |a| {
                    a.add_failure(format!("unknown coordinate system '{name}'"))
                }),
                Some(cs) => {
                    let got = if other.0 == "input" {
                        ct.input_ndim()
                    } else {
                        ct.output_ndim()
                    };
                    check_ndim(accum, other.0, got, Some(cs.ndim()));
                }
            },
            (None, None) => (), // already reported
        }

        let intrinsic_ndim = self.maybe_ndim();
        if other.0 == "output" {
            check_ndim(accum, "input", ct.input_ndim(), intrinsic_ndim);
        } else {
            check_ndim(accum, "output", ct.output_ndim(), intrinsic_ndim);
        }
    }
}

/// [`MultiscaleImage`] `datasets` element metadata. Describes an individual resolution level.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MultiscaleImageDataset {
    /// The path to the array for this resolution relative to the current zarr group.
    pub path: String,
    /// A single transformation from this array's coordinates to the intrinsic coordinate system.
    pub coordinate_transformations: Vec<CoordinateTransform>,
}

impl MultiscaleImageDataset {
    /// The transformation from array coordinates to the intrinsic coordinate system.
    pub fn transform(&self) -> Option<&CoordinateTransform> {
        self.coordinate_transformations.first()
    }

    /// Name of the output (intrinsic) coordinate system.
    pub fn output_name(&self) -> Option<&str> {
        self.transform().and_then(|t| t.output.name.as_deref())
    }
}

impl MaybeNDim for MultiscaleImageDataset {
    fn maybe_ndim(&self) -> Option<usize> {
        self.transform().and_then(|t| t.input_ndim())
    }
}

impl Validate for MultiscaleImageDataset {
    fn validate_inner(&self, accum: &mut Accumulator) {
        if self.path.is_empty() {
            accum.add_failure_at("path", "empty");
        }
        accum.with_key("coordinateTransformations", |a| {
            if self.coordinate_transformations.len() != 1 {
                a.add_failure(format!(
                    "expected exactly 1 transformation, got {}",
                    self.coordinate_transformations.len()
                ));
            }
            a.validate_iter(&self.coordinate_transformations);
            let Some(ct) = self.transform() else {
                return;
            };
            a.with_key(0, |a2| {
                match ct.input.path.as_deref() {
                    None => a2.with_keys(&["input".into(), "path".into()], |a3| {
                        a3.add_failure("missing")
                    }),
                    Some(p) if p != self.path => a2
                        .with_keys(&["input".into(), "path".into()], |a3| {
                            a3.add_failure("does not match dataset path")
                        }),
                    _ => (),
                }
                if ct.output.name.is_none() {
                    a2.with_keys(&["output".into(), "name".into()], |a3| {
                        a3.add_failure("missing")
                    });
                }
                if !is_valid_dataset_transform(&ct.transform) {
                    a2.add_failure(
                        "must be identity, scale, or a sequence of a scale then a translation",
                    );
                }
            });
        });
    }
}

fn is_valid_dataset_transform(t: &Transform) -> bool {
    match t {
        Transform::Identity(_) | Transform::Scale(_) => true,
        Transform::Sequence(s) => matches!(
            s.transformations.as_slice(),
            [Transform::Scale(_), Transform::Translation(_)]
        ),
        _ => false,
    }
}

/// Name given to the intrinsic coordinate system when upgrading from v0.5.
pub const UPGRADE_INTRINSIC_NAME: &str = "intrinsic";
/// Name given to the additional coordinate system when upgrading from v0.5
/// metadata with multiscale-level coordinate transformations.
pub const UPGRADE_TRANSFORMED_NAME: &str = "physical";

impl TryFrom<crate::v0_5::MultiscaleImage> for MultiscaleImage {
    type Error = Error;

    fn try_from(value: crate::v0_5::MultiscaleImage) -> Result<Self, Self::Error> {
        let axes: Vec<_> = value.axes.into_iter().map(Into::into).collect();
        let mut coordinate_systems = vec![CoordinateSystem {
            name: UPGRADE_INTRINSIC_NAME.into(),
            axes: axes.clone(),
        }];

        let datasets = value
            .datasets
            .into_iter()
            .map(|ds| {
                let ct = CoordinateTransform::new(
                    CoordinateSystemRef::from_path(ds.path.clone()),
                    CoordinateSystemRef::from_name(UPGRADE_INTRINSIC_NAME),
                    convert_prev_transforms(ds.coordinate_transformations)?,
                );
                Ok(MultiscaleImageDataset {
                    path: ds.path,
                    coordinate_transformations: vec![ct],
                })
            })
            .collect::<Result<Vec<_>, Error>>()?;

        let coordinate_transformations = match value.coordinate_transformations {
            Some(cts) if !cts.is_empty() => {
                coordinate_systems.push(CoordinateSystem {
                    name: UPGRADE_TRANSFORMED_NAME.into(),
                    axes,
                });
                let ct = CoordinateTransform::new(
                    CoordinateSystemRef::from_name(UPGRADE_INTRINSIC_NAME),
                    CoordinateSystemRef::from_name(UPGRADE_TRANSFORMED_NAME),
                    convert_prev_transforms(cts)?,
                );
                vec![ct]
            }
            _ => vec![],
        };

        Ok(Self {
            name: value.name,
            coordinate_systems,
            datasets,
            coordinate_transformations,
            r#type: value.r#type,
            metadata: value.metadata,
        })
    }
}
