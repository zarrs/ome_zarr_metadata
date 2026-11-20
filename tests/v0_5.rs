mod common;
use common::{test_case, test_example};
use ome_zarr_metadata::{v0_4, v0_5};
use rstest::rstest;

use crate::common::{test_oztest_case, test_upgrade};

#[rstest]
fn v05_test_case(
    #[files("**/*.json")]
    #[base_dir = "tests/fixtures/generated/0.5"]
    // excluded for incorrect test data; see https://github.com/ome/ngff/issues/325
    #[exclude("strict_no_acquisitions")]
    #[exclude("strict_acquisitions")]
    #[exclude("minimal_acquisitions")]
    #[exclude("minimal_no_acquisitions")]
    #[exclude("mismatch_axes_units")]
    #[exclude("non_alphanumeric_row")]
    #[exclude("image-label/no_colors")]
    // missing multiscales
    #[exclude("image-label/minimal")]
    // missing multiscales
    #[exclude("image-label/minimal_properties")]
    #[mode = bytes]
    bytes: &[u8],
) {
    test_case::<v0_5::OmeZarrGroupAttributes>(bytes);
}

#[ignore = "some examples are not valid standalone metadata"]
#[rstest]
fn v05_examples(
    #[files("**/*.json")]
    #[base_dir = "ome-zarr/specifications/0.5/examples"]
    #[exclude("ome/series-2")]
    #[mode = bytes]
    bytes: &[u8],
) {
    test_example::<v0_5::OmeZarrGroupMetadata>(bytes);
}

#[rstest]
fn v04_to_v05(
    #[files("**/*.json")]
    #[base_dir = "ome-zarr/specifications/0.4/examples"]
    #[exclude("ome/series-2")]
    // missing multiscales
    #[exclude("label_strict/colors_properties")]
    #[mode = bytes]
    bytes: &[u8],
) {
    test_upgrade::<v0_4::OmeNgffGroupAttributes, v0_5::OmeFields>(bytes);
}

#[rstest]
#[test_log::test]
fn v05_oztest_valid(
    #[files("**/*.json")]
    #[base_dir = "oztest/cases/parse_attributes/v0.5"]
    #[exclude("(^strict)|(/invalid/)")]
    #[mode = bytes]
    bytes: &[u8],
) {
    test_oztest_case::<v0_5::OmeZarrGroupAttributes>(bytes, false);
}

#[rstest]
#[test_log::test]
fn v05_oztest_invalid(
    #[files("**/*.json")]
    #[base_dir = "oztest/cases/parse_attributes/v0.5"]
    #[exclude("(^strict)|(/valid/)")]
    #[mode = bytes]
    bytes: &[u8],
) {
    test_oztest_case::<v0_5::OmeZarrGroupAttributes>(bytes, true);
}
