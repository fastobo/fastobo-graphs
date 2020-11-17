//! Standard error implementation for this crate.

/// The error type for this crate.
#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    YamlError(#[from] serde_yaml::Error),
    #[error(transparent)]
    JsonError(#[from] serde_json::Error),
    #[error(transparent)]
    IOError(#[from] std::io::Error),
    #[error(transparent)]
    OboSyntaxError(#[from] fastobo::error::SyntaxError),
    #[error("{0}: {1:?}")]
    InvalidBoolean(#[source] std::str::ParseBoolError, String),
    #[error("invalid synonym type: {0:?}")]
    InvalidSynonymType(String),
    #[error("invalid term clause: {0:?}")]
    InvalidTermClause(String),
    #[error("invalid instance clause: {0:?}")]
    InvalidInstanceClause(String),
}

/// The result type for this crate.
pub type Result<T> = std::result::Result<T, Error>;

impl Error {
    /// Create a new `Error::InvalidInstanceClause` error variant.
    pub fn invalid_instance_clause<S: Into<String>>(clause: S) -> Self {
        Error::InvalidInstanceClause(clause.into())
    }

    /// Create a new `Error::InvalidTermClause` error variant.
    pub fn invalid_term_clause<S: Into<String>>(clause: S) -> Self {
        Error::InvalidTermClause(clause.into())
    }
}
