pub(crate) mod axes;
pub(crate) mod bioformats2raw_layout;
pub(crate) mod coordinate_transformations;
pub(crate) mod labels;
pub(crate) mod multiscales;
pub(crate) mod omero;
pub(crate) mod plate;
pub(crate) mod well;

pub use axes::*;
pub use bioformats2raw_layout::*;
pub use coordinate_transformations::*;
pub use labels::*;
pub use multiscales::*;
pub use omero::*;
pub use plate::*;
use serde::{Deserialize, Serialize};
use validatrix::{Accumulator, Validate};
pub use well::*;

/// OME-NGFF top-level group attributes.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct OmeNgffGroupAttributes {
    /// Transitional `bioformats2raw.layout` metadata.
    #[serde(
        flatten,
        skip_serializing_if = "Option::is_none",
        rename = "bioformats2raw.layout"
    )]
    pub bioformats2raw: Option<Bioformats2Raw>,
    /// Multiscales image metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiscales: Option<Vec<MultiscaleImage>>,
    /// Labels metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Labels>,
    /// Image label metadata.
    #[serde(skip_serializing_if = "Option::is_none", rename = "image-label")]
    pub image_label: Option<ImageLabel>,
    /// Plate metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plate: Option<Plate>,
    /// Well metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub well: Option<Well>,
    /// Transitional OMERO metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub omero: Option<Omero>,
}

impl Validate for OmeNgffGroupAttributes {
    fn validate_inner(&self, accum: &mut Accumulator) {
        if let Some(m) = self.multiscales.as_ref() {
            accum.with_key("multiscales", |a| {
                if m.is_empty() {
                    a.add_failure("empty multiscales");
                }
                a.validate_iter(m);
            });
        }

        if let Some(i) = self.image_label.as_ref() {
            accum.validate_member_at("imageLabel", i);
        }

        if let Some(p) = self.plate.as_ref() {
            accum.validate_member_at("plate", p);
        }

        if let Some(o) = self.omero.as_ref() {
            accum.validate_member_at("omero", o);
        }
    }
}
