
use std::fmt;

/// Custom error type for ManifoldBuilder.
#[derive(Debug)]
pub enum ManifoldError {
    Field1Missing,
    Field2Missing,
    // Add more fields as needed
}

impl fmt::Display for ManifoldError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ManifoldError::Field1Missing => write!(f, "field1 is required"),
            ManifoldError::Field2Missing => write!(f, "field2 is required"),
            // Add more fields as needed
        }
    }
}

impl std::error::Error for ManifoldError {}