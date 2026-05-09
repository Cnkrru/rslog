//! Writer module
//! 
//! Provides asynchronous log writing functionality using a background thread.

use std::fs::{self, File, OpenOptions};
use std::io::{self, Write};
use std::path::{PathBuf};
use std::sync::mpsc::{channel, Sender, Receiver};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH, Duration};

use crate::config::Config;
use crate::formatter::{Formatter, OutputFormat};

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

    let (year, month, day) = unix_days_to_date(days);
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

        Writer {
            sender,
            is_running,
            formatter,
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
        let log_line = self.formatter.format(level, message);

        if *self.console_enabled.lock().unwrap() {
            let mut stdout = io::stdout();
            let _ = writeln!(stdout, "{}", log_line);
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
