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
use crate::rotator::{Rotator, RotatorConfig};

/// Flush signal
///
/// Used to request the writer thread to flush buffered entries to disk.
struct FlushSignal;

/// Log entry
///
/// Represents a single log record to be written.
#[derive(Debug, Clone)]
pub struct LogEntry {
    level: String,
    message: String,
    timestamp: String,
    output_format: OutputFormat,
}

impl LogEntry {
    /// Create a new log entry
    ///
    /// # Parameters
    ///
    /// * `level` - Log level string
    /// * `message` - Log message
    /// * `timestamp` - Timestamp string
    /// * `output_format` - Output format for this entry
    pub fn new(
        level: &str,
        message: &str,
        timestamp: &str,
        output_format: OutputFormat,
    ) -> Self {
        LogEntry {
            level: level.to_string(),
            message: message.to_string(),
            timestamp: timestamp.to_string(),
            output_format,
        }
    }

    /// Format the entry according to its output format
    pub fn format(&self) -> String {
        match self.output_format {
            OutputFormat::Json => self.format_json(),
            _ => self.format_text(),
        }
    }

    /// Format as text
    fn format_text(&self) -> String {
        format!("[{}] [{}] {}", self.timestamp, self.level, self.message)
    }

    /// Format as JSON
    fn format_json(&self) -> String {
        format!(
            r#"{{"time":"{}","level":"{}","message":"{}"}}"#,
            self.timestamp,
            crate::formatter::Formatter::escape_json(&self.level),
            crate::formatter::Formatter::escape_json(&self.message),
        )
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
/// Supports log rotation via [`Rotator`] when configured.
///
/// # How it works
///
/// 1. Main thread sends log entries to background thread via channel
/// 2. Background thread receives entries and buffers them
/// 3. When buffer reaches specified size or timeout occurs, write batch to file
/// 4. Before each write, checks if rotation is needed (if rotation is configured)
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
    sender: Mutex<Option<Sender<LogEntry>>>,
    flush_sender: Mutex<Option<Sender<FlushSignal>>>,
    is_running: Arc<Mutex<bool>>,
    handle: Mutex<Option<thread::JoinHandle<()>>>,
    formatter: Formatter,
    color_formatter: ColorFormatter,
    config: Config,
    console_enabled: Mutex<bool>,
    output_format: OutputFormat,
}

impl Drop for Writer {
    fn drop(&mut self) {
        *self.is_running.lock().unwrap() = false;
        self.flush();
        self.sender.lock().unwrap().take();
        self.flush_sender.lock().unwrap().take();
        if let Some(handle) = self.handle.lock().unwrap().take() {
            let _ = handle.join();
        }
    }
}

impl Writer {
    /// Create a new writer
    ///
    /// # Parameters
    ///
    /// * `config` - Logger configuration
    pub fn new(config: Config) -> Self {
        let (sender, receiver) = channel::<LogEntry>();
        let (flush_sender, flush_receiver) = channel::<FlushSignal>();
        let is_running = Arc::new(Mutex::new(true));
        let running_clone = Arc::clone(&is_running);
        let file_path = config.get_log_file_path();
        let batch_size = 10;
        let max_wait_ms = 100;
        let console_enabled = config.console_enabled;
        let rotation = config.rotation.clone();
        let output_format = config.output_format.clone();

        let handle = thread::spawn(move || {
            Self::writer_thread(receiver, flush_receiver, &file_path, batch_size, max_wait_ms, &running_clone, rotation);
        });

        let formatter = match &config.output_format {
            OutputFormat::Custom(pattern) => Formatter::with_pattern(pattern.clone()),
            format => Formatter::with_format(format.clone()),
        };

        let color_formatter = ColorFormatter::with_scheme(config.color_scheme.clone());
        color_formatter.set_enabled(config.console_colors);

        Writer {
            sender: Mutex::new(Some(sender)),
            flush_sender: Mutex::new(Some(flush_sender)),
            is_running,
            handle: Mutex::new(Some(handle)),
            formatter,
            color_formatter,
            config,
            console_enabled: Mutex::new(console_enabled),
            output_format,
        }
    }

    /// Background writer thread
    ///
    /// Receives log entries and writes them to file in batches.
    /// Supports log rotation when a `RotatorConfig` is provided.
    fn writer_thread(
        receiver: Receiver<LogEntry>,
        flush_receiver: Receiver<FlushSignal>,
        file_path: &str,
        batch_size: usize,
        max_wait_ms: u64,
        _is_running: &Arc<Mutex<bool>>,
        rotation: Option<RotatorConfig>,
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

        // Set up rotator if rotation is configured
        let rotator = rotation.map(|r| Rotator::new(file_path, r));

        let mut last_rotation_check = std::time::Instant::now();
        let rotation_check_interval = Duration::from_secs(5);

        let mut buffer: Vec<LogEntry> = Vec::with_capacity(batch_size);

        loop {
            // Check flush channel first (non-blocking)
            if let Ok(_) = flush_receiver.try_recv() {
                if !buffer.is_empty() {
                    Self::flush_buffer(&mut buffer, &mut file, &rotator, &path);
                }
            }

            match receiver.recv_timeout(Duration::from_millis(max_wait_ms)) {
                Ok(entry) => {
                    buffer.push(entry);
                    if buffer.len() >= batch_size {
                        Self::flush_buffer(&mut buffer, &mut file, &rotator, &path);
                    }
                }
                Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => {
                    if !buffer.is_empty() {
                        Self::flush_buffer(&mut buffer, &mut file, &rotator, &path);
                    }
                    break;
                }
                Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {
                    if !buffer.is_empty() {
                        Self::flush_buffer(&mut buffer, &mut file, &rotator, &path);
                    }
                    if let Some(ref r) = rotator {
                        if last_rotation_check.elapsed() >= rotation_check_interval {
                            if let Ok(true) = r.needs_rotation() {
                                let _ = file.flush();
                                let _ = r.rotate();
                                file = match OpenOptions::new()
                                    .create(true)
                                    .append(true)
                                    .open(&path)
                                {
                                    Ok(f) => f,
                                    Err(e) => {
                                        eprintln!("Failed to reopen log file after rotation: {}", e);
                                        return;
                                    }
                                };
                            }
                            last_rotation_check = std::time::Instant::now();
                        }
                    }
                }
            }
        }

        // Final flush
        if !buffer.is_empty() {
            Self::flush_buffer(&mut buffer, &mut file, &rotator, &path);
        }
    }

    /// Flush buffer to file, checking rotation before writing
    fn flush_buffer(
        buffer: &mut Vec<LogEntry>,
        file: &mut File,
        rotator: &Option<Rotator>,
        path: &PathBuf,
    ) {
        // Check if rotation is needed before writing
        if let Some(ref r) = rotator {
            if let Ok(true) = r.needs_rotation() {
                let _ = file.flush();
                let _ = r.rotate();
                *file = match OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(path)
                {
                    Ok(f) => f,
                    Err(e) => {
                        eprintln!("Failed to reopen log file after rotation: {}", e);
                        return;
                    }
                };
            }
        }

        let mut lines = String::new();
        for entry in buffer.drain(..) {
            lines.push_str(&entry.format());
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

        let entry = LogEntry::new(&level.to_string(), message, &timestamp, self.output_format.clone());
        if let Some(ref sender) = *self.sender.lock().unwrap() {
            let _ = sender.send(entry);
        }
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

    /// Flush pending log entries
    ///
    /// Sends a signal to the writer thread to flush buffered entries to disk.
    pub fn flush(&self) {
        if let Some(ref sender) = *self.flush_sender.lock().unwrap() {
            let _ = sender.send(FlushSignal);
        }
    }

    /// Stop the writer
    ///
    /// Stops the background thread and flushes remaining logs.
    pub fn stop(&self) {
        self.flush();
        self.sender.lock().unwrap().take();
        self.flush_sender.lock().unwrap().take();
        if let Some(handle) = self.handle.lock().unwrap().take() {
            let _ = handle.join();
        }
    }
}
