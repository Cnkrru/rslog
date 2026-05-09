//! Logger module
//!
//! Provides the core logger implementation with singleton pattern and logging methods.

use std::sync::{Arc, Mutex, OnceLock};

use crate::config::Config;
use crate::level::LogLevel;
use crate::writer::Writer;
use crate::filter::LogFilter;

/// Logger
///
/// Singleton logger responsible for recording and outputting log messages.
///
/// # Examples
///
/// ```rust
/// use rslog::{Logger, LogLevel};
///
/// let logger = Logger::get_instance();
/// logger.info("Application started");
/// logger.debug("Debug message");
/// logger.warn("Warning message");
/// logger.error("Error message");
/// logger.critical("Critical error");
/// ```
pub struct Logger {
    level: Mutex<LogLevel>,
    writer: Arc<Writer>,
    filter: Mutex<LogFilter>,
}

static LOGGER: OnceLock<Arc<Logger>> = OnceLock::new();

impl Logger {
    /// Get the singleton logger instance
    ///
    /// Uses `OnceLock` to ensure only one initialization occurs, thread-safe.
    ///
    /// # Returns
    ///
    /// The global unique logger instance.
    pub fn get_instance() -> Arc<Self> {
        LOGGER.get_or_init(|| {
            let config = Config::default();
            let writer = Arc::new(Writer::new(config.clone()));
            Arc::new(Logger {
                level: Mutex::new(config.level),
                writer,
                filter: Mutex::new(config.filter),
            })
        }).clone()
    }

    /// Initialize logger with custom configuration
    ///
    /// Must be called before the first call to [`Logger::get_instance()`].
    /// Returns `Err` if the logger was already initialized.
    ///
    /// # Parameters
    ///
    /// * `config` - Logger configuration
    ///
    /// # Returns
    ///
    /// `Ok(())` on success, `Err` if the logger was already initialized.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rslog::{Logger, ConfigBuilder, LogLevel};
    ///
    /// let config = ConfigBuilder::new()
    ///     .level(LogLevel::Info)
    ///     .log_dir("my_logs")
    ///     .build();
    ///
    /// assert!(Logger::init_with_config(config).is_ok());
    /// let logger = Logger::get_instance();
    /// ```
    pub fn init_with_config(config: Config) -> Result<(), &'static str> {
        LOGGER.set({
            let writer = Arc::new(Writer::new(config.clone()));
            Arc::new(Logger {
                level: Mutex::new(config.level),
                writer,
                filter: Mutex::new(config.filter),
            })
        }).map_err(|_| "Logger is already initialized")
    }

    /// Set the log level
    ///
    /// Messages below this level will be filtered out.
    ///
    /// # Parameters
    ///
    /// * `level` - Log level
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rslog::{Logger, LogLevel};
    ///
    /// let logger = Logger::get_instance();
    /// logger.set_level(LogLevel::Warn);
    ///
    /// // These will be logged
    /// logger.warn("Warning");
    /// logger.error("Error");
    ///
    /// // These will be filtered
    /// logger.info("Info");
    /// logger.debug("Debug");
    /// ```
    pub fn set_level(&self, level: LogLevel) {
        *self.level.lock().unwrap() = level;
    }

    /// Get the current log level
    pub fn get_level(&self) -> LogLevel {
        *self.level.lock().unwrap()
    }

    /// Get the log filter for read-only inspection
    pub fn filter(&self) -> std::sync::MutexGuard<'_, LogFilter> {
        self.filter.lock().unwrap()
    }

    /// Set a per-module log level override at runtime
    ///
    /// # Parameters
    ///
    /// * `module` - Module path prefix (e.g., `"network"` or `"myapp::http"`)
    /// * `level` - Log level for this module
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rslog::{Logger, LogLevel};
    ///
    /// let logger = Logger::get_instance();
    /// logger.set_module_level("network", LogLevel::Error);
    /// ```
    pub fn set_module_level(&self, module: &str, level: LogLevel) {
        self.filter.lock().unwrap().set_module_level(module, level);
    }

    /// Remove a per-module log level override
    ///
    /// # Parameters
    ///
    /// * `module` - Module path prefix to remove
    pub fn remove_module_level(&self, module: &str) {
        self.filter.lock().unwrap().remove_module(module);
    }

    /// Set whether console output is enabled
    ///
    /// # Parameters
    ///
    /// * `enabled` - true to enable, false to disable
    pub fn set_console_enabled(&self, enabled: bool) {
        self.writer.set_console_enabled(enabled);
    }

    /// Set whether console colors are enabled
    ///
    /// # Parameters
    ///
    /// * `enabled` - true to enable colors, false to disable
    pub fn set_console_colors(&self, enabled: bool) {
        self.writer.set_console_colors(enabled);
    }

    /// Flush pending log entries to disk
    ///
    /// Forces the writer thread to flush buffered entries to file.
    /// Useful before program exit or when log durability is critical.
    pub fn flush(&self) {
        self.writer.flush();
    }

    /// Log a trace message
    ///
    /// # Parameters
    ///
    /// * `message` - Log message
    pub fn trace(&self, message: &str) {
        self.log(LogLevel::Trace, message);
    }

    /// Log a trace message with target module
    ///
    /// # Parameters
    ///
    /// * `target` - Module path for filter matching
    /// * `message` - Log message
    pub fn trace_target(&self, target: &str, message: &str) {
        self.log_with_target(LogLevel::Trace, target, message);
    }

    /// Log a debug message
    ///
    /// # Parameters
    ///
    /// * `message` - Log message
    pub fn debug(&self, message: &str) {
        self.log(LogLevel::Debug, message);
    }

    /// Log a debug message with target module
    ///
    /// # Parameters
    ///
    /// * `target` - Module path for filter matching
    /// * `message` - Log message
    pub fn debug_target(&self, target: &str, message: &str) {
        self.log_with_target(LogLevel::Debug, target, message);
    }

    /// Log an info message
    ///
    /// # Parameters
    ///
    /// * `message` - Log message
    pub fn info(&self, message: &str) {
        self.log(LogLevel::Info, message);
    }

    /// Log an info message with target module
    ///
    /// # Parameters
    ///
    /// * `target` - Module path for filter matching
    /// * `message` - Log message
    pub fn info_target(&self, target: &str, message: &str) {
        self.log_with_target(LogLevel::Info, target, message);
    }

    /// Log a warning message
    ///
    /// # Parameters
    ///
    /// * `message` - Log message
    pub fn warn(&self, message: &str) {
        self.log(LogLevel::Warn, message);
    }

    /// Log a warning message with target module
    ///
    /// # Parameters
    ///
    /// * `target` - Module path for filter matching
    /// * `message` - Log message
    pub fn warn_target(&self, target: &str, message: &str) {
        self.log_with_target(LogLevel::Warn, target, message);
    }

    /// Log an error message
    ///
    /// # Parameters
    ///
    /// * `message` - Log message
    pub fn error(&self, message: &str) {
        self.log(LogLevel::Error, message);
    }

    /// Log an error message with target module
    ///
    /// # Parameters
    ///
    /// * `target` - Module path for filter matching
    /// * `message` - Log message
    pub fn error_target(&self, target: &str, message: &str) {
        self.log_with_target(LogLevel::Error, target, message);
    }

    /// Log a critical message
    ///
    /// # Parameters
    ///
    /// * `message` - Log message
    pub fn critical(&self, message: &str) {
        self.log(LogLevel::Critical, message);
    }

    /// Log a critical message with target module
    ///
    /// # Parameters
    ///
    /// * `target` - Module path for filter matching
    /// * `message` - Log message
    pub fn critical_target(&self, target: &str, message: &str) {
        self.log_with_target(LogLevel::Critical, target, message);
    }

    /// General logging method (uses global level only)
    fn log(&self, level: LogLevel, message: &str) {
        if self.get_level().should_log(level) {
            self.writer.write(level, message);
        }
    }

    /// General logging method with target/filter support
    fn log_with_target(&self, level: LogLevel, target: &str, message: &str) {
        if !self.filter.lock().unwrap().should_log(target, level) {
            return;
        }
        if self.get_level().should_log(level) {
            self.writer.write(level, message);
        }
    }
}
