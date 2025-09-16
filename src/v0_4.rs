pub(crate) mod axes;
pub(crate) mod bioformats2raw_layout;
pub(crate) mod coordinate_transformations;
pub(crate) mod labels;
pub(crate) mod multiscales;
pub(crate) mod plate;
pub(crate) mod well;

pub use axes::*;
pub use bioformats2raw_layout::*;
pub use coordinate_transformations::*;
pub use labels::*;
pub use multiscales::*;
pub use plate::*;
use serde::{Deserialize, Serialize};
use validatrix::{Validate, Accumulator};
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
}

impl Validate for OmeNgffGroupAttributes {
    fn validate_inner(&self, accum: &mut Accumulator) -> usize {
        let mut total = 0;
        if let Some(m) = self.multiscales.as_ref() {
            accum.prefix.push("multiscales".into());
            if m.is_empty() {
                accum.add_failure("empty multiscales".into(), &[]);
            }
            total += accum.validate_iter(m);
            accum.prefix.pop();
        }

        if let Some(i) = self.image_label.as_ref() {
            accum.prefix.push("imageLabel".into());
            total += i.validate_inner(accum);
            accum.prefix.pop();
        }
        
        total
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::*;

    const VERSION: (u64, u64) = (0, 4);

    #[test]
    fn parse_examples() {
        run_examples_for_version::<OmeNgffGroupAttributes>(VERSION);
    }

    #[ignore]
    #[test]
    fn test_suite() {
        run_test_suites_for_version::<OmeNgffGroupAttributes>(VERSION);
    }
}
