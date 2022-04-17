#![deny(missing_docs)]

//! Logger implementation for Cloudflare Workers.
//! Bridges the [`log`](https://crates.io/crates/log) ecosystem to Cloudflare Worker.
//!
//! # Example
//!
//! Initialize the logger with a string. This crate uses the same filter syntax as
//! [`env_logger`](https://crates.io/crates/env_logger):
//!
//! ```rust
//! worker_logger::init_with_string("info");
//! ```
//!
//! For more details, please visit <https://docs.rs/env_logger/latest/env_logger/#enabling-logging>
//!
//! Or initialize with a set level:
//!
//! ```rust
//! use log::Level;
//! worker_logger::init_with_level(&Level::Debug);
//! ```
//!
//! Or with a Cloudflare Worker environment variable:
//!
//! ```rust,ignore
//! worker_logger::init_with_env(env, "LOG");
//! ```

pub mod error;

use log::{Level, Metadata, Record, set_max_level, set_boxed_logger};
use worker::{Env as WorkerEnv, console_log};
use env_logger::filter::{Builder, Filter};
use humantime::Timestamp;

use std::time::SystemTime;

pub use error::WorkerLoggerError;

/// Main logger struct
#[derive(Debug)]
pub struct Logger {
    filter: Filter,
}

impl Logger {
    /// Initialize the logger with a string
    pub fn new<S: AsRef<str>>(init_string: S) -> Self {
        let mut builder = Builder::new();
        builder.parse(init_string.as_ref());
        Logger{
            filter: builder.build(),
        }
    }

    /// Set the logger instance as the main logger
    pub fn set_logger(self) -> Result<(), WorkerLoggerError> {
        set_max_level(self.filter.filter());
        Ok(set_boxed_logger(Box::new(self))?)
    }
}

impl log::Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        self.filter.enabled(metadata)
    }

    fn log(&self, record: &Record) {
        if !self.filter.matches(record) {
            return;
        }
        let target = if record.file().is_some() && record.line().is_some() {
            format!("{file}:{line}", file=record.file().unwrap(), line=record.line().unwrap())
        } else {
            record.target().to_string()
        };
        console_log!(
            "[{time} {level} {target}] {text}",
            time = Timestamp::from(SystemTime::now()),
            level = record.level(),
            target = target,
            text = record.args()
        );
    }

    fn flush(&self) {}
}

/// Initialize and install a logger with a string
pub fn init_with_string<S: AsRef<str>>(init_string: S) -> Result<(), WorkerLoggerError> {
    Logger::new(init_string).set_logger()
}

/// Initialize and install a logger with a `log::Level`
pub fn init_with_level(level: &Level) -> Result<(), WorkerLoggerError> {
    Logger::new(level.as_str()).set_logger()
}

/// Initialize and install a logger with a Cloudflare Workers environment variable
pub fn init_with_env<S: AsRef<str>>(env: &WorkerEnv, env_name: S) -> Result<(), WorkerLoggerError> {
    Logger::new(env.var(env_name.as_ref())?.to_string()).set_logger()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
