//! Configuration module
//! 
//! Provides configuration options and builder pattern for the logging library.

use crate::formatter::OutputFormat;
use crate::level::LogLevel;
use crate::rotator::RotatorConfig;

/// Logging library configuration structure
/// 
/// Allows users to customize logging behavior including log level, output path, console output, etc.
/// 
/// Use [`ConfigBuilder`] for easier configuration building.
/// 
/// # Examples
/// 
/// ```rust
/// use rslog::{Config, LogLevel, OutputFormat};
/// 
/// let config = Config {
///     log_dir: "logs".to_string(),
///     file_prefix: "app_".to_string(),
///     file_extension: ".log".to_string(),
///     console_enabled: true,
///     level: LogLevel::Info,
///     output_format: OutputFormat::Text,
///     rotation: None,
/// };
/// ```
#[derive(Debug, Clone)]
pub struct Config {
    /// Log directory path
    pub log_dir: String,
    /// Log file prefix
    pub file_prefix: String,
    /// Log file extension
    pub file_extension: String,
    /// Whether console output is enabled
    pub console_enabled: bool,
    /// Default log level
    pub level: LogLevel,
    /// Output format
    pub output_format: OutputFormat,
    /// Log rotation configuration (None disables rotation)
    pub rotation: Option<RotatorConfig>,
}

impl Default for Config {
    /// Default configuration
    /// 
    /// ```rust
    /// use rslog::Config;
    /// 
    /// let config = Config::default();
    /// 
    /// assert_eq!(config.log_dir, "logs");
    /// assert_eq!(config.file_prefix, "serial_");
    /// assert_eq!(config.file_extension, ".log");
    /// assert!(config.console_enabled);
    /// ```
    fn default() -> Self {
        Self {
            log_dir: "logs".to_string(),
            file_prefix: "serial_".to_string(),
            file_extension: ".log".to_string(),
            console_enabled: true,
            level: LogLevel::Debug,
            output_format: OutputFormat::Text,
            rotation: Some(RotatorConfig::default()),
        }
    }
}

impl Config {
    /// Create a new configuration builder
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use rslog::Config;
    /// 
    /// let config = Config::builder()
    ///     .log_dir("my_logs")
    ///     .level(rslog::LogLevel::Info)
    ///     .build();
    /// ```
    pub fn builder() -> ConfigBuilder {
        ConfigBuilder::new()
    }

    /// Get the full log file path
    /// 
    /// Format: `{log_dir}/{file_prefix}{date}{file_extension}`
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use rslog::{Config, LogLevel};
    /// 
    /// let config = Config {
    ///     log_dir: "logs".to_string(),
    ///     file_prefix: "app_".to_string(),
    ///     file_extension: ".log".to_string(),
    ///     ..Default::default()
    /// };
    /// 
    /// let path = config.get_log_file_path();
    /// assert!(path.starts_with("logs/app_"));
    /// assert!(path.ends_with(".log"));
    /// ```
    pub fn get_log_file_path(&self) -> String {
        format!(
            "{}/{}{}{}",
            self.log_dir,
            self.file_prefix,
            crate::formatter::Formatter::get_date_string(),
            self.file_extension
        )
    }
}

/// Configuration builder
/// 
/// Provides a fluent interface for building configurations.
/// 
/// # Examples
/// 
/// ```rust
/// use rslog::{ConfigBuilder, LogLevel, OutputFormat};
/// 
/// let config = ConfigBuilder::new()
///     .log_dir("logs")
///     .file_prefix("app_")
///     .level(LogLevel::Info)
///     .output_format(OutputFormat::Json)
///     .build();
/// ```
pub struct ConfigBuilder {
    log_dir: Option<String>,
    file_prefix: Option<String>,
    file_extension: Option<String>,
    console_enabled: Option<bool>,
    level: Option<LogLevel>,
    output_format: Option<OutputFormat>,
    rotation: Option<RotatorConfig>,
}

impl ConfigBuilder {
    /// Create a new configuration builder
    pub fn new() -> Self {
        ConfigBuilder {
            log_dir: None,
            file_prefix: None,
            file_extension: None,
            console_enabled: None,
            level: None,
            output_format: None,
            rotation: None,
        }
    }

    /// Set the log directory
    /// 
    /// # Parameters
    /// 
    /// * `dir` - Log directory path
    pub fn log_dir(mut self, dir: &str) -> Self {
        self.log_dir = Some(dir.to_string());
        self
    }

    /// Set the log file prefix
    /// 
    /// # Parameters
    /// 
    /// * `prefix` - Log file prefix
    pub fn file_prefix(mut self, prefix: &str) -> Self {
        self.file_prefix = Some(prefix.to_string());
        self
    }

    /// Set the log file extension
    /// 
    /// # Parameters
    /// 
    /// * `ext` - Log file extension (including the dot)
    pub fn file_extension(mut self, ext: &str) -> Self {
        self.file_extension = Some(ext.to_string());
        self
    }

    /// Set whether console output is enabled
    /// 
    /// # Parameters
    /// 
    /// * `enabled` - true to enable, false to disable
    pub fn console_enabled(mut self, enabled: bool) -> Self {
        self.console_enabled = Some(enabled);
        self
    }

    /// Set the log level
    /// 
    /// # Parameters
    /// 
    /// * `level` - Log level
    pub fn level(mut self, level: LogLevel) -> Self {
        self.level = Some(level);
        self
    }

    /// Set the output format
    /// 
    /// # Parameters
    /// 
    /// * `format` - Output format
    pub fn output_format(mut self, format: OutputFormat) -> Self {
        self.output_format = Some(format);
        self
    }

    /// Enable log rotation with default configuration
    pub fn enable_rotation(mut self) -> Self {
        self.rotation = Some(RotatorConfig::default());
        self
    }

    /// Set log rotation configuration
    /// 
    /// # Parameters
    /// 
    /// * `config` - Rotation configuration
    pub fn rotation(mut self, config: RotatorConfig) -> Self {
        self.rotation = Some(config);
        self
    }

    /// Disable log rotation
    pub fn disable_rotation(mut self) -> Self {
        self.rotation = None;
        self
    }

    /// Build the final configuration
    /// 
    /// Returns a complete [`Config`] instance.
    pub fn build(self) -> Config {
        Config {
            log_dir: self.log_dir.unwrap_or_else(|| "logs".to_string()),
            file_prefix: self.file_prefix.unwrap_or_else(|| "serial_".to_string()),
            file_extension: self.file_extension.unwrap_or_else(|| ".log".to_string()),
            console_enabled: self.console_enabled.unwrap_or(true),
            level: self.level.unwrap_or(LogLevel::Debug),
            output_format: self.output_format.unwrap_or(OutputFormat::Text),
            rotation: self.rotation.or_else(|| Some(RotatorConfig::default())),
        }
    }
}
