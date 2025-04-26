
#[derive(Clone, PartialEq, Eq)]
pub enum Error {
    // PlatformError,
    // AppError,
    Unknown(String)
}
impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            // Error::PlatformError => write!(f, "PlatformError"),
            // Error::AppError => write!(f, "AppError"),
            Error::Unknown(s) => write!(f, "Unknown({})", s),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            // Error::PlatformError => write!(f, "Platform error"),
            // Error::AppError => write!(f, "App error"),
            Error::Unknown(s) => write!(f, "Unknown error: {}", s),
        }
    }
}
