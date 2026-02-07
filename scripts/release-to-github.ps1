# GitHub 发布脚本
# 用途：提交代码、打 tag、触发 GitHub Actions 自动构建

param(
    [Parameter(Mandatory=$false)]
    [string]$Version = "",
    
    [Parameter(Mandatory=$false)]
    [string]$Message = "发布新版本"
)

$ErrorActionPreference = "Stop"

# 颜色输出
function Write-Step {
    param([string]$Message)
    Write-Host "`n[$([DateTime]::Now.ToString('HH:mm:ss'))] " -ForegroundColor Gray -NoNewline
    Write-Host $Message -ForegroundColor Cyan
}

function Write-Success {
    param([string]$Message)
    Write-Host "  ✓ " -ForegroundColor Green -NoNewline
    Write-Host $Message -ForegroundColor White
}

function Write-Error-Custom {
    param([string]$Message)
    Write-Host "  ✗ " -ForegroundColor Red -NoNewline
    Write-Host $Message -ForegroundColor Red
}

# 检查 Git 仓库
if (-not (Test-Path ".git")) {
    Write-Error-Custom "当前目录不是 Git 仓库"
    exit 1
}

# 获取版本号
if ([string]::IsNullOrEmpty($Version)) {
    $tauri_config = Get-Content "src-tauri/tauri.conf.json" | ConvertFrom-Json
    $Version = $tauri_config.version
    Write-Step "从 tauri.conf.json 读取版本号: v$Version"
} else {
    Write-Step "使用指定版本号: v$Version"
}

# 检查是否有未提交的更改
Write-Step "检查 Git 状态..."
$gitStatus = git status --porcelain
if ($gitStatus) {
    Write-Host "`n未提交的更改：" -ForegroundColor Yellow
    git status --short
    Write-Host ""
    
    $response = Read-Host "是否提交这些更改？(y/n)"
    if ($response -eq 'y' -or $response -eq 'Y') {
        git add .
        git commit -m "$Message v$Version"
        Write-Success "已提交更改"
    } else {
        Write-Error-Custom "请先提交更改后再发布"
        exit 1
    }
} else {
    Write-Success "工作区干净"
}

# 检查 tag 是否已存在
$existingTag = git tag -l "v$Version"
if ($existingTag) {
    Write-Host ""
    Write-Host "  警告: Tag v$Version 已存在" -ForegroundColor Yellow
    $response = Read-Host "是否删除旧 tag 并重新创建？(y/n)"
    if ($response -eq 'y' -or $response -eq 'Y') {
        git tag -d "v$Version"
        git push origin --delete "v$Version" 2>$null
        Write-Success "已删除旧 tag"
    } else {
        Write-Error-Custom "发布已取消"
        exit 1
    }
}

# 创建 tag
Write-Step "创建 Git Tag: v$Version"
git tag -a "v$Version" -m "Release v$Version"
Write-Success "Tag 创建成功"

# 推送到远程
Write-Step "推送到 GitHub..."
git push origin main
git push origin "v$Version"
Write-Success "推送成功"

# 完成
Write-Host ""
Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" -ForegroundColor Green
Write-Host "  🎉 发布流程已启动！" -ForegroundColor Green
Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" -ForegroundColor Green
Write-Host ""
Write-Host "  版本: " -NoNewline; Write-Host "v$Version" -ForegroundColor Yellow
Write-Host ""
Write-Host "  📋 后续步骤:" -ForegroundColor Cyan
Write-Host "     1. 访问 GitHub Actions 查看构建进度" -ForegroundColor Gray
Write-Host "        https://github.com/你的用户名/browser-manager/actions" -ForegroundColor Gray
Write-Host ""
Write-Host "     2. 构建完成后，在 Releases 页面查看发布" -ForegroundColor Gray
Write-Host "        https://github.com/你的用户名/browser-manager/releases" -ForegroundColor Gray
Write-Host ""
Write-Host "     3. 更新后端 API 的版本信息和下载地址" -ForegroundColor Gray
Write-Host ""
