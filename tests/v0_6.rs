mod common;
use ome_zarr_metadata::{Valid, v0_5, v0_6};
use rstest::rstest;

// #[rstest]
// fn v06_examples(
//     #[files("**/*.json")]
//     #[base_dir = "ome-zarr/specifications/0.6/examples"]
//     #[exclude("ome/series-2")]
//     #[mode = bytes]
//     bytes: &[u8],
// ) {
//     common::test_example::<v0_6::OmeZarrGroupMetadata>(bytes);
// }

#[rstest]
fn v05_to_v06(
    #[files("**/*.json")]
    #[base_dir = "ome-zarr/specifications/0.5/examples"]
    #[exclude("ome/series-2")]
    #[mode = bytes]
    bytes: &[u8],
) {
    let s = json_comments::StripComments::new(bytes);
    let src: Valid<v0_5::OmeZarrGroupMetadata> = serde_json::from_reader(s).unwrap();
    let tgt = v0_6::OmeZarrGroupAttributes::try_from(src.into_inner().attributes).unwrap();
    Valid::try_new(tgt).unwrap();
}
