#![allow(unexpected_cfgs)]
//! # rslog
//!
//! A lightweight logging library for Rust built entirely using the standard library with zero external dependencies.
//!
//! ## Features
//!
//! - **Zero Dependencies**: Pure standard library implementation
//! - **Multiple Log Levels**: Trace, Debug, Info, Warn, Error, Critical
//! - **Multiple Output Targets**: Console and file output simultaneously
//! - **Async Writing**: Background thread for non-blocking file writes
//! - **Log Rotation**: Size-based and time-based rotation support with optional gzip compression
//! - **Log Filtering**: Per-module log level overrides (with `set_module_level`)
//! - **Formatting Macros**: `trace!`, `debug!`, `info!`, `warn!`, `error!`, `critical!` with `format!`-style syntax
//! - **Multiple Output Formats**: Text, JSON, and custom patterns
//! - **Configurable**: Flexible configuration options via `ConfigBuilder`
//! 
//! ## Quick Start
//! 
//! ```rust
//! use rslog::{Logger, LogLevel};
//! 
//! // Get the logger instance
//! let logger = Logger::get_instance();
//! 
//! // Log messages
//! logger.info("Application started");
//! logger.debug("Debug message");
//! logger.warn("Warning message");
//! logger.error("Error message");
//! logger.critical("Critical error");
//! ```
//! 
//! ## Custom Configuration
//! 
//! ```rust
//! use rslog::{Logger, LogLevel, ConfigBuilder, OutputFormat};
//! 
//! let config = ConfigBuilder::new()
//!     .log_dir("logs")
//!     .file_prefix("app_")
//!     .level(LogLevel::Info)
//!     .output_format(OutputFormat::Json)
//!     .build();
//! 
//! Logger::init_with_config(config);
//! let logger = Logger::get_instance();
//! logger.info("Hello, world!");
//! ```
//!
//! ## Per-Module Log Filtering
//!
//! ```rust
//! use rslog::{Logger, ConfigBuilder, LogLevel, LogFilter};
//!
//! let mut filter = LogFilter::new();
//! filter.set_module_level("network", LogLevel::Error);
//! filter.set_module_level("database", LogLevel::Trace);
//!
//! let config = ConfigBuilder::new()
//!     .level(LogLevel::Warn)
//!     .filter(filter)
//!     .build();
//!
//! Logger::init_with_config(config);
//! let logger = Logger::get_instance();
//!
//! // Or set module levels at runtime:
//! // logger.set_module_level("http", LogLevel::Debug);
//! ```
//!
//! ## Log Rotation & Compression
//!
//! ```rust
//! use rslog::{Logger, ConfigBuilder, RotationStrategy, RotatorConfig};
//!
//! let config = ConfigBuilder::new()
//!     .rotation(RotatorConfig {
//!         strategy: RotationStrategy::SizeBased(10 * 1024 * 1024), // 10MB
//!         max_files: 5,
//!         compress_old_files: true, // gzip-compress rotated files
//!     })
//!     .build();
//!
//! Logger::init_with_config(config);
//! ```

/// Compile-time maximum log level (resolved from crate features).
///
/// Used by macros to decide at compile time whether to emit or strip a log call.
///
/// | Value | Meaning |
/// |-------|---------|
/// | 0     | TRACE and above enabled (default) |
/// | 1     | DEBUG and above enabled |
/// | 2     | INFO and above enabled |
/// | 3     | WARN and above enabled |
/// | 4     | ERROR and above enabled |
/// | 5     | CRITICAL only |
/// | 6     | All macros stripped (`max_level_off`) |
pub const MAX_LEVEL: u8 = if cfg!(feature = "max_level_off") {
    6
} else if cfg!(feature = "max_level_critical") {
    5
} else if cfg!(feature = "max_level_error") {
    4
} else if cfg!(feature = "max_level_warn") {
    3
} else if cfg!(feature = "max_level_info") {
    2
} else if cfg!(feature = "max_level_debug") {
    1
} else {
    0
};

pub mod level;
pub mod formatter;
pub mod writer;
pub mod logger;
pub mod config;
pub mod rotator;
pub mod color;
pub mod filter;
pub mod macros;

pub use level::LogLevel;
pub use logger::Logger;
pub use config::Config;
pub use config::ConfigBuilder;
pub use formatter::OutputFormat;
pub use rotator::{RotationStrategy, RotatorConfig};
pub use writer::LogEntry;
pub use color::{Color, LogColorScheme, ColorFormatter};
pub use filter::LogFilter;
