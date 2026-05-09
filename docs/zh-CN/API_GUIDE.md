# rslog API 使用指南

## 核心组件

### 1. Logger - 日志记录器

`Logger` 是单例模式的日志记录器，负责记录和输出日志消息。

```rust
use rslog::{Logger, LogLevel};

// 获取日志记录器实例
let logger = Logger::get_instance();

// 记录不同级别的日志
logger.debug("调试信息");
logger.info("一般信息");
logger.warn("警告信息");
logger.error("错误信息");
logger.critical("严重错误");

// 设置日志级别
logger.set_level(LogLevel::Warn);

// 启用/禁用控制台输出
logger.set_console_enabled(true);

// 启用/禁用控制台颜色
logger.set_console_colors(true);
```

### 2. ConfigBuilder - 配置构建器

使用构建器模式创建配置。

```rust
use rslog::{ConfigBuilder, LogLevel, OutputFormat, RotationStrategy, RotatorConfig};

let config = ConfigBuilder::new()
    .log_dir("my_logs")                    // 日志目录
    .file_prefix("app_")                   // 文件前缀
    .file_extension(".log")                // 文件扩展名
    .console_enabled(true)                 // 启用控制台输出
    .level(LogLevel::Info)                 // 日志级别
    .output_format(OutputFormat::Json)     // 输出格式
    .console_colors(true)                  // 启用颜色
    .rotation(RotatorConfig {              // 轮转配置
        strategy: RotationStrategy::SizeBased(10 * 1024 * 1024),
        max_files: 5,
        compress_old_files: false,
    })
    .build();
```

### 3. LogLevel - 日志级别

```rust
use rslog::LogLevel;
use std::str::FromStr;

// 创建日志级别
let level = LogLevel::Info;

// 从字符串解析
let level_from_str = LogLevel::from_str("warn").unwrap();

// 检查是否应该记录
if level.should_log(LogLevel::Debug) {
    println!("会记录Debug级别的日志");
}

// 获取简短名称
println!("简短名称: {}", level.short_name()); // 输出: I
```

### 4. Color - 颜色支持

```rust
use rslog::{Color, LogColorScheme};

// 基本颜色使用
let colored_text = Color::colorize("红色文字", Color::Red);
println!("{}", colored_text);

// 背景颜色
let bg_text = Color::colorize_bg("绿色背景", Color::Green);

// 完整颜色（前景+背景）
let full_text = Color::colorize_full("蓝字白底", Color::Blue, Color::White);

// 创建自定义颜色方案
let scheme = LogColorScheme::new(
    Color::Cyan,        // Debug
    Color::Green,       // Info
    Color::Yellow,      // Warn
    Color::Red,         // Error
    Color::BrightRed,   // Critical
);
```

## 高级用法

### 异步写入

日志库使用后台线程进行文件写入，不会阻塞主线程。

```rust
use rslog::Logger;

let logger = Logger::get_instance();

// 这些调用会立即返回，写入在后台进行
for i in 0..1000 {
    logger.info(&format!("消息 {}", i));
}

// 主线程继续执行其他任务
println!("日志正在后台写入...");
```

### 自定义格式化

```rust
use rslog::{ConfigBuilder, OutputFormat};

// 自定义格式
let config = ConfigBuilder::new()
    .output_format(OutputFormat::Custom("%D %T | %P | %m".to_string()))
    .build();

// 输出示例: 2026-05-09 10:30:45.123456789 | I | 应用程序启动
```

### 运行时配置更改

```rust
use rslog::Logger;

let logger = Logger::get_instance();

// 根据环境动态调整
if is_production() {
    logger.set_level(LogLevel::Warn);
    logger.set_console_colors(false);
} else {
    logger.set_level(LogLevel::Debug);
    logger.set_console_colors(true);
}
```

## 最佳实践

### 1. 应用程序启动时初始化

```rust
use rslog::{Logger, ConfigBuilder, LogLevel};

fn main() {
    // 在应用程序启动时初始化日志
    let config = ConfigBuilder::new()
        .log_dir("logs")
        .level(if cfg!(debug_assertions) {
            LogLevel::Debug
        } else {
            LogLevel::Info
        })
        .build();
    
    Logger::init_with_config(config);
    
    // 现在可以安全地使用日志
    let logger = Logger::get_instance();
    logger.info("应用程序启动完成");
    
    // ... 应用程序逻辑
}
```

### 2. 模块化日志记录

```rust
use rslog::Logger;

pub struct UserService {
    logger: std::sync::Arc<rslog::Logger>,
}

impl UserService {
    pub fn new() -> Self {
        UserService {
            logger: Logger::get_instance(),
        }
    }
    
    pub fn create_user(&self, username: &str) -> Result<(), String> {
        self.logger.info(&format!("创建用户: {}", username));
        
        // ... 业务逻辑
        
        if success {
            self.logger.info(&format!("用户 {} 创建成功", username));
            Ok(())
        } else {
            self.logger.error(&format!("用户 {} 创建失败", username));
            Err("创建失败".to_string())
        }
    }
}
```

### 3. 错误处理与日志

```rust
use rslog::Logger;

fn process_data(data: &str) -> Result<(), Box<dyn std::error::Error>> {
    let logger = Logger::get_instance();
    
    logger.debug(&format!("开始处理数据: {}", data));
    
    match parse_data(data) {
        Ok(result) => {
            logger.info("数据解析成功");
            Ok(())
        }
        Err(e) => {
            logger.error(&format!("数据解析失败: {}", e));
            Err(e)
        }
    }
}
```

## 故障排除

### 1. 日志文件未创建
- 检查目录权限
- 确保 `log_dir` 配置正确
- 检查磁盘空间

### 2. 控制台无输出
- 检查 `console_enabled` 配置
- 确保日志级别足够低
- 检查是否调用了 `set_console_enabled(false)`

### 3. 颜色不显示
- 检查终端是否支持ANSI颜色
- 确保 `console_colors` 为 true
- 在非终端环境中禁用颜色

### 4. 性能问题
- 减少Debug级别日志的数量
- 考虑增加批量写入大小
- 检查磁盘I/O性能

## 更多资源

- [GitHub仓库](https://github.com/Cnkrru/rslog)
- [英文文档](../README.md)
- [在线API文档](https://docs.rs/rslog)
- [示例代码](../examples/)