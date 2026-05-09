//! Color module
//! 
//! Provides ANSI escape code support for colored console output.
//! 
//! # Examples
//! 
//! ```rust
//! use rslog::color::Color;
//! 
//! let colored = Color::colorize("Hello, world!", Color::Green);
//! println!("{}", colored);
//! ```

/// ANSI color codes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    /// Reset all colors and styles
    Reset,
    /// Black color
    Black,
    /// Red color
    Red,
    /// Green color
    Green,
    /// Yellow color
    Yellow,
    /// Blue color
    Blue,
    /// Magenta color
    Magenta,
    /// Cyan color
    Cyan,
    /// White color
    White,
    /// Bright black (gray)
    BrightBlack,
    /// Bright red
    BrightRed,
    /// Bright green
    BrightGreen,
    /// Bright yellow
    BrightYellow,
    /// Bright blue
    BrightBlue,
    /// Bright magenta
    BrightMagenta,
    /// Bright cyan
    BrightCyan,
    /// Bright white
    BrightWhite,
}

impl Color {
    /// Get the ANSI escape code for the color
    /// 
    /// # Returns
    /// 
    /// ANSI escape code string
    pub fn code(&self) -> &'static str {
        match self {
            Color::Reset => "\x1b[0m",
            Color::Black => "\x1b[30m",
            Color::Red => "\x1b[31m",
            Color::Green => "\x1b[32m",
            Color::Yellow => "\x1b[33m",
            Color::Blue => "\x1b[34m",
            Color::Magenta => "\x1b[35m",
            Color::Cyan => "\x1b[36m",
            Color::White => "\x1b[37m",
            Color::BrightBlack => "\x1b[90m",
            Color::BrightRed => "\x1b[91m",
            Color::BrightGreen => "\x1b[92m",
            Color::BrightYellow => "\x1b[93m",
            Color::BrightBlue => "\x1b[94m",
            Color::BrightMagenta => "\x1b[95m",
            Color::BrightCyan => "\x1b[96m",
            Color::BrightWhite => "\x1b[97m",
        }
    }

    /// Colorize a string with the specified color
    /// 
    /// # Parameters
    /// 
    /// * `text` - Text to colorize
    /// * `color` - Color to apply
    /// 
    /// # Returns
    /// 
    /// Colorized string with ANSI escape codes
    pub fn colorize(text: &str, color: Color) -> String {
        format!("{}{}{}", color.code(), text, Color::Reset.code())
    }

    /// Colorize a string with background color
    /// 
    /// # Parameters
    /// 
    /// * `text` - Text to colorize
    /// * `bg_color` - Background color
    /// 
    /// # Returns
    /// 
    /// String with background color
    pub fn colorize_bg(text: &str, bg_color: Color) -> String {
        let bg_code = match bg_color {
            Color::Black => "\x1b[40m",
            Color::Red => "\x1b[41m",
            Color::Green => "\x1b[42m",
            Color::Yellow => "\x1b[43m",
            Color::Blue => "\x1b[44m",
            Color::Magenta => "\x1b[45m",
            Color::Cyan => "\x1b[46m",
            Color::White => "\x1b[47m",
            Color::BrightBlack => "\x1b[100m",
            Color::BrightRed => "\x1b[101m",
            Color::BrightGreen => "\x1b[102m",
            Color::BrightYellow => "\x1b[103m",
            Color::BrightBlue => "\x1b[104m",
            Color::BrightMagenta => "\x1b[105m",
            Color::BrightCyan => "\x1b[106m",
            Color::BrightWhite => "\x1b[107m",
            _ => "\x1b[49m", // Default background
        };
        format!("{}{}{}", bg_code, text, Color::Reset.code())
    }

    /// Colorize a string with both foreground and background colors
    /// 
    /// # Parameters
    /// 
    /// * `text` - Text to colorize
    /// * `fg_color` - Foreground color
    /// * `bg_color` - Background color
    /// 
    /// # Returns
    /// 
    /// Fully colorized string
    pub fn colorize_full(text: &str, fg_color: Color, bg_color: Color) -> String {
        let bg_code = match bg_color {
            Color::Black => "\x1b[40m",
            Color::Red => "\x1b[41m",
            Color::Green => "\x1b[42m",
            Color::Yellow => "\x1b[43m",
            Color::Blue => "\x1b[44m",
            Color::Magenta => "\x1b[45m",
            Color::Cyan => "\x1b[46m",
            Color::White => "\x1b[47m",
            Color::BrightBlack => "\x1b[100m",
            Color::BrightRed => "\x1b[101m",
            Color::BrightGreen => "\x1b[102m",
            Color::BrightYellow => "\x1b[103m",
            Color::BrightBlue => "\x1b[104m",
            Color::BrightMagenta => "\x1b[105m",
            Color::BrightCyan => "\x1b[106m",
            Color::BrightWhite => "\x1b[107m",
            _ => "",
        };
        format!("{}{}{}{}", fg_color.code(), bg_code, text, Color::Reset.code())
    }

    /// Check if colors are supported (basic check for terminal support)
    ///
    /// Checks:
    /// 1. `NO_COLOR` environment variable (https://no-color.org)
    /// 2. `TERM` environment variable (e.g. "dumb" means no color)
    /// 3. `WT_SESSION` for Windows Terminal detection
    ///
    /// # Returns
    ///
    /// true if colors are likely supported
    pub fn is_supported() -> bool {
        // Respect NO_COLOR standard (https://no-color.org)
        if std::env::var_os("NO_COLOR").is_some() {
            return false;
        }

        // Check TERM — "dumb" terminals don't support color
        if let Some(term) = std::env::var_os("TERM") {
            let term = term.to_string_lossy();
            if term.eq_ignore_ascii_case("dumb") {
                return false;
            }
        }

        // On Windows, assume color is supported via Virtual Terminal Processing
        // (enabled since Windows 10 1511). WT_SESSION indicates Windows Terminal.
        if cfg!(windows) {
            return std::env::var_os("WT_SESSION").is_some()
                || std::env::var_os("TERM_PROGRAM").is_some();
        }

        true
    }
}

/// Color scheme for log levels
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LogColorScheme {
    /// Color for trace level
    pub trace: Color,
    /// Color for debug level
    pub debug: Color,
    /// Color for info level
    pub info: Color,
    /// Color for warn level
    pub warn: Color,
    /// Color for error level
    pub error: Color,
    /// Color for critical level
    pub critical: Color,
}

impl Default for LogColorScheme {
    /// Default color scheme
    ///
    /// - Trace: BrightBlack (gray)
    /// - Debug: Cyan
    /// - Info: Green
    /// - Warn: Yellow
    /// - Error: Red
    /// - Critical: BrightRed
    fn default() -> Self {
        LogColorScheme {
            trace: Color::BrightBlack,
            debug: Color::Cyan,
            info: Color::Green,
            warn: Color::Yellow,
            error: Color::Red,
            critical: Color::BrightRed,
        }
    }
}

impl LogColorScheme {
    /// Create a new color scheme
    pub fn new(
        trace: Color,
        debug: Color,
        info: Color,
        warn: Color,
        error: Color,
        critical: Color,
    ) -> Self {
        LogColorScheme {
            trace,
            debug,
            info,
            warn,
            error,
            critical,
        }
    }

    /// Get color for a log level
    pub fn get_color(&self, level: crate::level::LogLevel) -> Color {
        match level {
            crate::level::LogLevel::Trace => self.trace,
            crate::level::LogLevel::Debug => self.debug,
            crate::level::LogLevel::Info => self.info,
            crate::level::LogLevel::Warn => self.warn,
            crate::level::LogLevel::Error => self.error,
            crate::level::LogLevel::Critical => self.critical,
        }
    }

    /// Colorize a log message based on level
    /// 
    /// # Parameters
    /// 
    /// * `level` - Log level
    /// * `message` - Log message
    /// * `use_colors` - Whether to apply colors
    /// 
    /// # Returns
    /// 
    /// Colorized message if colors are enabled, otherwise original message
    pub fn colorize_message(&self, level: crate::level::LogLevel, message: &str, use_colors: bool) -> String {
        if use_colors && Color::is_supported() {
            let color = self.get_color(level);
            Color::colorize(message, color)
        } else {
            message.to_string()
        }
    }

    /// Colorize a complete log line (including timestamp and level)
    /// 
    /// # Parameters
    /// 
    /// * `timestamp` - Timestamp string
    /// * `level` - Log level
    /// * `message` - Log message
    /// * `use_colors` - Whether to apply colors
    /// 
    /// # Returns
    /// 
    /// Colorized log line
    pub fn colorize_log_line(&self, timestamp: &str, level: crate::level::LogLevel, message: &str, use_colors: bool) -> String {
        if use_colors && Color::is_supported() {
            let level_str = level.to_string();
            let color = self.get_color(level);
            
            // Colorize the level part only
            let colored_level = Color::colorize(&format!("[{}]", level_str), color);
            format!("[{}] {} {}", timestamp, colored_level, message)
        } else {
            format!("[{}] [{}] {}", timestamp, level, message)
        }
    }
}

/// Color formatter for log output
pub struct ColorFormatter {
    scheme: LogColorScheme,
    enabled: std::sync::Mutex<bool>,
}

impl ColorFormatter {
    /// Create a new color formatter with default scheme
    pub fn new() -> Self {
        ColorFormatter {
            scheme: LogColorScheme::default(),
            enabled: std::sync::Mutex::new(true),
        }
    }

    /// Create a new color formatter with custom scheme
    pub fn with_scheme(scheme: LogColorScheme) -> Self {
        ColorFormatter {
            scheme,
            enabled: std::sync::Mutex::new(true),
        }
    }

    /// Enable or disable color output
    pub fn set_enabled(&self, enabled: bool) {
        *self.enabled.lock().unwrap() = enabled;
    }

    /// Check if color output is enabled
    pub fn is_enabled(&self) -> bool {
        *self.enabled.lock().unwrap()
    }

    /// Format a log message with colors
    /// 
    /// # Parameters
    /// 
    /// * `timestamp` - Timestamp string
    /// * `level` - Log level
    /// * `message` - Log message
    /// * `for_console` - true for console output, false for file output
    /// 
    /// # Returns
    /// 
    /// Formatted log line
    pub fn format(&self, timestamp: &str, level: crate::level::LogLevel, message: &str, for_console: bool) -> String {
        if for_console && *self.enabled.lock().unwrap() {
            self.scheme.colorize_log_line(timestamp, level, message, true)
        } else {
            format!("[{}] [{}] {}", timestamp, level, message)
        }
    }

    /// Format only the message part with colors
    pub fn format_message(&self, level: crate::level::LogLevel, message: &str, for_console: bool) -> String {
        if for_console && *self.enabled.lock().unwrap() {
            self.scheme.colorize_message(level, message, true)
        } else {
            message.to_string()
        }
    }
}

impl Default for ColorFormatter {
    fn default() -> Self {
        ColorFormatter::new()
    }
}