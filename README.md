# 触宝指纹浏览器 (Chubao Fingerprint Browser)

> **本项目为"启动器" (Launcher) 部分，负责环境管理、代理转发、窗口控制等功能。**
> **浏览器内核为自研魔改 Chromium 146，由团队独立开发，后续整合到启动器中。**

## 项目架构

```
┌─────────────────────────────────────────────────────────────┐
│                    触宝指纹浏览器系统                         │
│                                                             │
│  ┌─────────────────────────────────────────────────────┐   │
│  │        🎮 启动器 (browser-manager)                   │ ← 本项目
│  │        Rust + Tauri 2.0 + Vue 3                     │
│  │        ✅ 环境配置管理                               │
│  │        ✅ 代理网桥 (Proxy Bridge)                    │
│  │        ✅ 窗口群控 (HWND 管理)                       │
│  │        ✅ 指纹参数下发                               │
│  └─────────────────────────────────────────────────────┘
│                          │
│                          ▼ 调用
│  ┌─────────────────────────────────────────────────────┐
│  │        🌐 自研魔改 Chromium 146 内核                 │ ← 自研开发
│  │        基于 Google 开源 Chromium 146 深度定制        │
│  │        ✅ 指纹伪装 (Canvas, WebGL, Audio)           │
│  │        ✅ TLS 指纹修改 (JA3)                        │
│  │        ✅ Client Hints 同步                         │
│  │        ✅ Skia 像素噪声注入                         │
│  └─────────────────────────────────────────────────────┘
└─────────────────────────────────────────────────────────────┘
```

## 核心功能

- **🚀 环境生命周期管理**：支持环境的创建、克隆、编辑、软删除（进入回收站）与恢复，具备完整的状态追踪。
- **⚡ 高性能批量操作**：支持对大量环境的批量启动、停止及删除，具备结果异步汇总展示。
- **🛡️ 指纹深度定制**：对接自研 Chromium 146 内核，支持 Canvas、WebGL、Audio、Client Hints、WebRTC、Fonts 等数十项指纹参数的动态注入。
- **🌐 独立代理管理**：内置代理检测系统（支持 SOCKS5/HTTP/HTTPS），支持出口 IP 识别及延迟检测。
- **📂 灵活的分组与标签**：提供多维度的环境管理方式，支持批量移动分组、多标签关联及备注搜索。
- **⚙️ 桌面端原生体验**：基于 Tauri 2.0，提供极低的内存占用、秒开的响应速度及系统级原生窗口控制。

## 技术栈

| 领域 | 技术方案 |
|------|------|
| **启动器前端** | Vue 3 (Vite 6) + TypeScript + Element Plus (Custom Native UI) |
| **启动器后端** | Rust + Tauri 2.0 |
| **持久化存储** | SQLite 3 + sqlx (支持自动迁移) |
| **浏览器内核** | 深度定制 Chromium 146 (支持 WebRTC/Cookie 同步优化) |
| **通信协议** | Tauri IPC + 后续支持的 WebSocket 控制协议 |
| **样式体系** | Tailwind CSS + SCSS (遵循桌面原生化 UI 设计规范) |

## 快速开始

### 1. 环境准备
- **Rust**: `rustc` 1.77+
- **Node.js**: 18+ (推荐使用 `pnpm`)
- **WebView2**: Windows 系统自带

### 2. 运行开发环境
```bash
# 安装依赖
npm install

# 启动前端及 Tauri 后端开发服务器
npm run tauri dev
```

### 3. 构建发布包
```bash
npm run tauri build
```

## 项目结构 (核心目录)

```
browser-manager/
├── src/                    # 🎨 前端 Vue 源码
│   ├── api/                # 后端 IPC 接口封装 (Tauri invoke)
│   ├── assets/styles/      # 遵循原生 UI 规范的样式库
│   ├── features/           # 业务功能模块 (Dashboard, Proxy, Recycle, etc.)
│   ├── stores/             # Pinia 状态管理 (Profile, UI)
│   └── types/              # TypeScript 全局类型定义
├── src-tauri/              # ⚙️ 后端 Rust 源码
│   ├── migrations/         # 🗄️ SQLite 数据库迁移脚本
│   ├── src/modules/        # 核心逻辑 (ProfileService, BrowserManager, etc.)
│   └── tauri.conf.json     # Tauri 配置文件
├── resources/              # 📦 静态资源
│   └── kernel/win32/       # 预留给魔改 Chromium 内核的存放路径
├── docs/                   # 📚 深度开发与架构文档
└── planning/               # 📋 开发进度记录与任务规划
```

## 开发文档导航

| 类别 | 核心文档 |
|------|------|
| **快速参考** | [CLAUDE.md](CLAUDE.md) / [项目架构规范.md](项目架构规范.md) |
| **核心方案** | [Rust-Tauri-多开器设计方案.md](docs/Rust-Tauri-多开器设计方案.md) / [方案三-自研Chromium深度定制方案.md](docs/方案三-自研Chromium深度定制方案.md) |
| **技术细节** | [IPC-接口说明.md](docs/IPC-接口说明.md) / [内核开发需求-WebRTC与Cookie同步.md](docs/内核开发需求-WebRTC与Cookie同步.md) |
| **UI 规范** | [桌面原生化UI设计规范-实施指南.md](docs/桌面原生化UI设计规范-实施指南.md) |
| **进度汇总** | [2026-01-28-V6开发进度总结.md](docs/2026-01-28-V6开发进度总结.md) / [下一步开发计划.md](下一步开发计划.md) |

## 开发状态

目前处于 **M1 (核心框架集成)** 阶段，大部分启动器功能已完成闭环：

- [x] 主框架、导航与 原生 UI 规范实现
- [x] 基于 sqlx 的数据库动态迁移系统
- [x] 环境列表、分组、标签管理逻辑
- [x] 批量启动、停止及 进程状态监控
- [x] 软删除机制（回收站）与 恢复逻辑
- [x] 代理服务器连通性及 地理位置检测
- [ ] 自研内核 (Chromium 146) 深度集成 (进行中)
- [ ] WebSocket 远程控制协议支持 (规划中)

---

**License**: Private
