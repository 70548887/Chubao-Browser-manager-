# Variations 与字体指纹开发方案

> 版本: 1.0.0  
> 适用: 触宝指纹浏览器 (Chromium 146)  
> 风险等级: ⬜ 最低

---

## 一、架构总览

```
┌─────────────────────────────────────────────────────────────────┐
│                         启动器 (Tauri)                           │
├─────────────────────────────────────────────────────────────────┤
│  前端 Vue3                    │           后端 Rust              │
│  ┌───────────────────────┐   │   ┌───────────────────────────┐ │
│  │ Step3Fingerprint.vue  │   │   │ config_writer.rs          │ │
│  │ - 字体配置 UI         │   │   │ - FontsConfig             │ │
│  │ - Variations 配置 UI  │   │   │ - VariationsConfig (新增) │ │
│  └───────────────────────┘   │   └───────────────────────────┘ │
│            │                  │              │                   │
│            ▼                  │              ▼                   │
│  ┌───────────────────────┐   │   ┌───────────────────────────┐ │
│  │ profileApi.ts         │◄──┼──►│ bm_fingerprint.json       │ │
│  │ - fingerprintToDto()  │   │   │ (配置文件输出)            │ │
│  └───────────────────────┘   │   └───────────────────────────┘ │
└─────────────────────────────────────────────────────────────────┘
                                      │
                                      ▼
┌─────────────────────────────────────────────────────────────────┐
│                      浏览器内核 (Chromium 146)                   │
├─────────────────────────────────────────────────────────────────┤
│  ┌───────────────────────────────────────────────────────────┐ │
│  │ --fingerprint-config="path/to/bm_fingerprint.json"        │ │
│  └───────────────────────────────────────────────────────────┘ │
│            │                                                     │
│            ▼                                                     │
│  ┌─────────────────────┐    ┌─────────────────────────────────┐│
│  │ variations_service  │    │ font_enumeration_cache          ││
│  │ - 读取 variations   │    │ - 读取 fonts 配置               ││
│  │ - 生成 Field Trials │    │ - 返回伪装字体列表              ││
│  └─────────────────────┘    └─────────────────────────────────┘│
└─────────────────────────────────────────────────────────────────┘
```

---

## 二、Variations 模块开发

### 2.1 配置结构设计

#### 后端 Rust 结构 (新增到 config_writer.rs)

```rust
/// Variations 配置 - 每个 Profile 独立
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariationsConfig {
    /// 变体 ID (格式: xxxxxxxx-xxxxxxxx)
    pub seed_id: String,
    /// 显示类型 (用于 chrome://version)
    pub seed_type: String,
    /// 启用的 Field Trials
    pub field_trials: Vec<FieldTrialConfig>,
    /// 启用的功能
    pub enabled_features: Vec<String>,
    /// 禁用的功能
    pub disabled_features: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldTrialConfig {
    pub study_name: String,
    pub group_name: String,
    /// 权重 (0-100)，用于模拟真实分布
    pub weight: u32,
}

impl Default for VariationsConfig {
    fn default() -> Self {
        Self {
            seed_id: generate_variations_id(),
            seed_type: "TouchBao Stable".to_string(),
            field_trials: vec![
                FieldTrialConfig {
                    study_name: "TouchBaoFingerprint".to_string(),
                    group_name: "Enabled".to_string(),
                    weight: 100,
                },
            ],
            enabled_features: vec![
                "OverlayScrollbar".to_string(),
                "ParallelDownloading".to_string(),
            ],
            disabled_features: vec![
                "AutomationControlled".to_string(),
                "CalculateNativeWinOcclusion".to_string(),
                "MediaRouter".to_string(),
            ],
        }
    }
}

/// 为每个 Profile 生成唯一的 Variations ID
fn generate_variations_id() -> String {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    format!("{:08x}-{:08x}", rng.gen::<u32>(), rng.gen::<u32>())
}

/// 基于 Profile ID 生成稳定的 Variations ID (重启不变)
fn generate_stable_variations_id(profile_id: &str) -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    
    let mut hasher = DefaultHasher::new();
    profile_id.hash(&mut hasher);
    let hash1 = hasher.finish() as u32;
    
    profile_id.to_uppercase().hash(&mut hasher);
    let hash2 = hasher.finish() as u32;
    
    format!("{:08x}-{:08x}", hash1, hash2)
}
```

#### bm_fingerprint.json 输出格式

```json
{
  "$schema": "bm_fingerprint_v2",
  "schema_version": 2,
  "profile_id": "abc123def456",
  
  "variations": {
    "seed_id": "f2f17cec-377be55a",
    "seed_type": "TouchBao Stable",
    "field_trials": [
      { "study_name": "TouchBaoFingerprint", "group_name": "Enabled", "weight": 100 }
    ],
    "enabled_features": ["OverlayScrollbar", "ParallelDownloading"],
    "disabled_features": ["AutomationControlled", "CalculateNativeWinOcclusion"]
  },
  
  "fonts": {
    "mode": "subset",
    "list": ["Arial", "Arial Black", "Calibri", ...]
  }
}
```

### 2.2 模拟真实分布算法

```rust
/// 模拟真实 Chrome 用户的 Feature 分布
pub fn generate_realistic_features(profile_id: &str) -> VariationsConfig {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    
    let mut hasher = DefaultHasher::new();
    profile_id.hash(&mut hasher);
    let seed = hasher.finish();
    
    // 基于 seed 生成伪随机数 (0.0 - 1.0)
    fn seeded_random(seed: u64, index: u32) -> f64 {
        let combined = seed.wrapping_add(index as u64);
        (combined % 10000) as f64 / 10000.0
    }
    
    let mut enabled = vec![];
    let mut disabled = vec![];
    
    // Feature 分布概率 (模拟真实 Chrome 用户)
    let feature_probabilities = [
        ("OverlayScrollbar", 0.85),           // 85% 用户启用
        ("ParallelDownloading", 0.90),        // 90% 用户启用
        ("SmoothScrolling", 0.95),            // 95% 用户启用
        ("BackForwardCache", 0.70),           // 70% 用户启用
        ("LazyFrameLoading", 0.80),           // 80% 用户启用
    ];
    
    for (i, (feature, probability)) in feature_probabilities.iter().enumerate() {
        if seeded_random(seed, i as u32) < *probability {
            enabled.push(feature.to_string());
        }
    }
    
    // 必须禁用的功能 (风控安全)
    disabled.push("AutomationControlled".to_string());
    disabled.push("CalculateNativeWinOcclusion".to_string());
    
    VariationsConfig {
        seed_id: generate_stable_variations_id(profile_id),
        seed_type: "TouchBao Stable".to_string(),
        field_trials: vec![
            FieldTrialConfig {
                study_name: "TouchBaoConfig".to_string(),
                group_name: "Stable".to_string(),
                weight: 100,
            },
        ],
        enabled_features: enabled,
        disabled_features: disabled,
    }
}
```

### 2.3 启动器命令行生成

```rust
/// 生成浏览器启动命令行参数
pub fn build_variations_args(config: &VariationsConfig) -> Vec<String> {
    let mut args = vec![];
    
    // 禁用 Google Variations 服务
    args.push("--disable-field-trial-config".to_string());
    args.push("--disable-background-networking".to_string());
    
    // 强制注入 Field Trials
    if !config.field_trials.is_empty() {
        let trials: Vec<String> = config.field_trials
            .iter()
            .map(|ft| format!("{}/{}", ft.study_name, ft.group_name))
            .collect();
        args.push(format!("--force-fieldtrials={}", trials.join("/")));
    }
    
    // 启用功能
    if !config.enabled_features.is_empty() {
        args.push(format!("--enable-features={}", config.enabled_features.join(",")));
    }
    
    // 禁用功能
    if !config.disabled_features.is_empty() {
        args.push(format!("--disable-features={}", config.disabled_features.join(",")));
    }
    
    args
}
```

---

## 三、字体指纹模块开发

### 3.1 后端配置结构 (已存在，需扩展)

```rust
/// Fonts 配置 (扩展版)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FontsConfig {
    /// 模式: subset | real | custom | random
    pub mode: String,
    /// 字体列表
    pub list: Vec<String>,
    /// 自定义字体 (mode=custom 时使用)
    pub custom_list: Option<Vec<String>>,
    /// 随机数量 (mode=random 时使用)
    pub random_count: Option<u32>,
}

impl FontsConfig {
    /// 按地区获取预设字体
    pub fn preset_for_locale(locale: &str) -> Self {
        match locale {
            "zh-CN" | "zh-TW" => Self::chinese_preset(),
            "ja-JP" => Self::japanese_preset(),
            "ko-KR" => Self::korean_preset(),
            _ => Self::western_preset(),
        }
    }
    
    fn chinese_preset() -> Self {
        Self {
            mode: "subset".to_string(),
            list: vec![
                "Arial", "Arial Black", "Calibri", "Cambria", "Courier New",
                "Georgia", "Helvetica", "Impact", "Microsoft YaHei", "SimSun",
                "SimHei", "NSimSun", "FangSong", "KaiTi", "Tahoma",
                "Times New Roman", "Trebuchet MS", "Verdana",
            ].into_iter().map(String::from).collect(),
            custom_list: None,
            random_count: None,
        }
    }
    
    fn japanese_preset() -> Self {
        Self {
            mode: "subset".to_string(),
            list: vec![
                "Arial", "Calibri", "Courier New", "Georgia", "Helvetica",
                "MS Gothic", "MS Mincho", "MS PGothic", "MS PMincho",
                "Meiryo", "Yu Gothic", "Yu Mincho", "Tahoma", "Verdana",
            ].into_iter().map(String::from).collect(),
            custom_list: None,
            random_count: None,
        }
    }
    
    fn korean_preset() -> Self {
        Self {
            mode: "subset".to_string(),
            list: vec![
                "Arial", "Calibri", "Courier New", "Georgia", "Gulim",
                "Batang", "Dotum", "Malgun Gothic", "Tahoma", "Verdana",
            ].into_iter().map(String::from).collect(),
            custom_list: None,
            random_count: None,
        }
    }
    
    fn western_preset() -> Self {
        Self {
            mode: "subset".to_string(),
            list: vec![
                "Arial", "Arial Black", "Calibri", "Cambria", "Courier New",
                "Georgia", "Helvetica", "Impact", "Lucida Console",
                "Palatino Linotype", "Segoe UI", "Tahoma", "Times New Roman",
                "Trebuchet MS", "Verdana",
            ].into_iter().map(String::from).collect(),
            custom_list: None,
            random_count: None,
        }
    }
}
```

### 3.2 前端 API 扩展 (profileApi.ts)

```typescript
// 字体配置接口
interface FontsConfigDTO {
    mode: 'subset' | 'real' | 'custom' | 'random';
    list: string[];
    custom_list?: string[];
    random_count?: number;
}

// 字体预设
const FONT_PRESETS = {
    'zh-CN': [
        'Arial', 'Arial Black', 'Calibri', 'Cambria', 'Courier New',
        'Georgia', 'Helvetica', 'Impact', 'Microsoft YaHei', 'SimSun',
        'SimHei', 'NSimSun', 'FangSong', 'KaiTi', 'Tahoma',
        'Times New Roman', 'Trebuchet MS', 'Verdana'
    ],
    'ja-JP': [
        'Arial', 'Calibri', 'Courier New', 'Georgia', 'Helvetica',
        'MS Gothic', 'MS Mincho', 'MS PGothic', 'MS PMincho',
        'Meiryo', 'Yu Gothic', 'Yu Mincho', 'Tahoma', 'Verdana'
    ],
    'ko-KR': [
        'Arial', 'Calibri', 'Courier New', 'Georgia', 'Gulim',
        'Batang', 'Dotum', 'Malgun Gothic', 'Tahoma', 'Verdana'
    ],
    'en-US': [
        'Arial', 'Arial Black', 'Calibri', 'Cambria', 'Courier New',
        'Georgia', 'Helvetica', 'Impact', 'Lucida Console',
        'Palatino Linotype', 'Segoe UI', 'Tahoma', 'Times New Roman',
        'Trebuchet MS', 'Verdana'
    ]
};

// fingerprintToDto 函数中的字体配置转换
function fingerprintToDto(form: any): FingerprintDTO {
    return {
        // ... 其他配置
        fonts: {
            mode: form.fontsMode || 'subset',
            list: form.fontsList || FONT_PRESETS['zh-CN'],
            custom_list: form.fontsMode === 'custom' ? form.customFonts : undefined,
            random_count: form.fontsMode === 'random' ? (form.randomFontCount || 15) : undefined,
        },
    };
}
```

### 3.3 前端 UI 组件 (Step3Fingerprint.vue 新增部分)

```vue
<!-- 字体指纹配置 - 插入到 Step3Fingerprint.vue -->

<!-- 字体指纹 -->
<div class="section-header">
  <h3 class="section-title">字体指纹</h3>
</div>

<div class="form-row">
  <label class="form-label">字体模式</label>
  <div class="radio-group">
    <label class="radio-item" :class="{ active: modelValue.fontsMode === 'subset' }">
      <input type="radio" value="subset" :checked="modelValue.fontsMode === 'subset'" 
             @change="updateField('fontsMode', 'subset')" />
      <span>子集模式</span>
    </label>
    <label class="radio-item" :class="{ active: modelValue.fontsMode === 'real' }">
      <input type="radio" value="real" :checked="modelValue.fontsMode === 'real'" 
             @change="updateField('fontsMode', 'real')" />
      <span>真实字体</span>
    </label>
    <label class="radio-item" :class="{ active: modelValue.fontsMode === 'custom' }">
      <input type="radio" value="custom" :checked="modelValue.fontsMode === 'custom'" 
             @change="updateField('fontsMode', 'custom')" />
      <span>自定义</span>
    </label>
    <label class="radio-item" :class="{ active: modelValue.fontsMode === 'random' }">
      <input type="radio" value="random" :checked="modelValue.fontsMode === 'random'" 
             @change="updateField('fontsMode', 'random')" />
      <span>随机</span>
    </label>
  </div>
</div>

<!-- 字体预设选择 (subset 模式) -->
<div v-if="modelValue.fontsMode === 'subset'" class="form-row sub-row">
  <label class="form-label">地区预设</label>
  <select 
    :value="modelValue.fontsPreset"
    class="form-select"
    @change="handleFontsPresetChange(($event.target as HTMLSelectElement).value)"
  >
    <option value="zh-CN">中文（简体）</option>
    <option value="zh-TW">中文（繁体）</option>
    <option value="ja-JP">日文</option>
    <option value="ko-KR">韩文</option>
    <option value="en-US">英文（西方）</option>
  </select>
</div>

<!-- 自定义字体列表 (custom 模式) -->
<div v-if="modelValue.fontsMode === 'custom'" class="form-row sub-row">
  <label class="form-label">字体列表</label>
  <textarea 
    :value="modelValue.customFonts?.join('\n')"
    class="form-textarea" 
    rows="5" 
    placeholder="每行一个字体名称，例如：&#10;Arial&#10;Times New Roman&#10;Microsoft YaHei"
    @input="handleCustomFontsChange(($event.target as HTMLTextAreaElement).value)"
  ></textarea>
  <p class="form-hint">当前 {{ modelValue.customFonts?.length || 0 }} 个字体</p>
</div>

<!-- 随机数量 (random 模式) -->
<div v-if="modelValue.fontsMode === 'random'" class="form-row sub-row">
  <label class="form-label">随机数量</label>
  <input 
    :value="modelValue.randomFontCount"
    class="form-input" 
    type="number" 
    min="5" 
    max="50" 
    placeholder="15"
    @input="updateField('randomFontCount', parseInt(($event.target as HTMLInputElement).value) || 15)"
  />
  <p class="form-hint">从常用字体库中随机选择 5-50 个字体</p>
</div>

<!-- 风险提示 -->
<div v-if="modelValue.fontsMode === 'real'" class="form-row">
  <div class="warning-box">
    <svg class="warning-icon" viewBox="0 0 20 20" fill="currentColor">
      <path fill-rule="evenodd" d="M8.257 3.099c.765-1.36 2.722-1.36 3.486 0l5.58 9.92c.75 1.334-.213 2.98-1.742 2.98H4.42c-1.53 0-2.493-1.646-1.743-2.98l5.58-9.92zM11 13a1 1 0 11-2 0 1 1 0 012 0zm-1-8a1 1 0 00-1 1v3a1 1 0 002 0V6a1 1 0 00-1-1z" clip-rule="evenodd"/>
    </svg>
    <span>使用真实字体可能导致浏览器指纹被唯一识别，风控风险较高</span>
  </div>
</div>
```

### 3.4 前端脚本扩展 (useCreateProfile.ts)

```typescript
// 表单数据类型扩展
interface CreateProfileFormData {
    // ... 已有字段
    
    // 字体配置
    fontsMode: 'subset' | 'real' | 'custom' | 'random';
    fontsPreset: string;
    customFonts: string[];
    randomFontCount: number;
}

// 默认值
const defaultFormData: CreateProfileFormData = {
    // ... 已有默认值
    
    fontsMode: 'subset',
    fontsPreset: 'zh-CN',
    customFonts: [],
    randomFontCount: 15,
};

// 字体预设数据
const FONT_PRESETS: Record<string, string[]> = {
    'zh-CN': ['Arial', 'Microsoft YaHei', 'SimSun', 'SimHei', ...],
    'ja-JP': ['Arial', 'MS Gothic', 'Meiryo', ...],
    'ko-KR': ['Arial', 'Gulim', 'Malgun Gothic', ...],
    'en-US': ['Arial', 'Times New Roman', 'Verdana', ...],
};

// 处理字体预设变更
function handleFontsPresetChange(preset: string) {
    formData.value.fontsPreset = preset;
    formData.value.customFonts = FONT_PRESETS[preset] || FONT_PRESETS['zh-CN'];
}

// 处理自定义字体变更
function handleCustomFontsChange(text: string) {
    formData.value.customFonts = text.split('\n').map(s => s.trim()).filter(Boolean);
}
```

---

## 四、浏览器内核开发

### 4.1 GN 编译配置 (args.gn)

```gn
# ===== 基础配置 =====
is_debug = false
is_official_build = false
is_chrome_branded = false
chrome_pgo_phase = 0

# ===== Variations 配置 =====
fieldtrial_testing_enabled = false
disable_fieldtrial_testing_config = true

# ===== 安全配置 =====
enable_nacl = false
enable_widevine = true

# ===== 性能配置 =====
symbol_level = 0
blink_symbol_level = 0
```

### 4.2 指纹配置加载器

**文件位置:** `chromium/touchbao/fingerprint_config_loader.h`

```cpp
#ifndef TOUCHBAO_FINGERPRINT_CONFIG_LOADER_H_
#define TOUCHBAO_FINGERPRINT_CONFIG_LOADER_H_

#include <string>
#include <vector>
#include "base/values.h"

namespace touchbao {

struct FontsConfig {
    std::string mode;  // "subset" | "real" | "custom" | "random"
    std::vector<std::string> list;
    std::vector<std::string> custom_list;
    int random_count;
};

struct VariationsConfig {
    std::string seed_id;
    std::string seed_type;
    std::vector<std::string> enabled_features;
    std::vector<std::string> disabled_features;
};

class FingerprintConfigLoader {
public:
    static FingerprintConfigLoader* GetInstance();
    
    // 从命令行参数加载配置
    bool LoadFromCommandLine();
    
    // 获取字体配置
    const FontsConfig& GetFontsConfig() const { return fonts_config_; }
    
    // 获取 Variations 配置
    const VariationsConfig& GetVariationsConfig() const { return variations_config_; }
    
    // 检查字体是否在允许列表中
    bool IsFontAllowed(const std::string& font_name) const;

private:
    FingerprintConfigLoader();
    ~FingerprintConfigLoader();
    
    bool ParseConfigFile(const std::string& path);
    void ParseFontsConfig(const base::Value::Dict& dict);
    void ParseVariationsConfig(const base::Value::Dict& dict);
    
    FontsConfig fonts_config_;
    VariationsConfig variations_config_;
    bool is_loaded_ = false;
};

}  // namespace touchbao

#endif  // TOUCHBAO_FINGERPRINT_CONFIG_LOADER_H_
```

**文件位置:** `chromium/touchbao/fingerprint_config_loader.cc`

```cpp
#include "touchbao/fingerprint_config_loader.h"

#include "base/command_line.h"
#include "base/files/file_util.h"
#include "base/json/json_reader.h"
#include "base/logging.h"

namespace touchbao {

// 单例实现
FingerprintConfigLoader* FingerprintConfigLoader::GetInstance() {
    static FingerprintConfigLoader instance;
    return &instance;
}

FingerprintConfigLoader::FingerprintConfigLoader() {
    LoadFromCommandLine();
}

bool FingerprintConfigLoader::LoadFromCommandLine() {
    if (is_loaded_) return true;
    
    auto* cmd = base::CommandLine::ForCurrentProcess();
    if (!cmd->HasSwitch("fingerprint-config")) {
        LOG(WARNING) << "No fingerprint config specified";
        return false;
    }
    
    std::string config_path = cmd->GetSwitchValueASCII("fingerprint-config");
    return ParseConfigFile(config_path);
}

bool FingerprintConfigLoader::ParseConfigFile(const std::string& path) {
    std::string json_content;
    if (!base::ReadFileToString(base::FilePath::FromUTF8Unsafe(path), &json_content)) {
        LOG(ERROR) << "Failed to read fingerprint config: " << path;
        return false;
    }
    
    auto config = base::JSONReader::Read(json_content);
    if (!config || !config->is_dict()) {
        LOG(ERROR) << "Invalid fingerprint config JSON";
        return false;
    }
    
    const auto& dict = config->GetDict();
    
    // 解析字体配置
    if (const auto* fonts = dict.FindDict("fonts")) {
        ParseFontsConfig(*fonts);
    }
    
    // 解析 Variations 配置
    if (const auto* variations = dict.FindDict("variations")) {
        ParseVariationsConfig(*variations);
    }
    
    is_loaded_ = true;
    return true;
}

void FingerprintConfigLoader::ParseFontsConfig(const base::Value::Dict& dict) {
    if (const auto* mode = dict.FindString("mode")) {
        fonts_config_.mode = *mode;
    }
    
    if (const auto* list = dict.FindList("list")) {
        for (const auto& item : *list) {
            if (item.is_string()) {
                fonts_config_.list.push_back(item.GetString());
            }
        }
    }
}

void FingerprintConfigLoader::ParseVariationsConfig(const base::Value::Dict& dict) {
    if (const auto* seed_id = dict.FindString("seed_id")) {
        variations_config_.seed_id = *seed_id;
    }
    
    if (const auto* seed_type = dict.FindString("seed_type")) {
        variations_config_.seed_type = *seed_type;
    }
    
    if (const auto* enabled = dict.FindList("enabled_features")) {
        for (const auto& item : *enabled) {
            if (item.is_string()) {
                variations_config_.enabled_features.push_back(item.GetString());
            }
        }
    }
    
    if (const auto* disabled = dict.FindList("disabled_features")) {
        for (const auto& item : *disabled) {
            if (item.is_string()) {
                variations_config_.disabled_features.push_back(item.GetString());
            }
        }
    }
}

bool FingerprintConfigLoader::IsFontAllowed(const std::string& font_name) const {
    if (fonts_config_.mode == "real") {
        return true;  // 真实模式，允许所有字体
    }
    
    for (const auto& font : fonts_config_.list) {
        if (base::EqualsCaseInsensitiveASCII(font, font_name)) {
            return true;
        }
    }
    return false;
}

}  // namespace touchbao
```

### 4.3 字体枚举 Hook

**文件位置:** `third_party/blink/renderer/modules/font_access/font_enumeration_cache.cc`

```cpp
// 找到 QueryFontsForFrame 函数，添加以下修改

#include "touchbao/fingerprint_config_loader.h"

void FontEnumerationCache::QueryFontsForFrame(
    LocalFrame* frame,
    QueryFontsCallback callback) {
    
    // 获取指纹配置
    auto* config = touchbao::FingerprintConfigLoader::GetInstance();
    const auto& fonts_config = config->GetFontsConfig();
    
    // 如果是真实模式，走原逻辑
    if (fonts_config.mode == "real") {
        QuerySystemFonts(std::move(callback));
        return;
    }
    
    // 否则返回配置的字体列表
    Vector<FontMetadata> results;
    for (const auto& font_name : fonts_config.list) {
        FontMetadata metadata;
        metadata.postscript_name = String::FromUTF8(font_name);
        metadata.full_name = String::FromUTF8(font_name);
        metadata.family = String::FromUTF8(font_name);
        metadata.style = "Regular";
        results.push_back(std::move(metadata));
    }
    
    std::move(callback).Run(std::move(results));
}
```

### 4.4 chrome://version 显示修改

**文件位置:** `chrome/browser/ui/webui/version/version_handler.cc`

```cpp
#include "touchbao/fingerprint_config_loader.h"

void VersionHandler::HandleRequestVariationInfo(const base::Value::List& args) {
    base::Value::Dict variations_info;
    
    // 获取配置
    auto* config = touchbao::FingerprintConfigLoader::GetInstance();
    const auto& var_config = config->GetVariationsConfig();
    
    // 设置显示值
    if (!var_config.seed_type.empty()) {
        variations_info.Set("variations-seed-type", var_config.seed_type);
    } else {
        variations_info.Set("variations-seed-type", "TouchBao Stable");
    }
    
    // 设置变体 ID
    if (!var_config.seed_id.empty()) {
        variations_info.Set("active-variations", var_config.seed_id);
    }
    
    // 命令行变体 (Base64 编码)
    std::string cmd_variations = base::Base64Encode(
        "{\"type\":\"touchbao\",\"version\":\"1.0\"}"
    );
    variations_info.Set("variations-cmd", cmd_variations);
    
    ResolveJavascriptCallback(args[0], variations_info);
}
```

---

## 五、配置文件对接

### 5.1 完整配置文件示例 (bm_fingerprint.json)

```json
{
  "$schema": "bm_fingerprint_v2",
  "schema_version": 2,
  "profile_id": "abc123def456789",
  "created_at": "2026-02-02T10:30:00Z",
  
  "seed": {
    "master": 1234567890,
    "canvas": 9876543210,
    "webgl": 1122334455,
    "audio": 5544332211
  },
  
  "navigator": {
    "user_agent": "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 Chrome/146.0.0.0",
    "platform": "Win32",
    "hardware_concurrency": 8,
    "device_memory": 16
  },
  
  "variations": {
    "seed_id": "f2f17cec-377be55a",
    "seed_type": "TouchBao Stable",
    "field_trials": [
      { "study_name": "TouchBaoConfig", "group_name": "Stable", "weight": 100 }
    ],
    "enabled_features": [
      "OverlayScrollbar",
      "ParallelDownloading",
      "SmoothScrolling"
    ],
    "disabled_features": [
      "AutomationControlled",
      "CalculateNativeWinOcclusion",
      "MediaRouter"
    ]
  },
  
  "fonts": {
    "mode": "subset",
    "list": [
      "Arial",
      "Arial Black",
      "Calibri",
      "Cambria",
      "Courier New",
      "Georgia",
      "Helvetica",
      "Impact",
      "Microsoft YaHei",
      "SimSun",
      "SimHei",
      "Tahoma",
      "Times New Roman",
      "Trebuchet MS",
      "Verdana"
    ],
    "custom_list": null,
    "random_count": null
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
  
  "webgl": {
    "vendor": "Google Inc. (NVIDIA)",
    "renderer": "ANGLE (NVIDIA, NVIDIA GeForce GTX 1660)",
    "noise_enabled": true
  },
  
  "webrtc": {
    "mode": "disabled",
    "public_ip": null,
    "local_ip": null
  }
}
```

### 5.2 启动器与内核对接流程

```
1. 用户创建/编辑 Profile
        │
        ▼
2. 前端收集表单数据 (Step3Fingerprint.vue)
        │
        ▼
3. 调用 profileApi.ts → fingerprintToDto()
        │
        ▼
4. 后端 Rust 处理 (config_writer.rs)
   - 生成 VariationsConfig (带唯一 seed_id)
   - 生成 FontsConfig
   - 写入 bm_fingerprint.json
        │
        ▼
5. 启动浏览器时传递参数:
   --fingerprint-config="path/to/bm_fingerprint.json"
   --force-fieldtrials=...
   --enable-features=...
   --disable-features=...
        │
        ▼
6. 内核加载配置 (FingerprintConfigLoader)
   - 读取 JSON 配置
   - Hook 字体枚举 API
   - Hook Variations 显示
```

---

## 六、实施计划

### 阶段一：启动器开发 (3天)

| 任务 | 文件 | 工时 |
|------|------|------|
| 后端 VariationsConfig 结构 | config_writer.rs | 4h |
| 后端 FontsConfig 扩展 | config_writer.rs | 2h |
| 前端字体 UI 组件 | Step3Fingerprint.vue | 4h |
| 前端 API 对接 | profileApi.ts | 2h |
| 命令行参数生成 | lib.rs | 2h |
| 单元测试 | - | 4h |

### 阶段二：内核开发 (5天)

| 任务 | 文件 | 工时 |
|------|------|------|
| GN 配置 | args.gn | 1h |
| FingerprintConfigLoader | touchbao/*.cc | 8h |
| 字体枚举 Hook | font_enumeration_cache.cc | 8h |
| Canvas 字体检测防护 | canvas_rendering_context_2d.cc | 4h |
| Variations 显示修改 | version_handler.cc | 4h |
| 编译测试 | - | 8h |

### 阶段三：集成测试 (2天)

| 任务 | 说明 | 工时 |
|------|------|------|
| 启动器-内核对接测试 | 验证配置文件传递 | 4h |
| 风控检测测试 | BrowserLeaks, CreepJS | 4h |
| 多 Profile 测试 | 验证每个 Profile 唯一 | 4h |
| 回归测试 | 确保现有功能正常 | 4h |

---

## 七、风控验证

### 测试网站

1. **BrowserLeaks** - https://browserleaks.com/fonts
2. **CreepJS** - https://abrahamjuliot.github.io/creepjs/
3. **FingerprintJS** - https://fingerprintjs.github.io/fingerprintjs/
4. **AmIUnique** - https://amiunique.org/fingerprint

### 验证清单

| 检查项 | 预期结果 | 通过标准 |
|--------|----------|----------|
| 字体数量 | 配置数量 | ±0 |
| 字体列表 | 仅配置字体 | 无额外字体 |
| Variations ID | 每个 Profile 不同 | 唯一性 |
| chrome://version | 自定义显示 | 无 Null |
| Feature Flags | 配置生效 | 功能正常 |

---

## 八、注意事项

1. **字体列表不要太少** - 少于 10 个字体可能被标记为异常
2. **字体列表不要太多** - 超过 100 个字体同样可能异常
3. **保持地区一致性** - 中文系统应包含中文字体
4. **Variations ID 要稳定** - 同一 Profile 重启后 ID 应不变
5. **禁用功能要谨慎** - 只禁用确认安全的功能
