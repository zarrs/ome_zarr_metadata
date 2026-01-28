use thiserror::Error;

/// Result type specific to errors from this crate.
pub type Result<T, E = Error> = std::result::Result<T, E>;

/// Error type wrapping over errors produced in this crate.
#[derive(Error, Debug)]
pub enum Error {
    /// Errors from serialising/deserialising JSON.
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
    /// Data fails validation.
    #[error(transparent)]
    Validation(#[from] validatrix::Error),
    /// Invalid OMERO color.
    #[error("invalid hex RGB color")]
    InvalidColor,
    /// Version string could not be parsed.
    #[error(transparent)]
    VersionParse(#[from] pep440_rs::VersionParseError),
    /// Version does not satisfy constraint.
    #[error("version {version} does not satisfy constraint {constraint}")]
    VersionConstraint {
        /// Version constraint string.
        constraint: &'static str,
        /// The version which failed to satisfy the constraint.
        version: pep440_rs::Version,
    },
    /// General error.
    #[error("{0}")]
    General(String),
}

impl Error {
    /// Create a general error from a string message.
    pub fn general<S: Into<String>>(msg: S) -> Self {
        Error::General(msg.into())
    }
}
