use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};

/// Custom error type for ManifoldBuilder.
#[derive(Debug)]
pub enum ManifoldError {
    Field1Missing,
    Field2Missing,
    // Add more fields as needed
}

impl Display for ManifoldError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            ManifoldError::Field1Missing => write!(f, "field1 is required"),
            ManifoldError::Field2Missing => write!(f, "field2 is required"),
            // Add more fields as needed
        }
    }
}

impl Error for ManifoldError {}
