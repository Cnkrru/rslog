#!/bin/bash
# 文档更新脚本

echo "=== 更新 rslog 文档 ==="

# 1. 生成API文档
echo "生成API文档..."
cargo doc --no-deps

# 2. 检查文档警告
echo "检查文档警告..."
cargo doc --no-deps 2>&1 | grep -i "warning\|error" || true

# 3. 同步版本号到中文文档
VERSION=$(grep '^version =' Cargo.toml | cut -d'"' -f2)
echo "当前版本: $VERSION"

# 4. 更新中文文档中的版本号
if [ -f docs/zh-CN/README.md ]; then
    sed -i "s/rslog = \"[0-9]*\.[0-9]*\.[0-9]*\"/rslog = \"$VERSION\"/g" docs/zh-CN/README.md
    echo "已更新中文文档版本号"
fi

# 5. 验证文档
echo "验证文档结构..."
if [ -f docs/zh-CN/README.md ]; then
    echo "✓ 中文README存在"
else
    echo "✗ 中文README缺失"
fi

if [ -f docs/zh-CN/API_GUIDE.md ]; then
    echo "✓ 中文API指南存在"
else
    echo "✗ 中文API指南缺失"
fi

if [ -f docs/zh-CN/INDEX.md ]; then
    echo "✓ 中文索引存在"
else
    echo "✗ 中文索引缺失"
fi

echo "=== 文档更新完成 ==="
echo ""
echo "下一步:"
echo "1. 查看文档: cargo doc --open"
echo "2. 提交更改: git add docs/ && git commit -m '更新文档'"
echo "3. 发布新版本: cargo publish"