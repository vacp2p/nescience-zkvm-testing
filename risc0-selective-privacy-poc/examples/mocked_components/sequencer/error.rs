#[derive(Debug)]
pub enum Error {
    NotFound,
    BadInput,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::NotFound => write!(f, "Not found"),
            Error::BadInput => write!(f, "Bad input"),
        }
    }
}

impl std::error::Error for Error {}
