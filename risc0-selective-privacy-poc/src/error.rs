#[derive(Debug)]
pub enum Error {
    /// For simplicity, this POC uses a generic error
    Generic,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Generic => write!(f, "An unexpected error occurred"),
        }
    }
}

impl std::error::Error for Error {}
