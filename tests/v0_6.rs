mod common;
use common::test_oztest_case;
use ome_zarr_metadata::{Valid, v0_5, v0_6};
use rstest::rstest;

#[ignore = "some examples are not valid standalone metadata"]
#[rstest]
fn v06_examples(
    #[files("**/*.json")]
    #[base_dir = "ome-zarr/specifications/0.6/examples"]
    #[exclude("ome/series-2")]
    #[mode = bytes]
    bytes: &[u8],
) {
    common::test_example::<v0_6::OmeZarrGroupMetadata>(bytes);
}

#[rstest]
#[test_log::test]
fn v06_oztest_valid(
    #[files("**/*.json")]
    #[base_dir = "oztest/cases/parse_attributes/v0.6"]
    #[exclude("(^strict)|(/invalid/)")]
    #[mode = bytes]
    bytes: &[u8],
) {
    test_oztest_case::<v0_6::OmeZarrGroupAttributes>(bytes, false);
}

#[rstest]
#[test_log::test]
fn v06_oztest_invalid(
    #[files("**/*.json")]
    #[base_dir = "oztest/cases/parse_attributes/v0.6"]
    #[exclude("(^strict)|(/valid/)")]
    #[exclude("scene/missing_scene")]
    #[mode = bytes]
    bytes: &[u8],
) {
    test_oztest_case::<v0_6::OmeZarrGroupAttributes>(bytes, true);
}

#[rstest]
fn v05_to_v06(
    #[files("**/*.json")]
    #[base_dir = "ome-zarr/specifications/0.5/examples"]
    #[exclude("ome/series-2")]
    // image-label but no multiscales
    #[exclude("label_strict/colors_properties")]
    #[mode = bytes]
    bytes: &[u8],
) {
    let s = json_comments::StripComments::new(bytes);
    let src: Valid<v0_5::OmeZarrGroupMetadata> = serde_json::from_reader(s).unwrap();
    let tgt = v0_6::OmeZarrGroupAttributes::try_from(src.into_inner().attributes).unwrap();
    Valid::try_new(tgt).unwrap();
}
