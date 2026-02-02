# 开发环境启动脚本
# 设置 DATABASE_URL 环境变量并启动 Tauri 开发服务器

$env:DATABASE_URL = "sqlite:///$env:LOCALAPPDATA/com.browser-manager.dev/browser-manager.db"

Write-Host "DATABASE_URL 已设置: $env:DATABASE_URL" -ForegroundColor Green

npm run tauri dev
