# 文档更新脚本 (PowerShell版本)

Write-Host "=== 更新 rslog 文档 ===" -ForegroundColor Green

# 1. 生成API文档
Write-Host "生成API文档..." -ForegroundColor Yellow
cargo doc --no-deps

# 2. 检查文档警告
Write-Host "检查文档警告..." -ForegroundColor Yellow
$output = cargo doc --no-deps 2>&1
$warnings = $output | Select-String -Pattern "warning|error" -CaseSensitive:$false
if ($warnings) {
    Write-Host "发现警告/错误:" -ForegroundColor Red
    $warnings
} else {
    Write-Host "✓ 没有发现警告" -ForegroundColor Green
}

# 3. 获取版本号
$versionLine = Get-Content Cargo.toml | Where-Object { $_ -match '^version =' }
$version = $versionLine -replace '.*"([^"]+)".*', '$1'
Write-Host "当前版本: $version" -ForegroundColor Cyan

# 4. 更新中文文档中的版本号
if (Test-Path "docs/zh-CN/README.md") {
    $content = Get-Content "docs/zh-CN/README.md" -Raw
    $updatedContent = $content -replace 'rslog = "[0-9]+\.[0-9]+\.[0-9]+"', "rslog = `"$version`""
    Set-Content "docs/zh-CN/README.md" $updatedContent
    Write-Host "✓ 已更新中文文档版本号" -ForegroundColor Green
}

# 5. 验证文档结构
Write-Host "验证文档结构..." -ForegroundColor Yellow

$filesToCheck = @(
    @{Path="docs/zh-CN/README.md"; Name="中文README"},
    @{Path="docs/zh-CN/API_GUIDE.md"; Name="中文API指南"},
    @{Path="docs/zh-CN/INDEX.md"; Name="中文索引"}
)

foreach ($file in $filesToCheck) {
    if (Test-Path $file.Path) {
        Write-Host "✓ $($file.Name)存在" -ForegroundColor Green
    } else {
        Write-Host "✗ $($file.Name)缺失" -ForegroundColor Red
    }
}

Write-Host "=== 文档更新完成 ===" -ForegroundColor Green
Write-Host ""
Write-Host "下一步:" -ForegroundColor Cyan
Write-Host "1. 查看文档: cargo doc --open"
Write-Host "2. 提交更改: git add docs/ && git commit -m '更新文档'"
Write-Host "3. 发布新版本: cargo publish"