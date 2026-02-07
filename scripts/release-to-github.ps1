# GitHub Release Script
# Purpose: Commit code, create tag, trigger GitHub Actions

param(
    [Parameter(Mandatory=$false)]
    [string]$Version = "",
    
    [Parameter(Mandatory=$false)]
    [string]$Message = "Release new version"
)

$ErrorActionPreference = "Stop"

# Color output
function Write-Step {
    param([string]$Message)
    Write-Host "`n[$([DateTime]::Now.ToString('HH:mm:ss'))] " -ForegroundColor Gray -NoNewline
    Write-Host $Message -ForegroundColor Cyan
}

function Write-Success {
    param([string]$Message)
    Write-Host "  [OK] " -ForegroundColor Green -NoNewline
    Write-Host $Message -ForegroundColor White
}

function Write-Error-Custom {
    param([string]$Message)
    Write-Host "  [ERROR] " -ForegroundColor Red -NoNewline
    Write-Host $Message -ForegroundColor Red
}

# Check Git repo
if (-not (Test-Path ".git")) {
    Write-Error-Custom "Current directory is not a Git repository"
    exit 1
}

# Get version
if ([string]::IsNullOrEmpty($Version)) {
    # Use a simple text search to get version, avoiding JSON parsing issues with encoding
    $tauriConf = Get-Content "src-tauri/tauri.conf.json" -Raw -Encoding UTF8
    $versionMatch = [regex]::Match($tauriConf, '"version"\s*:\s*"([^"]+)"')
    if ($versionMatch.Success) {
        $Version = $versionMatch.Groups[1].Value
        Write-Step "Get version from tauri.conf.json: v$Version"
    } else {
        Write-Error-Custom "Could not find version in tauri.conf.json"
        exit 1
    }
} else {
    Write-Step "Using specified version: v$Version"
}

# Check for uncommitted changes
Write-Step "Checking Git status..."
$gitStatus = git status --porcelain
if ($gitStatus) {
    Write-Host "`nUncommitted changes:" -ForegroundColor Yellow
    git status --short
    Write-Host ""
    
    $response = Read-Host "Commit these changes? (y/n)"
    if ($response -eq 'y' -or $response -eq 'Y') {
        git add .
        git commit -m "$Message v$Version"
        Write-Success "Changes committed"
    } else {
        Write-Error-Custom "Please commit changes before release"
        exit 1
    }
} else {
    Write-Success "Working directory clean"
}

# Check if tag exists
$existingTag = git tag -l "v$Version"
if ($existingTag) {
    Write-Host ""
    Write-Host "  Warning: Tag v$Version already exists" -ForegroundColor Yellow
    $response = Read-Host "Delete old tag and recreate? (y/n)"
    if ($response -eq 'y' -or $response -eq 'Y') {
        git tag -d "v$Version"
        git push origin --delete "v$Version" 2>$null
        Write-Success "Old tag deleted"
    } else {
        Write-Error-Custom "Release cancelled"
        exit 1
    }
}

# Create tag
Write-Step "Creating Git Tag: v$Version"

# Verify remote tag does not exist
$existingRemoteTag = git ls-remote --tags origin "v$Version" 2>$null
if ($existingRemoteTag) {
    Write-Host "  Warning: Remote repository already has tag v$Version" -ForegroundColor Yellow
    Write-Host "  Please delete remote tag or use a new version number" -ForegroundColor Yellow
    exit 1
}

git tag -a "v$Version" -m "Release v$Version"
Write-Success "Tag created successfully"

# Push to remote
Write-Step "Pushing to GitHub..."

# Push main branch
$pushResult = git push origin main 2>&1
if ($LASTEXITCODE -ne 0) {
    Write-Error-Custom "Push main branch failed: $pushResult"
    exit 1
}

# Push tag
$tagPushResult = git push origin "v$Version" 2>&1
if ($LASTEXITCODE -ne 0) {
    Write-Error-Custom "Push tag failed: $tagPushResult"
    exit 1
}

Write-Success "Push successful"

# Complete
Write-Host ""
Write-Host "===========================================" -ForegroundColor Green
Write-Host "  Release Process Started!" -ForegroundColor Green
Write-Host "===========================================" -ForegroundColor Green
Write-Host ""
Write-Host "  Version: " -NoNewline; Write-Host "v$Version" -ForegroundColor Yellow
Write-Host ""
Write-Host "  Next Steps:" -ForegroundColor Cyan
Write-Host "     1. Check GitHub Actions for build progress" -ForegroundColor Gray
Write-Host "        https://github.com/your-username/browser-manager/actions" -ForegroundColor Gray
Write-Host ""
Write-Host "     2. After build completes, check releases page" -ForegroundColor Gray
Write-Host "        https://github.com/your-username/browser-manager/releases" -ForegroundColor Gray
Write-Host ""
Write-Host "     3. Update backend API version info and download URLs" -ForegroundColor Gray
Write-Host ""