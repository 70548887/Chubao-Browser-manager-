# GitHub 自动化发布指南

## 🚀 快速开始

### 方式一：一键发布（推荐）

```bash
# 使用当前版本号（从 tauri.conf.json 读取）
npm run release:github

# 或指定版本号
.\scripts\release-to-github.ps1 -Version "0.3.0"

# 或指定提交信息
.\scripts\release-to-github.ps1 -Version "0.3.0" -Message "重大更新：新增 XXX 功能"
```

### 方式二：手动发布

```bash
# 1. 提交代码
git add .
git commit -m "发布 v0.2.0"

# 2. 创建 tag
git tag -a v0.2.0 -m "Release v0.2.0"

# 3. 推送到 GitHub
git push origin main
git push origin v0.2.0
```

---

## 📋 自动化流程说明

### 触发条件

推送 tag 时自动触发构建（格式：`v*`，如 `v0.2.0`、`v0.3.0`）

### 构建内容

GitHub Actions 会自动执行：

1. ✅ **构建 64 位版本**
   - `browser-manager_0.2.0_x64.msi`
   - `browser-manager_0.2.0_x64_setup.exe`
   - `kernel_0.2.0_windows_x64.zip`

2. ✅ **构建 32 位版本**
   - `browser-manager_0.2.0_x86.msi`
   - `browser-manager_0.2.0_x86_setup.exe`
   - `kernel_0.2.0_windows_x86.zip`

3. ✅ **创建 GitHub Release**
   - 自动上传所有安装包
   - 可在 Releases 页面下载

### 查看构建进度

1. 访问 Actions 页面：
   ```
   https://github.com/你的用户名/browser-manager/actions
   ```

2. 点击最新的 "Release Build" 工作流

3. 等待构建完成（约 10-15 分钟）

---

## 🔧 首次使用配置

### 1. 初始化 Git 仓库（如果还没有）

```bash
git init
git remote add origin https://github.com/你的用户名/browser-manager.git
```

### 2. 首次提交

```bash
git add .
git commit -m "Initial commit"
git branch -M main
git push -u origin main
```

### 3. 确保 Actions 已启用

在 GitHub 仓库设置中：
1. 进入 `Settings` → `Actions` → `General`
2. 确保 "Allow all actions and reusable workflows" 已选中

---

## 📦 发布后操作

### 1. 下载构建产物

构建完成后，在 GitHub Releases 页面下载所有文件：

```
https://github.com/你的用户名/browser-manager/releases
```

### 2. 更新后端 API

使用 GitHub Releases 的下载地址更新后端数据库：

```sql
INSERT INTO launcher_versions (
    version, 
    platform, 
    arch, 
    download_sources
) VALUES (
    '0.2.0',
    'windows',
    'x86_64',
    '[
        {
            "id": 1,
            "name": "GitHub Releases（推荐）",
            "url": "https://github.com/你的用户名/browser-manager/releases/download/v0.2.0/browser-manager_0.2.0_x64_setup.exe",
            "priority": 1,
            "region": "GLOBAL"
        }
    ]'
);
```

### 3. 计算文件哈希（用于更新验证）

下载文件后，计算 SHA256：

```powershell
Get-FileHash -Path "browser-manager_0.2.0_x64_setup.exe" -Algorithm SHA256
```

将哈希值更新到数据库的 `file_hash` 字段。

---

## 🛠️ 常见问题

### Q: 构建失败怎么办？

A: 
1. 查看 Actions 日志找出错误原因
2. 修复问题后，删除 tag 重新发布：
   ```bash
   git tag -d v0.2.0
   git push origin --delete v0.2.0
   npm run release:github
   ```

### Q: 如何编辑 Release 说明？

A: 
1. 进入 GitHub Releases 页面
2. 点击 Release 右侧的 "Edit" 按钮
3. 编辑 Release 说明和更新日志

### Q: 如何删除错误的 Release？

A:
1. 进入 Releases 页面
2. 点击对应 Release 的 "Delete" 按钮
3. 删除对应的 tag：
   ```bash
   git tag -d v0.2.0
   git push origin --delete v0.2.0
   ```

### Q: 为什么没有生成内核包？

A: 确保 `resources/kernel/win32` 目录存在且包含浏览器内核文件。

---

## 📊 工作流配置说明

GitHub Actions 工作流位于：`.github/workflows/release.yml`

主要配置：

```yaml
on:
  push:
    tags:
      - 'v*'  # 推送 v 开头的 tag 触发
```

如需修改构建行为，编辑该文件即可。

---

## 🎯 最佳实践

### 发布前检查清单

- [ ] 更新版本号（package.json、tauri.conf.json、Cargo.toml）
- [ ] 编写更新日志
- [ ] 本地测试应用功能
- [ ] 确保所有更改已提交
- [ ] 确保浏览器内核文件存在

### 版本号规范

遵循语义化版本 (Semantic Versioning)：

- **主版本号**：重大更新，不兼容的 API 变更
- **次版本号**：新功能，向后兼容
- **修订号**：问题修复

例如：`0.2.0` → `0.3.0` → `1.0.0`

---

## 🔗 相关链接

- [应用发布流程](./docs/应用发布流程.md) - 完整发布流程
- [GitHub Actions 文档](https://docs.github.com/actions)
- [Tauri 打包指南](https://tauri.app/v2/guides/building/)
