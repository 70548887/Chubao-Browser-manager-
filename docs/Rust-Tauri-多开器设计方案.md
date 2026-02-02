# Rust + Tauri 反指纹浏览器多开器设计方案

## 一、项目概述

### 1.1 项目目标

构建一个高性能、低资源占用的 Windows PC端浏览器多开管理器，实现：

- 多开 fingerprint-chromium 浏览器实例
- 每个实例独立指纹配置
- 通过 CDP 协议执行脚本（无扩展痕迹）
- 窗口显示/隐藏/排列管理
- 与 Go 服务端 WebSocket 通信

### 1.2 技术选型

| 层级 | 技术 | 说明 |
|------|------|------|
| **桌面框架** | Tauri 2.0 | 轻量级，打包仅3-8MB |
| **后端语言** | Rust | 高性能，低内存占用 |
| **CDP库** | chromiumoxide | Rust异步CDP库 |
| **异步运行时** | Tokio | 高性能异步运行时 |
| **Windows API** | windows-rs | 原生窗口控制 |
| **前端框架** | Vue 3 + TypeScript | 响应式UI |
| **状态管理** | Pinia | Vue状态管理 |
| **UI组件** | Element Plus | 成熟UI库 |
| **浏览器** | fingerprint-chromium | 底层指纹修改 |

### 1.3 系统架构图

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                              系统架构                                        │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│                           Go 服务端 (已有)                                   │
│                         ┌─────────────────┐                                 │
│                         │  • 脚本管理      │                                 │
│                         │  • 任务调度      │                                 │
│                         │  • 设备管理      │                                 │
│                         └────────┬────────┘                                 │
│                                  │ WebSocket                                │
│                                  ▼                                          │
│  ┌───────────────────────────────────────────────────────────────────────┐ │
│  │                    Tauri App (本项目)                                  │ │
│  │                                                                       │ │
│  │  ┌─────────────────────────────────────────────────────────────────┐ │ │
│  │  │                    Rust Backend                                 │ │ │
│  │  │                                                                 │ │ │
│  │  │  ┌───────────┐ ┌───────────┐ ┌───────────┐ ┌───────────┐       │ │ │
│  │  │  │ Profile   │ │ Browser   │ │ CDP       │ │ Window    │       │ │ │
│  │  │  │ Manager   │ │ Launcher  │ │ Manager   │ │ Manager   │       │ │ │
│  │  │  └───────────┘ └───────────┘ └───────────┘ └───────────┘       │ │ │
│  │  │                                                                 │ │ │
│  │  │  ┌───────────┐ ┌───────────┐ ┌───────────┐                     │ │ │
│  │  │  │ Server    │ │ State     │ │ Config    │                     │ │ │
│  │  │  │ Client    │ │ Store     │ │ Manager   │                     │ │ │
│  │  │  └───────────┘ └───────────┘ └───────────┘                     │ │ │
│  │  │                                                                 │ │ │
│  │  └─────────────────────────────────────────────────────────────────┘ │ │
│  │                              │                                       │ │
│  │                              │ Tauri IPC                             │ │
│  │                              ▼                                       │ │
│  │  ┌─────────────────────────────────────────────────────────────────┐ │ │
│  │  │                    Vue 3 Frontend                               │ │ │
│  │  │                                                                 │ │ │
│  │  │  ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌─────────┐  │ │ │
│  │  │  │ 设备列表 │ │ 指纹配置 │ │ 窗口控制 │ │ 任务监控 │ │ 系统设置 │  │ │ │
│  │  │  └─────────┘ └─────────┘ └─────────┘ └─────────┘ └─────────┘  │ │ │
│  │  └─────────────────────────────────────────────────────────────────┘ │ │
│  └───────────────────────────────────────────────────────────────────────┘ │
│                                  │                                          │
│                                  │ CDP Protocol                             │
│                                  ▼                                          │
│  ┌───────────────────────────────────────────────────────────────────────┐ │
│  │          fingerprint-chromium 实例 ×N                                 │ │
│  │  ┌─────────┐  ┌─────────┐  ┌─────────┐  ┌─────────┐                  │ │
│  │  │ 实例1   │  │ 实例2   │  │ 实例3   │  │ 实例N   │                  │ │
│  │  │ 指纹A   │  │ 指纹B   │  │ 指纹C   │  │ 指纹N   │                  │ │
│  │  │ CDP:9222│  │ CDP:9223│  │ CDP:9224│  │ CDP:...│                  │ │
│  │  └─────────┘  └─────────┘  └─────────┘  └─────────┘                  │ │
│  └───────────────────────────────────────────────────────────────────────┘ │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## 二、项目目录结构

```
browser-manager/
├── src-tauri/                      # Rust 后端
│   ├── src/
│   │   ├── main.rs                 # 入口文件
│   │   ├── lib.rs                  # 库导出
│   │   ├── commands/               # Tauri Commands
│   │   │   ├── mod.rs
│   │   │   ├── browser.rs          # 浏览器控制命令
│   │   │   ├── profile.rs          # 配置文件命令
│   │   │   ├── window.rs           # 窗口控制命令
│   │   │   └── server.rs           # 服务端通信命令
│   │   ├── managers/               # 核心管理器
│   │   │   ├── mod.rs
│   │   │   ├── profile_manager.rs  # 指纹配置管理
│   │   │   ├── browser_manager.rs  # 浏览器进程管理
│   │   │   ├── cdp_manager.rs      # CDP连接管理
│   │   │   ├── window_manager.rs   # 窗口管理
│   │   │   └── server_client.rs    # 服务端WebSocket客户端
│   │   ├── models/                 # 数据模型
│   │   │   ├── mod.rs
│   │   │   ├── profile.rs          # 配置文件模型
│   │   │   ├── device.rs           # 设备模型
│   │   │   ├── fingerprint.rs      # 指纹模型
│   │   │   └── message.rs          # 消息模型
│   │   ├── utils/                  # 工具函数
│   │   │   ├── mod.rs
│   │   │   ├── fingerprint_gen.rs  # 指纹生成
│   │   │   ├── port_finder.rs      # 端口查找
│   │   │   └── path_helper.rs      # 路径处理
│   │   ├── state.rs                # 全局状态
│   │   ├── config.rs               # 配置文件
│   │   └── error.rs                # 错误处理
│   ├── Cargo.toml                  # Rust依赖
│   ├── tauri.conf.json             # Tauri配置
│   └── build.rs                    # 构建脚本
├── src/                            # Vue 前端
│   ├── assets/                     # 静态资源
│   ├── components/                 # 通用组件
│   │   ├── DeviceCard.vue          # 设备卡片
│   │   ├── FingerprintEditor.vue   # 指纹编辑器
│   │   ├── WindowGrid.vue          # 窗口网格
│   │   └── StatusBar.vue           # 状态栏
│   ├── views/                      # 页面视图
│   │   ├── DeviceList.vue          # 设备列表页
│   │   ├── ProfileManager.vue      # 配置管理页
│   │   ├── TaskMonitor.vue         # 任务监控页
│   │   └── Settings.vue            # 设置页
│   ├── stores/                     # Pinia状态
│   │   ├── device.ts               # 设备状态
│   │   ├── profile.ts              # 配置状态
│   │   └── settings.ts             # 设置状态
│   ├── api/                        # Tauri API封装
│   │   ├── browser.ts              # 浏览器API
│   │   ├── profile.ts              # 配置API
│   │   ├── window.ts               # 窗口API
│   │   └── server.ts               # 服务端API
│   ├── types/                      # TypeScript类型
│   │   └── index.ts
│   ├── App.vue
│   └── main.ts
├── public/                         # 公共资源
├── package.json                    # 前端依赖
├── vite.config.ts                  # Vite配置
├── tsconfig.json                   # TypeScript配置
└── README.md
```

---

## 三、核心模块设计

### 3.1 数据模型

#### Profile (配置文件)

```rust
// src-tauri/src/models/profile.rs

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    /// 唯一ID
    pub id: String,
    /// 显示名称
    pub name: String,
    /// 用户数据目录
    pub data_dir: String,
    /// CDP端口
    pub cdp_port: u16,
    /// 指纹配置
    pub fingerprint: Fingerprint,
    /// 代理设置
    pub proxy: Option<ProxyConfig>,
    /// 启动参数
    pub extra_args: Vec<String>,
    /// 创建时间
    pub created_at: i64,
    /// 更新时间
    pub updated_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fingerprint {
    /// 指纹种子 (核心参数)
    pub seed: i64,
    /// 操作系统
    pub platform: String,           // windows, linux, macos
    /// 浏览器品牌
    pub brand: String,              // Chrome, Edge, etc.
    /// CPU核心数
    pub hardware_concurrency: u8,
    /// 内存大小 (GB)
    pub device_memory: u8,
    /// GPU厂商
    pub gpu_vendor: String,
    /// GPU型号
    pub gpu_renderer: String,
    /// 屏幕宽度
    pub screen_width: u16,
    /// 屏幕高度
    pub screen_height: u16,
    /// 时区
    pub timezone: String,
    /// 语言
    pub language: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProxyConfig {
    /// 代理类型: http, socks5
    pub proxy_type: String,
    /// 代理地址
    pub host: String,
    /// 代理端口
    pub port: u16,
    /// 用户名 (可选)
    pub username: Option<String>,
    /// 密码 (可选)
    pub password: Option<String>,
}

impl Profile {
    pub fn new(name: &str) -> Self {
        let id = Uuid::new_v4().to_string();
        let now = chrono::Utc::now().timestamp();
        
        Self {
            id: id.clone(),
            name: name.to_string(),
            data_dir: format!("profiles/{}", id),
            cdp_port: 0, // 动态分配
            fingerprint: Fingerprint::random(),
            proxy: None,
            extra_args: vec![],
            created_at: now,
            updated_at: now,
        }
    }
}

impl Fingerprint {
    /// 生成随机指纹
    pub fn random() -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        
        let screens = vec![
            (1920, 1080), (1366, 768), (1440, 900), 
            (1536, 864), (2560, 1440), (1280, 720),
        ];
        let screen = screens[rng.gen_range(0..screens.len())];
        
        let gpus = vec![
            ("Intel Inc.", "Intel Iris OpenGL Engine"),
            ("NVIDIA Corporation", "NVIDIA GeForce GTX 1660"),
            ("NVIDIA Corporation", "NVIDIA GeForce RTX 3060"),
            ("AMD", "AMD Radeon RX 580"),
            ("Google Inc. (NVIDIA)", "ANGLE (NVIDIA GeForce GTX 1060)"),
        ];
        let gpu = gpus[rng.gen_range(0..gpus.len())];
        
        Self {
            seed: rng.gen_range(10000..99999999),
            platform: "windows".to_string(),
            brand: "Chrome".to_string(),
            hardware_concurrency: [4, 8, 12, 16][rng.gen_range(0..4)],
            device_memory: [4, 8, 16, 32][rng.gen_range(0..4)],
            gpu_vendor: gpu.0.to_string(),
            gpu_renderer: gpu.1.to_string(),
            screen_width: screen.0,
            screen_height: screen.1,
            timezone: "Asia/Shanghai".to_string(),
            language: "zh-CN".to_string(),
        }
    }
}
```

#### Device (运行中的设备)

```rust
// src-tauri/src/models/device.rs

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Device {
    /// 配置文件ID
    pub profile_id: String,
    /// 本地ID
    pub local_id: String,
    /// 服务端分配的设备ID
    pub device_id: Option<String>,
    /// 进程ID
    pub pid: u32,
    /// CDP端口
    pub cdp_port: u16,
    /// 窗口句柄
    pub window_handle: Option<isize>,
    /// 运行状态
    pub status: DeviceStatus,
    /// 窗口是否可见
    pub visible: bool,
    /// WebSocket连接状态
    pub ws_online: bool,
    /// 当前URL
    pub current_url: Option<String>,
    /// 启动时间
    pub started_at: i64,
    /// 最后心跳时间
    pub last_heartbeat: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DeviceStatus {
    Starting,   // 启动中
    Running,    // 运行中
    Stopping,   // 停止中
    Stopped,    // 已停止
    Error,      // 错误
}

impl Device {
    pub fn new(profile_id: &str, pid: u32, cdp_port: u16) -> Self {
        Self {
            profile_id: profile_id.to_string(),
            local_id: format!("local-{}-{}", chrono::Utc::now().timestamp(), pid),
            device_id: None,
            pid,
            cdp_port,
            window_handle: None,
            status: DeviceStatus::Starting,
            visible: true,
            ws_online: false,
            current_url: None,
            started_at: chrono::Utc::now().timestamp(),
            last_heartbeat: chrono::Utc::now().timestamp(),
        }
    }
}
```

### 3.2 CDP管理器

```rust
// src-tauri/src/managers/cdp_manager.rs

use chromiumoxide::{Browser, BrowserConfig, Page};
use chromiumoxide::cdp::browser_protocol::browser::WindowState;
use futures::StreamExt;
use tokio::sync::RwLock;
use std::collections::HashMap;
use std::sync::Arc;
use anyhow::{Result, anyhow};

use crate::models::profile::Profile;
use crate::config::AppConfig;

pub struct CDPConnection {
    pub browser: Browser,
    pub pages: Vec<Page>,
}

pub struct CDPManager {
    connections: Arc<RwLock<HashMap<String, CDPConnection>>>,
    config: Arc<AppConfig>,
}

impl CDPManager {
    pub fn new(config: Arc<AppConfig>) -> Self {
        Self {
            connections: Arc::new(RwLock::new(HashMap::new())),
            config,
        }
    }

    /// 启动浏览器并建立CDP连接
    pub async fn launch(&self, profile: &Profile) -> Result<u32> {
        let browser_path = &self.config.browser_path;
        
        // 构建启动参数
        let mut args = vec![
            format!("--fingerprint={}", profile.fingerprint.seed),
            format!("--user-data-dir={}", profile.data_dir),
            format!("--remote-debugging-port={}", profile.cdp_port),
            format!("--fingerprint-platform={}", profile.fingerprint.platform),
            format!("--fingerprint-brand={}", profile.fingerprint.brand),
            format!("--fingerprint-hardware-concurrency={}", profile.fingerprint.hardware_concurrency),
            format!("--fingerprint-gpu-vendor={}", profile.fingerprint.gpu_vendor),
            format!("--fingerprint-gpu-renderer={}", profile.fingerprint.gpu_renderer),
            format!("--timezone={}", profile.fingerprint.timezone),
            format!("--lang={}", profile.fingerprint.language),
            "--disable-non-proxied-udp".to_string(),
            "--disable-background-networking".to_string(),
            "--disable-client-side-phishing-detection".to_string(),
            "--no-first-run".to_string(),
        ];

        // 添加代理
        if let Some(proxy) = &profile.proxy {
            args.push(format!("--proxy-server={}://{}:{}", 
                proxy.proxy_type, proxy.host, proxy.port));
        }

        // 添加额外参数
        args.extend(profile.extra_args.clone());

        // 构建浏览器配置
        let config = BrowserConfig::builder()
            .chrome_executable(browser_path)
            .args(args)
            .window_size(
                profile.fingerprint.screen_width as u32,
                profile.fingerprint.screen_height as u32,
            )
            .build()
            .map_err(|e| anyhow!("Failed to build browser config: {}", e))?;

        // 启动浏览器
        let (browser, mut handler) = Browser::launch(config).await?;
        
        // 获取进程ID
        let pid = browser.process().map(|p| p.id()).unwrap_or(0);

        // 后台处理CDP事件
        let profile_id = profile.id.clone();
        tokio::spawn(async move {
            while let Some(event) = handler.next().await {
                // 处理CDP事件
                tracing::debug!("CDP event for {}: {:?}", profile_id, event);
            }
        });

        // 保存连接
        let connection = CDPConnection {
            browser,
            pages: vec![],
        };
        self.connections.write().await.insert(profile.id.clone(), connection);

        Ok(pid)
    }

    /// 连接到已运行的浏览器
    pub async fn connect(&self, profile_id: &str, cdp_port: u16) -> Result<()> {
        let ws_url = format!("ws://127.0.0.1:{}", cdp_port);
        let browser = Browser::connect(&ws_url).await?;
        
        let connection = CDPConnection {
            browser,
            pages: vec![],
        };
        self.connections.write().await.insert(profile_id.to_string(), connection);
        
        Ok(())
    }

    /// 执行JavaScript脚本
    pub async fn execute_script(&self, profile_id: &str, script: &str) -> Result<String> {
        let connections = self.connections.read().await;
        let connection = connections.get(profile_id)
            .ok_or_else(|| anyhow!("Connection not found for {}", profile_id))?;

        // 获取当前页面
        let pages = connection.browser.pages().await?;
        let page = pages.first()
            .ok_or_else(|| anyhow!("No page found"))?;

        // 执行脚本
        let result = page.evaluate(script).await?;
        
        Ok(result.to_string())
    }

    /// 在新文档加载前注入脚本
    pub async fn inject_script(&self, profile_id: &str, script: &str) -> Result<()> {
        let connections = self.connections.read().await;
        let connection = connections.get(profile_id)
            .ok_or_else(|| anyhow!("Connection not found for {}", profile_id))?;

        let pages = connection.browser.pages().await?;
        if let Some(page) = pages.first() {
            page.evaluate_on_new_document(script).await?;
        }
        
        Ok(())
    }

    /// 导航到URL
    pub async fn navigate(&self, profile_id: &str, url: &str) -> Result<()> {
        let connections = self.connections.read().await;
        let connection = connections.get(profile_id)
            .ok_or_else(|| anyhow!("Connection not found for {}", profile_id))?;

        let pages = connection.browser.pages().await?;
        if let Some(page) = pages.first() {
            page.goto(url).await?;
        }
        
        Ok(())
    }

    /// 获取当前URL
    pub async fn get_current_url(&self, profile_id: &str) -> Result<String> {
        let connections = self.connections.read().await;
        let connection = connections.get(profile_id)
            .ok_or_else(|| anyhow!("Connection not found for {}", profile_id))?;

        let pages = connection.browser.pages().await?;
        if let Some(page) = pages.first() {
            let url = page.url().await?;
            return Ok(url.unwrap_or_default().to_string());
        }
        
        Ok(String::new())
    }

    /// 截图
    pub async fn screenshot(&self, profile_id: &str) -> Result<Vec<u8>> {
        let connections = self.connections.read().await;
        let connection = connections.get(profile_id)
            .ok_or_else(|| anyhow!("Connection not found for {}", profile_id))?;

        let pages = connection.browser.pages().await?;
        if let Some(page) = pages.first() {
            let screenshot = page.screenshot(
                chromiumoxide::page::ScreenshotParams::builder()
                    .full_page(false)
                    .build()
            ).await?;
            return Ok(screenshot);
        }
        
        Err(anyhow!("No page found"))
    }

    /// 设置窗口状态
    pub async fn set_window_bounds(
        &self, 
        profile_id: &str, 
        left: i32, 
        top: i32, 
        width: u32, 
        height: u32,
        state: WindowState,
    ) -> Result<()> {
        let connections = self.connections.read().await;
        let connection = connections.get(profile_id)
            .ok_or_else(|| anyhow!("Connection not found for {}", profile_id))?;

        // 通过CDP设置窗口位置和大小
        connection.browser
            .execute(chromiumoxide::cdp::browser_protocol::browser::SetWindowBoundsParams::builder()
                .window_id(1.into())
                .bounds(chromiumoxide::cdp::browser_protocol::browser::Bounds::builder()
                    .left(left)
                    .top(top)
                    .width(width)
                    .height(height)
                    .window_state(state)
                    .build())
                .build())
            .await?;

        Ok(())
    }

    /// 关闭浏览器
    pub async fn close(&self, profile_id: &str) -> Result<()> {
        let mut connections = self.connections.write().await;
        if let Some(connection) = connections.remove(profile_id) {
            // 浏览器会在drop时自动关闭
            drop(connection);
        }
        Ok(())
    }

    /// 检查连接是否存在
    pub async fn is_connected(&self, profile_id: &str) -> bool {
        self.connections.read().await.contains_key(profile_id)
    }

    /// 获取所有连接的配置文件ID
    pub async fn get_connected_profiles(&self) -> Vec<String> {
        self.connections.read().await.keys().cloned().collect()
    }
}
```

### 3.3 浏览器进程管理器

```rust
// src-tauri/src/managers/browser_manager.rs

use tokio::sync::RwLock;
use std::collections::HashMap;
use std::sync::Arc;
use std::process::Child;
use anyhow::{Result, anyhow};

use crate::models::device::{Device, DeviceStatus};
use crate::models::profile::Profile;
use crate::managers::cdp_manager::CDPManager;
use crate::utils::port_finder::find_available_port;
use crate::config::AppConfig;

pub struct BrowserManager {
    devices: Arc<RwLock<HashMap<String, Device>>>,
    processes: Arc<RwLock<HashMap<String, Child>>>,
    cdp_manager: Arc<CDPManager>,
    config: Arc<AppConfig>,
}

impl BrowserManager {
    pub fn new(cdp_manager: Arc<CDPManager>, config: Arc<AppConfig>) -> Self {
        Self {
            devices: Arc::new(RwLock::new(HashMap::new())),
            processes: Arc::new(RwLock::new(HashMap::new())),
            cdp_manager,
            config,
        }
    }

    /// 启动浏览器实例
    pub async fn start(&self, profile: &mut Profile) -> Result<Device> {
        // 分配CDP端口
        if profile.cdp_port == 0 {
            profile.cdp_port = find_available_port(9222, 9999)?;
        }

        // 通过CDP管理器启动浏览器
        let pid = self.cdp_manager.launch(profile).await?;

        // 创建设备记录
        let mut device = Device::new(&profile.id, pid, profile.cdp_port);
        device.status = DeviceStatus::Running;

        // 等待CDP连接就绪
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

        // 保存设备记录
        self.devices.write().await.insert(profile.id.clone(), device.clone());

        Ok(device)
    }

    /// 停止浏览器实例
    pub async fn stop(&self, profile_id: &str) -> Result<()> {
        // 更新状态
        if let Some(device) = self.devices.write().await.get_mut(profile_id) {
            device.status = DeviceStatus::Stopping;
        }

        // 关闭CDP连接
        self.cdp_manager.close(profile_id).await?;

        // 强制结束进程
        if let Some(mut process) = self.processes.write().await.remove(profile_id) {
            let _ = process.kill();
        }

        // 移除设备记录
        self.devices.write().await.remove(profile_id);

        Ok(())
    }

    /// 获取设备状态
    pub async fn get_device(&self, profile_id: &str) -> Option<Device> {
        self.devices.read().await.get(profile_id).cloned()
    }

    /// 获取所有设备
    pub async fn get_all_devices(&self) -> Vec<Device> {
        self.devices.read().await.values().cloned().collect()
    }

    /// 更新设备状态
    pub async fn update_device(&self, profile_id: &str, update_fn: impl FnOnce(&mut Device)) {
        if let Some(device) = self.devices.write().await.get_mut(profile_id) {
            update_fn(device);
        }
    }

    /// 批量启动
    pub async fn start_batch(&self, profiles: &mut [Profile]) -> Vec<Result<Device>> {
        let mut results = vec![];
        
        for profile in profiles.iter_mut() {
            let result = self.start(profile).await;
            results.push(result);
            
            // 间隔启动，避免资源竞争
            tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
        }
        
        results
    }

    /// 批量停止
    pub async fn stop_all(&self) -> Result<()> {
        let profile_ids: Vec<String> = self.devices.read().await.keys().cloned().collect();
        
        for profile_id in profile_ids {
            let _ = self.stop(&profile_id).await;
        }
        
        Ok(())
    }
}
```

### 3.4 窗口管理器

```rust
// src-tauri/src/managers/window_manager.rs

use windows::Win32::Foundation::{HWND, LPARAM, BOOL, TRUE, FALSE};
use windows::Win32::UI::WindowsAndMessaging::{
    EnumWindows, GetWindowThreadProcessId, ShowWindow, SetWindowPos,
    SW_HIDE, SW_SHOW, SW_MINIMIZE, SW_RESTORE,
    SWP_NOMOVE, SWP_NOSIZE, SWP_NOZORDER,
    HWND_TOP,
};
use tokio::sync::RwLock;
use std::collections::HashMap;
use std::sync::Arc;
use anyhow::{Result, anyhow};

pub struct WindowInfo {
    pub handle: HWND,
    pub pid: u32,
    pub visible: bool,
    pub position: (i32, i32),
    pub size: (i32, i32),
}

pub struct WindowManager {
    windows: Arc<RwLock<HashMap<String, WindowInfo>>>,
}

impl WindowManager {
    pub fn new() -> Self {
        Self {
            windows: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// 根据PID查找窗口句柄
    pub fn find_window_by_pid(&self, target_pid: u32) -> Option<HWND> {
        let mut result: Option<HWND> = None;
        let target_pid_ptr = &target_pid as *const u32;

        unsafe {
            EnumWindows(
                Some(enum_window_callback),
                LPARAM(target_pid_ptr as isize),
            );
        }

        result
    }

    /// 显示窗口
    pub async fn show(&self, profile_id: &str) -> Result<()> {
        let windows = self.windows.read().await;
        if let Some(info) = windows.get(profile_id) {
            unsafe {
                ShowWindow(info.handle, SW_RESTORE);
                ShowWindow(info.handle, SW_SHOW);
            }
            return Ok(());
        }
        Err(anyhow!("Window not found"))
    }

    /// 隐藏窗口
    pub async fn hide(&self, profile_id: &str) -> Result<()> {
        let windows = self.windows.read().await;
        if let Some(info) = windows.get(profile_id) {
            unsafe {
                ShowWindow(info.handle, SW_HIDE);
            }
            return Ok(());
        }
        Err(anyhow!("Window not found"))
    }

    /// 最小化窗口
    pub async fn minimize(&self, profile_id: &str) -> Result<()> {
        let windows = self.windows.read().await;
        if let Some(info) = windows.get(profile_id) {
            unsafe {
                ShowWindow(info.handle, SW_MINIMIZE);
            }
            return Ok(());
        }
        Err(anyhow!("Window not found"))
    }

    /// 设置窗口位置和大小
    pub async fn set_bounds(
        &self, 
        profile_id: &str, 
        x: i32, 
        y: i32, 
        width: i32, 
        height: i32
    ) -> Result<()> {
        let windows = self.windows.read().await;
        if let Some(info) = windows.get(profile_id) {
            unsafe {
                SetWindowPos(
                    info.handle,
                    HWND_TOP,
                    x, y, width, height,
                    SWP_NOZORDER,
                );
            }
            return Ok(());
        }
        Err(anyhow!("Window not found"))
    }

    /// 网格排列所有窗口
    pub async fn arrange_grid(&self, cols: u32, screen_width: u32, screen_height: u32) -> Result<()> {
        let windows = self.windows.read().await;
        let window_list: Vec<_> = windows.values().collect();
        
        let count = window_list.len() as u32;
        if count == 0 {
            return Ok(());
        }

        let rows = (count + cols - 1) / cols;
        let cell_width = screen_width / cols;
        let cell_height = screen_height / rows;

        for (i, info) in window_list.iter().enumerate() {
            let row = i as u32 / cols;
            let col = i as u32 % cols;
            let x = (col * cell_width) as i32;
            let y = (row * cell_height) as i32;

            unsafe {
                ShowWindow(info.handle, SW_RESTORE);
                SetWindowPos(
                    info.handle,
                    HWND_TOP,
                    x, y, cell_width as i32, cell_height as i32,
                    SWP_NOZORDER,
                );
            }
        }

        Ok(())
    }

    /// 隐藏所有窗口
    pub async fn hide_all(&self) -> Result<()> {
        let windows = self.windows.read().await;
        for info in windows.values() {
            unsafe {
                ShowWindow(info.handle, SW_HIDE);
            }
        }
        Ok(())
    }

    /// 显示所有窗口
    pub async fn show_all(&self) -> Result<()> {
        let windows = self.windows.read().await;
        for info in windows.values() {
            unsafe {
                ShowWindow(info.handle, SW_SHOW);
            }
        }
        Ok(())
    }

    /// 注册窗口
    pub async fn register(&self, profile_id: &str, pid: u32) {
        if let Some(handle) = self.find_window_by_pid(pid) {
            let info = WindowInfo {
                handle,
                pid,
                visible: true,
                position: (0, 0),
                size: (1280, 800),
            };
            self.windows.write().await.insert(profile_id.to_string(), info);
        }
    }

    /// 注销窗口
    pub async fn unregister(&self, profile_id: &str) {
        self.windows.write().await.remove(profile_id);
    }
}

/// 枚举窗口回调函数
unsafe extern "system" fn enum_window_callback(hwnd: HWND, lparam: LPARAM) -> BOOL {
    let target_pid = *(lparam.0 as *const u32);
    let mut pid: u32 = 0;
    GetWindowThreadProcessId(hwnd, Some(&mut pid));
    
    if pid == target_pid {
        // 找到了目标窗口
        FALSE // 停止枚举
    } else {
        TRUE // 继续枚举
    }
}
```

### 3.5 服务端WebSocket客户端

```rust
// src-tauri/src/managers/server_client.rs

use tokio_tungstenite::{connect_async, tungstenite::Message};
use futures::{StreamExt, SinkExt};
use tokio::sync::{mpsc, RwLock};
use std::sync::Arc;
use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};

use crate::models::message::{ServerMessage, ClientMessage};

pub struct ServerClient {
    ws_url: String,
    sender: Arc<RwLock<Option<mpsc::Sender<ClientMessage>>>>,
    connected: Arc<RwLock<bool>>,
}

impl ServerClient {
    pub fn new(ws_url: &str) -> Self {
        Self {
            ws_url: ws_url.to_string(),
            sender: Arc::new(RwLock::new(None)),
            connected: Arc::new(RwLock::new(false)),
        }
    }

    /// 连接到服务端
    pub async fn connect(&self) -> Result<mpsc::Receiver<ServerMessage>> {
        let (ws_stream, _) = connect_async(&self.ws_url).await?;
        let (mut write, mut read) = ws_stream.split();

        // 创建消息通道
        let (client_tx, mut client_rx) = mpsc::channel::<ClientMessage>(100);
        let (server_tx, server_rx) = mpsc::channel::<ServerMessage>(100);

        // 保存发送器
        *self.sender.write().await = Some(client_tx);
        *self.connected.write().await = true;

        let connected = self.connected.clone();

        // 发送任务
        tokio::spawn(async move {
            while let Some(msg) = client_rx.recv().await {
                let json = serde_json::to_string(&msg).unwrap();
                if write.send(Message::Text(json)).await.is_err() {
                    break;
                }
            }
        });

        // 接收任务
        tokio::spawn(async move {
            while let Some(Ok(msg)) = read.next().await {
                if let Message::Text(text) = msg {
                    if let Ok(server_msg) = serde_json::from_str::<ServerMessage>(&text) {
                        let _ = server_tx.send(server_msg).await;
                    }
                }
            }
            *connected.write().await = false;
        });

        Ok(server_rx)
    }

    /// 发送消息到服务端
    pub async fn send(&self, msg: ClientMessage) -> Result<()> {
        if let Some(sender) = self.sender.read().await.as_ref() {
            sender.send(msg).await.map_err(|e| anyhow!("Send error: {}", e))?;
        }
        Ok(())
    }

    /// 上报设备状态
    pub async fn report_device_status(&self, device_id: &str, status: &str) -> Result<()> {
        self.send(ClientMessage::DeviceStatus {
            device_id: device_id.to_string(),
            status: status.to_string(),
        }).await
    }

    /// 上报脚本执行结果
    pub async fn report_script_result(&self, task_id: &str, result: &str) -> Result<()> {
        self.send(ClientMessage::ScriptResult {
            task_id: task_id.to_string(),
            result: result.to_string(),
        }).await
    }

    /// 检查连接状态
    pub async fn is_connected(&self) -> bool {
        *self.connected.read().await
    }

    /// 断开连接
    pub async fn disconnect(&self) {
        *self.sender.write().await = None;
        *self.connected.write().await = false;
    }
}
```

---

## 四、Tauri Commands

```rust
// src-tauri/src/commands/browser.rs

use tauri::State;
use crate::managers::browser_manager::BrowserManager;
use crate::managers::cdp_manager::CDPManager;
use crate::models::profile::Profile;
use crate::models::device::Device;
use crate::state::AppState;

/// 启动浏览器
#[tauri::command]
pub async fn start_browser(
    mut profile: Profile,
    state: State<'_, AppState>,
) -> Result<Device, String> {
    state.browser_manager
        .start(&mut profile)
        .await
        .map_err(|e| e.to_string())
}

/// 停止浏览器
#[tauri::command]
pub async fn stop_browser(
    profile_id: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    state.browser_manager
        .stop(&profile_id)
        .await
        .map_err(|e| e.to_string())
}

/// 获取所有运行中的设备
#[tauri::command]
pub async fn get_devices(
    state: State<'_, AppState>,
) -> Result<Vec<Device>, String> {
    Ok(state.browser_manager.get_all_devices().await)
}

/// 执行脚本
#[tauri::command]
pub async fn execute_script(
    profile_id: String,
    script: String,
    state: State<'_, AppState>,
) -> Result<String, String> {
    state.cdp_manager
        .execute_script(&profile_id, &script)
        .await
        .map_err(|e| e.to_string())
}

/// 导航到URL
#[tauri::command]
pub async fn navigate_to(
    profile_id: String,
    url: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    state.cdp_manager
        .navigate(&profile_id, &url)
        .await
        .map_err(|e| e.to_string())
}

/// 截图
#[tauri::command]
pub async fn take_screenshot(
    profile_id: String,
    state: State<'_, AppState>,
) -> Result<Vec<u8>, String> {
    state.cdp_manager
        .screenshot(&profile_id)
        .await
        .map_err(|e| e.to_string())
}
```

```rust
// src-tauri/src/commands/window.rs

use tauri::State;
use crate::state::AppState;

/// 显示窗口
#[tauri::command]
pub async fn show_window(
    profile_id: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    state.window_manager
        .show(&profile_id)
        .await
        .map_err(|e| e.to_string())
}

/// 隐藏窗口
#[tauri::command]
pub async fn hide_window(
    profile_id: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    state.window_manager
        .hide(&profile_id)
        .await
        .map_err(|e| e.to_string())
}

/// 排列窗口
#[tauri::command]
pub async fn arrange_windows(
    cols: u32,
    state: State<'_, AppState>,
) -> Result<(), String> {
    state.window_manager
        .arrange_grid(cols, 1920, 1080)
        .await
        .map_err(|e| e.to_string())
}

/// 隐藏所有窗口
#[tauri::command]
pub async fn hide_all_windows(
    state: State<'_, AppState>,
) -> Result<(), String> {
    state.window_manager
        .hide_all()
        .await
        .map_err(|e| e.to_string())
}

/// 显示所有窗口
#[tauri::command]
pub async fn show_all_windows(
    state: State<'_, AppState>,
) -> Result<(), String> {
    state.window_manager
        .show_all()
        .await
        .map_err(|e| e.to_string())
}
```

---

## 五、前端代码

### 5.1 TypeScript类型定义

```typescript
// src/types/index.ts

export interface Fingerprint {
  seed: number;
  platform: string;
  brand: string;
  hardware_concurrency: number;
  device_memory: number;
  gpu_vendor: string;
  gpu_renderer: string;
  screen_width: number;
  screen_height: number;
  timezone: string;
  language: string;
}

export interface ProxyConfig {
  proxy_type: string;
  host: string;
  port: number;
  username?: string;
  password?: string;
}

export interface Profile {
  id: string;
  name: string;
  data_dir: string;
  cdp_port: number;
  fingerprint: Fingerprint;
  proxy?: ProxyConfig;
  extra_args: string[];
  created_at: number;
  updated_at: number;
}

export type DeviceStatus = 'Starting' | 'Running' | 'Stopping' | 'Stopped' | 'Error';

export interface Device {
  profile_id: string;
  local_id: string;
  device_id?: string;
  pid: number;
  cdp_port: number;
  window_handle?: number;
  status: DeviceStatus;
  visible: boolean;
  ws_online: boolean;
  current_url?: string;
  started_at: number;
  last_heartbeat: number;
}
```

### 5.2 API封装

```typescript
// src/api/browser.ts

import { invoke } from '@tauri-apps/api/tauri';
import type { Profile, Device } from '@/types';

export const browserApi = {
  // 启动浏览器
  async start(profile: Profile): Promise<Device> {
    return invoke('start_browser', { profile });
  },

  // 停止浏览器
  async stop(profileId: string): Promise<void> {
    return invoke('stop_browser', { profileId });
  },

  // 获取所有设备
  async getDevices(): Promise<Device[]> {
    return invoke('get_devices');
  },

  // 执行脚本
  async executeScript(profileId: string, script: string): Promise<string> {
    return invoke('execute_script', { profileId, script });
  },

  // 导航到URL
  async navigateTo(profileId: string, url: string): Promise<void> {
    return invoke('navigate_to', { profileId, url });
  },

  // 截图
  async screenshot(profileId: string): Promise<Uint8Array> {
    return invoke('take_screenshot', { profileId });
  },
};

export const windowApi = {
  // 显示窗口
  async show(profileId: string): Promise<void> {
    return invoke('show_window', { profileId });
  },

  // 隐藏窗口
  async hide(profileId: string): Promise<void> {
    return invoke('hide_window', { profileId });
  },

  // 排列窗口
  async arrange(cols: number): Promise<void> {
    return invoke('arrange_windows', { cols });
  },

  // 隐藏所有
  async hideAll(): Promise<void> {
    return invoke('hide_all_windows');
  },

  // 显示所有
  async showAll(): Promise<void> {
    return invoke('show_all_windows');
  },
};
```

### 5.3 设备列表页面

```vue
<!-- src/views/DeviceList.vue -->

<template>
  <div class="device-list">
    <!-- 工具栏 -->
    <div class="toolbar">
      <el-button type="primary" @click="createDevice">
        <el-icon><Plus /></el-icon>
        新建设备
      </el-button>
      <el-button @click="batchCreate">批量创建</el-button>
      <el-button @click="arrangeWindows">排列窗口</el-button>
      <el-button @click="hideAllWindows">全部隐藏</el-button>
      <el-button @click="showAllWindows">全部显示</el-button>
      <el-button type="danger" @click="stopAll">停止全部</el-button>
    </div>

    <!-- 设备卡片列表 -->
    <div class="device-grid">
      <DeviceCard
        v-for="device in devices"
        :key="device.profile_id"
        :device="device"
        :profile="getProfile(device.profile_id)"
        @toggle-visible="toggleVisible"
        @execute-script="openScriptDialog"
        @navigate="openNavigateDialog"
        @stop="stopDevice"
      />
    </div>

    <!-- 创建设备对话框 -->
    <el-dialog v-model="createDialogVisible" title="新建设备" width="600px">
      <ProfileEditor v-model="newProfile" />
      <template #footer>
        <el-button @click="createDialogVisible = false">取消</el-button>
        <el-button type="primary" @click="confirmCreate">确认创建</el-button>
      </template>
    </el-dialog>

    <!-- 执行脚本对话框 -->
    <el-dialog v-model="scriptDialogVisible" title="执行脚本" width="600px">
      <el-input
        v-model="scriptContent"
        type="textarea"
        :rows="10"
        placeholder="输入JavaScript代码"
      />
      <template #footer>
        <el-button @click="scriptDialogVisible = false">取消</el-button>
        <el-button type="primary" @click="executeScript">执行</el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue';
import { ElMessage } from 'element-plus';
import { Plus } from '@element-plus/icons-vue';
import { browserApi, windowApi } from '@/api/browser';
import { useDeviceStore } from '@/stores/device';
import { useProfileStore } from '@/stores/profile';
import DeviceCard from '@/components/DeviceCard.vue';
import ProfileEditor from '@/components/ProfileEditor.vue';
import type { Profile, Device } from '@/types';

const deviceStore = useDeviceStore();
const profileStore = useProfileStore();

const devices = ref<Device[]>([]);
const createDialogVisible = ref(false);
const scriptDialogVisible = ref(false);
const newProfile = ref<Profile>(createEmptyProfile());
const scriptContent = ref('');
const currentDeviceId = ref('');

// 轮询设备状态
let pollTimer: number;

onMounted(() => {
  loadDevices();
  pollTimer = setInterval(loadDevices, 3000);
});

onUnmounted(() => {
  clearInterval(pollTimer);
});

async function loadDevices() {
  try {
    devices.value = await browserApi.getDevices();
  } catch (e) {
    console.error('Failed to load devices:', e);
  }
}

function getProfile(profileId: string): Profile | undefined {
  return profileStore.profiles.find(p => p.id === profileId);
}

function createEmptyProfile(): Profile {
  return {
    id: '',
    name: `设备-${Date.now()}`,
    data_dir: '',
    cdp_port: 0,
    fingerprint: {
      seed: Math.floor(Math.random() * 99999999),
      platform: 'windows',
      brand: 'Chrome',
      hardware_concurrency: 8,
      device_memory: 8,
      gpu_vendor: 'NVIDIA Corporation',
      gpu_renderer: 'NVIDIA GeForce GTX 1660',
      screen_width: 1920,
      screen_height: 1080,
      timezone: 'Asia/Shanghai',
      language: 'zh-CN',
    },
    extra_args: [],
    created_at: 0,
    updated_at: 0,
  };
}

async function createDevice() {
  newProfile.value = createEmptyProfile();
  createDialogVisible.value = true;
}

async function confirmCreate() {
  try {
    const device = await browserApi.start(newProfile.value);
    devices.value.push(device);
    profileStore.addProfile(newProfile.value);
    createDialogVisible.value = false;
    ElMessage.success('设备创建成功');
  } catch (e) {
    ElMessage.error(`创建失败: ${e}`);
  }
}

async function batchCreate() {
  // 批量创建逻辑
}

async function toggleVisible(device: Device) {
  try {
    if (device.visible) {
      await windowApi.hide(device.profile_id);
    } else {
      await windowApi.show(device.profile_id);
    }
    device.visible = !device.visible;
  } catch (e) {
    ElMessage.error(`操作失败: ${e}`);
  }
}

async function arrangeWindows() {
  try {
    await windowApi.arrange(4);
    ElMessage.success('窗口排列完成');
  } catch (e) {
    ElMessage.error(`排列失败: ${e}`);
  }
}

async function hideAllWindows() {
  await windowApi.hideAll();
}

async function showAllWindows() {
  await windowApi.showAll();
}

async function stopDevice(device: Device) {
  try {
    await browserApi.stop(device.profile_id);
    devices.value = devices.value.filter(d => d.profile_id !== device.profile_id);
    ElMessage.success('设备已停止');
  } catch (e) {
    ElMessage.error(`停止失败: ${e}`);
  }
}

async function stopAll() {
  for (const device of devices.value) {
    await browserApi.stop(device.profile_id);
  }
  devices.value = [];
}

function openScriptDialog(device: Device) {
  currentDeviceId.value = device.profile_id;
  scriptContent.value = '';
  scriptDialogVisible.value = true;
}

async function executeScript() {
  try {
    const result = await browserApi.executeScript(currentDeviceId.value, scriptContent.value);
    ElMessage.success(`执行结果: ${result}`);
    scriptDialogVisible.value = false;
  } catch (e) {
    ElMessage.error(`执行失败: ${e}`);
  }
}

function openNavigateDialog(device: Device) {
  // 导航对话框逻辑
}
</script>

<style scoped>
.device-list {
  padding: 20px;
}

.toolbar {
  margin-bottom: 20px;
  display: flex;
  gap: 10px;
}

.device-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
  gap: 16px;
}
</style>
```

---

## 六、配置文件

### 6.1 Cargo.toml

```toml
[package]
name = "browser-manager"
version = "0.1.0"
edition = "2021"

[dependencies]
# Tauri
tauri = { version = "2.0", features = ["shell-open"] }
tauri-plugin-shell = "2.0"

# 异步运行时
tokio = { version = "1", features = ["full"] }
futures = "0.3"

# CDP
chromiumoxide = { version = "0.7", features = ["tokio-runtime"] }

# WebSocket
tokio-tungstenite = { version = "0.21", features = ["native-tls"] }

# 序列化
serde = { version = "1", features = ["derive"] }
serde_json = "1"

# Windows API
[target.'cfg(windows)'.dependencies]
windows = { version = "0.52", features = [
    "Win32_Foundation",
    "Win32_UI_WindowsAndMessaging",
    "Win32_System_Threading",
]}

# 工具库
anyhow = "1"
thiserror = "1"
tracing = "0.1"
tracing-subscriber = "0.3"
uuid = { version = "1", features = ["v4"] }
chrono = "0.4"
rand = "0.8"
dirs = "5"

[build-dependencies]
tauri-build = "2.0"

[profile.release]
lto = true
codegen-units = 1
strip = true
```

### 6.2 tauri.conf.json

```json
{
  "$schema": "https://schema.tauri.app/config/2.0.0",
  "productName": "Browser Manager",
  "version": "0.1.0",
  "identifier": "com.browser-manager.app",
  "build": {
    "beforeDevCommand": "npm run dev",
    "devUrl": "http://localhost:5173",
    "beforeBuildCommand": "npm run build",
    "frontendDist": "../dist"
  },
  "app": {
    "windows": [
      {
        "title": "浏览器多开管理器",
        "width": 1280,
        "height": 800,
        "resizable": true,
        "fullscreen": false
      }
    ],
    "security": {
      "csp": null
    }
  },
  "bundle": {
    "active": true,
    "targets": "all",
    "icon": [
      "icons/32x32.png",
      "icons/128x128.png",
      "icons/128x128@2x.png",
      "icons/icon.icns",
      "icons/icon.ico"
    ],
    "windows": {
      "wix": {
        "language": "zh-CN"
      }
    }
  }
}
```

---

## 七、开发步骤

### 7.1 环境准备

```bash
# 1. 安装 Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 2. 安装 Node.js (v18+)
# 下载: https://nodejs.org/

# 3. 安装 Tauri CLI
cargo install tauri-cli

# 4. 下载 fingerprint-chromium
# https://github.com/adryfish/fingerprint-chromium/releases
# 解压到项目 browsers/ 目录
```

### 7.2 创建项目

```bash
# 1. 创建项目
npm create tauri-app@latest browser-manager -- --template vue-ts

cd browser-manager

# 2. 安装前端依赖
npm install
npm install element-plus @element-plus/icons-vue pinia

# 3. 添加 Rust 依赖 (编辑 src-tauri/Cargo.toml)

# 4. 运行开发环境
npm run tauri dev
```

### 7.3 开发流程

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│                              开发流程                                           │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  阶段1 (第1周): 基础框架                                                        │
│  ┌─────────────────────────────────────────────────────────────────────────┐   │
│  │ • 搭建 Tauri + Vue 项目结构                                             │   │
│  │ • 实现 Profile 数据模型和持久化                                          │   │
│  │ • 实现基础 UI 界面                                                       │   │
│  └─────────────────────────────────────────────────────────────────────────┘   │
│                                                                                 │
│  阶段2 (第2周): 浏览器控制                                                      │
│  ┌─────────────────────────────────────────────────────────────────────────┐   │
│  │ • 集成 chromiumoxide CDP 库                                             │   │
│  │ • 实现浏览器启动/停止                                                    │   │
│  │ • 实现 CDP 脚本执行                                                      │   │
│  └─────────────────────────────────────────────────────────────────────────┘   │
│                                                                                 │
│  阶段3 (第3周): 窗口管理                                                        │
│  ┌─────────────────────────────────────────────────────────────────────────┐   │
│  │ • 集成 windows-rs 窗口控制                                               │   │
│  │ • 实现窗口显示/隐藏/排列                                                 │   │
│  │ • 实现窗口网格布局                                                       │   │
│  └─────────────────────────────────────────────────────────────────────────┘   │
│                                                                                 │
│  阶段4 (第4周): 服务端对接                                                      │
│  ┌─────────────────────────────────────────────────────────────────────────┐   │
│  │ • 实现 WebSocket 客户端                                                  │   │
│  │ • 对接现有 Go 服务端                                                     │   │
│  │ • 实现脚本下发和结果回传                                                 │   │
│  └─────────────────────────────────────────────────────────────────────────┘   │
│                                                                                 │
│  阶段5 (第5周): 测试和优化                                                      │
│  ┌─────────────────────────────────────────────────────────────────────────┐   │
│  │ • 指纹检测测试                                                          │   │
│  │ • 性能优化                                                               │   │
│  │ • 打包发布                                                               │   │
│  └─────────────────────────────────────────────────────────────────────────┘   │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

---

## 八、打包发布

### 8.1 打包命令

```bash
# 开发构建
npm run tauri dev

# 生产构建
npm run tauri build

# 输出目录
# Windows: src-tauri/target/release/bundle/msi/
# 或: src-tauri/target/release/browser-manager.exe
```

### 8.2 预期打包大小

| 组件 | 大小 |
|------|------|
| Tauri EXE | ~3-5MB |
| fingerprint-chromium | ~150MB |
| 总计 | ~155MB |

对比 Electron 方案约 200-300MB，Tauri 方案显著更小。

---

## 九、与 Go 服务端通信协议

### 9.1 消息格式

```typescript
// 客户端 → 服务端
interface ClientMessage {
  type: 'device_hello' | 'device_status' | 'script_result' | 'heartbeat';
  payload: any;
}

// 服务端 → 客户端
interface ServerMessage {
  type: 'execute_script' | 'navigate' | 'config_update' | 'task_assign';
  payload: any;
}
```

### 9.2 示例消息

```json
// 设备上线
{
  "type": "device_hello",
  "payload": {
    "local_id": "local-1706000000-12345",
    "profile_id": "uuid-xxx",
    "fingerprint": { ... }
  }
}

// 执行脚本
{
  "type": "execute_script",
  "payload": {
    "task_id": "task-001",
    "device_id": "device-xxx",
    "script": "document.title"
  }
}

// 脚本结果
{
  "type": "script_result",
  "payload": {
    "task_id": "task-001",
    "result": "Google",
    "success": true
  }
}
```

---

## 十、总结

### 10.1 技术优势

| 优势 | 说明 |
|------|------|
| **极小打包体积** | ~5MB vs Electron ~200MB |
| **低内存占用** | 管理器仅 ~20MB |
| **高性能** | Rust 原生性能 |
| **类型安全** | Rust + TypeScript 双重保障 |
| **原生窗口控制** | windows-rs 直接调用 Win32 API |
| **异步并发** | Tokio 高性能异步运行时 |

### 10.2 完整技术栈

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│                           完整技术栈                                            │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  前端: Vue 3 + TypeScript + Element Plus + Pinia                               │
│                                                                                 │
│  后端: Rust + Tauri 2.0 + Tokio                                                │
│                                                                                 │
│  CDP控制: chromiumoxide                                                         │
│                                                                                 │
│  窗口控制: windows-rs (Win32 API)                                               │
│                                                                                 │
│  通信: tokio-tungstenite (WebSocket)                                           │
│                                                                                 │
│  浏览器: fingerprint-chromium (预编译版)                                        │
│                                                                                 │
│  服务端: Go + gorilla/websocket (已有)                                         │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

### 10.3 下一步

1. **下载 fingerprint-chromium** 预编译版本
2. **创建 Tauri 项目** 按照上述结构
3. **实现核心模块** 按照阶段计划
4. **对接 Go 服务端** WebSocket 通信
5. **测试指纹隔离** 在检测网站验证

---

如有问题，请参考：
- Tauri 文档: https://tauri.app/
- chromiumoxide 文档: https://docs.rs/chromiumoxide
- fingerprint-chromium: https://github.com/adryfish/fingerprint-chromium