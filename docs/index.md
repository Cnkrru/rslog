---
# https://vitepress.dev/reference/default-theme-home-page
layout: home

hero:
  name: "rslog"
  text: "轻量级Rust日志库"
  tagline: 零依赖 · 高性能 · 易配置
  actions:
    - theme: brand
      text: 快速开始
      link: /zh-CN/README.md
    - theme: alt
      text: 查看GitHub
      link: https://github.com/Cnkrru/rslog

features:
  - title: 零依赖
    details: 完全使用Rust标准库实现，无需任何外部依赖
  - title: 多输出格式
    details: 支持文本、JSON和自定义格式，满足不同场景需求
  - title: 异步写入
    details: 后台线程进行非阻塞文件写入，不影响主程序性能
  - title: 日志轮转
    details: 支持基于大小和时间的轮转，自动管理日志文件
  - title: 颜色支持
    details: 控制台输出支持ANSI颜色，日志级别一目了然
  - title: 灵活配置
    details: 丰富的配置选项，可根据需求定制日志行为
---

