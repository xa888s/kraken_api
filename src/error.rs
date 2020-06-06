use std::error::Error;
use std::{fmt, fmt::Debug, fmt::Display};

pub struct KrakenError {
    errors: Vec<String>,
}

impl Display for KrakenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Errors: {}", self.errors.join("\n"))
    }
}

impl Debug for KrakenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.errors.join("\n"))
    }
}

impl Error for KrakenError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

impl From<Vec<String>> for KrakenError {
    fn from(errors: Vec<String>) -> Self {
        KrakenError { errors }
    }
}
