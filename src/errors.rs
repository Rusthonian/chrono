use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;

#[derive(Debug)]
pub enum ChronoError {
    ParseError(String),
    InvalidFormat(String),
    OutOfRange(String),
    InvalidTimezone(String),
}

impl From<ChronoError> for PyErr {
    fn from(err: ChronoError) -> PyErr {
        match err {
            ChronoError::ParseError(msg) => PyValueError::new_err(format!("Parse error: {}", msg)),
            ChronoError::InvalidFormat(msg) => PyValueError::new_err(format!("Invalid format: {}", msg)),
            ChronoError::OutOfRange(msg) => PyValueError::new_err(format!("Out of range: {}", msg)),
            ChronoError::InvalidTimezone(msg) => PyValueError::new_err(format!("Invalid timezone: {}", msg)),
        }
    }
}

impl From<chrono::ParseError> for ChronoError {
    fn from(err: chrono::ParseError) -> Self {
        ChronoError::ParseError(err.to_string())
    }
}
