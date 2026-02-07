# 触宝指纹浏览器 - 生产环境打包脚本
# 用途：打包启动器安装包和内核压缩包，生成更新信息 JSON

param(
    [Parameter(Mandatory=$false)]
    [string]$Version = "",
    
    [Parameter(Mandatory=$false)]
    [string]$OutputDir = "release"
)

$ErrorActionPreference = "Stop"

# 颜色输出函数
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

# 获取版本号
if ([string]::IsNullOrEmpty($Version)) {
    $tauri_config = Get-Content "src-tauri/tauri.conf.json" | ConvertFrom-Json
    $Version = $tauri_config.version
    Write-Step "从 tauri.conf.json 读取版本号: $Version"
} else {
    Write-Step "使用指定版本号: $Version"
}

# 创建输出目录
$ReleaseDir = Join-Path $PSScriptRoot "..\$OutputDir\v$Version"
if (Test-Path $ReleaseDir) {
    Write-Step "清理已存在的输出目录..."
    Remove-Item -Path $ReleaseDir -Recurse -Force
}
New-Item -ItemType Directory -Path $ReleaseDir -Force | Out-Null
Write-Success "输出目录: $ReleaseDir"

# ============== 步骤 1: 打包前端 ==============
Write-Step "步骤 1/4: 构建前端资源..."
npm run build
if ($LASTEXITCODE -ne 0) {
    Write-Error-Custom "前端构建失败"
    exit 1
}
Write-Success "前端构建完成"

# ============== 步骤 2: 打包 Tauri 应用 ==============
Write-Step "步骤 2/4: 打包 Tauri 启动器..."
npm run tauri build
if ($LASTEXITCODE -ne 0) {
    Write-Error-Custom "Tauri 打包失败"
    exit 1
}
Write-Success "Tauri 打包完成"

# ============== 步骤 3: 复制安装包 ==============
Write-Step "步骤 3/4: 收集安装包文件..."

$BundleDir = "src-tauri\target\release\bundle"

# 复制 MSI 安装包
$MsiFiles = Get-ChildItem -Path "$BundleDir\msi" -Filter "*.msi" -ErrorAction SilentlyContinue
foreach ($file in $MsiFiles) {
    $destName = "browser-manager_${Version}_x64.msi"
    Copy-Item $file.FullName -Destination (Join-Path $ReleaseDir $destName)
    Write-Success "MSI: $destName"
}

# 复制 NSIS 安装包
$NsisFiles = Get-ChildItem -Path "$BundleDir\nsis" -Filter "*setup.exe" -ErrorAction SilentlyContinue
foreach ($file in $NsisFiles) {
    $destName = "browser-manager_${Version}_x64_setup.exe"
    Copy-Item $file.FullName -Destination (Join-Path $ReleaseDir $destName)
    Write-Success "NSIS: $destName"
}

# ============== 步骤 4: 打包内核 ==============
Write-Step "步骤 4/4: 打包浏览器内核..."

$KernelDir = "resources\kernel\win32"
if (Test-Path $KernelDir) {
    $KernelZip = Join-Path $ReleaseDir "kernel_${Version}_windows_x64.zip"
    
    # 使用 PowerShell 原生压缩
    Compress-Archive -Path "$KernelDir\*" -DestinationPath $KernelZip -Force
    Write-Success "内核包: kernel_${Version}_windows_x64.zip"
} else {
    Write-Error-Custom "内核目录不存在: $KernelDir"
}

# ============== 步骤 5: 计算文件哈希 ==============
Write-Step "步骤 5/5: 生成文件校验信息..."

$UpdateInfo = @{
    launcher = @{
        version = $Version
        platform = "windows"
        arch = "x86_64"
        release_date = (Get-Date -Format "yyyy-MM-dd")
        files = @()
    }
    kernel = @{
        version = $Version
        platform = "windows"
        arch = "x86_64"
        release_date = (Get-Date -Format "yyyy-MM-dd")
        files = @()
    }
}

# 计算启动器文件哈希
Get-ChildItem -Path $ReleaseDir -Filter "*.msi","*.exe" | ForEach-Object {
    $hash = (Get-FileHash -Path $_.FullName -Algorithm SHA256).Hash.ToLower()
    $size = $_.Length
    
    $UpdateInfo.launcher.files += @{
        name = $_.Name
        size = $size
        sha256 = $hash
        download_url = "https://cdn.yourdomain.com/releases/v$Version/$($_.Name)"
    }
    
    Write-Success "$($_.Name) - SHA256: $hash"
}

# 计算内核文件哈希
Get-ChildItem -Path $ReleaseDir -Filter "kernel_*.zip" | ForEach-Object {
    $hash = (Get-FileHash -Path $_.FullName -Algorithm SHA256).Hash.ToLower()
    $size = $_.Length
    
    $UpdateInfo.kernel.files += @{
        name = $_.Name
        size = $size
        sha256 = $hash
        download_url = "https://cdn.yourdomain.com/releases/v$Version/$($_.Name)"
    }
    
    Write-Success "$($_.Name) - SHA256: $hash"
}

# 保存更新信息到 JSON 文件
$UpdateInfoPath = Join-Path $ReleaseDir "update-info.json"
$UpdateInfo | ConvertTo-Json -Depth 10 | Set-Content -Path $UpdateInfoPath -Encoding UTF8
Write-Success "更新信息: update-info.json"

# ============== 完成 ==============
Write-Host "`n" -NoNewline
Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" -ForegroundColor Green
Write-Host "  🎉 打包完成！" -ForegroundColor Green
Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" -ForegroundColor Green
Write-Host ""
Write-Host "  版本: " -NoNewline; Write-Host "v$Version" -ForegroundColor Yellow
Write-Host "  输出: " -NoNewline; Write-Host $ReleaseDir -ForegroundColor Yellow
Write-Host ""
Write-Host "  📦 打包文件列表:" -ForegroundColor Cyan
Get-ChildItem -Path $ReleaseDir | ForEach-Object {
    $sizeKB = [math]::Round($_.Length / 1KB, 2)
    Write-Host "     • $($_.Name) " -NoNewline -ForegroundColor White
    Write-Host "($sizeKB KB)" -ForegroundColor Gray
}
Write-Host ""
Write-Host "  📋 下一步操作:" -ForegroundColor Cyan
Write-Host "     1. 检查 update-info.json 中的下载地址" -ForegroundColor Gray
Write-Host "     2. 上传文件到 CDN 或服务器" -ForegroundColor Gray
Write-Host "     3. 更新后端 API 的版本信息" -ForegroundColor Gray
Write-Host ""
