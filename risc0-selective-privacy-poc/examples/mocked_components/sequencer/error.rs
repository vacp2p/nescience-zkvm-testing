#[derive(Debug)]
pub enum Error {
    BadInput,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::BadInput => write!(f, "Bad input"),
        }
    }
}

impl std::error::Error for Error {}
