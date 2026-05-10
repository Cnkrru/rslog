//! Logging macros
//!
//! Provides `format!`-style convenience macros for structured log messages,
//! avoiding the need to write `&format!("...", args)` manually.
//!
//! Supports compile-time log level filtering via Cargo features:
//!
//! | Feature | Strips |
//! |---------|--------|
//! | `max_level_debug` | `trace!` |
//! | `max_level_info` | `trace!`, `debug!` |
//! | `max_level_warn` | `trace!`, `debug!`, `info!` |
//! | `max_level_error` | `trace!`, `debug!`, `info!`, `warn!` |
//! | `max_level_critical` | `trace!`, `debug!`, `info!`, `warn!`, `error!` |
//! | `max_level_off` | all macros |
//!
//! When a macro is stripped, its arguments are **not evaluated** at all —
//! no `format!()` call, no allocation.
//!
//! # Examples
//!
//! ```rust
//! use rslog::{Logger, info, warn};
//!
//! let logger = Logger::get_instance();
//! info!("User {} logged in from {}", "admin", "192.168.1.1");
//! warn!("Disk usage at {:.1}%", 85.3);
//! ```

// ---- compile-time level gate helpers ----
//
// These use $crate::MAX_LEVEL (a const resolved from the library's Cargo features)
// rather than cfg!(feature = ...) to ensure the correct feature set is used
// regardless of the calling crate's feature configuration.

/// Internal: true when trace level is enabled at compile time
#[doc(hidden)]
#[macro_export]
macro_rules! _log_enabled_trace {
    () => { $crate::MAX_LEVEL <= 0 };
}

/// Internal: true when debug level is enabled at compile time
#[doc(hidden)]
#[macro_export]
macro_rules! _log_enabled_debug {
    () => { $crate::MAX_LEVEL <= 1 };
}

/// Internal: true when info level is enabled at compile time
#[doc(hidden)]
#[macro_export]
macro_rules! _log_enabled_info {
    () => { $crate::MAX_LEVEL <= 2 };
}

/// Internal: true when warn level is enabled at compile time
#[doc(hidden)]
#[macro_export]
macro_rules! _log_enabled_warn {
    () => { $crate::MAX_LEVEL <= 3 };
}

/// Internal: true when error level is enabled at compile time
#[doc(hidden)]
#[macro_export]
macro_rules! _log_enabled_error {
    () => { $crate::MAX_LEVEL <= 4 };
}

/// Internal: true when critical level is enabled at compile time
#[doc(hidden)]
#[macro_export]
macro_rules! _log_enabled_critical {
    () => { $crate::MAX_LEVEL <= 5 };
}

/// Log a trace message with format string and automatic target (module path).
///
/// Stripped at compile time when `max_level_debug` or higher is enabled.
///
/// # Examples
///
/// ```rust
/// use rslog::trace;
/// trace!("Entering function with args: {}", 42);
/// ```
#[macro_export]
macro_rules! trace {
    ($($arg:tt)*) => {{
        if $crate::_log_enabled_trace!() {
            let logger = $crate::Logger::get_instance();
            logger.trace_target(module_path!(), &format!($($arg)*));
        }
    }};
}

/// Log a debug message with format string and automatic target (module path).
///
/// Stripped at compile time when `max_level_info` or higher is enabled.
///
/// # Examples
///
/// ```rust
/// use rslog::debug;
/// debug!("Value: {}", 42);
/// ```
#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {{
        if $crate::_log_enabled_debug!() {
            let logger = $crate::Logger::get_instance();
            logger.debug_target(module_path!(), &format!($($arg)*));
        }
    }};
}

/// Log an info message with format string and automatic target (module path).
///
/// Stripped at compile time when `max_level_warn` or higher is enabled.
///
/// # Examples
///
/// ```rust
/// use rslog::info;
/// info!("Application started on port {}", 8080);
/// ```
#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {{
        if $crate::_log_enabled_info!() {
            let logger = $crate::Logger::get_instance();
            logger.info_target(module_path!(), &format!($($arg)*));
        }
    }};
}

/// Log a warning message with format string and automatic target (module path).
///
/// Stripped at compile time when `max_level_error` or higher is enabled.
///
/// # Examples
///
/// ```rust
/// use rslog::warn;
/// warn!("Connection timeout after {}s", 30);
/// ```
#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {{
        if $crate::_log_enabled_warn!() {
            let logger = $crate::Logger::get_instance();
            logger.warn_target(module_path!(), &format!($($arg)*));
        }
    }};
}

/// Log an error message with format string and automatic target (module path).
///
/// Stripped at compile time when `max_level_critical` is enabled.
///
/// # Examples
///
/// ```rust
/// use rslog::error;
/// error!("Failed to open file: {}", "not found");
/// ```
#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {{
        if $crate::_log_enabled_error!() {
            let logger = $crate::Logger::get_instance();
            logger.error_target(module_path!(), &format!($($arg)*));
        }
    }};
}

/// Log a critical message with format string and automatic target (module path).
///
/// Stripped at compile time when `max_level_off` is enabled.
///
/// # Examples
///
/// ```rust
/// use rslog::critical;
/// critical!("System out of memory: {} bytes available", 0);
/// ```
#[macro_export]
macro_rules! critical {
    ($($arg:tt)*) => {{
        if $crate::_log_enabled_critical!() {
            let logger = $crate::Logger::get_instance();
            logger.critical_target(module_path!(), &format!($($arg)*));
        }
    }};
}