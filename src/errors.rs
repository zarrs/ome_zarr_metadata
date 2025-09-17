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
    /// General error.
    #[error("{0}")]
    General(String),
}
