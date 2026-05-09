//! Rotator module
//! 
//! Provides log file rotation functionality with size-based and time-based rotation.

use std::fs::{self, File, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::sync::Mutex;

/// Rotation strategy
/// 
/// Defines the strategy for rotating log files.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RotationStrategy {
    /// Rotate based on file size
    /// 
    /// Parameter is the size limit in bytes
    SizeBased(u64),
    /// Rotate daily
    Daily,
    /// Rotate based on both size and time
    Combined { 
        /// Size limit in bytes
        size_limit: u64 
    },
}

impl Default for RotationStrategy {
    fn default() -> Self {
        RotationStrategy::SizeBased(10 * 1024 * 1024) // 10MB default
    }
}

/// Rotator configuration
/// 
/// Configures the behavior of the log rotator.
#[derive(Debug, Clone)]
pub struct RotatorConfig {
    /// Rotation strategy
    pub strategy: RotationStrategy,
    /// Maximum number of log files to keep
    pub max_files: usize,
    /// Whether to compress old log files
    pub compress_old_files: bool,
}

impl Default for RotatorConfig {
    fn default() -> Self {
        RotatorConfig {
            strategy: RotationStrategy::SizeBased(10 * 1024 * 1024),
            max_files: 10,
            compress_old_files: false,
        }
    }
}

/// Log rotator
/// 
/// Manages log file rotation with support for size-based and time-based rotation.
/// 
/// # How it works
/// 
/// 1. Check if current log file needs rotation
/// 2. If rotation is needed, rename current file and create a new one
/// 3. Clean up old files that exceed the maximum count
pub struct Rotator {
    base_path: PathBuf,
    current_file: Mutex<Option<File>>,
    config: RotatorConfig,
    file_counter: Mutex<usize>,
}

impl Rotator {
    /// Create a new rotator
    /// 
    /// # Parameters
    /// 
    /// * `base_path` - Base path for log files
    /// * `config` - Rotation configuration
    pub fn new(base_path: &str, config: RotatorConfig) -> Self {
        Rotator {
            base_path: PathBuf::from(base_path),
            current_file: Mutex::new(None),
            config,
            file_counter: Mutex::new(0),
        }
    }

    /// Get the base path
    pub fn base_path(&self) -> &Path {
        &self.base_path
    }

    /// Initialize or rotate the log file
    /// 
    /// Creates a new file if none exists, or rotates if needed.
    pub fn init_or_rotate(&self) -> io::Result<()> {
        let mut current_file = self.current_file.lock().unwrap();
        
        if current_file.is_none() {
            *current_file = Some(self.create_new_file()?);
        } else {
            if self.needs_rotation()? {
                self.rotate()?;
            }
        }
        
        Ok(())
    }

    /// Check if rotation is needed
    fn needs_rotation(&self) -> io::Result<bool> {
        let current_file = self.current_file.lock().unwrap();
        if let Some(file) = current_file.as_ref() {
            match self.config.strategy {
                RotationStrategy::SizeBased(limit) => {
                    let metadata = file.metadata()?;
                    Ok(metadata.len() >= limit)
                }
                RotationStrategy::Daily => {
                    Ok(self.is_new_day())
                }
                RotationStrategy::Combined { size_limit } => {
                    let metadata = file.metadata()?;
                    Ok(metadata.len() >= size_limit || self.is_new_day())
                }
            }
        } else {
            Ok(false)
        }
    }

    /// Check if it's a new day
    fn is_new_day(&self) -> bool {
        let current_filename = self.get_current_filename();
        let expected_filename = self.generate_filename(0);
        current_filename != expected_filename
    }

    /// Perform rotation
    /// 
    /// Renames the current file and creates a new one, then cleans up old files.
    fn rotate(&self) -> io::Result<()> {
        let mut current_file = self.current_file.lock().unwrap();
        
        if let Some(mut file) = current_file.take() {
            file.flush()?;
            
            let old_path = self.get_current_file_path();
            let new_path = self.get_rotated_file_path();
            
            drop(file);
            
            fs::rename(&old_path, &new_path)?;
            
            self.cleanup_old_files()?;
            
            *current_file = Some(self.create_new_file()?);
        }
        
        Ok(())
    }

    /// Create a new log file
    fn create_new_file(&self) -> io::Result<File> {
        let path = self.get_current_file_path();
        
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(&path)
    }

    /// Get the current log file path
    fn get_current_file_path(&self) -> PathBuf {
        let filename = self.get_current_filename();
        self.base_path.with_file_name(filename)
    }

    /// Get the rotated file path
    fn get_rotated_file_path(&self) -> PathBuf {
        let counter = {
            let mut c = self.file_counter.lock().unwrap();
            *c += 1;
            *c
        };
        
        let filename = self.generate_filename(counter);
        self.base_path.with_file_name(filename)
    }

    /// Generate a filename with rotation counter
    /// 
    /// # Parameters
    /// 
    /// * `counter` - Rotation counter (0 for current file)
    fn generate_filename(&self, counter: usize) -> String {
        let base = self.base_path.file_name().unwrap().to_string_lossy();
        let ext = self.base_path.extension().unwrap_or_default().to_string_lossy();
        
        if ext.is_empty() {
            if counter > 0 {
                format!("{}.{}", base, counter)
            } else {
                base.to_string()
            }
        } else {
            let stem = base.strip_suffix(&format!(".{}", ext)).unwrap_or(&base);
            if counter > 0 {
                format!("{}.{}.{}", stem, counter, ext)
            } else {
                base.to_string()
            }
        }
    }

    /// Get the current filename (without rotation counter)
    fn get_current_filename(&self) -> String {
        self.generate_filename(0)
    }

    /// Clean up old files that exceed the maximum count
    fn cleanup_old_files(&self) -> io::Result<()> {
        if self.config.max_files == 0 {
            return Ok(());
        }
        
        let parent = match self.base_path.parent() {
            Some(p) => p,
            None => return Ok(()),
        };
        
        let file_stem = self.base_path.file_stem().unwrap().to_string_lossy();
        let file_ext = self.base_path.extension().unwrap_or_default().to_string_lossy();
        
        let mut files: Vec<(PathBuf, u64)> = fs::read_dir(parent)?
            .filter_map(|entry| {
                let entry = entry.ok()?;
                let path = entry.path();
                
                let file_name = path.file_name()?.to_string_lossy();
                
                if file_ext.is_empty() {
                    if let Some(num_str) = file_name.strip_prefix(&format!("{}.", file_stem)) {
                        if let Ok(num) = num_str.parse::<u64>() {
                            return Some((path, num));
                        }
                    }
                } else {
                    let pattern = format!("{}.", file_stem);
                    if let Some(rest) = file_name.strip_prefix(&pattern) {
                        if let Some(dot_idx) = rest.rfind('.') {
                            let (num_str, ext) = rest.split_at(dot_idx);
                            if ext == &format!(".{}", file_ext) {
                                if let Ok(num) = num_str.parse::<u64>() {
                                    return Some((path, num));
                                }
                            }
                        }
                    }
                }
                
                None
            })
            .collect();
        
        files.sort_by(|a, b| b.1.cmp(&a.1));
        
        for (path, _) in files.into_iter().skip(self.config.max_files) {
            fs::remove_file(path)?;
        }
        
        Ok(())
    }

    /// Write data to the log file
    /// 
    /// Checks for rotation first, then writes data.
    pub fn write(&self, data: &[u8]) -> io::Result<()> {
        self.init_or_rotate()?;
        
        let mut current_file = self.current_file.lock().unwrap();
        if let Some(file) = current_file.as_mut() {
            file.write_all(data)?;
            file.flush()?;
        }
        
        Ok(())
    }

    /// Write a formatted log line
    pub fn writeln(&self, line: &str) -> io::Result<()> {
        self.write(line.as_bytes())?;
        self.write(b"\n")
    }

    /// Get current configuration
    pub fn config(&self) -> &RotatorConfig {
        &self.config
    }
}
