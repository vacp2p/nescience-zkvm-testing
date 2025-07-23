#[derive(Debug)]
pub enum Error {
    BadInput,
    Risc0(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::BadInput => write!(f, "Bad input"),
            Error::Risc0(_) => write!(f, "Risc0 error"),
        }
    }
}

impl std::error::Error for Error {}
