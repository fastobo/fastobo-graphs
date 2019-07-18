#[derive(Debug, Error)]
pub enum Error {
    #[error(display = "{}", 0)]
    YamlError(#[error(cause)] serde_yaml::Error),
    #[error(display = "{}", 0)]
    IOError(#[error(cause)] std::io::Error),
    #[error(display = "{}", 0)]
    OboSyntaxError(#[error(cause)] fastobo::error::SyntaxError),
    #[error(display = "{}: {:?}", 0, 1)]
    InvalidBoolean(#[error(cause)] std::str::ParseBoolError, String),
    #[error(display = "invalid synonym type: {:?}", 0)]
    InvalidSynonymType(String),
}

pub type Result<T> = std::result::Result<T, Error>;

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
