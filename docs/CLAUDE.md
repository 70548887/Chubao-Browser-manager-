# CLAUDE.md - 触宝指纹浏览器 (Chubao Fingerprint Browser) 代码生成规范

> **本规范用于指导 AI 进行“触宝指纹浏览器”项目的代码生成与重构。**
> **核心原则**：零硬编码 (No Hardcoding)、模块化隔离、极致隐匿性。

## 1. 项目基础 (Project Basics)
*   **项目名称**：触宝指纹浏览器 (Chubao Browser)
*   **应用类型**：Windows 桌面应用 (Tauri 2.0)
*   **根目录**：`browser-manager/`

## 2. 技术栈约束 (Tech Stack)
*   **后端 (Rust)**：
    *   **框架**：Tauri 2.0 (Beta/RC), Tokio (Async Runtime).
    *   **核心库**：`windows-rs` (Win32 API), `sqlx` (SQLite), `reqwest`, `serde`, `tracing` (Logging).
    *   **架构模式**：Commander (Rust 启动器) - Soldier (自研魔改 Chromium 146) 模式.
*   **前端 (Vue 3)**：
    *   **框架**：Vue 3 (Composition API, `<script setup>`) + TypeScript.
    *   **UI 库**：Element Plus (Dark Mode Only).
    *   **状态管理**：Pinia (持久化存储).
    *   **样式**：TailwindCSS (Utility-first) + SCSS (组件级).

## 3. 编码红线 (MANDATORY RULES)
*   **严禁硬编码 (NO Hardcoding)**：
    *   ❌ 禁止：`const API_URL = "http://127.0.0.1:8080"`
    *   ✅ 必须：使用 `WebConfig.apiUrl` 或 Rust `AppConfig::default().api_url`。
    *   所有常量必须提取到 `/config` 或 `/constants` 模块。
*   **现代语法 (Modern Best Practices)**：
    *   **Rust**：全异步 (`async/await`)，禁止阻塞主线程。使用 `Result<T, AppError>` 处理错误。
    *   **Vue**：必须使用 Composition API。禁止使用 `Options API`。
    *   **CSS**：优先使用 Tailwind 类，仅在无法满足时编写 SCSS。
*   **代码清理**：
    *   禁止保留注释掉的旧代码 (Dead Code)。
    *   文件末尾必须保留一行空行。

## 4. 模块化规范 (Modularization)
### 4.1 功能模块 (Functional Modules) - Rust
*   **Environment Manager**: 负责 UDD 创建、元数据 CRUD。
*   **Fingerprint Engine**: 负责 Seed 生成、JSON 参数构造。
*   **Network Bridge**: 负责 SOCKS5 -> HTTP 协议转换。
*   **Process Supervisor**: 负责子进程生命周期监控 (Heartbeat)。

### 4.2 UI 模块 (UI Modules) - Vue
*   **Atomic Components**: 原子组件 (如 `StatusBadge.vue`) 必须纯展示，不包含业务逻辑。
*   **Feature Views**: 业务页面 (如 `EnvironmentList.vue`) 负责调用 Store 和 API。
*   **Layouts**: 负责整体结构 (Sidebar, Header)。

## 5. 日志与调试 (Logging & Debugging)
*   **Rust**: 使用 `tracing` 库。
    *   `info!`: 关键操作 (启动/停止/保存)。
    *   `debug!`: 详细流程 (参数拼接/API响应)。
    *   `error!`: 异常捕获 (必须包含 `std::error::Error` 源信息)。
*   **Frontend**: 开发环境使用 `console.log`，生产环境必须静默或发送到日志服务。

## 6. 指纹对抗核心 (Anti-Fingerprinting)
*   **参数注入**：所有指纹参数通过 Rust 启动命令 `--bot-config-` 注入，**严禁** 在 JS 中通过 `Object.defineProperty` 注入 (会被检测)。
*   **网络一致性**：`Proxy` 设置必须在 Rust 端验证连通性后，再启动浏览器。
*   **窗口保护**：使用 `SetWindowDisplayAffinity` 防止恶意截图。
