# rslog 中文文档索引

## 文档列表

### 入门指南
- [README.md](README.md) - 项目概述和快速开始
- [API使用指南](API_GUIDE.md) - 详细的API使用说明

### 核心概念
- **日志级别** - Debug, Info, Warn, Error, Critical
- **输出目标** - 控制台和文件输出
- **异步写入** - 后台线程非阻塞写入
- **日志轮转** - 基于大小和时间的轮转
- **颜色支持** - ANSI颜色代码

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

### 示例代码
查看项目中的示例目录：
- [基本示例](../../examples/test.rs)
- [颜色示例](../../examples/color_test.rs)

### 在线资源
- [GitHub仓库](https://github.com/Cnkrru/rslog)
- [crates.io页面](https://crates.io/crates/rslog)
- [docs.rs文档](https://docs.rs/rslog)

### 英文文档
- [英文README](../../README.md)

## 快速链接

```bash
# 运行示例
cargo run --example test
cargo run --example color_test

# 生成文档
cargo doc --open

# 发布新版本
cargo publish
```

## 支持与反馈

如有问题或建议，请：
1. 查看 [GitHub Issues](https://github.com/Cnkrru/rslog/issues)
2. 提交新的 Issue
3. 创建 Pull Request

## 版本历史

- **v0.1.0** - 初始版本，基本日志功能
- **v0.1.1** - 添加颜色支持，修复静态变量警告