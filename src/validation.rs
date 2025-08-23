use std::borrow::Cow;

use validator::{Validate, ValidationError};

use crate::MaybeNDim;

pub(crate) type ValidationResult<T = ()> = Result<T, ValidationError>;

pub(crate) const DIMENSIONALITY_CONFLICT: &str = "dimensionality_conflict";
pub(crate) const UNIT_CONFLICT: &str = "unit_conflict";
pub(crate) const DISALLOWED_TRANSFORM: &str = "disallowed_transform";
pub(crate) const REPEATED_LABEL: &str = "repeated_label";
pub(crate) const DUPLICATE_AXES: &str = "duplicate_axes";

pub(crate) fn validate_ndims<'a, T: MaybeNDim + 'a, S: Into<Cow<'static, str>>>(
    dimensionals: impl IntoIterator<Item = &'a T> + 'a,
    msg: S,
) -> ValidationResult {
    let mut it = dimensionals.into_iter().filter_map(|d| d.maybe_ndim());
    let Some(first) = it.next() else {
        return Ok(());
    };
    for other in it {
        if first != other {
            return new_validation_err(DIMENSIONALITY_CONFLICT, msg);
        }
    }
    Ok(())
}

/// Shortcut for validation error creation.
pub(crate) fn new_validation_err<S: Into<Cow<'static, str>>>(
    code: &'static str,
    msg: S,
) -> ValidationResult {
    Err(ValidationError::new(code).with_message(msg.into()))
}

/// Trait for validating metadata, so that we don't need to expose crate dependencies.
pub trait OmeValidate: Validate + Sized {
    /// Validate this metadata.
    fn ome_validate(&self) -> crate::Result<()> {
        self.validate().map_err(Into::into)
    }
}

impl<T: Validate + Sized> OmeValidate for T {}
