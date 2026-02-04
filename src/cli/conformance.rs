//! CLI for validating OME-Zarr metadata, compatible with ome_zarr_conformance.py [1].
//!!
//! 1: https://github.com/ome/ngff-spec/blob/main/conformance/ome_zarr_conformance.py
#![cfg(feature = "cli")]

use clap::Parser;
#[cfg(feature = "next")]
use ome_zarr_metadata::next;
use ome_zarr_metadata::{v0_4, v0_5, AnyOmeFields, Valid};
use serde::{de::DeserializeOwned, Serialize};
use std::{error::Error, str::FromStr};
use validatrix::Validate;

#[derive(Debug, Clone)]
enum VersionSelection {
    V0_4,
    V0_5,
    #[cfg(feature = "next")]
    VNext,
}

impl FromStr for VersionSelection {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0.4" => Ok(VersionSelection::V0_4),
            "0.5" => Ok(VersionSelection::V0_5),
            #[cfg(feature = "next")]
            "next" => Ok(VersionSelection::VNext),
            _ => Err(format!("Unsupported version selection: {}", s)),
        }
    }
}

/// Command-line arguments for the conformance CLI
#[derive(Debug, Parser)]
#[command(author, version, about = "CLI for validating OME-Zarr metadata, compatible with ome_zarr_conformance.py.", long_about = None)]
struct CliArgs {
    /// Optional OME-Zarr version constraint as a PEP440 specifier (e.g., '>=0.4,<0.5')
    #[arg(short = 'o', long = "ome-zarr-version")]
    ome_zarr_version: Option<VersionSelection>,

    /// JSON string representing the `attributes` field of a Zarr metadata document; for OME-Zar v0.5 and later, this should include the "ome" key
    #[arg(value_name = "JSON")]
    json: String,
}

#[derive(Debug, Serialize)]
struct Output {
    valid: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<String>,
}

impl Output {
    fn print(self) {
        println!("{}", serde_json::to_string(&self).unwrap());
    }
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

fn parse_type<O: DeserializeOwned + Validate>(s: &str) -> Output {
    match serde_json::from_str::<Valid<O>>(s) {
        Ok(_) => Output {
            valid: true,
            message: None,
        },
        Err(e) => Output {
            valid: false,
            message: Some(e.to_string()),
        },
    }
}

fn get_output(parsed: &CliArgs) -> Output {
    let s = parsed.json.as_str();

    match parsed.ome_zarr_version {
        Some(VersionSelection::V0_4) => parse_type::<v0_4::OmeNgffGroupAttributes>(s),
        Some(VersionSelection::V0_5) => parse_type::<v0_5::OmeZarrGroupAttributes>(s),
        #[cfg(feature = "next")]
        Some(VersionSelection::VNext) => parse_type::<next::OmeZarrGroupAttributes>(s),
        None => parse_type::<AnyOmeFields>(s),
    }
}

fn main() {
    let parsed = CliArgs::parse();

    let output = get_output(&parsed);
    output.print();
}
