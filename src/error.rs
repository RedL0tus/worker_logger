#![deny(missing_docs)]

//! Error conversion for the crate
//!
//! Because there are only two possible types of errors, I'm not bothered to use fancy tools here.

use log::SetLoggerError;
use worker::Error as WorkerError;

use std::fmt;
use std::error::Error;

/// Error type of the crate
#[derive(Debug)]
pub enum WorkerLoggerError {
    /// Error from the `worker` crate
    WorkerError(WorkerError),
    /// Error from the `log` crate
    LoggerError(SetLoggerError),
}

impl fmt::Display for WorkerLoggerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Self::WorkerError(e) => format!("WorkerError: {}", e),
            Self::LoggerError(e) => format!("LoggerError: {}", e),
        })
    }
}

impl Error for WorkerLoggerError {}

impl From<WorkerError> for WorkerLoggerError {
    fn from(e: WorkerError) -> Self {
        Self::WorkerError(e)
    }
}

impl From<SetLoggerError> for WorkerLoggerError {
    fn from(e: SetLoggerError) -> Self {
        Self::LoggerError(e)
    }
}
