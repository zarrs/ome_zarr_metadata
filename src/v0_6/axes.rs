//! "axes" metadata.
//!
//! <https://ngff.openmicroscopy.org/0.6/#axes-md>.

use serde::{Deserialize, Serialize};

/// `axis` element metadata. Represents a dimension (axis) of a physical coordinate space.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct Axis {
    /// The name for this dimension.
    pub name: String,
    /// The optional type of this dimension.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<AxisType>,
    /// The optional physical unit of this dimension.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<AxisUnit>,
    /// The anatomical orientation of the dimension.
    #[serde(
        skip_serializing_if = "Option::is_none",
        rename = "anatomicalOrientation"
    )]
    pub anatomical_orientation: Option<AnatomicalOrientation>,
}

pub use crate::v0_4::AxisType;
pub use crate::v0_4::AxisUnit;
pub use crate::v0_4::AxisUnitSpace;
pub use crate::v0_4::AxisUnitTime;

/// `anatomicalOrientation` metadata. Represents an anotomical orientation.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "kebab-case")]
#[allow(missing_docs)]
pub enum AnatomicalOrientation {
    LeftToRight,
    RightToLeft,
    AnteriorToPosterior,
    PosteriorToAnterior,
    InferiorToSuperior,
    SuperiorToInferior,
    DorsalToVentral,
    VentralToDorsal,
    DorsalToPalmar,
    PalmarToDorsal,
    DorsalToPlantar,
    PlantarToDorsal,
    RostralToCaudal,
    CaudalToRostral,
    CranialToCaudal,
    CaudalToCranial,
    ProximalToDistal,
    DistalToProximal,
}
