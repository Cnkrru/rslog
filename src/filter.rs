//! Log filter module
//!
//! Provides per-module log level filtering, allowing different log levels
//! for different modules/targets.
//!
//! # Examples
//!
//! ```rust
//! use rslog::{LogFilter, LogLevel};
//!
//! let mut filter = LogFilter::new();
//! filter.set_module_level("network", LogLevel::Error);  // Only errors from network module
//! filter.set_module_level("database", LogLevel::Debug);  // Debug level for database module
//!
//! assert!(!filter.should_log("network", LogLevel::Info));  // Filtered
//! assert!(filter.should_log("network", LogLevel::Error));  // Not filtered
//! assert!(filter.should_log("database", LogLevel::Debug)); // Not filtered
//! ```

use std::collections::HashMap;
use crate::level::LogLevel;

/// Log filter configuration
///
/// Allows fine-grained control over which modules log at which level.
/// Works as an override on top of the global log level:
/// - If a module has a specific level set, that level is used
/// - If no override exists, the default (global) level is used
///
/// Module names are matched by prefix — setting a level for `"myapp::http"`
/// will also match `"myapp::http::server"`.
#[derive(Debug, Clone)]
pub struct LogFilter {
    /// Per-module level overrides (module prefix -> level)
    module_levels: HashMap<String, LogLevel>,
}

impl Default for LogFilter {
    fn default() -> Self {
        LogFilter {
            module_levels: HashMap::new(),
        }
    }
}

impl LogFilter {
    /// Create an empty log filter with no module overrides
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a log filter from an environment variable string
    ///
    /// Format: `module1=level,module2=level`
    /// Example: `network=error,database=debug,http=trace`
    ///
    /// Module prefixes are matched by longest match when filtering.
    pub fn from_env(env_str: &str) -> Self {
        let mut filter = LogFilter::new();
        for entry in env_str.split(',') {
            let entry = entry.trim();
            if let Some(eq_pos) = entry.find('=') {
                let module = entry[..eq_pos].trim().to_string();
                if let Ok(level) = entry[eq_pos + 1..].trim().parse::<LogLevel>() {
                    filter.set_module_level(&module, level);
                }
            }
        }
        filter
    }

    /// Set a per-module log level override
    ///
    /// # Parameters
    ///
    /// * `module` - Module path prefix (e.g., `"network"` or `"myapp::http"`)
    /// * `level` - Log level for this module
    pub fn set_module_level(&mut self, module: &str, level: LogLevel) {
        self.module_levels.insert(module.to_string(), level);
    }

    /// Remove a per-module override, falling back to global level
    ///
    /// # Parameters
    ///
    /// * `module` - Module path prefix to remove
    pub fn remove_module(&mut self, module: &str) {
        self.module_levels.remove(module);
    }

    /// Get the effective log level for a given module
    ///
    /// Uses longest-prefix matching: if `"myapp"` and `"myapp::http"` both
    /// have overrides, `"myapp::http::server"` will use `"myapp::http"`'s level.
    ///
    /// # Parameters
    ///
    /// * `module` - Module path (e.g., `"myapp::http::server"` or `"main"`)
    ///
    /// # Returns
    ///
    /// `None` if no module override matches (caller should use global level)
    pub fn get_effective_level(&self, module: &str) -> Option<LogLevel> {
        if self.module_levels.is_empty() {
            return None;
        }

        // Try exact match first, then longest prefix match
        let mut best_match: Option<(usize, &LogLevel)> = None;
        for (prefix, level) in &self.module_levels {
            if module == prefix || module.starts_with(&format!("{}::", prefix)) {
                let is_better = match best_match {
                    None => true,
                    Some((best_len, _)) => prefix.len() > best_len,
                };
                if is_better {
                    best_match = Some((prefix.len(), level));
                }
            }
        }

        best_match.map(|(_, level)| *level)
    }

    /// Check if a log message from a given module should be logged
    ///
    /// # Parameters
    ///
    /// * `module` - Module path (e.g., `"myapp::http::server"`)
    /// * `level` - The level of the log message
    ///
    /// # Returns
    ///
    /// `true` if the message should be logged
    pub fn should_log(&self, module: &str, level: LogLevel) -> bool {
        match self.get_effective_level(module) {
            Some(effective_level) => effective_level.should_log(level),
            None => true, // No override = defer to global level
        }
    }

    /// Clear all module overrides
    pub fn clear(&mut self) {
        self.module_levels.clear();
    }

    /// Get the number of module overrides
    pub fn len(&self) -> usize {
        self.module_levels.len()
    }

    /// Check if the filter is empty (no overrides)
    pub fn is_empty(&self) -> bool {
        self.module_levels.is_empty()
    }
}
