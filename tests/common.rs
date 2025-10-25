use std::io::Read;

use ome_zarr_metadata::{Valid, Validate};
use serde::{de::DeserializeOwned, Deserialize};

#[allow(unused)]
#[derive(Debug, Deserialize)]
struct TestCase {
    schema: serde_json::Value,
    test_case_description: String,
    test: Test,
}

#[derive(Debug, Deserialize)]
struct Test {
    description: String,
    data: serde_json::Value,
    valid: bool,
}

pub fn test_case<T: DeserializeOwned + Validate>(bytes: &[u8]) {
    let case: TestCase = serde_json::from_slice(bytes).unwrap();
    let should_be_valid = case.test.valid;

    match serde_json::from_value::<Valid<T>>(case.test.data) {
        Ok(_) => {
            if !should_be_valid {
                panic!("'{}' unexpectedly valid", case.test.description)
            }
        }
        Err(e) => {
            if should_be_valid {
                panic!("'{}' unexpectedly invalid: {e}", case.test.description);
            }
        }
    };
}

fn strip_comments(jsonc: &[u8]) -> Vec<u8> {
    let mut s = Vec::with_capacity(jsonc.len());
    let mut rd = json_comments::StripComments::new(jsonc);
    rd.read_to_end(&mut s).unwrap();
    s
}

pub fn test_example<T: DeserializeOwned + Validate>(bytes: &[u8]) {
    serde_json::from_slice::<Valid<T>>(&strip_comments(bytes)).unwrap();
}

// not actually unused, but only used in tests
#[allow(unused)]
pub fn test_upgrade<Src: DeserializeOwned + Validate, Tgt: From<Src> + Validate>(bytes: &[u8]) {
    let lower = serde_json::from_slice::<Src>(&strip_comments(bytes)).unwrap();
    lower.validate().expect("source should be valid");
    let upper = Tgt::from(lower);
    upper.validate().unwrap();
}
