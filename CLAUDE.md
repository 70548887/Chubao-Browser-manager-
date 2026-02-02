# CLAUDE.md - 触宝指纹浏览器 代码生成规范

> **本文档是 AI 代码生成的"宪法"，所有生成的代码必须严格遵守。**

---

## 零、系统架构概览

### 架构模式：Commander-Soldier

本系统采用 **"指挥官-士兵"** 双进程架构：

```
┌─────────────────────────────────────────────────┐
│  Commander (指挥官) - browser-manager           │
│  ├─ Tauri 2.0 (Rust 后端)                       │
│  │  ├─ 资源调度 (Profile Manager)               │
│  │  ├─ 代理转发 (Proxy Bridge)                  │
│  │  ├─ 进程管理 (Process Supervisor)            │
│  │  └─ 指纹生成 (Fingerprint Engine)            │
│  └─ Vue 3 (前端 UI)                             │
│     ├─ Element Plus (UI 组件)                   │
│     ├─ Tailwind CSS (样式)                      │
│     └─ Pinia (状态管理)                         │
└─────────────────────────────────────────────────┘
                    ↓ 启动并注入参数
┌─────────────────────────────────────────────────┐
│  Soldier (士兵) - Chromium 146 (自研魔改)        │
│  ├─ 底层渲染引擎                                 │
│  ├─ 指纹伪装 (Canvas/WebGL/Audio)               │
│  └─ 脚本执行环境                                 │
└─────────────────────────────────────────────────┘
```

**核心原则**：
- ✅ 参数通过 Rust 启动参数注入（`--bot-config-*`）
- ❌ 严禁在 JavaScript 中修改指纹参数（会被检测）

---

## 一、技术栈与运行环境

### 1.1 运行环境要求

| 环境 | 最低版本 | 推荐版本 |
|------|----------|----------|
| **Node.js** | 18.0+ | 20 LTS |
| **npm** | 9.0+ | 10+ |
| **Rust** | 1.70+ | 1.75+ (stable) |
| **Cargo** | 随 Rust | - |
| **操作系统** | Windows 10 21H2+ | Windows 11 |
| **WebView2** | 自动安装 | 最新版 |

### 1.2 开发工具

| 工具 | 用途 |
|------|------|
| **VS Code** | 主编辑器 |
| **Rust Analyzer** | Rust LSP |
| **Volar** | Vue 3 支持 |
| **Tauri CLI** | `npm run tauri dev` |

### 1.3 技术栈

```
┌─────────────────────────────────────────────────┐
│  前端 (Vue 3)                                    │
│  ├─ Vue 3.4+ (Composition API)                  │
│  ├─ TypeScript 5.0+                             │
│  ├─ Element Plus (Dark Mode)                    │
│  ├─ Pinia (状态管理)                             │
│  ├─ Tailwind CSS 3.4+                           │
│  └─ Vite 5.0+                                   │
├─────────────────────────────────────────────────┤
│  后端 (Rust)                                     │
│  ├─ Tauri 2.0                                   │
│  ├─ Tokio (异步运行时)                           │
│  ├─ SQLx (SQLite 异步)                          │
│  ├─ windows-rs (Win32 API)                      │
│  └─ tracing (日志)                              │
└─────────────────────────────────────────────────┘
```

---

## 二、编码红线 (MANDATORY)

### 2.1 严禁硬编码

```typescript
// ❌ 错误 - 硬编码
const API_URL = "http://127.0.0.1:8080"
const TIMEOUT = 5000
const PRIMARY_COLOR = "#409eff"

// ✅ 正确 - 使用配置
import { AppConfig } from '@/config'
const timeout = AppConfig.API_TIMEOUT
```

```rust
// ❌ 错误 - Rust 硬编码
let cache_dir = "D:\\chubao-cache";

// ✅ 正确 - 使用配置
let cache_dir = config.cache_dir();
```

### 2.2 类型安全

```typescript
// ❌ 错误 - any 类型
function process(data: any) { ... }

// ✅ 正确 - 明确类型
interface ProfileData { id: string; name: string }
function process(data: ProfileData) { ... }
```

### 2.3 错误处理

```typescript
// ❌ 错误 - 无错误处理
const data = await ProfileApi.getList()

// ✅ 正确 - 完整错误处理
try {
  const data = await ProfileApi.getList()
} catch (error) {
  ElMessage.error(`获取失败: ${(error as Error).message}`)
}
```

---

## 三、命名规范

### 3.1 文件命名

| 类型 | 格式 | 示例 |
|------|------|------|
| Vue 组件 | PascalCase.vue | `ProfileCard.vue` |
| TypeScript | camelCase.ts | `profile.api.ts` |
| Store | *.store.ts | `profile.store.ts` |
| Types | *.types.ts | `profile.types.ts` |
| Rust 模块 | snake_case.rs | `profile_cmd.rs` |
| SCSS | _*.scss | `_variables.scss` |
| **开发进度/日志** | **YYYY-MM-DD-名称.md** | `2026-01-27-开发进度总结.md` |
| 设计文档 | 中文名称.md | `项目架构设计.md` |
| 分析文档 | 类型-名称.md | `竞品分析-比特浏览器.md` |

### 3.2 变量命名

```typescript
// 布尔值 - is/has/can 前缀
const isLoading = ref(false)
const hasError = ref(false)
const canSubmit = computed(() => ...)

// 事件处理 - handle 前缀
const handleSubmit = () => {}
const handleDelete = () => {}

// 异步操作 - fetch/load/save 等动词
const fetchProfiles = async () => {}
const saveProfile = async () => {}
```

### 3.3 常量命名

```typescript
// 全大写 + 下划线
const MAX_RETRY_COUNT = 3
const DEFAULT_TIMEOUT_MS = 5000
```

---

## 四、Vue 组件规范

### 4.1 标准组件模板

```vue
<script setup lang="ts">
/**
 * @description 组件描述
 * @author DeepAgent
 */

// ==================== 导入 ====================
import { ref, computed, onMounted } from 'vue'
import type { Profile } from '@/types'

// ==================== Props & Emits ====================
interface Props {
  /** 环境数据 */
  profile: Profile
  /** 是否激活 */
  isActive?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  isActive: false,
})

const emit = defineEmits<{
  (e: 'update', value: Profile): void
  (e: 'delete', id: string): void
}>()

// ==================== 响应式状态 ====================
const isEditing = ref(false)

// ==================== 计算属性 ====================
const statusText = computed(() =>
  props.isActive ? '运行中' : '已停止'
)

// ==================== 方法 ====================
const handleEdit = () => {
  isEditing.value = true
}

// ==================== 生命周期 ====================
onMounted(() => {
  // 初始化
})
</script>

<template>
  <div class="profile-card">
    <!-- 内容 -->
  </div>
</template>

<style scoped lang="scss">
.profile-card {
  // 样式
}
</style>
```

### 4.2 文件头注释

```typescript
/**
 * @file profile.api.ts
 * @description 环境管理 API 封装
 * @author DeepAgent
 * @created 2026-01-27
 */
```

---

## 五、Rust 代码规范

### 5.1 错误处理模板

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("数据库错误: {0}")]
    Database(#[from] sqlx::Error),

    #[error("配置不存在: {0}")]
    ProfileNotFound(String),

    #[error("浏览器启动失败: {0}")]
    BrowserLaunchFailed(String),
}

pub type AppResult<T> = Result<T, AppError>;
```

### 5.2 Tauri Command 模板

```rust
use tauri::command;

/// 获取所有环境列表
#[command]
pub async fn get_profiles(
    state: tauri::State<'_, AppState>,
) -> Result<Vec<Profile>, String> {
    state
        .profile_service
        .get_all()
        .await
        .map_err(|e| e.to_string())
}
```

### 5.3 日志规范

```rust
use tracing::{info, debug, error, warn};

// 关键操作
info!(profile_id = %id, "启动浏览器");

// 调试信息
debug!(config = ?fingerprint, "生成指纹参数");

// 错误记录 (包含源错误)
error!(error = %e, "数据库连接失败");
```

---

## 六、模块化规范 (Modularization)

### 6.1 Rust 功能模块 (Functional Modules)

| 模块 | 职责 | 核心功能 |
|------|------|---------|
| **Environment Manager** | 环境管理 | UDD 创建、元数据 CRUD、配置持久化 |
| **Fingerprint Engine** | 指纹生成 | Seed 生成、160+ 参数构造、随机化算法 |
| **Network Bridge** | 代理网桥 | SOCKS5 → HTTP 协议转换、流量转发 |
| **Process Supervisor** | 进程监控 | 子进程生命周期管理、心跳检测 |
| **Browser Launcher** | 浏览器启动 | Chromium 启动、参数注入、窗口控制 |

### 6.2 Vue UI 模块 (UI Modules)

```typescript
// 原子组件 (Atomic Components) - 纯展示，无业务逻辑
components/common/
  ├── StatusBadge.vue      // 状态徽章
  ├── ActionButton.vue     // 操作按钮
  └── DataTable.vue        // 数据表格

// 业务组件 (Business Components) - 包含业务逻辑
components/business/
  ├── ProfileCard.vue      // 环境卡片
  └── FingerprintForm.vue  // 指纹表单

// 功能页面 (Feature Views) - 调用 Store 和 API
features/
  ├── dashboard/           // 仪表盘
  ├── profile-editor/      // 环境编辑器
  └── settings/            // 系统设置
```

### 6.3 架构模式

**Commander-Soldier 模式**：
- **Commander (Rust)**：启动器，负责参数注入、进程管理、资源隔离
- **Soldier (Chromium 146)**：自研魔改内核，接收参数并执行

```rust
// Commander 启动 Soldier
pub async fn launch_browser(profile: &Profile) -> Result<Process> {
    let args = build_chromium_args(profile)?;  // 构建启动参数
    let process = Command::new("chrome.exe")
        .args(args)
        .spawn()?;
    
    ProcessSupervisor::monitor(process)  // 监控进程
}
```

---

## 七、指纹对抗核心 (Anti-Fingerprinting)

### 7.1 参数注入规范

**严禁在 JavaScript 中注入指纹参数**（会被检测）

```javascript
// ❌ 错误 - JS 注入会被检测
Object.defineProperty(navigator, 'hardwareConcurrency', {
  get: () => 8
})

// ✅ 正确 - 通过 Rust 启动参数注入
// Rust 端：
let args = vec![
    "--bot-config-hardware-concurrency=8",
    "--bot-config-device-memory=16",
    "--bot-config-platform=Win32",
];
```

### 7.2 网络一致性

**代理设置必须在 Rust 端验证后再启动浏览器**

```rust
// 验证代理连通性
pub async fn verify_proxy(proxy: &ProxyConfig) -> Result<()> {
    let client = reqwest::Client::builder()
        .proxy(reqwest::Proxy::all(&proxy.url)?)
        .build()?;
    
    // 测试连接
    client.get("https://ipinfo.io/json")
        .send()
        .await?;
    
    Ok(())
}

// 验证通过后再启动
if verify_proxy(&profile.proxy).await.is_ok() {
    launch_browser(profile).await?;
}
```

### 7.3 窗口保护

**使用 Win32 API 防止恶意截图**

```rust
use windows::Win32::UI::WindowsAndMessaging::{
    SetWindowDisplayAffinity, WDA_EXCLUDEFROMCAPTURE
};

pub fn protect_window(hwnd: HWND) -> Result<()> {
    unsafe {
        SetWindowDisplayAffinity(hwnd, WDA_EXCLUDEFROMCAPTURE)?;
    }
    Ok(())
}
```

### 7.4 日志与调试

```rust
use tracing::{info, debug, error, warn};

// 关键操作 - 生产环境可见
info!(profile_id = %id, "启动浏览器");

// 详细流程 - 仅开发环境
debug!(config = ?fingerprint, "生成指纹参数");

// 异常捕获 - 必须包含源错误
error!(error = %e, "数据库连接失败");
```

```typescript
// Frontend - 开发环境使用 console.log，生产环境静默
if (import.meta.env.DEV) {
  console.log('Profile created:', profile)
}
```

---

## 八、禁止事项 (Anti-Patterns)

| ❌ 禁止 | ✅ 替代方案 |
|--------|----------|
| 硬编码 URL/颜色/数值 | 使用 `config/` 模块 |
| `any` 类型 | 定义具体 `interface` |
| `console.log` (生产) | 使用 `tracing` 或删除 |
| Options API | Composition API |
| 中文变量名 | 英文 + 注释 |
| 超过 300 行的文件 | 拆分模块 |
| 循环依赖 | 重构层级 |
| 注释掉的代码 | 直接删除 |
| 魔法数字 | 提取为常量 |

---

## 九、Git 提交规范

```
<type>(<scope>): <subject>

类型:
- feat: 新功能
- fix: 修复
- refactor: 重构
- docs: 文档
- style: 格式
- test: 测试
- chore: 构建/工具

示例:
feat(profile): 添加环境批量删除功能
fix(proxy): 修复代理连接超时问题
refactor(ui): 重构侧边栏组件
```

---

## 十、代码审查检查清单

- [ ] 无硬编码值
- [ ] 无 `any` 类型
- [ ] 有完整的错误处理
- [ ] 有必要的注释
- [ ] 命名符合规范
- [ ] 文件不超过 300 行
- [ ] 无 `console.log`
- [ ] 通过 ESLint/Clippy
