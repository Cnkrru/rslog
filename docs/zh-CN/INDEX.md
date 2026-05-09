---
layout: doc
---

# rslog 中文文档

欢迎来到 rslog 中文文档！rslog 是一个完全使用 Rust 标准库构建的零依赖轻量级日志库。

## 文档导航

### 入门指南
- [快速开始](README.md) - 安装和基本使用
- [API使用指南](API_GUIDE.md) - 详细的API说明和示例

### 核心功能
- **日志级别** - Debug, Info, Warn, Error, Critical
- **输出目标** - 控制台和文件同时输出
- **异步写入** - 后台线程非阻塞写入
- **日志轮转** - 基于大小和时间的轮转
- **颜色支持** - ANSI颜色代码，控制台输出更清晰

### 配置选项
| 配置项 | 描述 | 默认值 |
|--------|------|--------|
| `log_dir` | 日志目录 | `"logs"` |
| `file_prefix` | 文件前缀 | `"serial_"` |
| `file_extension` | 文件扩展名 | `".log"` |
| `console_enabled` | 控制台输出 | `true` |
| `level` | 日志级别 | `Debug` |
| `output_format` | 输出格式 | `Text` |
| `console_colors` | 控制台颜色 | `true` |

## 快速示例

```rust
use rslog::{Logger, LogLevel};

fn main() {
    let logger = Logger::get_instance();
    logger.info("应用程序启动");
    logger.debug("调试信息");
    logger.warn("警告信息");
    logger.error("错误信息");
}
```

## 在线资源

- [GitHub仓库](https://github.com/Cnkrru/rslog)
- [crates.io页面](https://crates.io/crates/rslog)
- [docs.rs文档](https://docs.rs/rslog)

## 英文文档

查看英文文档：[../README.md](../README.md)

## 支持与反馈

如有问题或建议，请：
1. 查看 [GitHub Issues](https://github.com/Cnkrru/rslog/issues)
2. 提交新的 Issue
3. 创建 Pull Request