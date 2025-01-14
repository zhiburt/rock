use std::fmt;
use std::fmt::{Debug, Formatter};

#[derive(Debug)]
pub enum RockError {
    ProfileUncompressFailed {
        reason: String,
    },
    DecodeFieldFailed {
        reason: String,
    },
    ValidationFailed {
        reason: String,
    },
    #[allow(dead_code)]
    Unknown {
        reason: String,
    },
}

impl fmt::Display for RockError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            RockError::ValidationFailed { reason } => {
                write!(f, "Profile validation failed, reason: {}", reason)
            }
            RockError::Unknown { reason } => write!(f, "Unknown error, reason: {}", reason),
            RockError::ProfileUncompressFailed { reason } => {
                write!(f, "Failed to read compressed data. Error: {}", reason)
            }
            _ => panic!("Unknown type of error"),
        }
    }
}
