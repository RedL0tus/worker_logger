#![deny(missing_docs)]

//! Error conversion for the crate
//!
//! Because there are only two possible types of errors, I'm not bothered to use fancy tools here.

use log::SetLoggerError;
use worker::Error as WorkerError;

use std::fmt;
use std::error::Error;

/// Error type of the crate
#[derive(Clone, Debug)]
pub struct WorkerLoggerError {
    /// Error string
    message: String,
}

impl fmt::Display for WorkerLoggerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for WorkerLoggerError {}

impl From<WorkerError> for WorkerLoggerError {
    fn from(e: WorkerError) -> Self {
        Self {
            message: format!("{}", e),
        }
    }
}

impl From<SetLoggerError> for WorkerLoggerError {
    fn from(e: SetLoggerError) -> Self {
        Self {
            message: format!("{}", e),
        }
    }
}
