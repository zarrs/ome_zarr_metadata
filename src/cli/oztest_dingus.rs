//! CLI for validating OME-Zarr metadata, compatible with oztest [^oztest].
//!
//! [^oztest]: <https://github.com/German-BioImaging/oztest>
#![cfg(feature = "cli")]
use clap::Parser;
#[cfg(feature = "next")]
use ome_zarr_metadata::next;
use ome_zarr_metadata::{AnyOmeFields, Valid, v0_4, v0_5, v0_6};
use serde::{Serialize, de::DeserializeOwned};
use std::fmt::Debug;
use std::path::PathBuf;
use std::process::ExitCode;
use std::{error::Error, str::FromStr};
use validatrix::Validate;

#[derive(Debug, Clone)]
enum VersionSelection {
    V0_4,
    V0_5,
    V0_6,
    #[cfg(feature = "next")]
    VNext,
}

impl FromStr for VersionSelection {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0.4" => Ok(VersionSelection::V0_4),
            "0.5" => Ok(VersionSelection::V0_5),
            "0.6" => Ok(VersionSelection::V0_6),
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
    #[clap(flatten)]
    verbosity: clap_verbosity_flag::Verbosity,

    /// Path to a JSON file representing the `attributes` field of a Zarr metadata document; for OME-Zar v0.5 and later, this should include the "ome" key
    path: PathBuf,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "lowercase")]
enum Validity {
    Valid,
    Invalid,
}

fn is_false(xfail: &bool) -> bool {
    !*xfail
}

#[derive(Debug, Serialize)]
struct Output {
    validity: Validity,
    #[serde(skip_serializing_if = "is_false")]
    xfail: bool,
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
                validity: Validity::Valid,
                xfail: false,
                message: None,
            },
            Err(e) => Self {
                validity: Validity::Invalid,
                xfail: false,
                message: Some(e.to_string()),
            },
        }
    }
}

fn parse_type<O: DeserializeOwned + Validate + Debug>(s: &str) -> Output {
    log::debug!("Parsing string: {s}");
    let res = serde_json::from_str::<Valid<O>>(s);
    log::info!("Parse result: {res:#?}");
    res.into()
}

fn get_output(parsed: &CliArgs) -> std::io::Result<Output> {
    let s = std::fs::read_to_string(&parsed.path)?;

    let res = match parsed.ome_zarr_version {
        Some(VersionSelection::V0_4) => parse_type::<v0_4::OmeNgffGroupAttributes>(&s),
        Some(VersionSelection::V0_5) => parse_type::<v0_5::OmeZarrGroupAttributes>(&s),
        Some(VersionSelection::V0_6) => parse_type::<v0_6::OmeZarrGroupAttributes>(&s),
        #[cfg(feature = "next")]
        Some(VersionSelection::VNext) => parse_type::<next::OmeZarrGroupAttributes>(&s),
        None => parse_type::<AnyOmeFields>(&s),
    };
    Ok(res)
}

fn main() -> Result<ExitCode, Box<dyn Error>> {
    let parsed = CliArgs::parse();
    env_logger::Builder::new()
        .filter_level(parsed.verbosity.into())
        .init();

    let output = get_output(&parsed)?;
    output.print();
    Ok(ExitCode::default())
}
