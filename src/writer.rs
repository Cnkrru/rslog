//! Writer module
//! 
//! Provides asynchronous log writing functionality using a background thread.

use std::fs::{self, File, OpenOptions};
use std::io::{self, Write};
use std::path::{PathBuf};
use std::sync::mpsc::{channel, Sender, Receiver};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use crate::config::Config;
use crate::formatter::{Formatter, OutputFormat};
use crate::color::ColorFormatter;

/// Log entry
/// 
/// Represents a single log record to be written.
#[derive(Debug, Clone)]
pub struct LogEntry {
    level: String,
    message: String,
    timestamp: String,
}

impl LogEntry {
    /// Create a new log entry
    /// 
    /// # Parameters
    /// 
    /// * `level` - Log level string
    /// * `message` - Log message
    /// * `timestamp` - Timestamp string
    pub fn new(level: &str, message: &str, timestamp: &str) -> Self {
        LogEntry {
            level: level.to_string(),
            message: message.to_string(),
            timestamp: timestamp.to_string(),
        }
    }

    /// Format as text
    pub fn format_text(&self) -> String {
        format!("[{}] [{}] {}", self.timestamp, self.level, self.message)
    }

    /// Format as JSON
    pub fn format_json(&self) -> String {
        format!(
            r#"{{"time":"{}","level":"{}","message":"{}"}}"#,
            self.timestamp, self.level, self.message
        )
    }
}

/// Async writer configuration
/// 
/// Configures the behavior of the async writer.
#[derive(Debug, Clone)]
pub struct AsyncWriterConfig {
    /// Channel capacity (maximum buffered log entries)
    pub channel_capacity: usize,
    /// Batch size for writing (write when this number is reached)
    pub batch_size: usize,
    /// Maximum wait time (ms), flush buffer after timeout
    pub max_wait_ms: u64,
}

impl Default for AsyncWriterConfig {
    fn default() -> Self {
        AsyncWriterConfig {
            channel_capacity: 1000,
            batch_size: 10,
            max_wait_ms: 100,
        }
    }
}

/// Get current timestamp string
/// 
/// Format: `YYYY-MM-DD HH:MM:SS.NNNNNNNNN`
fn get_timestamp() -> String {
    crate::formatter::Formatter::get_timestamp()
}

/// Log writer
/// 
/// Implements asynchronous log writing using a background thread to avoid blocking the main thread.
/// 
/// # How it works
/// 
/// 1. Main thread sends log entries to background thread via channel
/// 2. Background thread receives entries and buffers them
/// 3. When buffer reaches specified size or timeout occurs, write batch to file
/// 
/// # Examples
/// 
/// ```rust
/// use rslog::writer::Writer;
/// use rslog::config::Config;
/// 
/// let config = Config::default();
/// let writer = Writer::new(config);
/// writer.write(rslog::LogLevel::Info, "Hello, world!");
/// ```
pub struct Writer {
    sender: Sender<LogEntry>,
    is_running: Arc<Mutex<bool>>,
    formatter: Formatter,
    color_formatter: ColorFormatter,
    config: Config,
    console_enabled: Mutex<bool>,
}

impl Writer {
    /// Create a new writer
    /// 
    /// # Parameters
    /// 
    /// * `config` - Logger configuration
    pub fn new(config: Config) -> Self {
        let (sender, receiver) = channel::<LogEntry>();
        let is_running = Arc::new(Mutex::new(true));
        let running_clone = Arc::clone(&is_running);
        let file_path = config.get_log_file_path();
        let batch_size = 10;
        let max_wait_ms = 100;
        let output_format_clone = config.output_format.clone();
        let console_enabled = config.console_enabled;

        thread::spawn(move || {
            Self::writer_thread(receiver, &file_path, batch_size, max_wait_ms, &running_clone, output_format_clone);
        });

        let formatter = match &config.output_format {
            OutputFormat::Custom(pattern) => Formatter::with_pattern(pattern.clone()),
            format => Formatter::with_format(format.clone()),
        };

        let mut color_formatter = ColorFormatter::with_scheme(config.color_scheme.clone());
        color_formatter.set_enabled(config.console_colors);

        Writer {
            sender,
            is_running,
            formatter,
            color_formatter,
            config,
            console_enabled: Mutex::new(console_enabled),
        }
    }

    /// Background writer thread
    /// 
    /// Receives log entries and writes them to file in batches.
    fn writer_thread(
        receiver: Receiver<LogEntry>,
        file_path: &str,
        batch_size: usize,
        max_wait_ms: u64,
        is_running: &Arc<Mutex<bool>>,
        output_format: OutputFormat,
    ) {
        let path = PathBuf::from(file_path);
        
        if let Some(parent) = path.parent() {
            let _ = fs::create_dir_all(parent);
        }

        let mut file = match OpenOptions::new()
            .create(true)
            .append(true)
            .open(&path)
        {
            Ok(f) => f,
            Err(e) => {
                eprintln!("Failed to open log file: {}", e);
                return;
            }
        };

        let mut buffer: Vec<LogEntry> = Vec::with_capacity(batch_size);

        while *is_running.lock().unwrap() {
            match receiver.recv_timeout(Duration::from_millis(max_wait_ms)) {
                Ok(entry) => {
                    buffer.push(entry);
                    if buffer.len() >= batch_size {
                        Self::flush_buffer(&mut buffer, &mut file, &output_format);
                    }
                }
                Err(_) => {
                    if !buffer.is_empty() {
                        Self::flush_buffer(&mut buffer, &mut file, &output_format);
                    }
                }
            }
        }

        if !buffer.is_empty() {
            Self::flush_buffer(&mut buffer, &mut file, &output_format);
        }
    }

    /// Flush buffer to file
    /// 
    /// Writes buffered log entries to file in batch.
    fn flush_buffer(buffer: &mut Vec<LogEntry>, file: &mut File, output_format: &OutputFormat) {
        let mut lines = String::new();
        for entry in buffer.drain(..) {
            match output_format {
                OutputFormat::Json => lines.push_str(&entry.format_json()),
                _ => lines.push_str(&entry.format_text()),
            }
            lines.push('\n');
        }
        
        let _ = file.write_all(lines.as_bytes());
        let _ = file.flush();
    }

    /// Write a log message
    /// 
    /// Outputs to console (if enabled) and sends to background thread for file writing.
    /// 
    /// # Parameters
    /// 
    /// * `level` - Log level
    /// * `message` - Log message
    pub fn write(&self, level: crate::level::LogLevel, message: &str) {
        let timestamp = get_timestamp();
        
        // For file output: plain text without colors
        let file_log_line = self.formatter.format(level, message);
        
        // For console output: with colors if enabled
        let console_log_line = if *self.console_enabled.lock().unwrap() {
            self.color_formatter.format(&timestamp, level, message, true)
        } else {
            file_log_line.clone()
        };

        if *self.console_enabled.lock().unwrap() {
            let mut stdout = io::stdout();
            let _ = writeln!(stdout, "{}", console_log_line);
        }

        let entry = LogEntry::new(&level.to_string(), message, &timestamp);
        let _ = self.sender.send(entry);
    }

    /// Set whether console output is enabled
    /// 
    /// # Parameters
    /// 
    /// * `enabled` - true to enable, false to disable
    pub fn set_console_enabled(&self, enabled: bool) {
        *self.console_enabled.lock().unwrap() = enabled;
    }

    /// Set whether console colors are enabled
    /// 
    /// # Parameters
    /// 
    /// * `enabled` - true to enable colors, false to disable
    pub fn set_console_colors(&self, enabled: bool) {
        self.color_formatter.set_enabled(enabled);
    }

    /// Get current configuration
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// Stop the writer
    /// 
    /// Stops the background thread and flushes remaining logs.
    pub fn stop(&self) {
        *self.is_running.lock().unwrap() = false;
    }
}

impl Drop for Writer {
    fn drop(&mut self) {
        self.stop();
    }
}
