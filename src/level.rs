//! Log level definition module
//! 
//! Defines the log level enum and related implementations.

/// Log level enum
/// 
/// Defines five log levels, ordered by priority from lowest to highest:
/// 
/// - **Debug**: Detailed debug information, typically only used during development
/// - **Info**: General information about program execution
/// - **Warn**: Warning messages indicating potential issues
/// - **Error**: Error messages for recoverable errors
/// - **Critical**: Critical errors indicating unrecoverable failures
/// 
/// # Examples
/// 
/// ```rust
/// use rslog::LogLevel;
/// 
/// let level = LogLevel::Info;
/// println!("Log level: {}", level);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    /// Debug level, lowest priority
    Debug,
    /// Info level
    Info,
    /// Warning level
    Warn,
    /// Error level
    Error,
    /// Critical level, highest priority
    Critical,
}

impl std::fmt::Display for LogLevel {
    /// Convert log level to string representation
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use rslog::LogLevel;
    /// 
    /// assert_eq!(LogLevel::Debug.to_string(), "DEBUG");
    /// assert_eq!(LogLevel::Info.to_string(), "INFO");
    /// assert_eq!(LogLevel::Warn.to_string(), "WARN");
    /// assert_eq!(LogLevel::Error.to_string(), "ERROR");
    /// assert_eq!(LogLevel::Critical.to_string(), "CRITICAL");
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogLevel::Debug => write!(f, "DEBUG"),
            LogLevel::Info => write!(f, "INFO"),
            LogLevel::Warn => write!(f, "WARN"),
            LogLevel::Error => write!(f, "ERROR"),
            LogLevel::Critical => write!(f, "CRITICAL"),
        }
    }
}

impl std::str::FromStr for LogLevel {
    type Err = ();

    /// Parse log level from string
    /// 
    /// Supported values: `debug`, `info`, `warn`, `error`, `critical` (case-insensitive)
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use rslog::LogLevel;
    /// use std::str::FromStr;
    /// 
    /// assert_eq!(LogLevel::from_str("info").unwrap(), LogLevel::Info);
    /// assert_eq!(LogLevel::from_str("ERROR").unwrap(), LogLevel::Error);
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "debug" => Ok(LogLevel::Debug),
            "info" => Ok(LogLevel::Info),
            "warn" => Ok(LogLevel::Warn),
            "error" => Ok(LogLevel::Error),
            "critical" => Ok(LogLevel::Critical),
            _ => Err(()),
        }
    }
}

impl LogLevel {
    /// Check if the current level should log messages at the specified level
    /// 
    /// Returns true if the specified level is greater than or equal to the current level.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use rslog::LogLevel;
    /// 
    /// let current_level = LogLevel::Info;
    /// 
    /// assert!(!current_level.should_log(LogLevel::Debug));
    /// assert!(current_level.should_log(LogLevel::Info));
    /// assert!(current_level.should_log(LogLevel::Warn));
    /// ```
    pub fn should_log(&self, level: LogLevel) -> bool {
        level >= *self
    }

    /// Get the short name of the log level
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use rslog::LogLevel;
    /// 
    /// assert_eq!(LogLevel::Debug.short_name(), "D");
    /// assert_eq!(LogLevel::Info.short_name(), "I");
    /// assert_eq!(LogLevel::Warn.short_name(), "W");
    /// assert_eq!(LogLevel::Error.short_name(), "E");
    /// assert_eq!(LogLevel::Critical.short_name(), "C");
    /// ```
    pub fn short_name(&self) -> &'static str {
        match self {
            LogLevel::Debug => "D",
            LogLevel::Info => "I",
            LogLevel::Warn => "W",
            LogLevel::Error => "E",
            LogLevel::Critical => "C",
        }
    }
}
