# 前端启动器开发规划 (Frontend Launcher Development Plan)

> **Role**: Planner
> **Goal**: 按照“触宝指纹浏览器” (Chubao) 的设计规范，构建高性能、大厂级的 Vue 3 前端应用。

## 0. 预备工作 (Prerequisites)
- [ ] 确保 Node.js 环境就绪。
- [ ] 确保 Rust/Cargo 环境就绪 (Tauri 2.0 CLI).

## Phase 1: 基础设施搭建 (Infrastructure)
*目标：建立稳固的 Vue 3 + TypeScript + Element Plus 开发底座。*

1.  **初始化脚手架**:
    - [ ] `npm create tauri-app@latest` (Vue, TS).
    - [ ] 配置 `vite.config.ts` (Alias, Server Options).
2.  **集成核心库**:
    - [ ] 安装 `element-plus` & `@element-plus/icons-vue`.
    - [ ] 安装 `tailwindcss` & `postcss` & `autoprefixer` (配置 `tailwind.config.js`).
    - [ ] 安装 `pinia` (状态管理) & `pinia-plugin-persistedstate`.
    - [ ] 安装 `vue-router` (路由).
3.  **样式系统配置 (Deep Night Theme)**:
    - [ ] 创建 `src/assets/styles/main.scss` (重置 CSS).
    - [ ] 创建 `src/assets/styles/element-dark.scss` (覆盖 Element Plus 默认变量，实现 `#1a1b1e` 极黑主题).
    - [ ] 配置 Tailwind 颜色板 (`colors: { dark: { bg: '#1a1b1e', surface: '#25262b' } }`).

## Phase 2: 布局与导航 (Layout & Navigation)
*目标：实现 Mockup 中的高保真侧边栏与框架。*

1.  **主布局实现 (`src/layouts/MainLayout.vue`)**:
    - [ ] `Sidebar` (左): 240px 固定宽度，包含 Logo, Menu, UserProfile。
    - [ ] `Header` (上): 60px 高度，包含面包屑、全局搜索框。
    - [ ] `Content` (中): Flex 布局，自适应高度。
2.  **组件开发**:
    - [ ] `AppSidebar.vue`: 封装 Element Menu，实现路由跳转高亮。
    - [ ] `AppHeader.vue`: 实现面包屑动态生成逻辑。
    - [ ] `AtomicStatusDot.vue`: 状态指示灯 (Green/Gray) 原子组件。

## Phase 3: 核心仪表盘 (Dashboard Core)
*目标：实现高性能的“环境列表”管理界面。*

1.  **数据模型定义 TypeScript Interfaces**:
    - [ ] `types/profile.ts`: 定义 `UserProfile`, `ProxyConfig`, `Fingerprint`.
2.  **Store 实现 (`stores/profileStore.ts`)**:
    - [ ] State: `profileList: Profile[]`.
    - [ ] Actions: `fetchProfiles()`, `updateStatus()`.
3.  **虚拟滚动列表 (`EnvironmentList.vue`)**:
    - [ ] 引入 `vue-virtual-scroller` 或自行实现简单的 `recycle-view`。
    - [ ] 实现自定义 Table Row 样式 (Grid Layout)，复刻 Mockup 中的 Badge 和 Flag Icon。
4.  **Mock 数据桩**:
    - [ ] 在 `api/mock/` 中生成 50 条测试数据，用于 UI 调试。

## Phase 4: 指纹配置抽屉 (Configuration Drawer)
*目标：实现复杂的表单交互。*

1.  **抽屉组件 (`ProfileEditor.vue`)**:
    - [ ] 使用 `el-drawer` 封装。
    - [ ] Tab 分页: [基础信息] [指纹参数] [代理设置].
2.  **指纹参数表单**:
    - [ ] 操作系统/内核版本选择 (Dropdown).
    - [ ] 硬件并发数/内存大小 (Slider).
    - [ ] Canvas 噪声开关 (Switch).

## Phase 5: Tauri IPC 通信 (Backend Integration)
*目标：打通前端与 Rust 的连接。*

1.  **封装 API 层 (`api/tauri.ts`)**:
    - [ ] `invoke('get_profiles')` -> `ProfileApi.list()`.
    - [ ] `invoke('launch_browser')` -> `BrowserApi.launch(id)`.
2.  **错误处理中间件**:
    - [ ] 统一捕获 IPC 错误，弹出 `ElMessage.error`.

## Phase 6: 自动化构建与测试
- [ ] 配置 `npm run build` 流程。
- [ ] 编写简单的 Unit Test (Vitest) 验证 Store 逻辑。

---

**执行策略**:
我们将按照 `Phase 1` -> `Phase 2` 的顺序依次执行。每个 Phase 完成后会进行一次 UI 验收。
