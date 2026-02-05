mod common;
use common::{test_case, test_example};
use ome_zarr_metadata::v0_4::OmeNgffGroupAttributes;
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
    test_case::<OmeNgffGroupAttributes>(bytes);
}

#[rstest]
fn v04_examples(
    #[files("**/*.json")]
    #[base_dir = "ome-zarr/0.4/examples"]
    #[exclude("ome/series-2")]
    #[mode = bytes]
    bytes: &[u8],
) {
    test_example::<OmeNgffGroupAttributes>(bytes);
}
