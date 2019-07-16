#[derive(Debug, Error)]
pub enum Error {
    #[error(display = "{}", 0)]
    YamlError(#[error(cause)] serde_yaml::Error),
    #[error(display = "{}", 0)]
    IOError(#[error(cause)] std::io::Error),
    #[error(display = "{}", 0)]
    OboSyntaxError(#[error(cause)] fastobo::error::SyntaxError),
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
