import { defineConfig } from 'vitepress'

export default defineConfig({
  // 站点head基础配置
  lang: 'zh-CN',
  title: "rslog - 轻量级Rust日志库",
  description: "一个完全使用标准库构建的零依赖轻量级Rust日志库",
  // 禁用死链检查
  ignoreDeadLinks: true,
  // 构建输出目录
  outDir: '../dist',
  
  // 主题配置
  themeConfig: {
    
    // logo以及标题
    logo: '/logo.png',
    siteTitle: 'rslog - Rust日志库',

    // 页眉导航栏
    nav: [
      { text: '首页', link: '/' },
      { text: '中文文档', link: '/zh-CN/' },
      { text: '英文文档', link: '/README.md' },
      { text: 'GitHub', link: 'https://github.com/Cnkrru/rslog' },
    ],

    // 侧边栏导航栏
    sidebar: {
      '/zh-CN/': [
        { text: '中文文档首页', link: '/zh-CN/' },
        { text: '快速开始', link: '/zh-CN/README.md' },
        { text: 'API使用指南', link: '/zh-CN/API_GUIDE.md' },
        { text: '文档索引', link: '/zh-CN/INDEX.md' },
      ],
      '/': [
        { text: '首页', link: '/' },
        { text: '英文文档', link: '/README.md' },
      ]
    },

    // 侧边栏位置
    aside: 'right',

    // 侧边栏大纲
    outline: 2,

    // 社交链接
    socialLinks: [
      { icon: 'github', link: 'https://github.com' }
    ],

    // 页脚信息
    footer: {
      message: '基于 VitePress 构建的 rslog 文档站点',
      copyright: '© 2026 Cnkrru | rslog - 轻量级Rust日志库'
    },

    // 编辑链接
    editLink: {
      pattern: 'https://github.com/Cnkrru/rslog/edit/main/docs/:path',
      text: '在GitHub上编辑此页面'
    },

    // 最后更新时间
    lastUpdated: {
      text: 'Updated at',
      formatOptions: {
        dateStyle: 'full',
        timeStyle: 'medium'
      }
    },

    // 本地搜索配置（替代Algolia）
    search: {
      provider: 'local',
      options: {
        locales: {
          'zh-CN': {
            translations: {
              button: {
                buttonText: '搜索文档',
                buttonAriaLabel: '搜索文档'
              },
              modal: {
                noResultsText: '没有找到相关结果',
                resetButtonTitle: '清除查询',
                footer: {
                  selectText: '选择',
                  navigateText: '切换'
                }
              }
            }
          }
        }
      }
    },

    // Carbon Ads广告配置
    // carbonAds: {
    //   code: 'your-carbon-code',
    //   placement: 'your-carbon-placement'
    // },

    // 文档页脚信息
    docFooter: {
      prev: '上一页',
      next: '下一页'
    },

    darkModeSwitchLabel: '切换到深色模式',
    lightModeSwitchTitle: '切换到浅色模式',
    darkModeSwitchTitle: '切换到深色模式',
    sidebarMenuLabel: '侧边栏菜单',
    returnToTopLabel: '返回顶部',
    langMenuLabel: '语言',
    externalLinkIcon: true
  }
})
