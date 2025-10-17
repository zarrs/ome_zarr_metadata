mod common;
use common::{test_case, test_example};
use ome_zarr_metadata::{
    v0_4,
    v0_5::{self, OmeZarrGroupAttributes, OmeZarrGroupMetadata},
};
use rstest::rstest;

use crate::common::test_upgrade;

#[rstest]
fn v05_test_case(
    #[files("**/*.json")]
    #[base_dir = "tests/fixtures/generated/0.5"]
    #[mode = bytes]
    bytes: &[u8],
) {
    test_case::<OmeZarrGroupAttributes>(bytes);
}

#[rstest]
fn v05_examples(
    #[files("**/*.json")]
    #[base_dir = "ome-zarr/0.5/examples"]
    #[mode = bytes]
    bytes: &[u8],
) {
    test_example::<OmeZarrGroupMetadata>(bytes);
}

#[rstest]
fn v04_to_v05(
    #[files("**/*.json")]
    #[base_dir = "ome-zarr/0.4/examples"]
    #[mode = bytes]
    bytes: &[u8],
) {
    test_upgrade::<v0_4::OmeNgffGroupAttributes, v0_5::OmeFields>(bytes);
}
