//! Standard error implementation for this crate.

/// The error type for this crate.
#[derive(Debug, Error)]
pub enum Error {
    #[error(display = "{}", 0)]
    YamlError(#[error(cause)] serde_yaml::Error),
    #[error(display = "{}", 0)]
    JsonError(#[error(cause)] serde_json::Error),
    #[error(display = "{}", 0)]
    IOError(#[error(cause)] std::io::Error),
    #[error(display = "{}", 0)]
    OboSyntaxError(#[error(cause)] fastobo::error::SyntaxError),
    #[error(display = "{}: {:?}", 0, 1)]
    InvalidBoolean(#[error(cause)] std::str::ParseBoolError, String),
    #[error(display = "invalid synonym type: {:?}", 0)]
    InvalidSynonymType(String),
    #[error(display = "invalid term clause: {:?}", 0)]
    InvalidTermClause(String),
    #[error(display = "invalid instance clause: {:?}", 0)]
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

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::JsonError(err)
    }
}

impl From<serde_yaml::Error> for Error {
    fn from(err: serde_yaml::Error) -> Self {
        Error::YamlError(err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::IOError(err)
    }
}

impl From<fastobo::error::SyntaxError> for Error {
    fn from(err: fastobo::error::SyntaxError) -> Self {
        Error::OboSyntaxError(err)
    }
}
