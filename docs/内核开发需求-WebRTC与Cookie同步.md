# 内核开发需求：WebRTC 替换模式 + Cookie 同步优化

> **创建日期**: 2026-02-01  
> **优先级**: P0（商业级产品核心功能）  
> **依赖**: Chromium 146 源码环境

---

## 一、需求背景

当前启动器层面已支持 WebRTC 的三种模式配置（`real | fake | disabled`），但这些仅是启动参数级别的控制。商业级指纹浏览器（如 Multilogin、AdsPower）采用的是 **源码级替换模式**，安全性更高。

---

## 二、需求详情

### 2.1 WebRTC 指纹优化 - 替换模式（Replace）

**问题现状**：
- 当前 `disabled` 模式完全禁用 WebRTC，容易被检测为"隐私模式"
- `fake` 模式依赖命令行参数，不够底层

**商业级方案**：
```
┌─────────────────────────────────────────────────────────────┐
│  WebRTC 替换模式 (Replace Mode)                             │
├─────────────────────────────────────────────────────────────┤
│  1. 保留 WebRTC 功能正常工作（网站检测不到禁用）            │
│  2. 替换 ICE Candidate 中的真实 IP                         │
│  3. 本地 IP → 随机化私有 IP（如 192.168.x.x）              │
│  4. 公网 IP → 代理出口 IP（如果使用代理）                  │
│  5. 保持 STUN/TURN 正常工作但返回伪装数据                  │
└─────────────────────────────────────────────────────────────┘
```

**修改位置**（Chromium 源码）：
```cpp
// third_party/blink/renderer/modules/peerconnection/
// - rtc_peer_connection.cc
// - rtc_ice_candidate.cc

// content/browser/media/
// - webrtc_internals.cc

// net/
// - stun_client.cc
```

**配置文件字段**：
```json
{
  "webrtc": {
    "mode": "replace",              // "real" | "replace" | "disabled"
    "public_ip_policy": "proxy_only", // "default" | "proxy_only" | "custom"
    "custom_public_ip": null,         // 自定义公网 IP（可选）
    "local_ip_policy": "randomize",   // "default" | "randomize" | "hide"
    "custom_local_ip": null           // 自定义本地 IP（可选）
  }
}
```

**检测对抗**：
| 检测方式 | disabled 模式 | replace 模式（目标） |
|----------|---------------|---------------------|
| `RTCPeerConnection` 存在性 | ❌ 被发现禁用 | ✅ 正常存在 |
| `createOffer/createAnswer` | ❌ 失败 | ✅ 正常工作 |
| ICE Candidate 获取 | ❌ 无结果 | ✅ 返回伪装 IP |
| WebRTC 功能检测 | ❌ 异常 | ✅ 完全正常 |

---

### 2.2 标签页与 Cookie 同步问题

**问题现状**：
- 同一 Profile 下打开多个标签页时，部分情况下 Cookie 不同步
- 可能原因：Profile 数据目录隔离机制、Session Storage 冲突

**排查方向**：
```
1. Profile 启动参数
   --user-data-dir 是否正确指向同一目录
   
2. Cookie 存储路径
   {user-data-dir}/Default/Cookies (SQLite)
   
3. Session 隔离
   检查是否意外启用了 --incognito 或 --guest
   
4. 多进程架构
   检查 Renderer 进程间 IPC 通信
```

**内核修改点**：
```cpp
// chrome/browser/profiles/profile_impl.cc
// - InitCookieStore()

// net/cookies/
// - cookie_store.cc
// - canonical_cookie.cc

// content/browser/
// - storage_partition_impl.cc
```

**同步机制优化**：
```
┌─────────────────────────────────────────────────────────────┐
│  Cookie 同步流程（需确认/优化）                             │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  Tab A (Renderer)  ←──IPC──→  Browser Process  ←──→ SQLite │
│                                    ↑                        │
│  Tab B (Renderer)  ←──IPC──────────┘                        │
│                                    ↑                        │
│  Tab C (Renderer)  ←──IPC──────────┘                        │
│                                                             │
│  确保: 所有 Tab 共享同一 CookieStore 实例                   │
└─────────────────────────────────────────────────────────────┘
```

---

## 三、实现优先级

| 序号 | 功能 | 优先级 | 工期预估 | 说明 |
|------|------|--------|----------|------|
| 1 | WebRTC Replace 模式 | P0 | 3-5 天 | 商业级核心功能 |
| 2 | Cookie 同步排查与修复 | P0 | 1-2 天 | 先定位问题根因 |
| 3 | ICE Candidate 伪装 | P1 | 2-3 天 | 依赖 #1 |
| 4 | STUN/TURN 响应伪装 | P2 | 2-3 天 | 高级功能 |

---

## 四、测试验证

### 4.1 WebRTC 测试网站
```
https://browserleaks.com/webrtc
https://ipleak.net/
https://whoer.net/
https://www.expressvpn.com/webrtc-leak-test
```

### 4.2 验收标准
```
✅ WebRTC 功能检测显示"正常"（非禁用）
✅ ICE Candidate 不泄露真实 IP
✅ 使用代理时，公网 IP 显示为代理出口
✅ 本地 IP 为随机化私有地址
✅ 多标签页 Cookie 完全同步
✅ 登录状态跨标签页保持一致
```

---

## 五、参考资料

### 5.1 商业产品实现参考
- **Multilogin**: 替换真实指纹，不是隐藏
- **AdsPower**: 收集真实指纹变量，生成可靠的独特指纹
- **比特浏览器**: WebRTC 替换模式 + 自定义 IP

### 5.2 开源项目参考
- **BotBrowser**: `--bot-webrtc-ice=<policy>` 命令行参数
- **puppeteer-extra-stealth**: `webrtc.transpor` 方式（JS 层面，不够安全）

### 5.3 Chromium 源码参考路径
```
src/
├── third_party/blink/renderer/modules/peerconnection/
│   ├── rtc_peer_connection.cc        # RTCPeerConnection 实现
│   ├── rtc_ice_candidate.cc          # ICE Candidate 处理
│   └── rtc_data_channel.cc
├── content/browser/media/
│   └── webrtc_internals.cc           # WebRTC 内部管理
├── net/
│   ├── stun_client.cc                # STUN 客户端
│   └── turn_client.cc                # TURN 客户端
└── chrome/browser/profiles/
    └── profile_impl.cc               # Profile 初始化
```

---

## 六、启动器侧配套更新

内核完成后，启动器需要配套更新：

### 6.1 类型定义更新
```typescript
// src/types/profile.types.ts
webrtc: 'real' | 'replace' | 'disabled'  // 新增 'replace'
webrtcMode?: 'real' | 'replace' | 'disabled'  // 兼容字段
```

### 6.2 配置写入更新
```rust
// src-tauri/src/modules/config_writer.rs
pub struct WebRTCConfig {
    pub mode: String,           // "real" | "replace" | "disabled"
    pub public_ip_policy: String,
    pub custom_public_ip: Option<String>,
    pub local_ip_policy: String,
    pub custom_local_ip: Option<String>,
}
```

### 6.3 UI 更新
```
高级指纹设置 → WebRTC 下拉框：
- 替换模式 (Replace) - 推荐 ← 默认选项
- 真实 (Real)
- 禁用 (Disabled)
```

---

## 七、SOCKS5 UDP ASSOCIATE 支持（已完成）

### 7.1 启动器侧实现（✅ 已完成）

**修改文件**：`proxy_bridge.rs`

```rust
// 新增配置字段
pub struct ProxyBridgeConfig {
    // ... 原有字段
    pub enable_udp: bool,  // 启用 UDP 支持
}

// 新增统计字段
pub struct BridgeStats {
    // ... 原有字段
    pub udp_packets_sent: u64,
    pub udp_packets_received: u64,
}

// 新增方法
async fn establish_udp_associate(config: &ProxyBridgeConfig) 
    -> Result<(TcpStream, String), String>

async fn start_udp_relay(&self) -> Result<(), String>
```

**工作流程**：
```
┌─────────────────────────────────────────────────────────────┐
│  浏览器 WebRTC UDP 请求                                     │
│       ↓                                                     │
│  本地 UDP 端口 (127.0.0.1:50002)                           │
│       ↓                                                     │
│  SOCKS5 UDP ASSOCIATE (CMD=0x03)                           │
│       ↓                                                     │
│  中继服务器 (proxy:relay_port)                             │
│       ↓                                                     │
│  ✅ UDP 流量通过代理转发，IP 安全                          │
└─────────────────────────────────────────────────────────────┘
```

### 7.2 前端 UI 更新（✅ 已完成）

**修改文件**：
- `useCreateProfile.ts` - 添加 `enableProxyUdp` 字段
- `Step4ProxySettings.vue` - 添加 UDP 开关（仅 SOCKS5 显示）

**UI 效果**：
```
代理协议: [SOCKS5] [HTTP] [HTTPS]

代理服务器: [________:____]
认证信息:   [用户名] [密码]

UDP 支持 ⓘ: [✓] 已启用（WebRTC UDP 将通过代理）
            [ ] 未启用（WebRTC UDP 将被禁用）
```

---

## 八、备注

> 此文档为内核开发需求说明，启动器侧已准备好配置传递机制。  
> 待内核实现完成后，可快速对接集成。
