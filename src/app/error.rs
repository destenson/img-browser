use windows::Win32::Foundation::ERROR_INVALID_WINDOW_HANDLE;

#[derive(Clone, PartialEq, Eq)]
pub enum Error {
    PlatformError(String),
    WindowError(String),
    ImageError(String),
    ResourceError(String),
    // AppError,
    Unknown(String)
}
impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::PlatformError(s) => write!(f, "PlatformError({})", s),
            Error::WindowError(s) => write!(f, "WindowError({})", s),
            Error::ImageError(s) => write!(f, "ImageError({})", s),
            Error::ResourceError(s) => write!(f, "ResourceError({})", s),
            // Error::AppError => write!(f, "AppError"),
            Error::Unknown(s) => write!(f, "Unknown({})", s),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::PlatformError(s) => write!(f, "Platform error: {}", s),
            Error::WindowError(s) => write!(f, "Window error: {}", s),
            Error::ImageError(s) => write!(f, "Image error: {}", s),
            Error::ResourceError(s) => write!(f, "Resource error: {}", s),
            // Error::AppError => write!(f, "App error"),
            Error::Unknown(s) => write!(f, "Unknown error: {}", s),
        }
    }
}

impl From<windows::core::Error> for Error {
    fn from(e: windows::core::Error) -> Self {
        // Determine if this is a window-specific error
        if e.code().0 == ERROR_INVALID_WINDOW_HANDLE.0 as i32 { // INVALID_WINDOW_HANDLE
            Error::WindowError(format!("Invalid window handle. ({})", e))
        } else {
            Error::PlatformError(format!("{}", e))
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::ResourceError(format!("{}", e))
    }
}

impl From<image::ImageError> for Error {
    fn from(e: image::ImageError) -> Self {
        Error::ImageError(format!("{}", e))
    }
}
