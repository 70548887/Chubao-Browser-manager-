# Rust 工具链安装和编译指南

> 适用于 Windows 64-bit
> 更新时间：2026-01-27

## 一、安装 Rust 工具链

### 方法 1：使用 rustup-init.exe（推荐）

1. **下载安装程序**
   - 访问：https://rustup.rs/
   - 或直接下载：https://win.rustup.rs/x86_64
   - 文件名：`rustup-init.exe`

2. **运行安装程序**
   ```powershell
   # 下载后双击运行，或在 PowerShell 中执行
   .\rustup-init.exe
   ```

3. **安装选项**
   - 选择 `1) Proceed with installation (default)`
   - 等待安装完成（约 5-10 分钟）

4. **验证安装**
   ```powershell
   # 重新打开 PowerShell 或 CMD
   rustc --version
   cargo --version
   ```

   **预期输出：**
   ```
   rustc 1.75.0 (82e1608df 2023-12-21)
   cargo 1.75.0 (1d8b05cdd 2023-11-20)
   ```

### 方法 2：使用 WSL（可选）

如果您使用 WSL，可以在 WSL 终端中运行：
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

---

## 二、安装 C++ 构建工具（必需）

Rust 在 Windows 上需要 MSVC 工具链。

### 选项 1：安装 Visual Studio Build Tools

1. **下载**
   - 访问：https://visualstudio.microsoft.com/downloads/
   - 选择 "Build Tools for Visual Studio 2022"

2. **安装组件**
   - 勾选 "Desktop development with C++"
   - 或最小安装：
     - MSVC v143 - VS 2022 C++ x64/x86 build tools
     - Windows 10/11 SDK

### 选项 2：使用现有 Visual Studio

如果已安装 Visual Studio，确保包含 C++ 工作负载。

---

## 三、配置 SQLx（数据库）

### 1. 创建 .env 文件

在 `src-tauri/` 目录下创建 `.env` 文件：

```bash
cd f:/user/Desktop/PC端浏览器多开器/browser-manager/src-tauri
echo DATABASE_URL=sqlite://browser-manager.db > .env
```

### 2. 安装 SQLx CLI（可选）

```powershell
cargo install sqlx-cli --no-default-features --features sqlite
```

### 3. 运行数据库迁移

```powershell
cd src-tauri
cargo sqlx migrate run
```

---

## 四、编译项目

### 1. 首次编译（下载依赖）

```powershell
cd f:/user/Desktop/PC端浏览器多开器/browser-manager/src-tauri
cargo build
```

**预期时间：** 5-15 分钟（首次编译会下载所有依赖）

**预期输出：**
```
   Compiling tokio v1.35.1
   Compiling sqlx v0.8.0
   Compiling tauri v2.0.0
   ...
   Compiling browser-manager v0.1.0
    Finished dev [unoptimized + debuginfo] target(s) in 8m 32s
```

### 2. 可能的错误和解决方案

#### 错误 1：SQLx 编译时检查失败
```
error: error returned from database: (code: 1) no such table: profiles
```

**解决方案：**
```powershell
# 方法 1：运行迁移
cargo sqlx migrate run

# 方法 2：使用 offline mode
cargo sqlx prepare
cargo build --features sqlx/offline
```

#### 错误 2：找不到 MSVC
```
error: linker `link.exe` not found
```

**解决方案：**
- 安装 Visual Studio Build Tools（见上文）
- 或切换到 GNU 工具链：
  ```powershell
  rustup default stable-x86_64-pc-windows-gnu
  ```

#### 错误 3：Windows SDK 未找到
```
error: failed to run custom build command for `windows v0.58.0`
```

**解决方案：**
- 安装 Windows 10/11 SDK
- 或更新 Visual Studio Build Tools

---

## 五、运行开发服务器

### 1. 启动 Tauri 开发服务器

```powershell
# 在项目根目录
cd f:/user/Desktop/PC端浏览器多开器/browser-manager
npm run tauri dev
```

**这会同时启动：**
- Vite 前端开发服务器（http://localhost:1420）
- Tauri 后端（Rust）

### 2. 验证功能

打开应用后，测试以下功能：
- ✅ 前端界面正常显示
- ✅ 点击 "新建环境" 打开抽屉
- ✅ 填写表单并保存（会调用 Rust 后端）
- ✅ 环境列表显示（从 SQLite 读取）

---

## 六、常用命令

### 编译相关
```powershell
# 开发编译（快速，包含调试信息）
cargo build

# 生产编译（优化，体积小）
cargo build --release

# 检查代码（不生成二进制）
cargo check

# 运行测试
cargo test

# 格式化代码
cargo fmt

# 代码检查
cargo clippy
```

### Tauri 相关
```powershell
# 开发模式
npm run tauri dev

# 构建生产版本
npm run tauri build

# 查看 Tauri 信息
npm run tauri info
```

---

## 七、故障排除

### 问题 1：编译速度慢

**优化方案：**
1. 使用 `cargo check` 代替 `cargo build`（快 3-5 倍）
2. 启用增量编译（默认已启用）
3. 使用 `sccache` 缓存编译结果：
   ```powershell
   cargo install sccache
   $env:RUSTC_WRAPPER = "sccache"
   ```

### 问题 2：依赖下载失败

**解决方案：**
```powershell
# 使用国内镜像（可选）
# 编辑 ~/.cargo/config.toml
[source.crates-io]
replace-with = 'ustc'

[source.ustc]
registry = "https://mirrors.ustc.edu.cn/crates.io-index"
```

### 问题 3：SQLx 迁移失败

**解决方案：**
```powershell
# 手动创建数据库
sqlite3 browser-manager.db < migrations/001_init.sql

# 或使用 SQLx CLI
cargo sqlx database create
cargo sqlx migrate run
```

---

## 八、下一步

编译成功后：

1. **前后端联调**
   - 修改 `DashboardView.vue`
   - 替换 `mockProfiles` 为 `invoke('list_profiles')`

2. **测试 CRUD 功能**
   - 创建环境
   - 编辑环境
   - 删除环境

3. **集成 Chromium 内核**
   - 将 `chrome.exe` 放入 `resources/kernel/win32/`
   - 实现浏览器启动功能

---

## 九、参考资源

- **Rust 官方文档**：https://doc.rust-lang.org/
- **Tauri 文档**：https://tauri.app/
- **SQLx 文档**：https://github.com/launchbadge/sqlx
- **Tokio 文档**：https://tokio.rs/

---

**祝编译顺利！如有问题，请随时反馈。**
