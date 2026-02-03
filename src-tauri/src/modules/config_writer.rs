// 配置文件写入器 - 生成 bm_fingerprint.json 和 bm_cloud.json
// 适配内核 JSON 格式规范（驼峰命名，type 字段）

use serde::{Deserialize, Serialize};
use std::path::Path;
use std::fs;

// ============================================================================
// bm_fingerprint.json - 内核格式 (驼峰命名)
// type: 0/1=关闭, 2=启用
// init: 必须为 2 才启用指纹功能
// ============================================================================

/// UA 配置
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UaConfig {
    pub r#type: u8,
    pub user_agent: String,
}

impl Default for UaConfig {
    fn default() -> Self {
        Self {
            r#type: 2,
            user_agent: "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/146.0.0.0 Safari/537.36".to_string(),
        }
    }
}

/// 硬件资源配置
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceInfoConfig {
    pub r#type: u8,
    pub cpu: u32,
    pub memory: f64,
}

impl Default for ResourceInfoConfig {
    fn default() -> Self {
        Self {
            r#type: 2,
            cpu: 8,
            memory: 16.0,
        }
    }
}

/// 分辨率配置
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResolutionConfig {
    pub r#type: u8,
    pub monitor_width: u32,
    pub monitor_height: u32,
    pub avail_width: u32,
    pub avail_height: u32,
    pub color_depth: u32,
}

impl Default for ResolutionConfig {
    fn default() -> Self {
        Self {
            r#type: 2,
            monitor_width: 1920,
            monitor_height: 1080,
            avail_width: 1920,
            avail_height: 1040,
            color_depth: 24,
        }
    }
}

/// 时区配置
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeZoneConfig {
    pub r#type: u8,
    pub gmt: String,
}

impl Default for TimeZoneConfig {
    fn default() -> Self {
        Self {
            r#type: 2,
            gmt: "Asia/Shanghai".to_string(),
        }
    }
}

/// 语言配置
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LanguageConfig {
    pub r#type: u8,
    pub interface_language: String,
    pub languages: Vec<String>,
}

impl Default for LanguageConfig {
    fn default() -> Self {
        Self {
            r#type: 2,
            interface_language: "zh-CN".to_string(),
            languages: vec!["zh-CN".to_string(), "zh".to_string(), "en-US".to_string(), "en".to_string()],
        }
    }
}

/// WebGL 设备配置
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebGLDeviceConfig {
    pub r#type: u8,
    pub vendors: String,
    pub renderer: String,
}

impl Default for WebGLDeviceConfig {
    fn default() -> Self {
        Self {
            r#type: 2,
            vendors: "Google Inc. (NVIDIA)".to_string(),
            renderer: "ANGLE (NVIDIA, NVIDIA GeForce GTX 1660 SUPER)".to_string(),
        }
    }
}

/// Canvas 配置（简单模式）
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CanvasConfig {
    pub r#type: u8,
    pub noise_enabled: bool,
    pub noise_factor: f64,
}

impl Default for CanvasConfig {
    fn default() -> Self {
        Self {
            r#type: 2,
            noise_enabled: true,
            noise_factor: 0.0001,
        }
    }
}

/// AudioContext 配置
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AudioContextConfig {
    pub r#type: u8,
    pub noise: Vec<f64>,
}

impl Default for AudioContextConfig {
    fn default() -> Self {
        Self {
            r#type: 2,
            noise: vec![0.0001, 0.0002, 0.0001],
        }
    }
}

/// ClientRects 配置
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientRectsConfig {
    pub r#type: u8,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl Default for ClientRectsConfig {
    fn default() -> Self {
        Self {
            r#type: 2,
            x: 0.0001,
            y: 0.0001,
            width: 0.0001,
            height: 0.0001,
        }
    }
}

/// 地理位置配置
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocationConfig {
    pub r#type: u8,
    pub permissions: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latitude: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub longitude: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accuracy: Option<f64>,
}

impl Default for LocationConfig {
    fn default() -> Self {
        Self {
            r#type: 2,
            permissions: false,
            latitude: None,
            longitude: None,
            accuracy: None,
        }
    }
}

/// WebRTC 配置
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebRTCConfig {
    pub r#type: u8,
    pub mode: String,  // "real" | "fake" | "disabled"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_ip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_ip: Option<String>,
}

impl Default for WebRTCConfig {
    fn default() -> Self {
        Self {
            r#type: 0,  // 默认禁用
            mode: "disabled".to_string(),
            public_ip: None,
            local_ip: None,
        }
    }
}

/// 字体配置
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FontsConfig {
    pub r#type: u8,
    pub mode: String,  // "subset" | "real" | "custom" | "random"
    pub list: Vec<String>,
}

impl Default for FontsConfig {
    fn default() -> Self {
        Self {
            r#type: 2,
            mode: "subset".to_string(),
            list: vec![
                "Arial".to_string(),
                "Arial Black".to_string(),
                "Calibri".to_string(),
                "Cambria".to_string(),
                "Courier New".to_string(),
                "Georgia".to_string(),
                "Helvetica".to_string(),
                "Impact".to_string(),
                "Microsoft YaHei".to_string(),
                "SimSun".to_string(),
                "SimHei".to_string(),
                "Tahoma".to_string(),
                "Times New Roman".to_string(),
                "Trebuchet MS".to_string(),
                "Verdana".to_string(),
            ],
        }
    }
}

/// Field Trial 配置
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FieldTrialConfig {
    pub trial_name: String,
    pub group_name: String,
}

/// Variations 配置（内核实验分组）
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VariationsConfig {
    pub r#type: u8,
    pub enabled: bool,
    pub seed_id: String,
    pub seed_type: String,  // "stable" | "random" | "profile_based"
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub field_trials: Vec<FieldTrialConfig>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub enabled_features: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub disabled_features: Vec<String>,
}

impl Default for VariationsConfig {
    fn default() -> Self {
        Self {
            r#type: 2,
            enabled: true,
            seed_id: String::new(),
            seed_type: "profile_based".to_string(),
            field_trials: vec![],
            enabled_features: vec![],
            disabled_features: vec![
                "AutofillServerCommunication".to_string(),
            ],
        }
    }
}

/// 完整指纹配置 (bm_fingerprint.json) - 内核格式
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FingerprintFileConfig {
    /// 初始化标记，必须为 2 才启用指纹功能
    pub init: u8,
    
    pub ua: UaConfig,
    pub resource_info: ResourceInfoConfig,
    pub resolution: ResolutionConfig,
    pub time_zone: TimeZoneConfig,
    pub language: LanguageConfig,
    #[serde(rename = "webGLDevice")]
    pub web_gl_device: WebGLDeviceConfig,
    pub canvas: CanvasConfig,
    pub audio_context: AudioContextConfig,
    pub client_rects: ClientRectsConfig,
    pub location: LocationConfig,
    
    // 扩展字段（内核支持）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webrtc: Option<WebRTCConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fonts: Option<FontsConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variations: Option<VariationsConfig>,
}

impl Default for FingerprintFileConfig {
    fn default() -> Self {
        Self {
            init: 2,  // 启用指纹功能
            ua: UaConfig::default(),
            resource_info: ResourceInfoConfig::default(),
            resolution: ResolutionConfig::default(),
            time_zone: TimeZoneConfig::default(),
            language: LanguageConfig::default(),
            web_gl_device: WebGLDeviceConfig::default(),
            canvas: CanvasConfig::default(),
            audio_context: AudioContextConfig::default(),
            client_rects: ClientRectsConfig::default(),
            location: LocationConfig::default(),
            webrtc: None,
            fonts: None,
            variations: None,
        }
    }
}

// ============================================================================
// bm_cloud.json Schema (保持原有格式)
// ============================================================================

/// 云端连接配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudConnectionConfig {
    pub enabled: bool,
    pub url: String,
    pub reconnect_strategy: String,
    pub max_reconnect_attempts: u32,
    pub initial_delay_ms: u32,
    pub max_delay_ms: u32,
}

impl Default for CloudConnectionConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            url: "wss://your-server.com/ws".to_string(),
            reconnect_strategy: "exponential".to_string(),
            max_reconnect_attempts: 10,
            initial_delay_ms: 1000,
            max_delay_ms: 60000,
        }
    }
}

/// 本地连接配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalConnectionConfig {
    pub enabled: bool,
    pub url: String,
    pub priority: u32,
}

impl Default for LocalConnectionConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            url: "ws://127.0.0.1:9527/ws".to_string(),
            priority: 1,
        }
    }
}

/// 连接配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionConfig {
    pub mode: String,
    pub cloud: CloudConnectionConfig,
    pub local: LocalConnectionConfig,
}

impl Default for ConnectionConfig {
    fn default() -> Self {
        Self {
            mode: "none".to_string(),
            cloud: CloudConnectionConfig::default(),
            local: LocalConnectionConfig::default(),
        }
    }
}

/// 设备信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceInfo {
    pub id: String,
    pub name: String,
    pub group: String,
}

/// 心跳配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeartbeatConfig {
    pub interval_seconds: u32,
    pub timeout_seconds: u32,
}

impl Default for HeartbeatConfig {
    fn default() -> Self {
        Self {
            interval_seconds: 30,
            timeout_seconds: 10,
        }
    }
}

/// 功能开关
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeaturesConfig {
    pub script_engine: bool,
    pub hibernate: bool,
    pub screenshot: bool,
}

impl Default for FeaturesConfig {
    fn default() -> Self {
        Self {
            script_engine: true,
            hibernate: true,
            screenshot: true,
        }
    }
}

/// 云端配置文件 (bm_cloud.json)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudFileConfig {
    pub schema_version: u32,
    pub connection: ConnectionConfig,
    pub device: DeviceInfo,
    pub heartbeat: HeartbeatConfig,
    pub features: FeaturesConfig,
}

impl CloudFileConfig {
    pub fn new(device_id: &str, device_name: &str, group: &str) -> Self {
        Self {
            schema_version: 1,
            connection: ConnectionConfig::default(),
            device: DeviceInfo {
                id: device_id.to_string(),
                name: device_name.to_string(),
                group: group.to_string(),
            },
            heartbeat: HeartbeatConfig::default(),
            features: FeaturesConfig::default(),
        }
    }
}

// ============================================================================
// 配置文件写入器
// ============================================================================

/// 配置文件写入器
pub struct ConfigWriter;

impl ConfigWriter {
    /// 写入指纹配置文件
    pub fn write_fingerprint_config(
        user_data_dir: &Path,
        config: &FingerprintFileConfig,
    ) -> Result<(), String> {
        fs::create_dir_all(user_data_dir)
            .map_err(|e| format!("创建目录失败: {}", e))?;
        
        let json = serde_json::to_string_pretty(config)
            .map_err(|e| format!("序列化失败: {}", e))?;
        
        let config_path = user_data_dir.join("bm_fingerprint.json");
        fs::write(&config_path, json)
            .map_err(|e| format!("写入文件失败: {}", e))?;
        
        tracing::info!("指纹配置已写入: {:?}", config_path);
        Ok(())
    }
    
    /// 写入云端配置文件
    pub fn write_cloud_config(
        user_data_dir: &Path,
        config: &CloudFileConfig,
    ) -> Result<(), String> {
        fs::create_dir_all(user_data_dir)
            .map_err(|e| format!("创建目录失败: {}", e))?;
        
        let json = serde_json::to_string_pretty(config)
            .map_err(|e| format!("序列化失败: {}", e))?;
        
        let config_path = user_data_dir.join("bm_cloud.json");
        fs::write(&config_path, json)
            .map_err(|e| format!("写入文件失败: {}", e))?;
        
        tracing::info!("云端配置已写入: {:?}", config_path);
        Ok(())
    }
    
    /// 一次性写入所有配置文件（启动前调用）
    pub fn setup_profile_configs(
        user_data_dir: &Path,
        profile_id: &str,
        profile_name: &str,
        group: &str,
        fingerprint: &crate::modules::profile::Fingerprint,
    ) -> Result<(), String> {
        let fp_config = Self::build_fingerprint_config(profile_id, fingerprint);
        Self::write_fingerprint_config(user_data_dir, &fp_config)?;
        
        let cloud_config = CloudFileConfig::new(profile_id, profile_name, group);
        Self::write_cloud_config(user_data_dir, &cloud_config)?;
        
        Ok(())
    }
    
    /// 从 Profile Fingerprint 构建内核格式配置
    fn build_fingerprint_config(
        profile_id: &str,
        fp: &crate::modules::profile::Fingerprint,
    ) -> FingerprintFileConfig {
        let (width, height) = Self::parse_screen_resolution(&fp.screen_resolution);
        
        // 判断 Canvas 是否启用
        let canvas_enabled = fp.canvas_noise;
        let canvas_type = if canvas_enabled { 2 } else { 0 };
        
        // 判断 Audio 是否启用
        let audio_enabled = fp.audio_noise;
        let audio_type = if audio_enabled { 2 } else { 0 };
        
        // WebRTC 模式转 type
        let webrtc_mode = fp.webrtc.clone().unwrap_or_else(|| "disabled".to_string());
        let webrtc_type = match webrtc_mode.as_str() {
            "real" | "fake" => 2,
            _ => 0,
        };
        
        // 字体模式
        let fonts_mode = fp.fonts_mode.clone().unwrap_or_else(|| "subset".to_string());
        let fonts_type = if fonts_mode != "real" { 2 } else { 0 };
        
        FingerprintFileConfig {
            init: 2,  // 启用指纹功能
            
            ua: UaConfig {
                r#type: 2,
                user_agent: fp.user_agent.clone(),
            },
            
            resource_info: ResourceInfoConfig {
                r#type: 2,
                cpu: fp.hardware_concurrency as u32,
                memory: fp.device_memory as f64,
            },
            
            resolution: ResolutionConfig {
                r#type: 2,
                monitor_width: width,
                monitor_height: height,
                avail_width: width,
                avail_height: height.saturating_sub(40),
                color_depth: 24,
            },
            
            time_zone: TimeZoneConfig {
                r#type: 2,
                gmt: fp.timezone.clone(),
            },
            
            language: LanguageConfig {
                r#type: 2,
                interface_language: fp.language.clone(),
                languages: vec![
                    fp.language.clone(),
                    "zh".to_string(),
                    "en-US".to_string(),
                    "en".to_string(),
                ],
            },
            
            web_gl_device: WebGLDeviceConfig {
                r#type: if fp.webgl_noise { 2 } else { 0 },
                vendors: fp.webgl_vendor.clone().unwrap_or_else(|| "Google Inc. (NVIDIA)".to_string()),
                renderer: fp.webgl_renderer.clone().unwrap_or_else(|| "ANGLE (NVIDIA, NVIDIA GeForce GTX 1660 SUPER)".to_string()),
            },
            
            canvas: CanvasConfig {
                r#type: canvas_type,
                noise_enabled: canvas_enabled,
                noise_factor: 0.0001,
            },
            
            audio_context: AudioContextConfig {
                r#type: audio_type,
                noise: if audio_enabled {
                    vec![0.0001, 0.0002, 0.0001]
                } else {
                    vec![]
                },
            },
            
            client_rects: ClientRectsConfig {
                r#type: 2,
                x: 0.0001,
                y: 0.0001,
                width: 0.0001,
                height: 0.0001,
            },
            
            location: {
                let geo_mode = fp.geolocation_mode.as_deref().unwrap_or("disabled");
                let geo_prompt = fp.geolocation_prompt.as_deref().unwrap_or("ask");
                
                // type: 0=禁用, 2=启用
                let location_type = match geo_mode {
                    "auto" | "ip" | "custom" => 2,
                    _ => 0,
                };
                
                // permissions: allow=true, ask/block=false
                let permissions = geo_prompt == "allow";
                
                LocationConfig {
                    r#type: location_type,
                    permissions,
                    latitude: if geo_mode == "custom" || geo_mode == "ip" { fp.geolocation_latitude } else { None },
                    longitude: if geo_mode == "custom" || geo_mode == "ip" { fp.geolocation_longitude } else { None },
                    accuracy: if geo_mode == "custom" || geo_mode == "ip" { fp.geolocation_accuracy.or(Some(100.0)) } else { None },
                }
            },
            
            webrtc: if webrtc_type > 0 {
                Some(WebRTCConfig {
                    r#type: webrtc_type,
                    mode: webrtc_mode,
                    public_ip: fp.webrtc_public_ip.clone(),
                    local_ip: fp.webrtc_local_ip.clone(),
                })
            } else {
                None
            },
            
            fonts: if fonts_type > 0 {
                Some(FontsConfig {
                    r#type: fonts_type,
                    mode: fonts_mode,
                    list: fp.fonts_list.clone().unwrap_or_else(|| FontsConfig::default().list),
                })
            } else {
                None
            },
            
            variations: if fp.variations_enabled.unwrap_or(true) {
                let seed_id = fp.variations_seed_id.clone()
                    .unwrap_or_else(|| Self::generate_variations_seed_id(profile_id));
                Some(VariationsConfig {
                    r#type: 2,
                    enabled: true,
                    seed_id,
                    seed_type: "profile_based".to_string(),
                    field_trials: vec![],
                    enabled_features: vec![],
                    disabled_features: vec!["AutofillServerCommunication".to_string()],
                })
            } else {
                None
            },
        }
    }
    
    /// 基于 profile_id 生成稳定的 Variations Seed ID
    fn generate_variations_seed_id(profile_id: &str) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        profile_id.hash(&mut hasher);
        let hash = hasher.finish();
        
        format!("{:016x}", hash)
    }
    
    /// 解析屏幕分辨率字符串
    fn parse_screen_resolution(s: &str) -> (u32, u32) {
        let parts: Vec<&str> = s.split('x').collect();
        if parts.len() == 2 {
            let w = parts[0].parse().unwrap_or(1920);
            let h = parts[1].parse().unwrap_or(1080);
            (w, h)
        } else {
            (1920, 1080)
        }
    }
}
