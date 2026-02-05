# Chromium Kernel Packaging Script
# Usage: .\package_kernel.ps1 -SourceDir "D:\chromium_src\chromium_139_src\src\out\Release" -OutputFile "chromium-kernel-win64.7z"

param(
    [string]$SourceDir = "D:\chromium_src\chromium_139_src\src\out\Release",
    [string]$OutputDir = "D:\browser-manager\dist",
    [string]$OutputFile = "chromium-kernel-win64-v146.zip"
)

$ErrorActionPreference = "Stop"

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "   Chromium Kernel Packaging Script" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan

# Check source directory
if (-not (Test-Path $SourceDir)) {
    Write-Error "Source directory not found: $SourceDir"
    exit 1
}

# Check chrome.exe exists
$chromeExe = Join-Path $SourceDir "chrome.exe"
if (-not (Test-Path $chromeExe)) {
    Write-Error "chrome.exe not found: $chromeExe"
    exit 1
}

# Create output directory
if (-not (Test-Path $OutputDir)) {
    New-Item -ItemType Directory -Path $OutputDir -Force | Out-Null
}

# Create temp directory
$TempDir = Join-Path $env:TEMP "chromium_kernel_pack_$(Get-Date -Format 'yyyyMMddHHmmss')"
Write-Host "`n[1/5] Creating temp directory: $TempDir" -ForegroundColor Yellow
New-Item -ItemType Directory -Path $TempDir -Force | Out-Null

# Define required files
$RequiredFiles = @(
    # Core executables
    "chrome.exe",
    "chrome.dll",
    "chrome_elf.dll",
    "chrome_proxy.exe",
    "chrome_pwa_launcher.exe",
    "chrome_wer.dll",
    "elevation_service.exe",
    "notification_helper.exe",
    
    # Critical V8 and snapshot files
    "snapshot_blob.bin",
    "v8_context_snapshot.bin",
    
    # Resources (critical)
    "chrome_100_percent.pak",
    "chrome_200_percent.pak", 
    "resources.pak",
    "icudtl.dat",
    
    # Graphics
    "libEGL.dll",
    "libGLESv2.dll",
    "d3dcompiler_47.dll",
    "dxcompiler.dll",
    "dxil.dll",
    "vk_swiftshader.dll",
    "vk_swiftshader_icd.json",
    "vulkan-1.dll",
    
    # ANGLE
    "libEGL.dll.pdb",
    "libGLESv2.dll.pdb",
    
    # Runtime (MSVC)
    "msvcp140.dll",
    "vcruntime140.dll",
    "vcruntime140_1.dll",
    "vccorlib140.dll",
    
    # MEI (Media Engagement Index)
    "MEIPreload",
    
    # Manifest (version-specific, may not exist)
    "146.0.7652.0.manifest",
    "139.0.0.0.manifest",
    "chrome.VisualElementsManifest.xml"
)

# Define required directories
$RequiredDirs = @(
    "locales"
)

# Copy files
Write-Host "`n[2/5] Copying core files..." -ForegroundColor Yellow
$copiedCount = 0
$totalSize = 0

foreach ($file in $RequiredFiles) {
    $srcPath = Join-Path $SourceDir $file
    if (Test-Path $srcPath) {
        $destPath = Join-Path $TempDir $file
        Copy-Item $srcPath $destPath -Force
        $fileSize = (Get-Item $srcPath).Length
        $totalSize += $fileSize
        $copiedCount++
        Write-Host "  + $file ($([math]::Round($fileSize/1MB, 2)) MB)" -ForegroundColor Green
    } else {
        Write-Host "  - $file (not found, skipped)" -ForegroundColor DarkGray
    }
}

# Copy directories
Write-Host "`n[3/5] Copying directories..." -ForegroundColor Yellow
foreach ($dir in $RequiredDirs) {
    $srcPath = Join-Path $SourceDir $dir
    if (Test-Path $srcPath) {
        $destPath = Join-Path $TempDir $dir
        Copy-Item $srcPath $destPath -Recurse -Force
        $dirSize = (Get-ChildItem $srcPath -Recurse -File | Measure-Object -Property Length -Sum).Sum
        $totalSize += $dirSize
        $fileCount = (Get-ChildItem $srcPath -Recurse -File).Count
        Write-Host "  + $dir/ ($fileCount files, $([math]::Round($dirSize/1MB, 2)) MB)" -ForegroundColor Green
    } else {
        Write-Host "  - $dir/ (not found, skipped)" -ForegroundColor DarkGray
    }
}

Write-Host "`n  Total: $copiedCount files, $([math]::Round($totalSize/1MB, 2)) MB" -ForegroundColor Cyan

# Create version info
Write-Host "`n[4/5] Creating version info..." -ForegroundColor Yellow
# Detect Chromium version from chrome.exe
$chromeExePath = Join-Path $TempDir "chrome.exe"
if (Test-Path $chromeExePath) {
    $chromeVersion = (Get-Item $chromeExePath).VersionInfo.ProductVersion
    if (-not $chromeVersion) {
        $chromeVersion = "146.0.0.0"
    }
} else {
    $chromeVersion = "146.0.0.0"
}

$versionInfo = @{
    version = $chromeVersion
    build_date = (Get-Date -Format "yyyy-MM-dd HH:mm:ss")
    platform = "win64"
    source = "chromium_139_src"
    files_count = $copiedCount
    total_size_bytes = $totalSize
}
$versionInfo | ConvertTo-Json | Set-Content (Join-Path $TempDir "kernel_version.json") -Encoding UTF8
Write-Host "  + kernel_version.json" -ForegroundColor Green

# Package to 7z
Write-Host "`n[5/5] Packaging to 7z..." -ForegroundColor Yellow
$OutputPath = Join-Path $OutputDir $OutputFile

# Check if 7z is available
$7zPath = $null
if (Get-Command "7z" -ErrorAction SilentlyContinue) {
    $7zPath = "7z"
} elseif (Test-Path "C:\Program Files\7-Zip\7z.exe") {
    $7zPath = "C:\Program Files\7-Zip\7z.exe"
} elseif (Test-Path "C:\Program Files (x86)\7-Zip\7z.exe") {
    $7zPath = "C:\Program Files (x86)\7-Zip\7z.exe"
}

if ($7zPath) {
    Write-Host "  Using 7-Zip (LZMA2)..." -ForegroundColor Gray
    & $7zPath a -t7z -mx=9 -mfb=64 -md=32m -ms=on $OutputPath "$TempDir\*" | Out-Null
    
    if ($LASTEXITCODE -eq 0) {
        $compressedSize = (Get-Item $OutputPath).Length
        $ratio = [math]::Round(($compressedSize / $totalSize) * 100, 1)
        Write-Host "`n========================================" -ForegroundColor Cyan
        Write-Host "  Packaging Complete!" -ForegroundColor Green
        Write-Host "  Output: $OutputPath" -ForegroundColor Cyan
        Write-Host "  Before: $([math]::Round($totalSize/1MB, 2)) MB" -ForegroundColor Gray
        Write-Host "  After: $([math]::Round($compressedSize/1MB, 2)) MB ($ratio%)" -ForegroundColor Green
        Write-Host "========================================" -ForegroundColor Cyan
    } else {
        Write-Error "7z compression failed"
    }
} else {
    Write-Host "  7-Zip not found, using ZIP format..." -ForegroundColor Yellow
    $OutputPath = $OutputPath -replace '\.7z$', '.zip'
    Compress-Archive -Path "$TempDir\*" -DestinationPath $OutputPath -Force
    
    $compressedSize = (Get-Item $OutputPath).Length
    $ratio = [math]::Round(($compressedSize / $totalSize) * 100, 1)
    Write-Host "`n========================================" -ForegroundColor Cyan
    Write-Host "  Packaging Complete! (ZIP format)" -ForegroundColor Green
    Write-Host "  Output: $OutputPath" -ForegroundColor Cyan
    Write-Host "  Before: $([math]::Round($totalSize/1MB, 2)) MB" -ForegroundColor Gray
    Write-Host "  After: $([math]::Round($compressedSize/1MB, 2)) MB ($ratio%)" -ForegroundColor Yellow
    Write-Host "  Tip: Install 7-Zip for better compression" -ForegroundColor DarkGray
    Write-Host "========================================" -ForegroundColor Cyan
}

# Cleanup temp directory
Write-Host "`nCleaning up temp files..." -ForegroundColor Gray
Remove-Item $TempDir -Recurse -Force

# Output next steps
Write-Host "`nNext Steps:" -ForegroundColor Yellow
Write-Host "  1. Upload the archive to GitHub Releases or cloud storage" -ForegroundColor White
Write-Host "  2. Get the download URL" -ForegroundColor White
Write-Host "  3. Configure the kernel download URL in app settings" -ForegroundColor White
Write-Host ""

return $OutputPath
