
#[derive(Clone, PartialEq, Eq)]
pub enum Error {
    PlatformError(String),
    // AppError,
    Unknown(String)
}
impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::PlatformError(s) => write!(f, "PlatformError({})", s),
            // Error::AppError => write!(f, "AppError"),
            Error::Unknown(s) => write!(f, "Unknown({})", s),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::PlatformError(s) => write!(f, "Platform error: {}", s),
            // Error::AppError => write!(f, "App error"),
            Error::Unknown(s) => write!(f, "Unknown error: {}", s),
        }
    }
}

impl From<windows::core::Error> for Error {
    fn from(e: windows::core::Error) -> Self {
        Error::PlatformError(format!("{}", e))
    }
}
