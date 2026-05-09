# rslog - 轻量级Rust日志库

一个完全使用标准库构建的零依赖轻量级Rust日志库。

## 特性

- **零依赖**: 完全使用Rust标准库实现
- **多日志级别**: Debug, Info, Warn, Error, Critical
- **多输出目标**: 控制台和文件同时输出
- **异步写入**: 后台线程进行非阻塞文件写入
- **日志轮转**: 支持基于大小和时间的轮转
- **多输出格式**: 文本、JSON和自定义格式
- **可配置**: 灵活的配置选项
- **颜色支持**: 控制台输出支持ANSI颜色

## 快速开始

将 `rslog` 添加到你的 `Cargo.toml`:

```toml
[dependencies]
rslog = "0.1.1"
```

### 基本使用

```rust
use rslog::{Logger, LogLevel};

fn main() {
    // 获取日志记录器实例
    let logger = Logger::get_instance();
    
    // 记录不同级别的日志
    logger.debug("调试信息");
    logger.info("应用程序启动");
    logger.warn("内存不足警告");
    logger.error("连接失败");
    logger.critical("系统关闭");
}
```

### 自定义配置

```rust
use rslog::{Logger, LogLevel, ConfigBuilder, OutputFormat};

fn main() {
    // 构建自定义配置
    let config = ConfigBuilder::new()
        .log_dir("logs")
        .file_prefix("app_")
        .level(LogLevel::Info)
        .output_format(OutputFormat::Json)
        .build();
    
    // 使用自定义配置初始化日志记录器
    Logger::init_with_config(config);
    let logger = Logger::get_instance();
    logger.info("你好，世界！");
}
```

## 日志轮转

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

## 输出格式

### 文本格式（默认）
```
[2026-05-09 10:30:45.123456789] [INFO] 应用程序启动
```

### JSON格式
```json
{"time":"2026-05-09 10:30:45.123456789","level":"INFO","message":"应用程序启动"}
```

### 自定义格式
```rust
use rslog::{ConfigBuilder, OutputFormat};

let config = ConfigBuilder::new()
    .output_format(OutputFormat::Custom("%D %T [%P] %m".to_string()))
    .build();
```

#### 自定义格式占位符

| 占位符 | 描述 | 示例 |
|--------|------|------|
| `%d` | 完整时间戳 | `2026-05-09 10:30:45.123456789` |
| `%D` | 日期 | `2026-05-09` |
| `%T` | 时间 | `10:30:45.123456789` |
| `%p` | 完整日志级别 | `DEBUG`, `INFO`, `WARN`, `ERROR`, `CRITICAL` |
| `%P` | 简短日志级别 | `D`, `I`, `W`, `E`, `C` |
| `%m` | 日志消息 | 用户提供的消息 |
| `%n` | 换行 | `\n` |

## 颜色支持

rslog支持ANSI颜色代码，使控制台输出中的日志级别易于区分。

### 默认颜色方案

| 日志级别 | 默认颜色 | ANSI代码 |
|----------|----------|----------|
| `Debug` | 青色 | `\x1b[36m` |
| `Info` | 绿色 | `\x1b[32m` |
| `Warn` | 黄色 | `\x1b[33m` |
| `Error` | 红色 | `\x1b[31m` |
| `Critical` | 亮红色 | `\x1b[91m` |

### 自定义颜色配置

```rust
use rslog::{ConfigBuilder, LogColorScheme, Color};

// 创建自定义颜色方案
let custom_scheme = LogColorScheme::new(
    Color::BrightCyan,    // Debug
    Color::BrightGreen,   // Info
    Color::BrightYellow,  // Warn
    Color::BrightRed,     // Error
    Color::Magenta,       // Critical
);

let config = ConfigBuilder::new()
    .color_scheme(custom_scheme)
    .console_colors(true)  // 启用颜色
    .build();
```

### 运行时颜色控制

```rust
use rslog::Logger;

let logger = Logger::get_instance();

// 运行时启用/禁用颜色
logger.set_console_colors(true);   // 启用颜色
logger.set_console_colors(false);  // 禁用颜色
```

### 注意事项

- 颜色仅应用于控制台输出，文件输出保持纯文本
- 可以通过配置全局禁用颜色
- 自动检测终端支持（基本终端检测）
- 在非终端环境中使用 `logger.set_console_colors(false)`

## 配置选项

| 选项 | 类型 | 默认值 | 描述 |
|------|------|--------|------|
| `log_dir` | String | `"logs"` | 日志文件目录 |
| `file_prefix` | String | `"serial_"` | 日志文件前缀 |
| `file_extension` | String | `".log"` | 文件扩展名 |
| `console_enabled` | bool | `true` | 启用控制台输出 |
| `level` | LogLevel | `Debug` | 最低日志级别 |
| `output_format` | OutputFormat | `Text` | 输出格式 |
| `rotation` | Option\<RotatorConfig\> | 默认 | 日志轮转配置 |
| `console_colors` | bool | `true` | 在控制台输出中启用ANSI颜色 |
| `color_scheme` | LogColorScheme | 默认 | 日志级别的颜色方案 |

## 日志级别

| 级别 | 优先级 | 描述 |
|------|--------|------|
| `Debug` | 1 | 详细的调试信息 |
| `Info` | 2 | 一般信息 |
| `Warn` | 3 | 潜在问题 |
| `Error` | 4 | 可恢复的错误 |
| `Critical` | 5 | 不可恢复的错误 |

## 示例

运行基本示例：

```bash
cargo run --example test
```

运行颜色示例：

```bash
cargo run --example color_test
```

## 文档

生成文档：

```bash
cargo doc --open
```

在线文档：https://docs.rs/rslog

## 许可证

MIT 许可证

## 贡献

欢迎贡献！请随时提交问题和拉取请求。

## 英文文档

查看英文文档：[../../README.md](../../README.md)