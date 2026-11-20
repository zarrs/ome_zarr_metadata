mod common;
use common::{test_case, test_example, test_oztest_case};
use ome_zarr_metadata::v0_4;
use rstest::rstest;

#[ignore = "focusing on v0.5"]
#[rstest]
fn v04_test_case(
    #[files("**/*.json")]
    #[base_dir = "tests/fixtures/generated/0.4"]
    // excluded for incorrect test data; see https://github.com/ome/ngff/issues/325
    #[exclude("strict_no_acquisitions")]
    #[exclude("strict_acquisitions")]
    #[exclude("minimal_acquisitions")]
    #[exclude("minimal_no_acquisitions")]
    #[exclude("mismatch_axes_units")]
    #[exclude("non_alphanumeric_row")]
    #[exclude("image-label/no_colors")]
    #[mode = bytes]
    bytes: &[u8],
) {
    test_case::<v0_4::OmeNgffGroupAttributes>(bytes);
}

#[ignore = "focusing on v0.5+"]
#[rstest]
fn v04_examples(
    #[files("**/*.json")]
    #[base_dir = "ome-zarr/specifications/0.4/examples"]
    #[exclude("ome/series-2")]
    #[mode = bytes]
    bytes: &[u8],
) {
    test_example::<v0_4::OmeNgffGroupAttributes>(bytes);
}

#[ignore = "focusing on v0.5+"]
#[rstest]
#[test_log::test]
fn v04_oztest_valid(
    #[files("**/*.json")]
    #[base_dir = "oztest/cases/parse_attributes/v0.4"]
    #[exclude("(^strict)|(/invalid/)")]
    #[mode = bytes]
    bytes: &[u8],
) {
    test_oztest_case::<v0_4::OmeNgffGroupAttributes>(bytes, false);
}

#[ignore = "focusing on v0.5+"]
#[rstest]
#[test_log::test]
fn v04_oztest_invalid(
    #[files("**/*.json")]
    #[base_dir = "oztest/cases/parse_attributes/v0.4"]
    #[exclude("(^strict)|(/valid/)")]
    #[mode = bytes]
    bytes: &[u8],
) {
    test_oztest_case::<v0_4::OmeNgffGroupAttributes>(bytes, true);
}
