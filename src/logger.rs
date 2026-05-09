//! Logger module
//! 
//! Provides the core logger implementation with singleton pattern and logging methods.

use std::sync::{Arc, Mutex, OnceLock};

use crate::config::Config;
use crate::level::LogLevel;
use crate::writer::Writer;

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
            })
        }).clone()
    }

    /// Initialize logger with custom configuration
    /// 
    /// Must be called before the first call to [`Logger::get_instance()`].
    /// 
    /// # Parameters
    /// 
    /// * `config` - Logger configuration
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
    /// Logger::init_with_config(config);
    /// let logger = Logger::get_instance();
    /// ```
    pub fn init_with_config(config: Config) {
        LOGGER.get_or_init(|| {
            let writer = Arc::new(Writer::new(config.clone()));
            Arc::new(Logger {
                level: Mutex::new(config.level),
                writer,
            })
        });
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

    /// Log a debug message
    /// 
    /// # Parameters
    /// 
    /// * `message` - Log message
    pub fn debug(&self, message: &str) {
        self.log(LogLevel::Debug, message);
    }

    /// Log an info message
    /// 
    /// # Parameters
    /// 
    /// * `message` - Log message
    pub fn info(&self, message: &str) {
        self.log(LogLevel::Info, message);
    }

    /// Log a warning message
    /// 
    /// # Parameters
    /// 
    /// * `message` - Log message
    pub fn warn(&self, message: &str) {
        self.log(LogLevel::Warn, message);
    }

    /// Log an error message
    /// 
    /// # Parameters
    /// 
    /// * `message` - Log message
    pub fn error(&self, message: &str) {
        self.log(LogLevel::Error, message);
    }

    /// Log a critical message
    /// 
    /// # Parameters
    /// 
    /// * `message` - Log message
    pub fn critical(&self, message: &str) {
        self.log(LogLevel::Critical, message);
    }

    /// General logging method
    /// 
    /// Checks log level and logs if appropriate.
    fn log(&self, level: LogLevel, message: &str) {
        if self.get_level().should_log(level) {
            self.writer.write(level, message);
        }
    }
}
