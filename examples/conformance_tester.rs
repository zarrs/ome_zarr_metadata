//! CLI for testing with <https://github.com/clbarnes/ome_zarr_conformance>
use serde::Serialize;
use std::{env, error::Error};

use ome_zarr_metadata::{v0_4, v0_5, Valid};

#[derive(Debug, Serialize)]
struct Output {
    valid: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<String>,
}

impl<T, E: Error> From<Result<Valid<T>, E>> for Output {
    fn from(value: Result<Valid<T>, E>) -> Self {
        match value {
            Ok(_) => Self {
                valid: true,
                message: None,
            },
            Err(e) => Self {
                valid: false,
                message: Some(e.to_string()),
            },
        }
    }
}

fn input_output(version: &str, s: &str) -> Output {
    match version {
        "0.5" => serde_json::from_str::<Valid<v0_5::OmeZarrGroupAttributes>>(s).into(),
        "0.4" => serde_json::from_str::<Valid<v0_4::OmeNgffGroupAttributes>>(s).into(),
        s => panic!("Unsupported OME-Zarr version {s}"),
    }
}

fn main() {
    let args: Vec<_> = env::args().collect();
    let res = input_output(&args[1], &args[2]);
    println!("{}", serde_json::to_string(&res).unwrap());
}
