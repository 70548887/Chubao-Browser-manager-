# Chromium 146 源码级指纹浏览器开发方案

> 基于本地仓库开源项目深度分析，提供可实施的 Chromium 底层修改开发方案

---

## 一、仓库项目分析总览

### 1.1 已克隆项目清单

| 项目 | 技术路线 | 参考价值 | 核心内容 |
|------|---------|---------|---------|
| **fingerprint-chromium** | 命令行参数控制 | ⭐⭐⭐⭐⭐ | 完整的启动参数体系，支持 Canvas/WebGL/UA/时区 |
| **openFpchromium-114** | Chromium 源码 Patch | ⭐⭐⭐⭐⭐ | 包含完整的 Blink 渲染引擎修改文件 |
| **chromium_fake_fingerprint** | 完整 Chromium 源码 | ⭐⭐⭐⭐⭐ | 包含 `fakefingerprint` 模块，Mojo IPC 通信 |
| **browser_fingerprint** | 独立指纹库 | ⭐⭐⭐⭐ | 精简的指纹核心代码，易于移植 |
| **BotBrowser** | 商业级方案 | ⭐⭐⭐⭐⭐ | Patch 文件、CLI 参数、Profile 加密配置 |
| **VirtualBrowser** | Vue + 后端管理 | ⭐⭐⭐⭐ | 完整的 UI 管理系统参考 |
| **puppeteer-extra-stealth** | JS 注入层 | ⭐⭐⭐ | 反检测逻辑参考（非底层修改） |

---

## 二、核心源码修改架构

### 2.1 Chromium 源码目录结构（需修改的关键位置）

```
chromium/src/
├── base/
│   └── singleton_fingerprint.h      # [NEW] 指纹配置单例
│
├── third_party/blink/
│   ├── public/common/fingerprint/
│   │   └── fingerprint.h            # [NEW] 指纹数据结构定义
│   │
│   └── renderer/
│       ├── core/
│       │   ├── html/canvas/
│       │   │   └── html_canvas_element_fp.h  # [MODIFY] Canvas 指纹注入
│       │   ├── frame/
│       │   │   └── navigator.cc     # [MODIFY] Navigator 对象伪装
│       │   └── timezone/
│       │       └── timezone_controller.cc  # [MODIFY] 时区伪装
│       │
│       └── modules/
│           ├── canvas/canvas2d/
│           │   ├── canvas_rendering_context_2d.cc     # [MODIFY]
│           │   └── canvas_rendering_context_2d_fp.h   # [NEW]
│           ├── webgl/
│           │   ├── webgl_rendering_context_base.cc    # [MODIFY]
│           │   └── webgl_rendering_context_base_fp.h  # [NEW]
│           └── webaudio/
│               └── audio_destination_node.cc  # [MODIFY] 音频指纹
│
├── content/
│   └── browser/
│       └── fingerprint/             # [NEW] 指纹管理模块
│
└── chrome/
    └── browser/
        └── fingerprint_switches.cc  # [NEW] 命令行参数注册
```

---

## 三、指纹配置系统设计

### 3.1 指纹配置存储架构

参考 `browser_fingerprint` 项目的 `Settings` 类设计：

```
┌─────────────────────────────────────────────────────────────┐
│                    指纹配置加载流程                          │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  启动参数 ──┬──> FingerprintContext (单例)                  │
│             │                                                │
│  配置文件 ──┤──> Settings (键值存储)                         │
│             │                                                │
│  Profile ───┘──> 各模块读取配置                              │
│                                                             │
│  ┌─────────────────────────────────────────────────────┐   │
│  │ Navigator │ Canvas │ WebGL │ Audio │ Screen │ ...  │   │
│  └─────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────┘
```

### 3.2 核心数据结构（参考 openFpchromium-114）

**位置**: `third_party/blink/public/common/fingerprint/fingerprint.h`

```cpp
namespace blink {
namespace fp {

// 指纹颜色点（用于 Canvas/WebGL 噪声注入）
struct ColoredPoint {
  int x;
  int y;
  uint8_t r, g, b, a;
};

// WebGL 指纹配置
struct WebGLFingerprint {
  int type;  // 0: 禁用, 1: 随机, 2: 自定义
  std::string vendor;
  std::string renderer;
  std::vector<ColoredPoint> coloredPointList;
};

// Canvas 指纹配置
struct CanvasFingerprint {
  int type;
  std::vector<ColoredPoint> coloredPointList;
};

// 完整指纹配置
struct Fingerprint {
  // Navigator
  std::string platform;
  std::string userAgent;
  int hardwareConcurrency;
  int deviceMemory;
  std::string language;
  std::vector<std::string> languages;
  
  // Screen
  int screenWidth;
  int screenHeight;
  int colorDepth;
  int availWidth;
  int availHeight;
  
  // Graphics
  CanvasFingerprint canvas;
  WebGLFingerprint webGL;
  
  // Audio
  float audioNoiseLevel;
  
  // Time
  std::string timezone;
  int timezoneOffset;
  
  // WebRTC
  std::string webrtcPolicy;  // "disabled", "default", "proxy_only"
  
  // Proxy
  std::string proxyServer;
};

}  // namespace fp
}  // namespace blink
```

### 3.3 配置单例管理器（参考 browser_fingerprint）

**位置**: `base/singleton_fingerprint.h`

```cpp
namespace base {

class SingletonFingerprint {
 public:
  static SingletonFingerprint* ForCurrentProcess();
  static bool GetInit(SingletonFingerprint* instance);
  
  void Initialize(const blink::fp::Fingerprint& fp);
  const blink::fp::Fingerprint& GetFingerprint() const;
  
  // 配置加载
  bool LoadFromFile(const std::string& path);
  bool LoadFromCommandLine();
  
 private:
  SingletonFingerprint();
  ~SingletonFingerprint();
  
  blink::fp::Fingerprint fingerprint_;
  bool initialized_ = false;
};

}  // namespace base
```

---

## 四、核心指纹修改实现

### 4.1 Canvas 指纹修改

#### 4.1.1 实现原理

参考 `openFpchromium-114` 的 `canvas_rendering_context_2d.cc` 修改：

1. **噪声注入时机**: 在 `toDataURL()`、`getImageData()` 调用前
2. **噪声算法**: 对像素数据添加微小随机偏移
3. **一致性保证**: 使用 seed 确保同一会话内结果一致

#### 4.1.2 关键代码（参考 openFpchromium-114）

**文件**: `third_party/blink/renderer/modules/canvas/canvas2d/canvas_rendering_context_2d_fp.h`

```cpp
#ifndef CANVAS_RENDERING_CONTEXT_2D_FP_H_
#define CANVAS_RENDERING_CONTEXT_2D_FP_H_

#include "base/singleton_fingerprint.h"

namespace blink {
namespace fp {

// Canvas 全覆盖检测（用于判断是否需要注入噪声）
inline void fpFullCoverage(CanvasRenderingContext2D* context,
                           double x, double y,
                           double width, double height,
                           int canvasX, int canvasY,
                           int canvasWidth, int canvasHeight) {
  // 仅在全画布操作时触发噪声注入标记
  if (x <= canvasX && y <= canvasY && 
      width >= canvasWidth && height >= canvasHeight) {
    // 标记需要噪声注入
    context->SetNeedsNoiseInjection(true);
  }
}

// Canvas 像素噪声注入
inline void ApplyCanvasNoise(uint8_t* data, 
                              int width, int height,
                              const std::vector<ColoredPoint>& noisePoints) {
  for (const auto& point : noisePoints) {
    if (point.x >= 0 && point.x < width && 
        point.y >= 0 && point.y < height) {
      int offset = (point.y * width + point.x) * 4;  // RGBA
      data[offset + 0] = (data[offset + 0] + point.r) % 256;  // R
      data[offset + 1] = (data[offset + 1] + point.g) % 256;  // G
      data[offset + 2] = (data[offset + 2] + point.b) % 256;  // B
      // Alpha 通常保持不变
    }
  }
}

}  // namespace fp
}  // namespace blink

#endif
```

#### 4.1.3 需修改的函数

| 文件 | 函数 | 修改内容 |
|------|------|---------|
| `canvas_rendering_context_2d.cc` | `clearRect()` | 添加全覆盖检测调用 |
| `html_canvas_element.cc` | `toDataURL()` | 调用噪声注入 |
| `base_rendering_context_2d.cc` | `getImageData()` | 调用噪声注入 |

---

### 4.2 WebGL 指纹修改

#### 4.2.1 实现原理

参考 `browser_fingerprint` 和 `BotBrowser` 的实现：

1. **元数据伪装**: 修改 `GL_VENDOR`、`GL_RENDERER` 返回值
2. **图像噪声**: 在 `readPixels()` 结果中注入噪声
3. **扩展列表**: 随机排序或过滤扩展

#### 4.2.2 关键代码（参考 browser_fingerprint）

**文件**: `third_party/blink/renderer/modules/webgl/webgl_rendering_context_base_fp.h`

```cpp
#ifndef WEBGL_RENDERING_CONTEXT_BASE_FP_H_
#define WEBGL_RENDERING_CONTEXT_BASE_FP_H_

#include "base/singleton_fingerprint.h"
#include "third_party/blink/public/common/fingerprint/fingerprint.h"

namespace blink {

// WebGL readPixels 噪声注入
void fpReadPixelsHelper(GLint x, GLint y,
                        GLsizei width, GLsizei height,
                        GLenum format, GLenum type,
                        DOMArrayBufferView* pixels, int64_t offset,
                        int canvasWidth, int canvasHeight) {
  // 仅处理全画布读取
  if (x > 0 || y > 0 || width < canvasWidth || height < canvasHeight) {
    return;
  }
  
  // 仅处理 RGBA + UNSIGNED_BYTE 格式
  if (format != GL_RGBA || type != GL_UNSIGNED_BYTE) {
    return;
  }
  
  auto* singleton = base::SingletonFingerprint::ForCurrentProcess();
  if (!base::SingletonFingerprint::GetInit(singleton)) {
    return;
  }
  
  auto fingerprint = singleton->GetFingerprint();
  if (fingerprint.webGL.type <= 1) {
    return;
  }
  
  // 注入噪声点
  uint8_t* data = static_cast<uint8_t*>(pixels->BaseAddressMaybeShared()) + offset;
  for (const auto& point : fingerprint.webGL.coloredPointList) {
    if (point.x >= 0 && point.x < canvasWidth &&
        point.y >= 0 && point.y < canvasHeight) {
      int idx = (point.y * canvasWidth + point.x) * 4;
      data[idx + 0] = std::clamp(data[idx + 0] + point.r, 0, 255);
      data[idx + 1] = std::clamp(data[idx + 1] + point.g, 0, 255);
      data[idx + 2] = std::clamp(data[idx + 2] + point.b, 0, 255);
    }
  }
}

}  // namespace blink

#endif
```

#### 4.2.3 GPU 信息伪装（参考 BotBrowser patches）

**需修改**: `webgl_rendering_context_base.cc` 的 `getParameter()` 函数

```cpp
// 在 getParameter() 中添加拦截逻辑
case WebGLDebugRendererInfo::kUnmaskedVendorWebgl:
  if (ExtensionEnabled(kWebGLDebugRendererInfoName)) {
    // 原始逻辑
    auto vendor = String(ContextGL()->GetString(GL_VENDOR));
    
    // 指纹伪装
    auto* fp = base::SingletonFingerprint::ForCurrentProcess();
    if (base::SingletonFingerprint::GetInit(fp)) {
      auto& config = fp->GetFingerprint();
      if (!config.webGL.vendor.empty()) {
        vendor = String(config.webGL.vendor.c_str());
      }
    }
    
    return WebGLAny(script_state, vendor);
  }
  break;

case WebGLDebugRendererInfo::kUnmaskedRendererWebgl:
  // 类似逻辑处理 GL_RENDERER
  break;
```

---

### 4.3 Navigator 对象修改

#### 4.3.1 需修改的属性

| 属性 | 文件位置 | 修改方式 |
|------|---------|---------|
| `userAgent` | `navigator.cc` | 从配置读取替换 |
| `platform` | `navigator.cc` | 从配置读取替换 |
| `hardwareConcurrency` | `navigator_concurrent_hardware.cc` | 返回配置值 |
| `deviceMemory` | `navigator_device_memory.cc` | 返回配置值 |
| `language` / `languages` | `navigator_language.cc` | 从配置读取 |

#### 4.3.2 userAgent 修改示例

```cpp
// navigator.cc
String Navigator::userAgent() const {
  auto* fp = base::SingletonFingerprint::ForCurrentProcess();
  if (base::SingletonFingerprint::GetInit(fp)) {
    auto& config = fp->GetFingerprint();
    if (!config.userAgent.empty()) {
      return String(config.userAgent.c_str());
    }
  }
  
  // 回落到原始逻辑
  return GetFrame()->Loader().UserAgent();
}
```

---

### 4.4 时区伪装

#### 4.4.1 实现位置

参考 `BotBrowser` 的 `timezone.diff` 补丁：

**文件**: `third_party/blink/renderer/core/timezone/timezone_controller.cc`

```cpp
bool SetIcuTimeZoneAndNotifyV8(const String& timezone_id) {
  const char* timezoneIdChars;
  
  // 检查是否有伪装时区配置
  auto* fp = base::SingletonFingerprint::ForCurrentProcess();
  if (base::SingletonFingerprint::GetInit(fp)) {
    auto& config = fp->GetFingerprint();
    if (!config.timezone.empty()) {
      VLOG(1) << "Spoofing timezone: " << config.timezone;
      timezoneIdChars = config.timezone.c_str();
    } else {
      DCHECK(!timezone_id.empty());
      timezoneIdChars = timezone_id.Ascii().data();
    }
  } else {
    DCHECK(!timezone_id.empty());
    timezoneIdChars = timezone_id.Ascii().data();
  }
  
  std::unique_ptr<icu::TimeZone> timezone(icu::TimeZone::createTimeZone(
      icu::UnicodeString(timezoneIdChars, -1, US_INV)));
  CHECK(timezone);
  
  // ... 后续原始逻辑
}
```

---

### 4.5 WebGL 噪声注入库（参考 browser_fingerprint）

**文件**: `fingerprint/webgl_noise.h`

```cpp
namespace fingerprint {

constexpr float gkGLnoiseMinimum = 0.0001f;
constexpr float gkGLnoiseMaximum = 0.001f;
constexpr unsigned int gkGLnoiseFloatArraySize = 64u;

constexpr float gkGLclipSpaceMinimum = -1.0f;
constexpr float gkGLclipSpaceMaximum = 1.0f;

// 生成噪声数组
template <class FloatArray>
void GLnewNoiseFloatArray(FloatArray& out) {
  out.resize(gkGLnoiseFloatArraySize);
  FillRandomFloatArray(out.begin(), out.end(), 
                       gkGLnoiseMinimum, gkGLnoiseMaximum);
}

// 应用噪声到顶点数据
template <class Target, class Source, class Count, class NoiseFloatArray>
Target* GLapplyNoise(Target* target, 
                     const Source* source, 
                     Count count,
                     const NoiseFloatArray& noiseArray) {
  ApplyFloatNoise<NoiseFloatArray> transform{
    gkGLclipSpaceMinimum, gkGLclipSpaceMaximum, noiseArray
  };
  
  auto first = static_cast<const Target*>(source);
  auto last = first;
  std::advance(last, count);
  transform(first, last, target);
  
  return target;
}

}  // namespace fingerprint
```

---

## 五、命令行参数系统

### 5.1 参数定义（参考 fingerprint-chromium）

| 参数 | 说明 | 示例值 |
|------|------|--------|
| `--fingerprint` | 指纹种子 (核心) | `12345678` |
| `--fingerprint-platform` | 操作系统 | `windows`, `linux`, `macos` |
| `--fingerprint-brand` | 浏览器品牌 | `Chrome`, `Edge`, `Opera` |
| `--fingerprint-brand-version` | 品牌版本 | `142.0.0.0` |
| `--fingerprint-hardware-concurrency` | CPU 核心数 | `8` |
| `--fingerprint-device-memory` | 内存大小 (GB) | `16` |
| `--fingerprint-gpu-vendor` | GPU 厂商 | `NVIDIA Corporation` |
| `--fingerprint-gpu-renderer` | GPU 型号 | `NVIDIA GeForce RTX 3080` |
| `--timezone` | 时区 | `America/New_York` |
| `--lang` | 界面语言 | `zh-CN` |
| `--accept-lang` | 接受语言 | `zh-CN,en-US` |
| `--proxy-server` | 代理服务器 | `socks5://127.0.0.1:1080` |
| `--disable-gpu-fingerprint` | 禁用 GPU 指纹 | (无值) |
| `--disable-non-proxied-udp` | WebRTC 策略 | (无值) |

### 5.2 参数注册（chrome/browser/about_flags.cc 或独立文件）

```cpp
// chrome/browser/fingerprint_switches.cc

namespace switches {

// 核心指纹种子
const char kFingerprint[] = "fingerprint";

// 平台相关
const char kFingerprintPlatform[] = "fingerprint-platform";
const char kFingerprintPlatformVersion[] = "fingerprint-platform-version";

// 浏览器品牌
const char kFingerprintBrand[] = "fingerprint-brand";
const char kFingerprintBrandVersion[] = "fingerprint-brand-version";

// 硬件信息
const char kFingerprintHardwareConcurrency[] = "fingerprint-hardware-concurrency";
const char kFingerprintDeviceMemory[] = "fingerprint-device-memory";
const char kFingerprintGpuVendor[] = "fingerprint-gpu-vendor";
const char kFingerprintGpuRenderer[] = "fingerprint-gpu-renderer";

// 开关
const char kDisableGpuFingerprint[] = "disable-gpu-fingerprint";

}  // namespace switches
```

---

## 六、编译与构建流程

### 6.1 Windows 编译环境准备

```powershell
# 1. 安装 Visual Studio 2022 (包含 C++ 桌面开发工作负载)
# 2. 安装 Windows 10/11 SDK
# 3. 安装 depot_tools

git clone https://chromium.googlesource.com/chromium/tools/depot_tools.git
$env:PATH = "C:\depot_tools;$env:PATH"

# 4. 配置 depot_tools
gclient

# 5. 获取 Chromium 146 源码
mkdir chromium && cd chromium
fetch --nohooks chromium
cd src
git checkout tags/146.0.xxxx.xx

# 6. 同步依赖
gclient sync --with_branch_heads --with_tags

# 7. 应用自定义修改
# (将修改后的文件复制到对应位置)

# 8. 生成构建文件
gn gen out/Release --args='
  is_debug=false
  is_component_build=false
  symbol_level=0
  enable_nacl=false
  blink_symbol_level=0
  is_official_build=true
  chrome_pgo_phase=0
  target_cpu="x64"
'

# 9. 编译
autoninja -C out/Release chrome
```

### 6.2 Patch 文件管理

建议采用 Git Patch 方式管理修改（参考 BotBrowser）：

```bash
# 生成 Patch
git diff third_party/blink/renderer/modules/webgl/webgl_rendering_context_base.cc > patches/webgl.patch

# 应用 Patch
git apply patches/webgl.patch
```

---

## 七、开发优先级建议

### 7.1 第一阶段：核心指纹（2-3周）

| 优先级 | 模块 | 参考项目 | 预计工时 |
|--------|------|---------|---------|
| P0 | 命令行参数解析 | fingerprint-chromium | 2天 |
| P0 | 指纹配置单例 | browser_fingerprint | 2天 |
| P0 | Navigator 伪装 | 多个项目 | 3天 |
| P0 | Canvas 噪声注入 | openFpchromium-114 | 5天 |
| P0 | WebGL 元数据伪装 | browser_fingerprint | 3天 |

### 7.2 第二阶段：增强指纹（2-3周）

| 优先级 | 模块 | 参考项目 | 预计工时 |
|--------|------|---------|---------|
| P1 | WebGL 图像噪声 | browser_fingerprint | 5天 |
| P1 | 时区伪装 | BotBrowser | 2天 |
| P1 | AudioContext 噪声 | fingerprint-chromium | 4天 |
| P1 | Screen 属性伪装 | chromium_fake_fingerprint | 2天 |
| P2 | 字体列表伪装 | openFpchromium-114 | 4天 |

### 7.3 第三阶段：完善功能（2周）

| 优先级 | 模块 | 参考项目 | 预计工时 |
|--------|------|---------|---------|
| P2 | WebRTC 伪装 | 多个项目 | 3天 |
| P2 | ClientRects 噪声 | fingerprint-chromium | 2天 |
| P2 | 配置文件加密 | BotBrowser | 3天 |
| P3 | 自动化检测规避 | puppeteer-stealth | 2天 |

---

## 八、测试与验证

### 8.1 指纹检测网站

| 网站 | 检测重点 | 通过标准 |
|------|---------|---------|
| [CreepJS](https://abrahamjuliot.github.io/creepjs/) | 综合指纹 | Trust Score > 50% |
| [BrowserLeaks](https://browserleaks.com/) | Canvas/WebGL/Audio | 各项随机化 |
| [PixelScan](https://pixelscan.net/) | 音频/Canvas | 通过检测 |
| [BrowserScan](https://browserscan.net/) | GPU/字体 | 无异常标记 |
| [Cloudflare Turnstile](https://nopecha.com/demo/turnstile) | 人机验证 | 通过验证 |

### 8.2 一致性测试

```bash
# 使用相同 seed 多次启动，验证指纹一致
chrome.exe --fingerprint=12345 --user-data-dir=%TEMP%\test1
chrome.exe --fingerprint=12345 --user-data-dir=%TEMP%\test2

# 两个实例访问 CreepJS，应获得相同的指纹哈希
```

---

## 九、核心文件修改清单

### 9.1 新增文件

| 文件路径 | 功能 |
|---------|------|
| `base/singleton_fingerprint.h` | 指纹配置单例 |
| `base/singleton_fingerprint.cc` | 单例实现 |
| `third_party/blink/public/common/fingerprint/fingerprint.h` | 指纹数据结构 |
| `third_party/blink/renderer/modules/canvas/canvas2d/canvas_rendering_context_2d_fp.h` | Canvas 辅助函数 |
| `third_party/blink/renderer/modules/webgl/webgl_rendering_context_base_fp.h` | WebGL 辅助函数 |
| `chrome/browser/fingerprint_switches.h` | 命令行参数定义 |

### 9.2 修改文件

| 文件路径 | 修改内容 |
|---------|---------|
| `third_party/blink/renderer/core/frame/navigator.cc` | userAgent/platform 伪装 |
| `third_party/blink/renderer/core/frame/navigator_concurrent_hardware.cc` | hardwareConcurrency 伪装 |
| `third_party/blink/renderer/modules/canvas/canvas2d/canvas_rendering_context_2d.cc` | Canvas 噪声注入 |
| `third_party/blink/renderer/modules/webgl/webgl_rendering_context_base.cc` | WebGL 伪装 |
| `third_party/blink/renderer/core/timezone/timezone_controller.cc` | 时区伪装 |
| `content/browser/renderer_host/render_process_host_impl.cc` | 启动参数传递 |

---

## 十、风险与注意事项

### 10.1 技术风险

| 风险 | 缓解措施 |
|------|---------|
| Chromium 版本更新导致 Patch 失效 | 使用语义化 Patch，避免依赖行号 |
| 指纹一致性被检测为异常 | 保持指纹参数的合理性和多样性 |
| 编译耗时过长 | 使用增量编译，仅编译修改的模块 |
| 内存占用过高 | 控制噪声点数量，使用延迟加载 |

### 10.2 法律风险

本方案仅供技术研究和学习使用，请勿用于：
- 绕过网站风控系统
- 批量账号注册
- 自动化刷单/薅羊毛
- 其他违反服务条款的行为

---

## 十一、参考资源

### 11.1 本地仓库路径

```
F:\user\Desktop\PC端浏览器多开器\仓库\
├── fingerprint-chromium-main\      # 命令行参数参考
├── openFpchromium-114\             # Canvas/WebGL 源码修改
├── chromium_fake_fingerprint\      # 完整 Chromium 修改
├── browser_fingerprint-main\       # 指纹核心库
├── BotBrowser-main\                # Patch 文件和 CLI 设计
└── VirtualBrowser-main\            # UI 管理系统
```

### 11.2 官方文档

- [Chromium 构建文档](https://chromium.googlesource.com/chromium/src/+/main/docs/building.md)
- [Blink 渲染引擎文档](https://chromium.org/blink/)
- [Chrome DevTools Protocol](https://chromedevtools.github.io/devtools-protocol/)

---

*文档版本: 1.0 | 更新时间: 2026-01-26*
