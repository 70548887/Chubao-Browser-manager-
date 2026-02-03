# 内核开发详细方案 - 字体与 Variations

> 版本: 1.0.0  
> 适用: Chromium 146 内核  
> 配置接口: `--fingerprint-config`

---

## 一、配置加载架构

### 1.1 数据流向

```
┌─────────────────────────────────────────────────────────────────────────┐
│                           启动器 (Tauri)                                 │
│  生成配置文件: C:\Users\xxx\AppData\Roaming\TouchBao\Profiles\{id}\     │
│               └── bm_fingerprint.json                                    │
└─────────────────────────────────────────────────────────────────────────┘
                                    │
                                    │ 启动参数
                                    ▼
┌─────────────────────────────────────────────────────────────────────────┐
│  --fingerprint-config="C:\...\bm_fingerprint.json"                      │
│  --user-data-dir="C:\...\BrowserCache\{profile_id}"                     │
│  --force-fieldtrials="TouchBaoConfig/Stable"                            │
│  --enable-features=OverlayScrollbar,ParallelDownloading                 │
│  --disable-features=AutomationControlled                                │
└─────────────────────────────────────────────────────────────────────────┘
                                    │
                                    ▼
┌─────────────────────────────────────────────────────────────────────────┐
│                        Chromium 内核进程                                 │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                          │
│  ┌──────────────────────────────────────────────────────────────────┐  │
│  │  Browser Process (主进程)                                         │  │
│  │  ┌────────────────────────────────────────────────────────────┐  │  │
│  │  │  FingerprintConfigLoader::GetInstance()                    │  │  │
│  │  │  - 解析 --fingerprint-config 参数                          │  │  │
│  │  │  - 读取 JSON 配置文件                                      │  │  │
│  │  │  - 存储为单例供全局访问                                    │  │  │
│  │  └────────────────────────────────────────────────────────────┘  │  │
│  │                              │                                    │  │
│  │                              │ IPC 传递配置                       │  │
│  │                              ▼                                    │  │
│  └──────────────────────────────────────────────────────────────────┘  │
│                                                                          │
│  ┌──────────────────────────────────────────────────────────────────┐  │
│  │  Renderer Process (渲染进程) - 每个 Tab 一个                      │  │
│  │  ┌────────────────────────────────────────────────────────────┐  │  │
│  │  │  FingerprintConfigLoader::GetInstance()                    │  │  │
│  │  │  - 从 Browser Process 接收配置                             │  │  │
│  │  │  - 提供给各 Hook 点使用                                    │  │  │
│  │  └────────────────────────────────────────────────────────────┘  │  │
│  │                              │                                    │  │
│  │              ┌───────────────┼───────────────┐                   │  │
│  │              ▼               ▼               ▼                   │  │
│  │  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐           │  │
│  │  │ Font API     │  │ Canvas API   │  │ WebGL API    │           │  │
│  │  │ Hook         │  │ Hook         │  │ Hook         │           │  │
│  │  └──────────────┘  └──────────────┘  └──────────────┘           │  │
│  └──────────────────────────────────────────────────────────────────┘  │
│                                                                          │
└─────────────────────────────────────────────────────────────────────────┘
```

### 1.2 配置文件存储位置

```
C:\Users\{用户名}\AppData\Roaming\TouchBao\
├── Profiles\
│   ├── {profile_id_1}\
│   │   ├── bm_fingerprint.json    ← 指纹配置文件
│   │   └── bm_cloud.json          ← 云端同步配置
│   ├── {profile_id_2}\
│   │   └── ...
│   └── ...
├── BrowserCache\
│   ├── {profile_id_1}\            ← Chrome user-data-dir
│   │   ├── Default\
│   │   ├── Cookies
│   │   └── ...
│   └── ...
└── Extensions\
    └── ...
```

---

## 二、核心配置加载器实现

### 2.1 文件结构

```
chromium/src/
├── touchbao/                              ← 新建目录
│   ├── BUILD.gn                           ← 编译配置
│   ├── fingerprint_config_loader.h        ← 头文件
│   ├── fingerprint_config_loader.cc       ← 实现文件
│   ├── fingerprint_config_switches.h      ← 命令行开关定义
│   └── fingerprint_config_switches.cc
└── ...
```

### 2.2 命令行开关定义

**文件:** `chromium/src/touchbao/fingerprint_config_switches.h`

```cpp
// Copyright 2026 TouchBao. All rights reserved.

#ifndef TOUCHBAO_FINGERPRINT_CONFIG_SWITCHES_H_
#define TOUCHBAO_FINGERPRINT_CONFIG_SWITCHES_H_

namespace touchbao {
namespace switches {

// 指纹配置文件路径
// 用法: --fingerprint-config="C:\path\to\bm_fingerprint.json"
extern const char kFingerprintConfig[];

// 启用指纹保护 (默认启用)
// 用法: --enable-fingerprint-protection
extern const char kEnableFingerprintProtection[];

// 调试模式 - 打印配置加载日志
// 用法: --fingerprint-debug
extern const char kFingerprintDebug[];

}  // namespace switches
}  // namespace touchbao

#endif  // TOUCHBAO_FINGERPRINT_CONFIG_SWITCHES_H_
```

**文件:** `chromium/src/touchbao/fingerprint_config_switches.cc`

```cpp
// Copyright 2026 TouchBao. All rights reserved.

#include "touchbao/fingerprint_config_switches.h"

namespace touchbao {
namespace switches {

const char kFingerprintConfig[] = "fingerprint-config";
const char kEnableFingerprintProtection[] = "enable-fingerprint-protection";
const char kFingerprintDebug[] = "fingerprint-debug";

}  // namespace switches
}  // namespace touchbao
```

### 2.3 完整配置加载器

**文件:** `chromium/src/touchbao/fingerprint_config_loader.h`

```cpp
// Copyright 2026 TouchBao. All rights reserved.

#ifndef TOUCHBAO_FINGERPRINT_CONFIG_LOADER_H_
#define TOUCHBAO_FINGERPRINT_CONFIG_LOADER_H_

#include <memory>
#include <string>
#include <vector>
#include <optional>

#include "base/no_destructor.h"
#include "base/synchronization/lock.h"
#include "base/values.h"

namespace touchbao {

// ============================================================================
// 配置结构体定义
// ============================================================================

/// 种子配置 - 用于生成稳定的随机噪声
struct SeedConfig {
    int64_t master = 0;
    int64_t canvas = 0;
    int64_t webgl = 0;
    int64_t audio = 0;
};

/// Navigator 配置
struct NavigatorConfig {
    std::string user_agent;
    std::string platform;
    std::string vendor;
    std::string app_version;
    std::string language;
    std::vector<std::string> languages;
    int hardware_concurrency = 8;
    int device_memory = 16;
    int max_touch_points = 0;
    std::optional<std::string> do_not_track;  // "1", "0", or nullopt
    bool webdriver = false;
};

/// Screen 配置
struct ScreenConfig {
    int width = 1920;
    int height = 1080;
    int avail_width = 1920;
    int avail_height = 1040;
    int color_depth = 24;
    int pixel_depth = 24;
    double device_pixel_ratio = 1.0;
};

/// WebGL 配置
struct WebGLConfig {
    std::string vendor;
    std::string renderer;
    std::string unmasked_vendor;
    std::string unmasked_renderer;
    bool noise_enabled = true;
    int max_texture_size = 16384;
};

/// Canvas 配置
struct CanvasConfig {
    std::string mode = "noise";  // "noise" | "block" | "off"
    double noise_level = 0.0001;
};

/// Audio 配置
struct AudioConfig {
    std::string mode = "noise";  // "noise" | "block" | "off"
    int sample_rate = 44100;
    int max_channel_count = 2;
};

/// 字体配置
struct FontsConfig {
    std::string mode = "subset";  // "subset" | "real" | "custom" | "random"
    std::vector<std::string> list;
    std::vector<std::string> custom_list;
    int random_count = 15;
};

/// WebRTC 配置
struct WebRTCConfig {
    std::string mode = "disabled";  // "real" | "fake" | "disabled"
    std::optional<std::string> public_ip;
    std::optional<std::string> local_ip;
};

/// Timezone 配置
struct TimezoneConfig {
    std::string id = "Asia/Shanghai";
    int offset_minutes = -480;
};

/// Variations 配置
struct VariationsConfig {
    std::string seed_id;
    std::string seed_type;
    std::vector<std::string> enabled_features;
    std::vector<std::string> disabled_features;
};

/// Client Hints 配置
struct ClientHintsConfig {
    std::string full_version;
    std::string platform;
    std::string platform_version;
    std::string architecture = "x86";
    std::string bitness = "64";
    bool mobile = false;
};

/// 隐私配置
struct PrivacyConfig {
    bool client_rects_noise = true;
    bool port_scan_protection = true;
};

/// 设备配置
struct DeviceConfig {
    std::string name;
    std::string mac_address;
};

// ============================================================================
// 完整指纹配置
// ============================================================================

struct FingerprintConfig {
    std::string profile_id;
    int schema_version = 2;
    
    SeedConfig seed;
    NavigatorConfig navigator;
    ScreenConfig screen;
    WebGLConfig webgl;
    CanvasConfig canvas;
    AudioConfig audio;
    FontsConfig fonts;
    WebRTCConfig webrtc;
    TimezoneConfig timezone;
    VariationsConfig variations;
    ClientHintsConfig client_hints;
    PrivacyConfig privacy;
    DeviceConfig device;
};

// ============================================================================
// 配置加载器 (单例)
// ============================================================================

class FingerprintConfigLoader {
public:
    // 获取单例实例
    static FingerprintConfigLoader* GetInstance();
    
    // 禁止拷贝和赋值
    FingerprintConfigLoader(const FingerprintConfigLoader&) = delete;
    FingerprintConfigLoader& operator=(const FingerprintConfigLoader&) = delete;
    
    // ========== 初始化 ==========
    
    /// 从命令行加载配置 (Browser Process 调用)
    bool LoadFromCommandLine();
    
    /// 从 JSON 字符串加载配置 (Renderer Process 通过 IPC 接收)
    bool LoadFromJson(const std::string& json_content);
    
    /// 检查配置是否已加载
    bool IsLoaded() const;
    
    // ========== 配置访问接口 ==========
    
    /// 获取完整配置 (只读)
    const FingerprintConfig& GetConfig() const;
    
    /// 获取 Profile ID
    const std::string& GetProfileId() const;
    
    // ========== 便捷访问接口 ==========
    
    // --- Navigator ---
    const std::string& GetUserAgent() const;
    const std::string& GetPlatform() const;
    int GetHardwareConcurrency() const;
    int GetDeviceMemory() const;
    bool IsWebDriver() const;
    
    // --- Screen ---
    int GetScreenWidth() const;
    int GetScreenHeight() const;
    double GetDevicePixelRatio() const;
    
    // --- WebGL ---
    const std::string& GetWebGLVendor() const;
    const std::string& GetWebGLRenderer() const;
    bool IsWebGLNoiseEnabled() const;
    
    // --- Canvas ---
    const std::string& GetCanvasMode() const;
    double GetCanvasNoiseLevel() const;
    
    // --- Audio ---
    const std::string& GetAudioMode() const;
    
    // --- Fonts ---
    const std::string& GetFontsMode() const;
    const std::vector<std::string>& GetFontsList() const;
    bool IsFontAllowed(const std::string& font_name) const;
    
    // --- WebRTC ---
    const std::string& GetWebRTCMode() const;
    std::optional<std::string> GetWebRTCPublicIP() const;
    std::optional<std::string> GetWebRTCLocalIP() const;
    
    // --- Timezone ---
    const std::string& GetTimezoneId() const;
    int GetTimezoneOffsetMinutes() const;
    
    // --- Variations ---
    const std::string& GetVariationsSeedId() const;
    const std::string& GetVariationsSeedType() const;
    
    // --- Seed (噪声生成) ---
    int64_t GetMasterSeed() const;
    int64_t GetCanvasSeed() const;
    int64_t GetWebGLSeed() const;
    int64_t GetAudioSeed() const;
    
    // --- Privacy ---
    bool IsClientRectsNoiseEnabled() const;
    bool IsPortScanProtectionEnabled() const;

private:
    friend class base::NoDestructor<FingerprintConfigLoader>;
    
    FingerprintConfigLoader();
    ~FingerprintConfigLoader();
    
    // 解析 JSON 配置
    bool ParseJsonConfig(const base::Value::Dict& dict);
    void ParseSeedConfig(const base::Value::Dict& dict);
    void ParseNavigatorConfig(const base::Value::Dict& dict);
    void ParseScreenConfig(const base::Value::Dict& dict);
    void ParseWebGLConfig(const base::Value::Dict& dict);
    void ParseCanvasConfig(const base::Value::Dict& dict);
    void ParseAudioConfig(const base::Value::Dict& dict);
    void ParseFontsConfig(const base::Value::Dict& dict);
    void ParseWebRTCConfig(const base::Value::Dict& dict);
    void ParseTimezoneConfig(const base::Value::Dict& dict);
    void ParseVariationsConfig(const base::Value::Dict& dict);
    void ParseClientHintsConfig(const base::Value::Dict& dict);
    void ParsePrivacyConfig(const base::Value::Dict& dict);
    void ParseDeviceConfig(const base::Value::Dict& dict);
    
    // 配置数据
    FingerprintConfig config_;
    bool is_loaded_ = false;
    mutable base::Lock lock_;
    
    // 调试模式
    bool debug_mode_ = false;
};

}  // namespace touchbao

#endif  // TOUCHBAO_FINGERPRINT_CONFIG_LOADER_H_
```

### 2.4 配置加载器实现

**文件:** `chromium/src/touchbao/fingerprint_config_loader.cc`

```cpp
// Copyright 2026 TouchBao. All rights reserved.

#include "touchbao/fingerprint_config_loader.h"
#include "touchbao/fingerprint_config_switches.h"

#include "base/command_line.h"
#include "base/files/file_util.h"
#include "base/json/json_reader.h"
#include "base/logging.h"
#include "base/strings/string_util.h"
#include "base/no_destructor.h"

namespace touchbao {

// ============================================================================
// 单例实现
// ============================================================================

FingerprintConfigLoader* FingerprintConfigLoader::GetInstance() {
    static base::NoDestructor<FingerprintConfigLoader> instance;
    return instance.get();
}

FingerprintConfigLoader::FingerprintConfigLoader() {
    auto* cmd = base::CommandLine::ForCurrentProcess();
    debug_mode_ = cmd->HasSwitch(switches::kFingerprintDebug);
    
    if (debug_mode_) {
        LOG(INFO) << "[TouchBao] FingerprintConfigLoader initialized in debug mode";
    }
}

FingerprintConfigLoader::~FingerprintConfigLoader() = default;

// ============================================================================
// 配置加载
// ============================================================================

bool FingerprintConfigLoader::LoadFromCommandLine() {
    base::AutoLock auto_lock(lock_);
    
    if (is_loaded_) {
        if (debug_mode_) {
            LOG(INFO) << "[TouchBao] Config already loaded, skipping";
        }
        return true;
    }
    
    auto* cmd = base::CommandLine::ForCurrentProcess();
    
    if (!cmd->HasSwitch(switches::kFingerprintConfig)) {
        LOG(WARNING) << "[TouchBao] No --fingerprint-config specified";
        return false;
    }
    
    base::FilePath config_path = cmd->GetSwitchValuePath(switches::kFingerprintConfig);
    
    if (debug_mode_) {
        LOG(INFO) << "[TouchBao] Loading config from: " << config_path.value();
    }
    
    std::string json_content;
    if (!base::ReadFileToString(config_path, &json_content)) {
        LOG(ERROR) << "[TouchBao] Failed to read config file: " << config_path.value();
        return false;
    }
    
    return LoadFromJson(json_content);
}

bool FingerprintConfigLoader::LoadFromJson(const std::string& json_content) {
    auto result = base::JSONReader::ReadAndReturnValueWithError(json_content);
    
    if (!result.has_value()) {
        LOG(ERROR) << "[TouchBao] JSON parse error: " << result.error().message;
        return false;
    }
    
    if (!result->is_dict()) {
        LOG(ERROR) << "[TouchBao] Config root is not an object";
        return false;
    }
    
    bool success = ParseJsonConfig(result->GetDict());
    
    if (success) {
        is_loaded_ = true;
        if (debug_mode_) {
            LOG(INFO) << "[TouchBao] Config loaded successfully, profile_id: " 
                      << config_.profile_id;
        }
    }
    
    return success;
}

bool FingerprintConfigLoader::IsLoaded() const {
    base::AutoLock auto_lock(lock_);
    return is_loaded_;
}

// ============================================================================
// JSON 解析
// ============================================================================

bool FingerprintConfigLoader::ParseJsonConfig(const base::Value::Dict& dict) {
    // Schema 版本
    if (auto version = dict.FindInt("schema_version")) {
        config_.schema_version = *version;
    }
    
    // Profile ID
    if (const auto* profile_id = dict.FindString("profile_id")) {
        config_.profile_id = *profile_id;
    }
    
    // 各模块配置
    if (const auto* seed = dict.FindDict("seed")) {
        ParseSeedConfig(*seed);
    }
    
    if (const auto* navigator = dict.FindDict("navigator")) {
        ParseNavigatorConfig(*navigator);
    }
    
    if (const auto* screen = dict.FindDict("screen")) {
        ParseScreenConfig(*screen);
    }
    
    if (const auto* webgl = dict.FindDict("webgl")) {
        ParseWebGLConfig(*webgl);
    }
    
    if (const auto* canvas = dict.FindDict("canvas")) {
        ParseCanvasConfig(*canvas);
    }
    
    if (const auto* audio = dict.FindDict("audio")) {
        ParseAudioConfig(*audio);
    }
    
    if (const auto* fonts = dict.FindDict("fonts")) {
        ParseFontsConfig(*fonts);
    }
    
    if (const auto* webrtc = dict.FindDict("webrtc")) {
        ParseWebRTCConfig(*webrtc);
    }
    
    if (const auto* timezone = dict.FindDict("timezone")) {
        ParseTimezoneConfig(*timezone);
    }
    
    if (const auto* variations = dict.FindDict("variations")) {
        ParseVariationsConfig(*variations);
    }
    
    if (const auto* client_hints = dict.FindDict("client_hints")) {
        ParseClientHintsConfig(*client_hints);
    }
    
    if (const auto* privacy = dict.FindDict("privacy")) {
        ParsePrivacyConfig(*privacy);
    }
    
    if (const auto* device = dict.FindDict("device")) {
        ParseDeviceConfig(*device);
    }
    
    return true;
}

void FingerprintConfigLoader::ParseSeedConfig(const base::Value::Dict& dict) {
    if (auto master = dict.FindDouble("master")) {
        config_.seed.master = static_cast<int64_t>(*master);
    }
    if (auto canvas = dict.FindDouble("canvas")) {
        config_.seed.canvas = static_cast<int64_t>(*canvas);
    }
    if (auto webgl = dict.FindDouble("webgl")) {
        config_.seed.webgl = static_cast<int64_t>(*webgl);
    }
    if (auto audio = dict.FindDouble("audio")) {
        config_.seed.audio = static_cast<int64_t>(*audio);
    }
    
    if (debug_mode_) {
        LOG(INFO) << "[TouchBao] Seed config: master=" << config_.seed.master
                  << ", canvas=" << config_.seed.canvas;
    }
}

void FingerprintConfigLoader::ParseNavigatorConfig(const base::Value::Dict& dict) {
    if (const auto* ua = dict.FindString("user_agent")) {
        config_.navigator.user_agent = *ua;
    }
    if (const auto* platform = dict.FindString("platform")) {
        config_.navigator.platform = *platform;
    }
    if (const auto* vendor = dict.FindString("vendor")) {
        config_.navigator.vendor = *vendor;
    }
    if (const auto* app_version = dict.FindString("app_version")) {
        config_.navigator.app_version = *app_version;
    }
    if (const auto* language = dict.FindString("language")) {
        config_.navigator.language = *language;
    }
    if (const auto* languages = dict.FindList("languages")) {
        config_.navigator.languages.clear();
        for (const auto& lang : *languages) {
            if (lang.is_string()) {
                config_.navigator.languages.push_back(lang.GetString());
            }
        }
    }
    if (auto hc = dict.FindInt("hardware_concurrency")) {
        config_.navigator.hardware_concurrency = *hc;
    }
    if (auto dm = dict.FindInt("device_memory")) {
        config_.navigator.device_memory = *dm;
    }
    if (auto mtp = dict.FindInt("max_touch_points")) {
        config_.navigator.max_touch_points = *mtp;
    }
    if (const auto* dnt = dict.FindString("do_not_track")) {
        config_.navigator.do_not_track = *dnt;
    }
    if (auto wd = dict.FindBool("webdriver")) {
        config_.navigator.webdriver = *wd;
    }
    
    if (debug_mode_) {
        LOG(INFO) << "[TouchBao] Navigator: UA=" << config_.navigator.user_agent.substr(0, 50) << "...";
    }
}

void FingerprintConfigLoader::ParseScreenConfig(const base::Value::Dict& dict) {
    if (auto width = dict.FindInt("width")) {
        config_.screen.width = *width;
    }
    if (auto height = dict.FindInt("height")) {
        config_.screen.height = *height;
    }
    if (auto avail_width = dict.FindInt("avail_width")) {
        config_.screen.avail_width = *avail_width;
    }
    if (auto avail_height = dict.FindInt("avail_height")) {
        config_.screen.avail_height = *avail_height;
    }
    if (auto color_depth = dict.FindInt("color_depth")) {
        config_.screen.color_depth = *color_depth;
    }
    if (auto pixel_depth = dict.FindInt("pixel_depth")) {
        config_.screen.pixel_depth = *pixel_depth;
    }
    if (auto dpr = dict.FindDouble("device_pixel_ratio")) {
        config_.screen.device_pixel_ratio = *dpr;
    }
}

void FingerprintConfigLoader::ParseWebGLConfig(const base::Value::Dict& dict) {
    if (const auto* vendor = dict.FindString("vendor")) {
        config_.webgl.vendor = *vendor;
    }
    if (const auto* renderer = dict.FindString("renderer")) {
        config_.webgl.renderer = *renderer;
    }
    if (const auto* unmasked_vendor = dict.FindString("unmasked_vendor")) {
        config_.webgl.unmasked_vendor = *unmasked_vendor;
    }
    if (const auto* unmasked_renderer = dict.FindString("unmasked_renderer")) {
        config_.webgl.unmasked_renderer = *unmasked_renderer;
    }
    if (auto noise = dict.FindBool("noise_enabled")) {
        config_.webgl.noise_enabled = *noise;
    }
    if (auto max_tex = dict.FindInt("max_texture_size")) {
        config_.webgl.max_texture_size = *max_tex;
    }
}

void FingerprintConfigLoader::ParseCanvasConfig(const base::Value::Dict& dict) {
    if (const auto* mode = dict.FindString("mode")) {
        config_.canvas.mode = *mode;
    }
    if (auto noise = dict.FindDouble("noise_level")) {
        config_.canvas.noise_level = *noise;
    }
}

void FingerprintConfigLoader::ParseAudioConfig(const base::Value::Dict& dict) {
    if (const auto* mode = dict.FindString("mode")) {
        config_.audio.mode = *mode;
    }
    if (auto sr = dict.FindInt("sample_rate")) {
        config_.audio.sample_rate = *sr;
    }
    if (auto mcc = dict.FindInt("max_channel_count")) {
        config_.audio.max_channel_count = *mcc;
    }
}

void FingerprintConfigLoader::ParseFontsConfig(const base::Value::Dict& dict) {
    if (const auto* mode = dict.FindString("mode")) {
        config_.fonts.mode = *mode;
    }
    if (const auto* list = dict.FindList("list")) {
        config_.fonts.list.clear();
        for (const auto& font : *list) {
            if (font.is_string()) {
                config_.fonts.list.push_back(font.GetString());
            }
        }
    }
    if (const auto* custom_list = dict.FindList("custom_list")) {
        config_.fonts.custom_list.clear();
        for (const auto& font : *custom_list) {
            if (font.is_string()) {
                config_.fonts.custom_list.push_back(font.GetString());
            }
        }
    }
    if (auto count = dict.FindInt("random_count")) {
        config_.fonts.random_count = *count;
    }
    
    if (debug_mode_) {
        LOG(INFO) << "[TouchBao] Fonts: mode=" << config_.fonts.mode
                  << ", count=" << config_.fonts.list.size();
    }
}

void FingerprintConfigLoader::ParseWebRTCConfig(const base::Value::Dict& dict) {
    if (const auto* mode = dict.FindString("mode")) {
        config_.webrtc.mode = *mode;
    }
    if (const auto* public_ip = dict.FindString("public_ip")) {
        config_.webrtc.public_ip = *public_ip;
    }
    if (const auto* local_ip = dict.FindString("local_ip")) {
        config_.webrtc.local_ip = *local_ip;
    }
}

void FingerprintConfigLoader::ParseTimezoneConfig(const base::Value::Dict& dict) {
    if (const auto* id = dict.FindString("id")) {
        config_.timezone.id = *id;
    }
    if (auto offset = dict.FindInt("offset_minutes")) {
        config_.timezone.offset_minutes = *offset;
    }
}

void FingerprintConfigLoader::ParseVariationsConfig(const base::Value::Dict& dict) {
    if (const auto* seed_id = dict.FindString("seed_id")) {
        config_.variations.seed_id = *seed_id;
    }
    if (const auto* seed_type = dict.FindString("seed_type")) {
        config_.variations.seed_type = *seed_type;
    }
    if (const auto* enabled = dict.FindList("enabled_features")) {
        for (const auto& f : *enabled) {
            if (f.is_string()) {
                config_.variations.enabled_features.push_back(f.GetString());
            }
        }
    }
    if (const auto* disabled = dict.FindList("disabled_features")) {
        for (const auto& f : *disabled) {
            if (f.is_string()) {
                config_.variations.disabled_features.push_back(f.GetString());
            }
        }
    }
}

void FingerprintConfigLoader::ParseClientHintsConfig(const base::Value::Dict& dict) {
    if (const auto* fv = dict.FindString("full_version")) {
        config_.client_hints.full_version = *fv;
    }
    if (const auto* platform = dict.FindString("platform")) {
        config_.client_hints.platform = *platform;
    }
    if (const auto* pv = dict.FindString("platform_version")) {
        config_.client_hints.platform_version = *pv;
    }
    if (const auto* arch = dict.FindString("architecture")) {
        config_.client_hints.architecture = *arch;
    }
    if (const auto* bitness = dict.FindString("bitness")) {
        config_.client_hints.bitness = *bitness;
    }
    if (auto mobile = dict.FindBool("mobile")) {
        config_.client_hints.mobile = *mobile;
    }
}

void FingerprintConfigLoader::ParsePrivacyConfig(const base::Value::Dict& dict) {
    if (auto crn = dict.FindBool("client_rects_noise")) {
        config_.privacy.client_rects_noise = *crn;
    }
    if (auto psp = dict.FindBool("port_scan_protection")) {
        config_.privacy.port_scan_protection = *psp;
    }
}

void FingerprintConfigLoader::ParseDeviceConfig(const base::Value::Dict& dict) {
    if (const auto* name = dict.FindString("name")) {
        config_.device.name = *name;
    }
    if (const auto* mac = dict.FindString("mac_address")) {
        config_.device.mac_address = *mac;
    }
}

// ============================================================================
// 配置访问接口
// ============================================================================

const FingerprintConfig& FingerprintConfigLoader::GetConfig() const {
    return config_;
}

const std::string& FingerprintConfigLoader::GetProfileId() const {
    return config_.profile_id;
}

// --- Navigator ---
const std::string& FingerprintConfigLoader::GetUserAgent() const {
    return config_.navigator.user_agent;
}

const std::string& FingerprintConfigLoader::GetPlatform() const {
    return config_.navigator.platform;
}

int FingerprintConfigLoader::GetHardwareConcurrency() const {
    return config_.navigator.hardware_concurrency;
}

int FingerprintConfigLoader::GetDeviceMemory() const {
    return config_.navigator.device_memory;
}

bool FingerprintConfigLoader::IsWebDriver() const {
    return config_.navigator.webdriver;
}

// --- Screen ---
int FingerprintConfigLoader::GetScreenWidth() const {
    return config_.screen.width;
}

int FingerprintConfigLoader::GetScreenHeight() const {
    return config_.screen.height;
}

double FingerprintConfigLoader::GetDevicePixelRatio() const {
    return config_.screen.device_pixel_ratio;
}

// --- WebGL ---
const std::string& FingerprintConfigLoader::GetWebGLVendor() const {
    return config_.webgl.vendor;
}

const std::string& FingerprintConfigLoader::GetWebGLRenderer() const {
    return config_.webgl.renderer;
}

bool FingerprintConfigLoader::IsWebGLNoiseEnabled() const {
    return config_.webgl.noise_enabled;
}

// --- Canvas ---
const std::string& FingerprintConfigLoader::GetCanvasMode() const {
    return config_.canvas.mode;
}

double FingerprintConfigLoader::GetCanvasNoiseLevel() const {
    return config_.canvas.noise_level;
}

// --- Audio ---
const std::string& FingerprintConfigLoader::GetAudioMode() const {
    return config_.audio.mode;
}

// --- Fonts ---
const std::string& FingerprintConfigLoader::GetFontsMode() const {
    return config_.fonts.mode;
}

const std::vector<std::string>& FingerprintConfigLoader::GetFontsList() const {
    return config_.fonts.list;
}

bool FingerprintConfigLoader::IsFontAllowed(const std::string& font_name) const {
    if (config_.fonts.mode == "real") {
        return true;
    }
    
    for (const auto& font : config_.fonts.list) {
        if (base::EqualsCaseInsensitiveASCII(font, font_name)) {
            return true;
        }
    }
    return false;
}

// --- WebRTC ---
const std::string& FingerprintConfigLoader::GetWebRTCMode() const {
    return config_.webrtc.mode;
}

std::optional<std::string> FingerprintConfigLoader::GetWebRTCPublicIP() const {
    return config_.webrtc.public_ip;
}

std::optional<std::string> FingerprintConfigLoader::GetWebRTCLocalIP() const {
    return config_.webrtc.local_ip;
}

// --- Timezone ---
const std::string& FingerprintConfigLoader::GetTimezoneId() const {
    return config_.timezone.id;
}

int FingerprintConfigLoader::GetTimezoneOffsetMinutes() const {
    return config_.timezone.offset_minutes;
}

// --- Variations ---
const std::string& FingerprintConfigLoader::GetVariationsSeedId() const {
    return config_.variations.seed_id;
}

const std::string& FingerprintConfigLoader::GetVariationsSeedType() const {
    return config_.variations.seed_type;
}

// --- Seed ---
int64_t FingerprintConfigLoader::GetMasterSeed() const {
    return config_.seed.master;
}

int64_t FingerprintConfigLoader::GetCanvasSeed() const {
    return config_.seed.canvas;
}

int64_t FingerprintConfigLoader::GetWebGLSeed() const {
    return config_.seed.webgl;
}

int64_t FingerprintConfigLoader::GetAudioSeed() const {
    return config_.seed.audio;
}

// --- Privacy ---
bool FingerprintConfigLoader::IsClientRectsNoiseEnabled() const {
    return config_.privacy.client_rects_noise;
}

bool FingerprintConfigLoader::IsPortScanProtectionEnabled() const {
    return config_.privacy.port_scan_protection;
}

}  // namespace touchbao
```

---

## 三、字体指纹 Hook 点详解

### 3.1 字体检测方式与 Hook 点对照表

| 检测方式 | 检测原理 | Hook 文件路径 | 函数名 |
|----------|----------|---------------|--------|
| Font Access API | `navigator.fonts.query()` | `third_party/blink/renderer/modules/font_access/font_enumeration_cache.cc` | `QueryFontsForFrame` |
| CSS Font Loading | `document.fonts` | `third_party/blink/renderer/core/css/font_face_set_document.cc` | `GetAvailableFonts` |
| Canvas measureText | `ctx.measureText()` 测量宽度 | `third_party/blink/renderer/core/html/canvas/canvas_rendering_context_2d.cc` | `measureText` |
| Canvas fillText | 绘制文本检测 | `third_party/blink/renderer/modules/canvas/canvas2d/canvas_rendering_context_2d.cc` | `fillText` |
| getComputedStyle | CSS 字体回退检测 | `third_party/blink/renderer/core/css/resolver/style_resolver.cc` | `ResolveStyle` |
| @font-face | 字体加载事件 | `third_party/blink/renderer/core/css/font_face.cc` | `Load` |

### 3.2 Font Access API Hook (最重要)

**文件:** `third_party/blink/renderer/modules/font_access/font_enumeration_cache.cc`

```cpp
// 原文件位置，找到并修改 QueryFontsForFrame 函数

#include "touchbao/fingerprint_config_loader.h"  // 添加头文件

void FontEnumerationCache::QueryFontsForFrame(
    LocalFrame* frame,
    blink::mojom::blink::FontEnumerationStatus status,
    base::OnceCallback<void(FontEnumerationStatus, Vector<FontMetadata>)> callback) {
    
    // ========== TouchBao: 字体指纹保护开始 ==========
    auto* config = touchbao::FingerprintConfigLoader::GetInstance();
    
    if (config->IsLoaded()) {
        const auto& fonts_mode = config->GetFontsMode();
        
        // 非真实模式，返回配置的字体列表
        if (fonts_mode != "real") {
            Vector<FontMetadata> filtered_fonts;
            const auto& font_list = config->GetFontsList();
            
            for (const auto& font_name : font_list) {
                FontMetadata metadata;
                metadata.postscript_name = String::FromUTF8(font_name);
                metadata.full_name = String::FromUTF8(font_name);
                metadata.family = String::FromUTF8(font_name);
                metadata.style = "Regular";
                filtered_fonts.push_back(std::move(metadata));
            }
            
            std::move(callback).Run(
                blink::mojom::blink::FontEnumerationStatus::kOk,
                std::move(filtered_fonts)
            );
            return;
        }
    }
    // ========== TouchBao: 字体指纹保护结束 ==========
    
    // 原始逻辑...
    QuerySystemFontsInternal(frame, std::move(callback));
}
```

### 3.3 Canvas measureText Hook

**文件:** `third_party/blink/renderer/core/html/canvas/canvas_rendering_context_2d.cc`

```cpp
#include "touchbao/fingerprint_config_loader.h"

TextMetrics* CanvasRenderingContext2D::measureText(const String& text) {
    // ========== TouchBao: 字体检测防护 ==========
    auto* config = touchbao::FingerprintConfigLoader::GetInstance();
    
    if (config->IsLoaded()) {
        // 检查当前设置的字体是否在允许列表中
        String current_font = GetState().font();
        
        // 解析字体名称 (可能是 "16px Arial, sans-serif" 格式)
        Vector<String> font_families = ParseFontFamilies(current_font);
        
        bool has_allowed_font = false;
        for (const auto& family : font_families) {
            if (config->IsFontAllowed(family.Utf8())) {
                has_allowed_font = true;
                break;
            }
        }
        
        // 如果字体不在允许列表中，使用 fallback 字体的宽度
        if (!has_allowed_font && config->GetFontsMode() != "real") {
            // 临时切换到 fallback 字体计算
            State saved_state = GetState();
            GetState().SetFont("16px sans-serif");
            TextMetrics* metrics = MeasureTextInternal(text);
            GetState() = saved_state;
            return metrics;
        }
    }
    // ========== TouchBao: 字体检测防护结束 ==========
    
    return MeasureTextInternal(text);
}

// 辅助函数：解析字体家族名称
Vector<String> CanvasRenderingContext2D::ParseFontFamilies(const String& font_string) {
    Vector<String> families;
    // 解析逻辑：从 "16px Arial, sans-serif" 提取 ["Arial", "sans-serif"]
    // ... 实现细节
    return families;
}
```

### 3.4 CSS 字体检测 Hook

**文件:** `third_party/blink/renderer/core/css/font_face_set_document.cc`

```cpp
#include "touchbao/fingerprint_config_loader.h"

bool FontFaceSetDocument::check(const String& font, const String& text) {
    // ========== TouchBao: CSS 字体检测防护 ==========
    auto* config = touchbao::FingerprintConfigLoader::GetInstance();
    
    if (config->IsLoaded() && config->GetFontsMode() != "real") {
        // 解析字体名称
        String family = ParseFontFamily(font);
        
        // 检查是否在允许列表中
        if (!config->IsFontAllowed(family.Utf8())) {
            return false;  // 假装字体不存在
        }
    }
    // ========== TouchBao: CSS 字体检测防护结束 ==========
    
    return CheckInternal(font, text);
}
```

---

## 四、Variations Hook 点详解

### 4.1 chrome://version 页面

**文件:** `chrome/browser/ui/webui/version/version_handler.cc`

```cpp
#include "touchbao/fingerprint_config_loader.h"
#include "base/base64.h"

void VersionHandler::HandleRequestVariationInfo(const base::Value::List& args) {
    base::Value::Dict variations_info;
    
    // ========== TouchBao: 自定义 Variations 显示 ==========
    auto* config = touchbao::FingerprintConfigLoader::GetInstance();
    
    if (config->IsLoaded()) {
        const auto& seed_type = config->GetVariationsSeedType();
        const auto& seed_id = config->GetVariationsSeedId();
        
        // 变体版本种子类型
        if (!seed_type.empty()) {
            variations_info.Set("variations-seed-type", seed_type);
        } else {
            variations_info.Set("variations-seed-type", "TouchBao Stable");
        }
        
        // 使用中的变体
        if (!seed_id.empty()) {
            variations_info.Set("active-variations", seed_id);
        }
        
        // 命令行变体 (Base64 编码的 JSON)
        std::string config_json = base::StringPrintf(
            R"({"type":"touchbao","profile":"%s","version":"1.0"})",
            config->GetProfileId().c_str()
        );
        variations_info.Set("variations-cmd", base::Base64Encode(config_json));
        
        ResolveJavascriptCallback(args[0], variations_info);
        return;
    }
    // ========== TouchBao: 自定义 Variations 显示结束 ==========
    
    // 原始逻辑
    HandleRequestVariationInfoInternal(args);
}
```

### 4.2 Field Trial 初始化

**文件:** `components/variations/field_trial_creator.cc`

```cpp
#include "touchbao/fingerprint_config_loader.h"

void FieldTrialCreator::SetUpFieldTrials() {
    // ========== TouchBao: 跳过 Google Variations 服务 ==========
    auto* config = touchbao::FingerprintConfigLoader::GetInstance();
    
    if (config->IsLoaded()) {
        // 不从 Google 服务器获取 Variations
        // 使用命令行参数 --force-fieldtrials 代替
        LOG(INFO) << "[TouchBao] Using custom field trials, skipping Google variations";
        return;
    }
    // ========== TouchBao: 跳过 Google Variations 服务结束 ==========
    
    // 原始逻辑
    SetUpFieldTrialsInternal();
}
```

---

## 五、Navigator 相关 Hook 点

### 5.1 navigator.userAgent

**文件:** `third_party/blink/renderer/core/frame/navigator.cc`

```cpp
#include "touchbao/fingerprint_config_loader.h"

String Navigator::userAgent() const {
    auto* config = touchbao::FingerprintConfigLoader::GetInstance();
    
    if (config->IsLoaded() && !config->GetUserAgent().empty()) {
        return String::FromUTF8(config->GetUserAgent());
    }
    
    return GetFrame()->Loader().UserAgent();
}
```

### 5.2 navigator.platform

**文件:** `third_party/blink/renderer/core/frame/navigator_id.cc`

```cpp
#include "touchbao/fingerprint_config_loader.h"

String NavigatorID::platform() const {
    auto* config = touchbao::FingerprintConfigLoader::GetInstance();
    
    if (config->IsLoaded() && !config->GetPlatform().empty()) {
        return String::FromUTF8(config->GetPlatform());
    }
    
    return NavigatorIDInternal::platform();
}
```

### 5.3 navigator.hardwareConcurrency

**文件:** `third_party/blink/renderer/core/frame/navigator_concurrent_hardware.cc`

```cpp
#include "touchbao/fingerprint_config_loader.h"

unsigned NavigatorConcurrentHardware::hardwareConcurrency() const {
    auto* config = touchbao::FingerprintConfigLoader::GetInstance();
    
    if (config->IsLoaded()) {
        return config->GetHardwareConcurrency();
    }
    
    return base::SysInfo::NumberOfProcessors();
}
```

### 5.4 navigator.deviceMemory

**文件:** `third_party/blink/renderer/core/frame/navigator_device_memory.cc`

```cpp
#include "touchbao/fingerprint_config_loader.h"

float NavigatorDeviceMemory::deviceMemory() const {
    auto* config = touchbao::FingerprintConfigLoader::GetInstance();
    
    if (config->IsLoaded()) {
        return static_cast<float>(config->GetDeviceMemory());
    }
    
    return ApproximatedDeviceMemory::GetApproximatedDeviceMemory();
}
```

### 5.5 navigator.webdriver

**文件:** `third_party/blink/renderer/core/frame/navigator.cc`

```cpp
#include "touchbao/fingerprint_config_loader.h"

bool Navigator::webdriver() const {
    auto* config = touchbao::FingerprintConfigLoader::GetInstance();
    
    if (config->IsLoaded()) {
        return config->IsWebDriver();  // 通常返回 false
    }
    
    return GetFrame()->GetPage()->GetAutomationController()->IsActive();
}
```

---

## 六、Screen 相关 Hook 点

### 6.1 screen.width / screen.height

**文件:** `third_party/blink/renderer/core/frame/screen.cc`

```cpp
#include "touchbao/fingerprint_config_loader.h"

int Screen::width() const {
    auto* config = touchbao::FingerprintConfigLoader::GetInstance();
    
    if (config->IsLoaded()) {
        return config->GetScreenWidth();
    }
    
    return GetScreenInfo().rect.width();
}

int Screen::height() const {
    auto* config = touchbao::FingerprintConfigLoader::GetInstance();
    
    if (config->IsLoaded()) {
        return config->GetScreenHeight();
    }
    
    return GetScreenInfo().rect.height();
}

int Screen::availWidth() const {
    auto* config = touchbao::FingerprintConfigLoader::GetInstance();
    
    if (config->IsLoaded()) {
        return config->GetConfig().screen.avail_width;
    }
    
    return GetScreenInfo().available_rect.width();
}

int Screen::availHeight() const {
    auto* config = touchbao::FingerprintConfigLoader::GetInstance();
    
    if (config->IsLoaded()) {
        return config->GetConfig().screen.avail_height;
    }
    
    return GetScreenInfo().available_rect.height();
}

int Screen::colorDepth() const {
    auto* config = touchbao::FingerprintConfigLoader::GetInstance();
    
    if (config->IsLoaded()) {
        return config->GetConfig().screen.color_depth;
    }
    
    return GetScreenInfo().depth;
}
```

### 6.2 window.devicePixelRatio

**文件:** `third_party/blink/renderer/core/frame/local_dom_window.cc`

```cpp
#include "touchbao/fingerprint_config_loader.h"

double LocalDOMWindow::devicePixelRatio() const {
    auto* config = touchbao::FingerprintConfigLoader::GetInstance();
    
    if (config->IsLoaded()) {
        return config->GetDevicePixelRatio();
    }
    
    return GetFrame()->DevicePixelRatio();
}
```

---

## 七、WebGL 相关 Hook 点

### 7.1 WebGL getParameter

**文件:** `third_party/blink/renderer/modules/webgl/webgl_rendering_context_base.cc`

```cpp
#include "touchbao/fingerprint_config_loader.h"

ScriptValue WebGLRenderingContextBase::getParameter(ScriptState* script_state,
                                                     GLenum pname) {
    auto* config = touchbao::FingerprintConfigLoader::GetInstance();
    
    if (config->IsLoaded()) {
        switch (pname) {
            case GL_VENDOR:
                return ScriptValue::From(script_state, 
                    String::FromUTF8(config->GetWebGLVendor()));
            
            case GL_RENDERER:
                return ScriptValue::From(script_state,
                    String::FromUTF8(config->GetWebGLRenderer()));
            
            case GL_MAX_TEXTURE_SIZE:
                return ScriptValue::From(script_state,
                    config->GetConfig().webgl.max_texture_size);
        }
    }
    
    return GetParameterInternal(script_state, pname);
}
```

### 7.2 WebGL 扩展 (WEBGL_debug_renderer_info)

**文件:** `third_party/blink/renderer/modules/webgl/webgl_debug_renderer_info.cc`

```cpp
#include "touchbao/fingerprint_config_loader.h"

ScriptValue WebGLDebugRendererInfo::getParameter(GLenum pname) {
    auto* config = touchbao::FingerprintConfigLoader::GetInstance();
    
    if (config->IsLoaded()) {
        switch (pname) {
            case GL_UNMASKED_VENDOR_WEBGL:
                return String::FromUTF8(config->GetConfig().webgl.unmasked_vendor);
            
            case GL_UNMASKED_RENDERER_WEBGL:
                return String::FromUTF8(config->GetConfig().webgl.unmasked_renderer);
        }
    }
    
    return GetParameterInternal(pname);
}
```

---

## 八、Canvas 指纹 Hook 点

### 8.1 Canvas toDataURL / toBlob

**文件:** `third_party/blink/renderer/core/html/canvas/html_canvas_element.cc`

```cpp
#include "touchbao/fingerprint_config_loader.h"
#include <random>

String HTMLCanvasElement::toDataURL(const String& mime_type,
                                     const ScriptValue& quality_argument) {
    auto* config = touchbao::FingerprintConfigLoader::GetInstance();
    
    if (config->IsLoaded()) {
        const auto& mode = config->GetCanvasMode();
        
        if (mode == "block") {
            return String();  // 返回空，阻止获取
        }
        
        if (mode == "noise") {
            // 添加噪声
            String original = ToDataURLInternal(mime_type, quality_argument);
            return AddCanvasNoise(original, config->GetCanvasSeed(), 
                                  config->GetCanvasNoiseLevel());
        }
    }
    
    return ToDataURLInternal(mime_type, quality_argument);
}

// 添加 Canvas 噪声的辅助函数
String HTMLCanvasElement::AddCanvasNoise(const String& data, 
                                          int64_t seed, 
                                          double noise_level) {
    // 1. Base64 解码图像数据
    // 2. 对像素进行微小的随机修改 (基于 seed，保证同一 profile 结果一致)
    // 3. Base64 编码返回
    
    std::mt19937_64 rng(seed);
    std::uniform_real_distribution<double> dist(-noise_level, noise_level);
    
    // 实际实现需要操作像素数据...
    // 这里简化表示
    
    return data;  // 返回添加噪声后的数据
}
```

---

## 九、Audio 指纹 Hook 点

### 9.1 AudioContext 相关

**文件:** `third_party/blink/renderer/modules/webaudio/audio_context.cc`

```cpp
#include "touchbao/fingerprint_config_loader.h"

float AudioContext::sampleRate() const {
    auto* config = touchbao::FingerprintConfigLoader::GetInstance();
    
    if (config->IsLoaded()) {
        return static_cast<float>(config->GetConfig().audio.sample_rate);
    }
    
    return BaseAudioContext::sampleRate();
}
```

### 9.2 AnalyserNode (音频指纹关键)

**文件:** `third_party/blink/renderer/modules/webaudio/analyser_node.cc`

```cpp
#include "touchbao/fingerprint_config_loader.h"
#include <random>

void AnalyserNode::getFloatFrequencyData(DOMFloat32Array* array) {
    auto* config = touchbao::FingerprintConfigLoader::GetInstance();
    
    if (config->IsLoaded() && config->GetAudioMode() == "noise") {
        // 先获取原始数据
        GetFloatFrequencyDataInternal(array);
        
        // 添加基于 seed 的噪声
        int64_t seed = config->GetAudioSeed();
        std::mt19937_64 rng(seed);
        std::uniform_real_distribution<float> dist(-0.0001f, 0.0001f);
        
        float* data = array->Data();
        size_t length = array->length();
        
        for (size_t i = 0; i < length; ++i) {
            data[i] += dist(rng);
        }
        
        return;
    }
    
    GetFloatFrequencyDataInternal(array);
}
```

---

## 十、BUILD.gn 配置

**文件:** `chromium/src/touchbao/BUILD.gn`

```gn
# Copyright 2026 TouchBao. All rights reserved.

import("//build/config/features.gni")

source_set("touchbao") {
  sources = [
    "fingerprint_config_loader.cc",
    "fingerprint_config_loader.h",
    "fingerprint_config_switches.cc",
    "fingerprint_config_switches.h",
  ]
  
  deps = [
    "//base",
    "//base:base_static",
  ]
  
  public_deps = [
    "//base",
  ]
}
```

**修改:** `chromium/src/chrome/browser/BUILD.gn`

```gn
# 在 deps 中添加
deps = [
  # ... 其他依赖
  "//touchbao",
]
```

**修改:** `chromium/src/third_party/blink/renderer/BUILD.gn`

```gn
# 在 deps 中添加
deps = [
  # ... 其他依赖
  "//touchbao",
]
```

---

## 十一、编译与测试

### 11.1 编译命令

```bash
# 生成编译配置
gn gen out/Release --args='
  is_debug=false
  is_official_build=false
  is_chrome_branded=false
  fieldtrial_testing_enabled=false
  disable_fieldtrial_testing_config=true
  symbol_level=0
  blink_symbol_level=0
'

# 编译 Chrome
autoninja -C out/Release chrome
```

### 11.2 测试命令

```bash
# 测试配置加载
out/Release/chrome.exe ^
  --fingerprint-config="C:\test\bm_fingerprint.json" ^
  --fingerprint-debug ^
  --no-sandbox

# 查看日志输出
# [TouchBao] FingerprintConfigLoader initialized in debug mode
# [TouchBao] Loading config from: C:\test\bm_fingerprint.json
# [TouchBao] Config loaded successfully, profile_id: abc123
```

---

## 十二、Hook 点完整清单

| 分类 | API | 源码文件 | 函数名 |
|------|-----|----------|--------|
| **Navigator** | userAgent | `navigator.cc` | `userAgent()` |
| | platform | `navigator_id.cc` | `platform()` |
| | hardwareConcurrency | `navigator_concurrent_hardware.cc` | `hardwareConcurrency()` |
| | deviceMemory | `navigator_device_memory.cc` | `deviceMemory()` |
| | webdriver | `navigator.cc` | `webdriver()` |
| | languages | `navigator_language.cc` | `languages()` |
| **Screen** | width/height | `screen.cc` | `width()`, `height()` |
| | availWidth/availHeight | `screen.cc` | `availWidth()`, `availHeight()` |
| | colorDepth | `screen.cc` | `colorDepth()` |
| | devicePixelRatio | `local_dom_window.cc` | `devicePixelRatio()` |
| **WebGL** | getParameter | `webgl_rendering_context_base.cc` | `getParameter()` |
| | UNMASKED_VENDOR | `webgl_debug_renderer_info.cc` | `getParameter()` |
| | UNMASKED_RENDERER | `webgl_debug_renderer_info.cc` | `getParameter()` |
| **Canvas** | toDataURL | `html_canvas_element.cc` | `toDataURL()` |
| | toBlob | `html_canvas_element.cc` | `toBlob()` |
| | measureText | `canvas_rendering_context_2d.cc` | `measureText()` |
| | getImageData | `canvas_rendering_context_2d.cc` | `getImageData()` |
| **Audio** | sampleRate | `audio_context.cc` | `sampleRate()` |
| | getFloatFrequencyData | `analyser_node.cc` | `getFloatFrequencyData()` |
| | getByteFrequencyData | `analyser_node.cc` | `getByteFrequencyData()` |
| **Fonts** | query | `font_enumeration_cache.cc` | `QueryFontsForFrame()` |
| | check | `font_face_set_document.cc` | `check()` |
| **WebRTC** | getStats | `rtc_peer_connection.cc` | `getStats()` |
| | createOffer | `rtc_peer_connection.cc` | `createOffer()` |
| **Timezone** | getTimezoneOffset | `date.cc` | `getTimezoneOffset()` |
| | Intl.DateTimeFormat | `intl_date_time_format.cc` | `resolvedOptions()` |
| **ClientHints** | brands | `navigator_ua.cc` | `brands()` |
| | getHighEntropyValues | `navigator_ua_data.cc` | `getHighEntropyValues()` |
| **Variations** | chrome://version | `version_handler.cc` | `HandleRequestVariationInfo()` |

---

## 十三、配置文件示例 (bm_fingerprint.json)

```json
{
  "$schema": "bm_fingerprint_v2",
  "schema_version": 2,
  "profile_id": "57b9c2824307467aa407f7defd47e7b7",
  "created_at": "2026-02-02T15:30:00Z",
  
  "seed": {
    "master": 1234567890123456,
    "canvas": 9876543210987654,
    "webgl": 1122334455667788,
    "audio": 8877665544332211
  },
  
  "navigator": {
    "user_agent": "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/146.0.0.0 Safari/537.36",
    "platform": "Win32",
    "vendor": "Google Inc.",
    "app_version": "5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36",
    "language": "zh-CN",
    "languages": ["zh-CN", "en-US", "en"],
    "hardware_concurrency": 8,
    "device_memory": 16,
    "max_touch_points": 0,
    "do_not_track": null,
    "webdriver": false,
    "pdf_viewer_enabled": true,
    "cookie_enabled": true
  },
  
  "screen": {
    "width": 1920,
    "height": 1080,
    "avail_width": 1920,
    "avail_height": 1040,
    "color_depth": 24,
    "pixel_depth": 24,
    "device_pixel_ratio": 1.0,
    "orientation_type": "landscape-primary",
    "orientation_angle": 0
  },
  
  "webgl": {
    "vendor": "Google Inc. (NVIDIA)",
    "renderer": "ANGLE (NVIDIA, NVIDIA GeForce GTX 1660 Direct3D11 vs_5_0 ps_5_0, D3D11)",
    "unmasked_vendor": "NVIDIA Corporation",
    "unmasked_renderer": "NVIDIA GeForce GTX 1660/PCIe/SSE2",
    "version": "WebGL 1.0 (OpenGL ES 2.0 Chromium)",
    "shading_language_version": "WebGL GLSL ES 1.0",
    "max_texture_size": 16384,
    "max_vertex_attribs": 16,
    "noise_enabled": true
  },
  
  "canvas": {
    "mode": "noise",
    "noise_level": 0.0001
  },
  
  "audio": {
    "mode": "noise",
    "sample_rate": 44100,
    "max_channel_count": 2
  },
  
  "fonts": {
    "mode": "subset",
    "list": [
      "Arial", "Arial Black", "Calibri", "Cambria", "Courier New",
      "Georgia", "Helvetica", "Impact", "Microsoft YaHei", "SimSun",
      "SimHei", "Tahoma", "Times New Roman", "Trebuchet MS", "Verdana"
    ],
    "custom_list": null,
    "random_count": null
  },
  
  "webrtc": {
    "mode": "disabled",
    "public_ip": null,
    "local_ip": null
  },
  
  "timezone": {
    "id": "Asia/Shanghai",
    "offset_minutes": -480
  },
  
  "variations": {
    "seed_id": "f2f17cec-377be55a",
    "seed_type": "TouchBao Stable",
    "enabled_features": ["OverlayScrollbar", "ParallelDownloading"],
    "disabled_features": ["AutomationControlled", "CalculateNativeWinOcclusion"]
  },
  
  "client_hints": {
    "brands": [
      { "brand": "Not_A Brand", "version": "8" },
      { "brand": "Chromium", "version": "146" },
      { "brand": "Google Chrome", "version": "146" }
    ],
    "full_version": "146.0.0.0",
    "platform": "Windows",
    "platform_version": "10.0.0",
    "architecture": "x86",
    "bitness": "64",
    "model": "",
    "mobile": false,
    "wow64": false
  },
  
  "privacy": {
    "client_rects_noise": true,
    "port_scan_protection": true
  },
  
  "device": {
    "name": "DESKTOP-TOUCHBAO",
    "mac_address": "64-2B-7A-4D-96-E1"
  }
}
```

---

## 十四、注意事项

### 14.1 线程安全
- `FingerprintConfigLoader` 使用 `base::Lock` 保护
- 单例在首次访问时初始化
- 所有 Get 方法都是线程安全的

### 14.2 性能考虑
- 配置只加载一次，后续访问直接返回缓存
- 字体列表比较使用 O(n) 查找，可考虑用 HashSet 优化
- 噪声生成使用确定性随机数（基于 seed），保证同一 Profile 结果一致

### 14.3 调试建议
```bash
# 启用调试日志
--fingerprint-debug

# 查看日志
[TouchBao] Config loaded successfully, profile_id: xxx
[TouchBao] Fonts: mode=subset, count=15
[TouchBao] Navigator: UA=Mozilla/5.0...
```

### 14.4 风控注意
- 字体列表不要太少（< 10）或太多（> 100）
- WebGL 参数要与真实设备一致
- Canvas/Audio 噪声要足够小（0.0001 级别）
- 每个 Profile 的 Variations ID 必须唯一且稳定

---

## 十五、WebRTC 详细实现

### 15.1 WebRTC 指纹检测原理

```
网站检测 WebRTC 的方式：
1. 通过 RTCPeerConnection 获取本地 IP（即使使用代理也能泄露）
2. 通过 ICE Candidate 获取公网/内网 IP
3. 通过 getStats() 获取网络统计信息
```

### 15.2 配置结构

```json
{
  "webrtc": {
    "mode": "fake",           // "real" | "fake" | "disabled"
    "public_ip": "203.0.113.50",
    "local_ip": "192.168.1.100",
    "disable_non_proxied_udp": true,
    "ice_servers": []
  }
}
```

### 15.3 RTCPeerConnection Hook

**文件:** `third_party/blink/renderer/modules/peerconnection/rtc_peer_connection.cc`

```cpp
#include "touchbao/fingerprint_config_loader.h"

RTCPeerConnection::RTCPeerConnection(
    ExecutionContext* context,
    const RTCConfiguration* configuration,
    const Dictionary& media_constraints,
    ExceptionState& exception_state) {
    
    // ========== TouchBao: WebRTC 控制 ==========
    auto* config = touchbao::FingerprintConfigLoader::GetInstance();
    
    if (config->IsLoaded()) {
        const auto& webrtc_mode = config->GetWebRTCMode();
        
        if (webrtc_mode == "disabled") {
            // 完全禁用 WebRTC
            exception_state.ThrowDOMException(
                DOMExceptionCode::kNotSupportedError,
                "WebRTC is disabled"
            );
            return;
        }
        
        if (webrtc_mode == "fake") {
            // 修改 ICE 配置，阻止真实 IP 泄露
            RTCConfiguration* modified_config = 
                ModifyRTCConfiguration(configuration, config);
            InitializeInternal(context, modified_config, media_constraints);
            return;
        }
    }
    // ========== TouchBao: WebRTC 控制结束 ==========
    
    InitializeInternal(context, configuration, media_constraints);
}

// 修改 RTC 配置
RTCConfiguration* RTCPeerConnection::ModifyRTCConfiguration(
    const RTCConfiguration* original,
    touchbao::FingerprintConfigLoader* config) {
    
    RTCConfiguration* modified = RTCConfiguration::Create();
    
    // 复制原配置
    if (original) {
        modified->setIceServers(original->iceServers());
        modified->setBundlePolicy(original->bundlePolicy());
        modified->setRtcpMuxPolicy(original->rtcpMuxPolicy());
    }
    
    // 强制使用 relay 模式（只通过 TURN 服务器，不泄露真实 IP）
    modified->setIceTransportPolicy("relay");
    
    return modified;
}
```

### 15.4 ICE Candidate 过滤

**文件:** `third_party/blink/renderer/modules/peerconnection/rtc_ice_candidate.cc`

```cpp
#include "touchbao/fingerprint_config_loader.h"
#include "base/strings/string_util.h"
#include <regex>

RTCIceCandidate* RTCIceCandidate::Create(
    const RTCIceCandidateInit* candidate_init,
    ExceptionState& exception_state) {
    
    auto* config = touchbao::FingerprintConfigLoader::GetInstance();
    
    if (config->IsLoaded() && config->GetWebRTCMode() == "fake") {
        String candidate = candidate_init->candidate();
        
        // 替换 IP 地址
        String modified_candidate = ReplaceIPInCandidate(
            candidate,
            config->GetWebRTCPublicIP(),
            config->GetWebRTCLocalIP()
        );
        
        RTCIceCandidateInit* modified_init = RTCIceCandidateInit::Create();
        modified_init->setCandidate(modified_candidate);
        modified_init->setSdpMid(candidate_init->sdpMid());
        modified_init->setSdpMLineIndex(candidate_init->sdpMLineIndex());
        
        return CreateInternal(modified_init, exception_state);
    }
    
    return CreateInternal(candidate_init, exception_state);
}

// 替换 ICE Candidate 中的 IP 地址
String RTCIceCandidate::ReplaceIPInCandidate(
    const String& candidate,
    const std::optional<std::string>& fake_public_ip,
    const std::optional<std::string>& fake_local_ip) {
    
    std::string candidate_str = candidate.Utf8();
    
    // ICE Candidate 格式示例:
    // "candidate:0 1 UDP 2122252543 192.168.1.100 52000 typ host"
    // "candidate:1 1 UDP 1686052863 203.0.113.50 52001 typ srflx"
    
    // 正则匹配 IP 地址
    std::regex ip_regex(R"((\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}))");
    
    std::string result = candidate_str;
    
    // 检测是 host (本地) 还是 srflx (公网)
    if (candidate_str.find("typ host") != std::string::npos) {
        if (fake_local_ip.has_value()) {
            result = std::regex_replace(result, ip_regex, *fake_local_ip);
        }
    } else if (candidate_str.find("typ srflx") != std::string::npos) {
        if (fake_public_ip.has_value()) {
            result = std::regex_replace(result, ip_regex, *fake_public_ip);
        }
    }
    
    return String::FromUTF8(result);
}
```

### 15.5 getStats Hook

**文件:** `third_party/blink/renderer/modules/peerconnection/rtc_peer_connection.cc`

```cpp
ScriptPromise RTCPeerConnection::getStats(ScriptState* script_state,
                                           MediaStreamTrack* selector) {
    auto* config = touchbao::FingerprintConfigLoader::GetInstance();
    
    if (config->IsLoaded() && config->GetWebRTCMode() == "fake") {
        // 返回修改后的统计信息
        return GetStatsWithFakeData(script_state, selector, config);
    }
    
    return GetStatsInternal(script_state, selector);
}

ScriptPromise RTCPeerConnection::GetStatsWithFakeData(
    ScriptState* script_state,
    MediaStreamTrack* selector,
    touchbao::FingerprintConfigLoader* config) {
    
    auto* resolver = MakeGarbageCollected<ScriptPromiseResolver>(script_state);
    ScriptPromise promise = resolver->Promise();
    
    // 获取原始统计数据
    GetStatsInternal(script_state, selector).Then(
        [resolver, config](RTCStatsReport* report) {
            // 修改报告中的 IP 相关信息
            RTCStatsReport* modified_report = ModifyStatsReport(report, config);
            resolver->Resolve(modified_report);
        },
        [resolver](DOMException* error) {
            resolver->Reject(error);
        }
    );
    
    return promise;
}
```

---

## 十六、Timezone 详细实现

### 16.1 时区检测原理

```
网站检测时区的方式：
1. Date.getTimezoneOffset() - 返回分钟偏移
2. Intl.DateTimeFormat().resolvedOptions().timeZone - 返回时区 ID
3. new Date().toString() - 包含时区缩写
```

### 16.2 配置结构

```json
{
  "timezone": {
    "id": "America/New_York",
    "offset_minutes": 300,
    "abbreviation": "EST"
  }
}
```

### 16.3 Date.getTimezoneOffset Hook

**文件:** `v8/src/builtins/builtins-date.cc`

```cpp
#include "touchbao/fingerprint_config_loader.h"

BUILTIN(DatePrototypeGetTimezoneOffset) {
    HandleScope scope(isolate);
    
    // ========== TouchBao: 时区伪装 ==========
    auto* config = touchbao::FingerprintConfigLoader::GetInstance();
    
    if (config->IsLoaded()) {
        int offset = config->GetTimezoneOffsetMinutes();
        return Smi::FromInt(offset);
    }
    // ========== TouchBao: 时区伪装结束 ==========
    
    // 原始逻辑
    CHECK(IsJSDate(*args.receiver()));
    Handle<JSDate> date = Handle<JSDate>::cast(args.receiver());
    return Smi::FromInt(date->TimezoneOffset());
}
```

### 16.4 Intl.DateTimeFormat Hook

**文件:** `v8/src/objects/intl-objects.cc`

```cpp
#include "touchbao/fingerprint_config_loader.h"

MaybeHandle<JSObject> Intl::ResolvedOptions(
    Isolate* isolate,
    Handle<JSReceiver> receiver,
    const char* service_name) {
    
    auto* config = touchbao::FingerprintConfigLoader::GetInstance();
    
    if (config->IsLoaded()) {
        // 创建修改后的 resolved options 对象
        Handle<JSObject> options = isolate->factory()->NewJSObject(
            isolate->object_function());
        
        // 设置伪装的时区
        Handle<String> timezone_key = 
            isolate->factory()->NewStringFromAsciiChecked("timeZone");
        Handle<String> timezone_value = 
            isolate->factory()->NewStringFromUtf8(
                config->GetTimezoneId().c_str()).ToHandleChecked();
        
        JSObject::SetProperty(isolate, options, timezone_key, timezone_value,
                              StoreOrigin::kNamed).Check();
        
        // 其他选项保持原值...
        return options;
    }
    
    return ResolvedOptionsInternal(isolate, receiver, service_name);
}
```

### 16.5 Date.toString Hook

**文件:** `v8/src/date/dateparser.cc`

```cpp
#include "touchbao/fingerprint_config_loader.h"

Handle<String> DateCache::ToDateString(double time_val, 
                                        DateCache* date_cache,
                                        ToDateStringMode mode) {
    auto* config = touchbao::FingerprintConfigLoader::GetInstance();
    
    if (config->IsLoaded()) {
        // 修改时区显示部分
        // 原始: "Sat Feb 02 2026 15:30:00 GMT+0800 (China Standard Time)"
        // 伪装: "Sat Feb 02 2026 02:30:00 GMT-0500 (Eastern Standard Time)"
        
        const std::string& tz_id = config->GetTimezoneId();
        int offset = config->GetTimezoneOffsetMinutes();
        
        // 计算调整后的时间
        double adjusted_time = AdjustTimeForTimezone(time_val, offset);
        
        // 格式化时区字符串
        std::string tz_string = FormatTimezoneString(tz_id, offset);
        
        return FormatDateWithTimezone(adjusted_time, tz_string);
    }
    
    return ToDateStringInternal(time_val, date_cache, mode);
}
```

---

## 十七、Client Hints 详细实现

### 17.1 Client Hints 检测原理

```
Client Hints 是新的用户代理信息 API：
1. navigator.userAgentData.brands - 浏览器品牌列表
2. navigator.userAgentData.mobile - 是否移动设备
3. navigator.userAgentData.platform - 操作系统
4. navigator.userAgentData.getHighEntropyValues() - 详细信息
```

### 17.2 配置结构

```json
{
  "client_hints": {
    "brands": [
      { "brand": "Not_A Brand", "version": "8" },
      { "brand": "Chromium", "version": "146" },
      { "brand": "Google Chrome", "version": "146" }
    ],
    "full_version": "146.0.6723.91",
    "platform": "Windows",
    "platform_version": "10.0.0",
    "architecture": "x86",
    "bitness": "64",
    "model": "",
    "mobile": false,
    "wow64": false
  }
}
```

### 17.3 NavigatorUAData Hook

**文件:** `third_party/blink/renderer/core/frame/navigator_ua_data.cc`

```cpp
#include "touchbao/fingerprint_config_loader.h"

HeapVector<Member<NavigatorUABrandVersion>> NavigatorUAData::brands() const {
    auto* config = touchbao::FingerprintConfigLoader::GetInstance();
    
    if (config->IsLoaded()) {
        HeapVector<Member<NavigatorUABrandVersion>> result;
        const auto& client_hints = config->GetConfig().client_hints;
        
        // 从配置中读取 brands (需要在配置中添加 brands 结构)
        // 这里使用默认的 Chrome 品牌信息
        auto* brand1 = MakeGarbageCollected<NavigatorUABrandVersion>();
        brand1->setBrand("Not_A Brand");
        brand1->setVersion("8");
        result.push_back(brand1);
        
        auto* brand2 = MakeGarbageCollected<NavigatorUABrandVersion>();
        brand2->setBrand("Chromium");
        brand2->setVersion(String::FromUTF8(client_hints.full_version.substr(0, 3)));
        result.push_back(brand2);
        
        auto* brand3 = MakeGarbageCollected<NavigatorUABrandVersion>();
        brand3->setBrand("Google Chrome");
        brand3->setVersion(String::FromUTF8(client_hints.full_version.substr(0, 3)));
        result.push_back(brand3);
        
        return result;
    }
    
    return brands_;
}

bool NavigatorUAData::mobile() const {
    auto* config = touchbao::FingerprintConfigLoader::GetInstance();
    
    if (config->IsLoaded()) {
        return config->GetConfig().client_hints.mobile;
    }
    
    return mobile_;
}

String NavigatorUAData::platform() const {
    auto* config = touchbao::FingerprintConfigLoader::GetInstance();
    
    if (config->IsLoaded()) {
        return String::FromUTF8(config->GetConfig().client_hints.platform);
    }
    
    return platform_;
}
```

### 17.4 getHighEntropyValues Hook

**文件:** `third_party/blink/renderer/core/frame/navigator_ua_data.cc`

```cpp
ScriptPromise NavigatorUAData::getHighEntropyValues(
    ScriptState* script_state,
    const Vector<String>& hints) {
    
    auto* config = touchbao::FingerprintConfigLoader::GetInstance();
    
    if (config->IsLoaded()) {
        auto* resolver = MakeGarbageCollected<ScriptPromiseResolver>(script_state);
        ScriptPromise promise = resolver->Promise();
        
        const auto& ch = config->GetConfig().client_hints;
        
        UADataValues* values = MakeGarbageCollected<UADataValues>();
        
        // 基础字段
        values->setBrands(brands());
        values->setMobile(ch.mobile);
        values->setPlatform(String::FromUTF8(ch.platform));
        
        // 高熵值字段 (根据请求的 hints 返回)
        for (const String& hint : hints) {
            if (hint == "architecture") {
                values->setArchitecture(String::FromUTF8(ch.architecture));
            } else if (hint == "bitness") {
                values->setBitness(String::FromUTF8(ch.bitness));
            } else if (hint == "model") {
                values->setModel(String::FromUTF8(ch.model));
            } else if (hint == "platformVersion") {
                values->setPlatformVersion(String::FromUTF8(ch.platform_version));
            } else if (hint == "fullVersionList") {
                values->setFullVersionList(GetFullVersionList(ch));
            } else if (hint == "uaFullVersion") {
                values->setUaFullVersion(String::FromUTF8(ch.full_version));
            } else if (hint == "wow64") {
                values->setWow64(ch.wow64);
            }
        }
        
        resolver->Resolve(values);
        return promise;
    }
    
    return GetHighEntropyValuesInternal(script_state, hints);
}

HeapVector<Member<NavigatorUABrandVersion>> NavigatorUAData::GetFullVersionList(
    const touchbao::ClientHintsConfig& ch) {
    
    HeapVector<Member<NavigatorUABrandVersion>> result;
    
    auto* brand1 = MakeGarbageCollected<NavigatorUABrandVersion>();
    brand1->setBrand("Not_A Brand");
    brand1->setVersion("8.0.0.0");
    result.push_back(brand1);
    
    auto* brand2 = MakeGarbageCollected<NavigatorUABrandVersion>();
    brand2->setBrand("Chromium");
    brand2->setVersion(String::FromUTF8(ch.full_version));
    result.push_back(brand2);
    
    auto* brand3 = MakeGarbageCollected<NavigatorUABrandVersion>();
    brand3->setBrand("Google Chrome");
    brand3->setVersion(String::FromUTF8(ch.full_version));
    result.push_back(brand3);
    
    return result;
}
```

### 17.5 HTTP Client Hints Header Hook

**文件:** `content/browser/client_hints/client_hints.cc`

```cpp
#include "touchbao/fingerprint_config_loader.h"

void AddClientHintsHeaders(
    const GURL& url,
    net::HttpRequestHeaders* headers,
    BrowserContext* context) {
    
    auto* config = touchbao::FingerprintConfigLoader::GetInstance();
    
    if (config->IsLoaded()) {
        const auto& ch = config->GetConfig().client_hints;
        
        // Sec-CH-UA
        std::string brands_header = base::StringPrintf(
            R"("Not_A Brand";v="8", "Chromium";v="%s", "Google Chrome";v="%s")",
            ch.full_version.substr(0, 3).c_str(),
            ch.full_version.substr(0, 3).c_str()
        );
        headers->SetHeader("Sec-CH-UA", brands_header);
        
        // Sec-CH-UA-Mobile
        headers->SetHeader("Sec-CH-UA-Mobile", ch.mobile ? "?1" : "?0");
        
        // Sec-CH-UA-Platform
        headers->SetHeader("Sec-CH-UA-Platform", 
            base::StringPrintf(R"("%s")", ch.platform.c_str()));
        
        // Sec-CH-UA-Platform-Version (高熵值)
        headers->SetHeader("Sec-CH-UA-Platform-Version",
            base::StringPrintf(R"("%s")", ch.platform_version.c_str()));
        
        // Sec-CH-UA-Full-Version-List (高熵值)
        headers->SetHeader("Sec-CH-UA-Full-Version-List", brands_header);
        
        // Sec-CH-UA-Arch (高熵值)
        headers->SetHeader("Sec-CH-UA-Arch",
            base::StringPrintf(R"("%s")", ch.architecture.c_str()));
        
        // Sec-CH-UA-Bitness (高熵值)
        headers->SetHeader("Sec-CH-UA-Bitness",
            base::StringPrintf(R"("%s")", ch.bitness.c_str()));
        
        return;
    }
    
    AddClientHintsHeadersInternal(url, headers, context);
}
```

---

## 十八、IPC 配置传递

### 18.1 Browser → Renderer 配置传递架构

```
┌─────────────────────────────────────────────────────────────────┐
│  Browser Process                                                 │
│  ┌─────────────────────────────────────────────────────────┐   │
│  │  FingerprintConfigLoader::LoadFromCommandLine()          │   │
│  │  - 读取 --fingerprint-config 参数                        │   │
│  │  - 解析 JSON 配置文件                                    │   │
│  │  - 存储配置到单例                                        │   │
│  └─────────────────────────────────────────────────────────┘   │
│                          │                                       │
│                          │ Mojo IPC                              │
│                          ▼                                       │
│  ┌─────────────────────────────────────────────────────────┐   │
│  │  RenderProcessHost::OnRendererCreated()                  │   │
│  │  - 序列化配置为 JSON                                     │   │
│  │  - 通过 Mojo 发送到 Renderer                             │   │
│  └─────────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────────┘
                           │
                           │ Mojo IPC
                           ▼
┌─────────────────────────────────────────────────────────────────┐
│  Renderer Process                                                │
│  ┌─────────────────────────────────────────────────────────┐   │
│  │  FingerprintConfigLoader::LoadFromJson()                 │   │
│  │  - 接收 JSON 配置                                        │   │
│  │  - 解析并存储到本进程单例                                │   │
│  └─────────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────────┘
```

### 18.2 Mojo 接口定义

**文件:** `touchbao/public/mojom/fingerprint_config.mojom`

```mojom
module touchbao.mojom;

// 指纹配置接口
interface FingerprintConfigService {
    // 设置配置（Browser → Renderer）
    SetFingerprintConfig(string json_config) => (bool success);
    
    // 获取配置（Renderer → Browser）
    GetFingerprintConfig() => (string json_config);
};
```

### 18.3 Browser 端实现

**文件:** `content/browser/renderer_host/render_process_host_impl.cc`

```cpp
#include "touchbao/fingerprint_config_loader.h"
#include "touchbao/public/mojom/fingerprint_config.mojom.h"

void RenderProcessHostImpl::OnRendererCreated(
    mojo::PendingRemote<mojom::Renderer> renderer) {
    
    // ... 其他初始化代码 ...
    
    // ========== TouchBao: 发送指纹配置到 Renderer ==========
    auto* config = touchbao::FingerprintConfigLoader::GetInstance();
    
    if (config->IsLoaded()) {
        // 序列化配置为 JSON
        std::string json_config = config->SerializeToJson();
        
        // 获取 Mojo 接口
        mojo::Remote<touchbao::mojom::FingerprintConfigService> service;
        BindFingerprintConfigService(service.BindNewPipeAndPassReceiver());
        
        // 发送配置
        service->SetFingerprintConfig(json_config, base::DoNothing());
    }
    // ========== TouchBao: 发送指纹配置到 Renderer 结束 ==========
}
```

### 18.4 Renderer 端实现

**文件:** `content/renderer/render_thread_impl.cc`

```cpp
#include "touchbao/fingerprint_config_loader.h"
#include "touchbao/public/mojom/fingerprint_config.mojom.h"

class FingerprintConfigServiceImpl 
    : public touchbao::mojom::FingerprintConfigService {
public:
    explicit FingerprintConfigServiceImpl(
        mojo::PendingReceiver<touchbao::mojom::FingerprintConfigService> receiver)
        : receiver_(this, std::move(receiver)) {}
    
    // 接收配置
    void SetFingerprintConfig(const std::string& json_config,
                               SetFingerprintConfigCallback callback) override {
        auto* loader = touchbao::FingerprintConfigLoader::GetInstance();
        bool success = loader->LoadFromJson(json_config);
        std::move(callback).Run(success);
    }
    
    // 获取配置（如果需要）
    void GetFingerprintConfig(GetFingerprintConfigCallback callback) override {
        auto* loader = touchbao::FingerprintConfigLoader::GetInstance();
        std::string json = loader->SerializeToJson();
        std::move(callback).Run(json);
    }

private:
    mojo::Receiver<touchbao::mojom::FingerprintConfigService> receiver_;
};

void RenderThreadImpl::OnRendererStart() {
    // 注册 Mojo 服务
    registry_.AddInterface<touchbao::mojom::FingerprintConfigService>(
        base::BindRepeating(&CreateFingerprintConfigService));
}
```

### 18.5 配置序列化

**添加到:** `touchbao/fingerprint_config_loader.h`

```cpp
class FingerprintConfigLoader {
public:
    // ... 现有接口 ...
    
    // 序列化配置为 JSON (用于 IPC 传递)
    std::string SerializeToJson() const;
};
```

**添加到:** `touchbao/fingerprint_config_loader.cc`

```cpp
std::string FingerprintConfigLoader::SerializeToJson() const {
    base::Value::Dict dict;
    
    dict.Set("profile_id", config_.profile_id);
    dict.Set("schema_version", config_.schema_version);
    
    // Navigator
    base::Value::Dict navigator;
    navigator.Set("user_agent", config_.navigator.user_agent);
    navigator.Set("platform", config_.navigator.platform);
    navigator.Set("hardware_concurrency", config_.navigator.hardware_concurrency);
    navigator.Set("device_memory", config_.navigator.device_memory);
    navigator.Set("webdriver", config_.navigator.webdriver);
    dict.Set("navigator", std::move(navigator));
    
    // Screen
    base::Value::Dict screen;
    screen.Set("width", config_.screen.width);
    screen.Set("height", config_.screen.height);
    screen.Set("device_pixel_ratio", config_.screen.device_pixel_ratio);
    dict.Set("screen", std::move(screen));
    
    // ... 其他配置同理 ...
    
    // Fonts
    base::Value::Dict fonts;
    fonts.Set("mode", config_.fonts.mode);
    base::Value::List font_list;
    for (const auto& font : config_.fonts.list) {
        font_list.Append(font);
    }
    fonts.Set("list", std::move(font_list));
    dict.Set("fonts", std::move(fonts));
    
    // 序列化为 JSON 字符串
    std::string json;
    base::JSONWriter::Write(dict, &json);
    return json;
}
```

---

## 十九、TLS/JA3 指纹 (高级)

### 19.1 JA3 指纹原理

```
JA3 指纹基于 TLS Client Hello 消息的以下字段：
1. TLS 版本
2. 支持的密码套件列表
3. 支持的扩展列表
4. 支持的椭圆曲线
5. 支持的椭圆曲线点格式

格式: MD5(version,ciphers,extensions,curves,point_formats)
示例: 769,47-53-5-10-49161-49162-49171-49172-50-56-19-4,0-10-11,23-24-25,0
```

### 19.2 配置结构

```json
{
  "tls": {
    "ja3_fingerprint": "random",  // "real" | "random" | "chrome" | "firefox"
    "min_version": "TLS1.2",
    "max_version": "TLS1.3",
    "cipher_suites": [
      "TLS_AES_128_GCM_SHA256",
      "TLS_AES_256_GCM_SHA384",
      "TLS_CHACHA20_POLY1305_SHA256"
    ],
    "extensions_order": "randomize"
  }
}
```

### 19.3 SSL Context 修改

**文件:** `net/socket/ssl_client_socket_impl.cc`

```cpp
#include "touchbao/fingerprint_config_loader.h"
#include <random>
#include <algorithm>

int SSLClientSocketImpl::DoHandshake() {
    auto* config = touchbao::FingerprintConfigLoader::GetInstance();
    
    if (config->IsLoaded()) {
        // 修改 TLS 握手参数
        ModifyTLSHandshake(ssl_.get(), config);
    }
    
    return DoHandshakeInternal();
}

void SSLClientSocketImpl::ModifyTLSHandshake(
    SSL* ssl,
    touchbao::FingerprintConfigLoader* config) {
    
    // 1. 随机化密码套件顺序
    RandomizeCipherSuites(ssl, config->GetMasterSeed());
    
    // 2. 随机化扩展顺序
    RandomizeExtensionsOrder(ssl, config->GetMasterSeed());
    
    // 3. 修改支持的曲线
    SetSupportedCurves(ssl);
}

void SSLClientSocketImpl::RandomizeCipherSuites(SSL* ssl, int64_t seed) {
    // 获取当前密码套件列表
    STACK_OF(SSL_CIPHER)* ciphers = SSL_get_ciphers(ssl);
    
    if (!ciphers || sk_SSL_CIPHER_num(ciphers) == 0) {
        return;
    }
    
    // 使用 seed 进行确定性随机排序
    std::mt19937_64 rng(seed);
    
    std::vector<const SSL_CIPHER*> cipher_vec;
    for (size_t i = 0; i < sk_SSL_CIPHER_num(ciphers); ++i) {
        cipher_vec.push_back(sk_SSL_CIPHER_value(ciphers, i));
    }
    
    std::shuffle(cipher_vec.begin(), cipher_vec.end(), rng);
    
    // 重新设置密码套件顺序
    std::string cipher_string;
    for (const auto* cipher : cipher_vec) {
        if (!cipher_string.empty()) {
            cipher_string += ":";
        }
        cipher_string += SSL_CIPHER_get_name(cipher);
    }
    
    SSL_set_cipher_list(ssl, cipher_string.c_str());
}

void SSLClientSocketImpl::RandomizeExtensionsOrder(SSL* ssl, int64_t seed) {
    // 注意：扩展顺序的修改需要更底层的 BoringSSL 修改
    // 这里提供思路，实际实现需要修改 BoringSSL 源码
    
    // BoringSSL 扩展在 ssl/extensions.cc 中定义
    // 需要修改 tls_extension_order 数组的顺序
}
```

### 19.4 BoringSSL 扩展顺序修改

**文件:** `third_party/boringssl/src/ssl/extensions.cc`

```cpp
#include "touchbao/fingerprint_config_loader.h"

// 扩展 ID 列表 (原始顺序)
static const uint16_t kExtensionOrder[] = {
    TLSEXT_TYPE_server_name,              // 0
    TLSEXT_TYPE_extended_master_secret,   // 23
    TLSEXT_TYPE_renegotiate,              // 65281
    TLSEXT_TYPE_supported_groups,         // 10
    TLSEXT_TYPE_ec_point_formats,         // 11
    TLSEXT_TYPE_session_ticket,           // 35
    TLSEXT_TYPE_application_layer_protocol_negotiation, // 16
    TLSEXT_TYPE_status_request,           // 5
    TLSEXT_TYPE_signature_algorithms,     // 13
    TLSEXT_TYPE_signed_certificate_timestamp, // 18
    TLSEXT_TYPE_key_share,                // 51
    TLSEXT_TYPE_psk_key_exchange_modes,   // 45
    TLSEXT_TYPE_supported_versions,       // 43
    TLSEXT_TYPE_compress_certificate,     // 27
    // ... 更多扩展
};

// TouchBao: 获取随机化后的扩展顺序
std::vector<uint16_t> GetRandomizedExtensionOrder(int64_t seed) {
    std::vector<uint16_t> order(
        kExtensionOrder, 
        kExtensionOrder + sizeof(kExtensionOrder) / sizeof(kExtensionOrder[0])
    );
    
    // 使用 seed 进行确定性随机
    std::mt19937_64 rng(seed);
    
    // 只随机化中间部分，保留必要的首尾扩展
    // server_name 通常在开头，supported_versions 通常在结尾
    if (order.size() > 4) {
        std::shuffle(order.begin() + 1, order.end() - 2, rng);
    }
    
    return order;
}
```

### 19.5 JA3 指纹预设配置

```json
{
  "ja3_presets": {
    "chrome_146": {
      "version": "0x0303",
      "ciphers": "4865-4866-4867-49195-49199-49196-49200-52393-52392-49171-49172-156-157-47-53",
      "extensions": "0-23-65281-10-11-35-16-5-13-18-51-45-43-27-21",
      "curves": "29-23-24",
      "point_formats": "0"
    },
    "firefox_121": {
      "version": "0x0303",
      "ciphers": "4865-4867-4866-49195-49199-52393-52392-49196-49200-49162-49161-49171-49172-156-157-47-53",
      "extensions": "0-23-65281-10-11-35-16-5-34-51-43-13-45-28-21",
      "curves": "29-23-24-25",
      "point_formats": "0"
    }
  }
}
```

---

## 二十、Hook 点完整清单（更新版）

| 分类 | API | 源码文件 | 函数名 | 优先级 |
|------|-----|----------|--------|--------|
| **Navigator** | userAgent | `navigator.cc` | `userAgent()` | 🟥 高 |
| | platform | `navigator_id.cc` | `platform()` | 🟥 高 |
| | hardwareConcurrency | `navigator_concurrent_hardware.cc` | `hardwareConcurrency()` | 🟥 高 |
| | deviceMemory | `navigator_device_memory.cc` | `deviceMemory()` | 🟥 高 |
| | webdriver | `navigator.cc` | `webdriver()` | 🟥 高 |
| | languages | `navigator_language.cc` | `languages()` | 🟨 中 |
| **Screen** | width/height | `screen.cc` | `width()`, `height()` | 🟥 高 |
| | availWidth/availHeight | `screen.cc` | `availWidth()`, `availHeight()` | 🟨 中 |
| | colorDepth | `screen.cc` | `colorDepth()` | 🟨 中 |
| | devicePixelRatio | `local_dom_window.cc` | `devicePixelRatio()` | 🟥 高 |
| **WebGL** | getParameter | `webgl_rendering_context_base.cc` | `getParameter()` | 🟥 高 |
| | UNMASKED_VENDOR | `webgl_debug_renderer_info.cc` | `getParameter()` | 🟥 高 |
| | UNMASKED_RENDERER | `webgl_debug_renderer_info.cc` | `getParameter()` | 🟥 高 |
| **Canvas** | toDataURL | `html_canvas_element.cc` | `toDataURL()` | 🟥 高 |
| | toBlob | `html_canvas_element.cc` | `toBlob()` | 🟥 高 |
| | measureText | `canvas_rendering_context_2d.cc` | `measureText()` | 🟥 高 |
| | getImageData | `canvas_rendering_context_2d.cc` | `getImageData()` | 🟨 中 |
| **Audio** | sampleRate | `audio_context.cc` | `sampleRate()` | 🟨 中 |
| | getFloatFrequencyData | `analyser_node.cc` | `getFloatFrequencyData()` | 🟥 高 |
| | getByteFrequencyData | `analyser_node.cc` | `getByteFrequencyData()` | 🟨 中 |
| **Fonts** | query | `font_enumeration_cache.cc` | `QueryFontsForFrame()` | 🟥 高 |
| | check | `font_face_set_document.cc` | `check()` | 🟥 高 |
| **WebRTC** | RTCPeerConnection | `rtc_peer_connection.cc` | `constructor()` | 🟥 高 |
| | ICE Candidate | `rtc_ice_candidate.cc` | `Create()` | 🟥 高 |
| | getStats | `rtc_peer_connection.cc` | `getStats()` | 🟨 中 |
| **Timezone** | getTimezoneOffset | `builtins-date.cc` | `DatePrototypeGetTimezoneOffset` | 🟥 高 |
| | resolvedOptions | `intl-objects.cc` | `ResolvedOptions()` | 🟥 高 |
| | toString | `dateparser.cc` | `ToDateString()` | 🟨 中 |
| **ClientHints** | brands | `navigator_ua_data.cc` | `brands()` | 🟥 高 |
| | getHighEntropyValues | `navigator_ua_data.cc` | `getHighEntropyValues()` | 🟥 高 |
| | HTTP Headers | `client_hints.cc` | `AddClientHintsHeaders()` | 🟥 高 |
| **Variations** | chrome://version | `version_handler.cc` | `HandleRequestVariationInfo()` | 🟨 中 |
| **TLS/JA3** | SSL Handshake | `ssl_client_socket_impl.cc` | `DoHandshake()` | 🟨 中 |
| | Cipher Order | `extensions.cc` | `RandomizeCipherSuites()` | 🟨 中 |