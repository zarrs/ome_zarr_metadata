//! CLI for testing with <https://github.com/clbarnes/ome_zarr_conformance>
#![cfg(feature = "cli")]

use clap::Parser;
use ome_zarr_metadata::{pep440_rs::Version, pep440_rs::VersionSpecifier, AnyOmeFields, Valid};
use serde::Serialize;
use std::error::Error;

/// Command-line arguments for the conformance CLI
#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct CliArgs {
    /// OME-Zarr version constraint as a PEP440 specifier (e.g., '>=0.4,<0.5')
    #[arg(short = 'o', long = "ome-zarr-version")]
    ome_zarr_version: Option<VersionSpecifier>,

    /// Attributes JSON
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

fn main() {
    let parsed = CliArgs::parse();

    let fields: AnyOmeFields = match serde_json::from_str(&parsed.json) {
        Ok(f) => f,
        Err(e) => {
            Output {
                valid: false,
                message: Some(e.to_string()),
            }
            .print();
            return;
        }
    };

    if let Some(version_spec) = parsed.ome_zarr_version {
        let version_str = fields.version();
        let version: Version = version_str.parse().unwrap();
        if !version_spec.contains(&version) {
            Output {
                valid: false,
                message: Some(format!(
                    "Version {} does not match constraint {}",
                    version, version_spec
                )),
            }
            .print();
            return;
        }
    }

    Output {
        valid: true,
        message: Some(format!("OME-Zarr v{}", fields.version())),
    }
    .print();
}
