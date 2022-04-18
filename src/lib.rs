#![deny(missing_docs)]

//! Logger implementation for Cloudflare Workers.
//! Bridges the [`log`](https://crates.io/crates/log) ecosystem to Cloudflare Worker.
//!
//! # Example
//!
//! Initialize the logger with a string:
//!
//! ```rust
//! worker_logger::init_with_string("info");
//! ```
//!
//! Or initialize with a level struct:
//!
//! ```rust
//! use log::Level;
//! worker_logger::init_with_level(&Level::Debug);
//! ```
//!
//! Or with a Cloudflare Worker environment variable:
//!
//! ```rust,ignore
//! worker_logger::init_with_env(env, "LOG")?;
//! ```
//!
//! # Features
//!
//!  - `env_logger_string`: Enables advanced logging filters. Uses the same syntax as
//!    [`env_logger`](https://crates.io/crates/env_logger). For more details, please visit
//!    <https://docs.rs/env_logger/latest/env_logger/#enabling-logging>.

use log::{Level, Metadata, Record, debug, set_max_level};
use worker::{Env as WorkerEnv, console_log, console_debug, console_error, console_warn, Date, Error as WorkerError};
#[cfg(feature = "env_logger_string")]
use env_logger::filter::{Builder, Filter};

#[cfg(feature = "env_logger_string")]
use log::set_boxed_logger;

#[cfg(not(feature = "env_logger_string"))]
use log::set_logger;

#[cfg(not(feature = "env_logger_string"))]
use std::str::FromStr;

#[cfg(not(feature = "env_logger_string"))]
static WORKER_LOGGER: Logger = Logger {};

/// Main logger struct
#[derive(Debug)]
pub struct Logger {
    #[cfg(feature = "env_logger_string")]
    filter: Filter,
}

impl Logger {
    /// Initialize the logger with a string
    pub fn new<S: AsRef<str>>(init_string: S) -> Self {
        #[cfg(not(feature = "env_logger_string"))]
        {
            let level = Level::from_str(init_string.as_ref());
            if let Err(ref e) = level {
                console_debug!("Failed to parse log level string, fallback to info: {}", e);
            }
            set_max_level(level.unwrap_or(Level::Info).to_level_filter());
        }
        Logger {
            #[cfg(feature = "env_logger_string")]
            filter: Builder::new().parse(init_string.as_ref()).build(),
        }
    }

    #[cfg(feature = "env_logger_string")]
    /// Set the logger instance as the main logger
    pub fn set_logger(self) {
        set_max_level(self.filter.filter());
        let result = set_boxed_logger(Box::new(self));
        if let Err(e) = result {
            debug!("Logger installation failed: {}", e);
        }
        #[cfg(not(feature = "env_logger_string"))]
        {
            let result = set_logger(&WORKER_LOGGER);
            if let Err(e) = result {
                debug!("Logger installation failed: {}", e);
            }
        }
    }

    #[cfg(not(feature = "env_logger_string"))]
    /// Set the logger instance as the main logger
    pub fn set_logger(self) {
        let result = set_logger(&WORKER_LOGGER);
        if let Err(e) = result {
            debug!("Logger installation failed: {}", e);
        }
    }
}

impl log::Log for Logger {
    #[cfg(feature = "env_logger_string")]
    fn enabled(&self, metadata: &Metadata) -> bool {
        self.filter.enabled(metadata)
    }

    #[cfg(not(feature = "env_logger_string"))]
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= log::max_level()
    }

    fn log(&self, record: &Record) {
        #[cfg(feature = "env_logger_string")]
        if !self.filter.matches(record) {
            return;
        }
        #[cfg(not(feature = "env_logger_string"))]
        if !self.enabled(record.metadata()) {
            return;
        }
        let target = if record.file().is_some() && record.line().is_some() {
            format!("{file}:{line}", file=record.file().unwrap(), line=record.line().unwrap())
        } else {
            record.target().to_string()
        };
        let text = format!(
            "[{time} {level} {target}] {text}",
            time = Date::now().to_string(),
            level = record.level(),
            target = target,
            text = record.args()
        );
        match record.level() {
            Level::Debug => console_debug!("{}", text),
            Level::Error => console_error!("{}", text),
            Level::Warn => console_warn!("{}", text),
            _ => console_log!("{}", text),
        }
    }

    fn flush(&self) {}
}

/// Initialize and install a logger with a string
pub fn init_with_string<S: AsRef<str>>(init_string: S) {
    Logger::new(init_string).set_logger();
}

/// Initialize and install a logger with a `log::Level`
pub fn init_with_level(level: &Level) {
    Logger::new(level.as_str()).set_logger();
}

/// Initialize and install a logger with a Cloudflare Workers environment variable
pub fn init_with_env<S: AsRef<str>>(env: &WorkerEnv, env_name: S) -> Result<(), WorkerError> {
    Logger::new(env.var(env_name.as_ref())?.to_string()).set_logger();
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
