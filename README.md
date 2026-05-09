# rslog

A lightweight, zero-dependency logging library for Rust, built entirely using the standard library.

## Features

- **Zero Dependencies**: Built entirely using Rust's standard library
- **Multiple Log Levels**: Debug, Info, Warn, Error, Critical
- **Multiple Output Targets**: Console and file output
- **Async Writing**: Background thread for non-blocking file writes
- **Log Rotation**: Size-based and time-based rotation
- **Multiple Output Formats**: Text, JSON, and custom patterns
- **Configurable**: Flexible configuration options

## Quick Start

Add `rslog` to your `Cargo.toml`:

```toml
[dependencies]
rslog = { path = "path/to/rslog" }
```

### Basic Usage

```rust
use rslog::{Logger, LogLevel};

fn main() {
    // Get the logger instance
    let logger = Logger::get_instance();
    
    // Log messages at different levels
    logger.debug("Debug message");
    logger.info("Application started");
    logger.warn("Low memory warning");
    logger.error("Connection failed");
    logger.critical("System shutdown");
}
```

### Custom Configuration

```rust
use rslog::{Logger, LogLevel, ConfigBuilder, OutputFormat};

fn main() {
    // Build a custom configuration
    let config = ConfigBuilder::new()
        .log_dir("logs")
        .file_prefix("app_")
        .level(LogLevel::Info)
        .output_format(OutputFormat::Json)
        .build();
    
    // Initialize logger with custom config
    Logger::init_with_config(config);
    
    let logger = Logger::get_instance();
    logger.info("Hello, world!");
}
```

## Log Rotation

```rust
use rslog::{Logger, ConfigBuilder, RotationStrategy, RotatorConfig};

fn main() {
    let config = ConfigBuilder::new()
        .rotation(RotatorConfig {
            strategy: RotationStrategy::SizeBased(10 * 1024 * 1024), // 10MB
            max_files: 5,
            compress_old_files: false,
        })
        .build();
    
    Logger::init_with_config(config);
    let logger = Logger::get_instance();
}
```

## Output Formats

### Text Format (Default)
```
[2026-05-09 10:30:45.123456789] [INFO] Application started
```

### JSON Format
```json
{"time":"2026-05-09 10:30:45.123456789","level":"INFO","message":"Application started"}
```

### Custom Format
```rust
use rslog::{ConfigBuilder, OutputFormat};

let config = ConfigBuilder::new()
    .output_format(OutputFormat::Custom("%D %T [%P] %m".to_string()))
    .build();
```

#### Custom Format Placeholders

| Placeholder | Description | Example |
|-------------|-------------|---------|
| `%d` | Full timestamp | `2026-05-09 10:30:45.123456789` |
| `%D` | Date | `2026-05-09` |
| `%T` | Time | `10:30:45.123456789` |
| `%p` | Full log level | `DEBUG`, `INFO`, `WARN`, `ERROR`, `CRITICAL` |
| `%P` | Short log level | `D`, `I`, `W`, `E`, `C` |
| `%m` | Log message | User-provided message |
| `%n` | Newline | `\n` |

## Configuration Options

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `log_dir` | String | `"logs"` | Directory for log files |
| `file_prefix` | String | `"serial_"` | Prefix for log file names |
| `file_extension` | String | `".log"` | File extension |
| `console_enabled` | bool | `true` | Enable console output |
| `level` | LogLevel | `Debug` | Minimum log level |
| `output_format` | OutputFormat | `Text` | Output format |
| `rotation` | Option\<RotatorConfig\> | Default | Log rotation config |

## Log Levels

| Level | Priority | Description |
|-------|----------|-------------|
| `Debug` | 1 | Detailed debug information |
| `Info` | 2 | General information |
| `Warn` | 3 | Potential issues |
| `Error` | 4 | Recoverable errors |
| `Critical` | 5 | Unrecoverable errors |

## Examples

Run the example:

```bash
cargo run --example test
```

## Documentation

Generate documentation:

```bash
cargo doc --open
```

## License

MIT License

## Contributing

Contributions are welcome! Please feel free to submit issues and pull requests.
