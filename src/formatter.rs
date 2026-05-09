//! Formatter module
//! 
//! Handles log message formatting with support for multiple output formats.

use std::time::{SystemTime, UNIX_EPOCH};

use crate::level::LogLevel;

/// Output format enum
/// 
/// Defines supported log output formats.
/// 
/// # Examples
/// 
/// ```rust
/// use rslog::OutputFormat;
/// 
/// let format = OutputFormat::Json;
/// let custom = OutputFormat::Custom("%d [%p] %m".to_string());
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OutputFormat {
    /// Text format: `[timestamp] [level] message`
    Text,
    /// JSON format: `{"time":"...","level":"...","message":"..."}`
    Json,
    /// Custom format using placeholders
    Custom(String),
}

/// Log formatter
/// 
/// Handles formatting log messages into the specified output format.
/// 
/// # Examples
/// 
/// ```rust
/// use rslog::{Formatter, OutputFormat, LogLevel};
/// 
/// let formatter = Formatter::with_format(OutputFormat::Text);
/// let line = formatter.format(LogLevel::Info, "Hello, world!");
/// println!("{}", line);
/// ```
pub struct Formatter {
    format: OutputFormat,
}

impl Formatter {
    /// Create a default formatter (text format)
    pub fn new() -> Self {
        Formatter {
            format: OutputFormat::Text,
        }
    }

    /// Create a formatter with the specified format
    /// 
    /// # Parameters
    /// 
    /// * `format` - Output format
    pub fn with_format(format: OutputFormat) -> Self {
        Formatter { format }
    }

    /// Create a formatter with a custom pattern
    /// 
    /// # Parameters
    /// 
    /// * `pattern` - Custom format pattern
    /// 
    /// # Supported Placeholders
    /// 
    /// | Placeholder | Description | Example |
    /// |--------|------|------|
    /// | `%d` | Full timestamp | `2026-05-09 10:30:45.123456789` |
    /// | `%D` | Date only | `2026-05-09` |
    /// | `%T` | Time only | `10:30:45.123456789` |
    /// | `%p` | Full log level | `DEBUG`, `INFO`, `WARN`, `ERROR`, `CRITICAL` |
    /// | `%P` | Short log level | `D`, `I`, `W`, `E`, `C` |
    /// | `%m` | Log message | User-provided message |
    /// | `%n` | Newline | `\n` |
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use rslog::{Formatter, LogLevel};
    /// 
    /// let formatter = Formatter::with_pattern("%D %T [%P] %m");
    /// let line = formatter.format(LogLevel::Info, "Hello");
    /// // Output: 2026-05-09 10:30:45.123456789 [I] Hello
    /// ```
    pub fn with_pattern(pattern: String) -> Self {
        Formatter {
            format: OutputFormat::Custom(pattern),
        }
    }

    /// Format a log message
    /// 
    /// # Parameters
    /// 
    /// * `level` - Log level
    /// * `message` - Log message
    /// 
    /// # Returns
    /// 
    /// Formatted log string
    pub fn format(&self, level: LogLevel, message: &str) -> String {
        match self.format {
            OutputFormat::Text => self.format_text(level, message),
            OutputFormat::Json => self.format_json(level, message),
            OutputFormat::Custom(ref pattern) => self.format_custom(pattern, level, message),
        }
    }

    /// Format as text
    fn format_text(&self, level: LogLevel, message: &str) -> String {
        format!(
            "[{}] [{}] {}",
            Self::get_timestamp(),
            level,
            message
        )
    }

    /// Format as JSON
    fn format_json(&self, level: LogLevel, message: &str) -> String {
        format!(
            r#"{{"time":"{}","level":"{}","message":"{}"}}"#,
            Self::get_timestamp(),
            level,
            message
        )
    }

    /// Format with custom pattern
    fn format_custom(&self, pattern: &str, level: LogLevel, message: &str) -> String {
        let timestamp = Self::get_timestamp();
        let (date, time) = timestamp.split_at(10);
        
        pattern
            .replace("%d", &timestamp)
            .replace("%D", date)
            .replace("%T", &time[1..])
            .replace("%p", &level.to_string())
            .replace("%P", level.short_name())
            .replace("%m", message)
            .replace("%n", "\n")
    }

    /// Get current date string (for file names)
    /// 
    /// Format: `YYYYMMDD`
    pub fn get_date_string() -> String {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default();

        let total_seconds = now.as_secs();
        let minutes = total_seconds / 60;
        let hours = minutes / 60;
        let days = hours / 24;

        let (year, month, day) = Self::unix_days_to_date(days);
        format!("{:04}{:02}{:02}", year, month, day)
    }

    /// Get current timestamp string
    /// 
    /// Format: `YYYY-MM-DD HH:MM:SS.NNNNNNNNN`
    pub fn get_timestamp() -> String {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default();

        let total_seconds = now.as_secs();
        let nanoseconds = now.subsec_nanos();

        let minutes = total_seconds / 60;
        let seconds = total_seconds % 60;
        let hours = minutes / 60;
        let minutes = minutes % 60;
        let days = hours / 24;
        let hours = hours % 24;

        let (year, month, day) = Self::unix_days_to_date(days);
        format!(
            "{:04}-{:02}-{:02} {:02}:{:02}:{:02}.{:09}",
            year, month, day, hours, minutes, seconds, nanoseconds
        )
    }

    /// Convert Unix days to date
    fn unix_days_to_date(mut days: u64) -> (u64, u64, u64) {
        let mut year = 1970;
        let mut month = 1;
        let mut day = 1;

        let days_in_month = |y: u64, m: u64| -> u64 {
            match m {
                1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
                4 | 6 | 9 | 11 => 30,
                2 => {
                    if y.is_multiple_of(4) && (!y.is_multiple_of(100) || y.is_multiple_of(400)) {
                        29
                    } else {
                        28
                    }
                }
                _ => 0,
            }
        };

        while days > 0 {
            let dim = days_in_month(year, month);
            if days >= dim {
                days -= dim;
                month += 1;
                if month > 12 {
                    month = 1;
                    year += 1;
                }
            } else {
                day += days;
                days = 0;
            }
        }

        (year, month, day)
    }
}
