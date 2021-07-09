use thiserror::Error;

use crate::AttributeError;

/// Represents an error that occurred when using a fallible method.
#[derive(Debug, Error)]
#[error(transparent)]
pub struct Error {
    source: Box<InnerError>,
}

impl From<InnerError> for Error {
    fn from(inner: InnerError) -> Self {
        Self {
            source: Box::new(inner),
        }
    }
}

impl From<AttributeError> for Error {
    fn from(inner: AttributeError) -> Self {
        Self {
            source: Box::new(inner.into()),
        }
    }
}

#[derive(Debug, Error)]
pub(crate) enum InnerError {
    #[error(transparent)]
    AttributeError {
        #[from]
        source: AttributeError,
    },
}
