// 配置文件写入器 - 生成 bm_fingerprint.json 和 bm_cloud.json
// 遵循方案A+云端WS完整开发规范

use serde::{Deserialize, Serialize};
use std::path::Path;
use std::fs;
use chrono::{DateTime, Utc};

// ============================================================================
// bm_fingerprint.json Schema V2
// ============================================================================

/// 种子配置
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SeedConfig {
    pub master: i64,
    pub canvas: i64,
    pub webgl: i64,
    pub audio: i64,
}

/// Navigator 配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NavigatorConfig {
    pub user_agent: String,
    pub platform: String,
    pub vendor: String,
    pub app_version: String,
    pub language: String,
    pub languages: Vec<String>,
    pub hardware_concurrency: u32,
    pub device_memory: u32,
    pub max_touch_points: u32,
    pub do_not_track: Option<String>,  // "1", "0", or null
    pub webdriver: bool,
    pub pdf_viewer_enabled: bool,
    pub cookie_enabled: bool,
}

impl Default for NavigatorConfig {
    fn default() -> Self {
        Self {
            user_agent: "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/139.0.0.0 Safari/537.36".to_string(),
            platform: "Win32".to_string(),
            vendor: "Google Inc.".to_string(),
            app_version: "5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36".to_string(),
            language: "zh-CN".to_string(),
            languages: vec!["zh-CN".to_string(), "en-US".to_string(), "en".to_string()],
            hardware_concurrency: 8,
            device_memory: 16,
            max_touch_points: 0,
            do_not_track: None,
            webdriver: false,
            pdf_viewer_enabled: true,
            cookie_enabled: true,
        }
    }
}

/// Screen 配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScreenConfig {
    pub width: u32,
    pub height: u32,
    pub avail_width: u32,
    pub avail_height: u32,
    pub color_depth: u32,
    pub pixel_depth: u32,
    pub device_pixel_ratio: f64,
    pub orientation_type: String,
    pub orientation_angle: u32,
}

impl Default for ScreenConfig {
    fn default() -> Self {
        Self {
            width: 1920,
            height: 1080,
            avail_width: 1920,
            avail_height: 1040,
            color_depth: 24,
            pixel_depth: 24,
            device_pixel_ratio: 1.0,
            orientation_type: "landscape-primary".to_string(),
            orientation_angle: 0,
        }
    }
}

/// WebGL 配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebGLConfig {
    pub vendor: String,
    pub renderer: String,
    pub unmasked_vendor: String,
    pub unmasked_renderer: String,
    pub version: String,
    pub shading_language_version: String,
    pub max_texture_size: u32,
    pub max_vertex_attribs: u32,
    pub noise_enabled: bool,
}

impl Default for WebGLConfig {
    fn default() -> Self {
        Self {
            vendor: "Google Inc. (NVIDIA)".to_string(),
            renderer: "ANGLE (NVIDIA, NVIDIA GeForce GTX 1660 Direct3D11 vs_5_0 ps_5_0, D3D11)".to_string(),
            unmasked_vendor: "NVIDIA Corporation".to_string(),
            unmasked_renderer: "NVIDIA GeForce GTX 1660/PCIe/SSE2".to_string(),
            version: "WebGL 1.0 (OpenGL ES 2.0 Chromium)".to_string(),
            shading_language_version: "WebGL GLSL ES 1.0 (OpenGL ES GLSL ES 1.0 Chromium)".to_string(),
            max_texture_size: 16384,
            max_vertex_attribs: 16,
            noise_enabled: true,
        }
    }
}

/// Canvas 配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanvasConfig {
    pub mode: String,
    pub noise_level: f64,
}

impl Default for CanvasConfig {
    fn default() -> Self {
        Self {
            mode: "noise".to_string(),
            noise_level: 0.0001,
        }
    }
}

/// Audio 配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioConfig {
    pub mode: String,
    pub sample_rate: u32,
    pub max_channel_count: u32,
}

impl Default for AudioConfig {
    fn default() -> Self {
        Self {
            mode: "noise".to_string(),
            sample_rate: 44100,
            max_channel_count: 2,
        }
    }
}

/// Timezone 配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimezoneConfig {
    pub id: String,
    pub offset_minutes: i32,
}

impl Default for TimezoneConfig {
    fn default() -> Self {
        Self {
            id: "Asia/Shanghai".to_string(),
            offset_minutes: -480,
        }
    }
}

/// Geolocation 配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeolocationConfig {
    pub mode: String,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub accuracy: Option<u32>,
}

impl Default for GeolocationConfig {
    fn default() -> Self {
        Self {
            mode: "disabled".to_string(),
            latitude: None,
            longitude: None,
            accuracy: None,
        }
    }
}

/// WebRTC 配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebRTCConfig {
    pub mode: String,
    pub public_ip: Option<String>,
    pub local_ip: Option<String>,
}

impl Default for WebRTCConfig {
    fn default() -> Self {
        Self {
            mode: "disabled".to_string(),
            public_ip: None,
            local_ip: None,
        }
    }
}

/// Fonts 配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FontsConfig {
    pub mode: String,
    pub list: Vec<String>,
}

impl Default for FontsConfig {
    fn default() -> Self {
        Self {
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

/// Client Hints Brand
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientHintsBrand {
    pub brand: String,
    pub version: String,
}

/// Client Hints 配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientHintsConfig {
    pub brands: Vec<ClientHintsBrand>,
    pub full_version: String,
    pub platform: String,
    pub platform_version: String,
    pub architecture: String,
    pub bitness: String,
    pub model: String,
    pub mobile: bool,
    pub wow64: bool,
}

impl Default for ClientHintsConfig {
    fn default() -> Self {
        Self {
            brands: vec![
                ClientHintsBrand { brand: "Not_A Brand".to_string(), version: "8".to_string() },
                ClientHintsBrand { brand: "Chromium".to_string(), version: "139".to_string() },
                ClientHintsBrand { brand: "Google Chrome".to_string(), version: "139".to_string() },
            ],
            full_version: "139.0.0.0".to_string(),
            platform: "Windows".to_string(),
            platform_version: "10.0.0".to_string(),
            architecture: "x86".to_string(),
            bitness: "64".to_string(),
            model: "".to_string(),
            mobile: false,
            wow64: false,
        }
    }
}

/// Battery 配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatteryConfig {
    pub charging: bool,
    pub charging_time: Option<f64>,
    pub discharging_time: Option<f64>,
    pub level: f64,
}

impl Default for BatteryConfig {
    fn default() -> Self {
        Self {
            charging: true,
            charging_time: None,
            discharging_time: None,
            level: 1.0,
        }
    }
}

/// Network 配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    pub effective_type: String,
    pub downlink: f64,
    pub rtt: u32,
    pub save_data: bool,
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            effective_type: "4g".to_string(),
            downlink: 10.0,
            rtt: 50,
            save_data: false,
        }
    }
}

/// Privacy 配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivacyConfig {
    pub client_rects_noise: bool,
    pub port_scan_protection: bool,
}

impl Default for PrivacyConfig {
    fn default() -> Self {
        Self {
            client_rects_noise: true,
            port_scan_protection: true,
        }
    }
}

/// Device 配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceConfig {
    pub name: String,
    pub mac_address: String,
}

impl Default for DeviceConfig {
    fn default() -> Self {
        Self {
            name: "DESKTOP-W0KJT6V0".to_string(),
            mac_address: "64-2B-7A-4D-96-E1".to_string(),
        }
    }
}

/// 完整指纹配置 (bm_fingerprint.json)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FingerprintFileConfig {
    #[serde(rename = "$schema")]
    pub schema: String,
    pub schema_version: u32,
    pub profile_id: String,
    pub created_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
    
    pub seed: SeedConfig,
    pub navigator: NavigatorConfig,
    pub screen: ScreenConfig,
    pub webgl: WebGLConfig,
    pub canvas: CanvasConfig,
    pub audio: AudioConfig,
    pub timezone: TimezoneConfig,
    pub geolocation: GeolocationConfig,
    pub webrtc: WebRTCConfig,
    pub fonts: FontsConfig,
    pub client_hints: ClientHintsConfig,
    pub battery: BatteryConfig,
    pub network: NetworkConfig,
    pub privacy: PrivacyConfig,
    pub device: DeviceConfig,
}

impl Default for FingerprintFileConfig {
    fn default() -> Self {
        Self {
            schema: "bm_fingerprint_v2".to_string(),
            schema_version: 2,
            profile_id: uuid::Uuid::new_v4().to_string().replace("-", ""),
            created_at: Utc::now().to_rfc3339(),
            signature: None,
            seed: SeedConfig::default(),
            navigator: NavigatorConfig::default(),
            screen: ScreenConfig::default(),
            webgl: WebGLConfig::default(),
            canvas: CanvasConfig::default(),
            audio: AudioConfig::default(),
            timezone: TimezoneConfig::default(),
            geolocation: GeolocationConfig::default(),
            webrtc: WebRTCConfig::default(),
            fonts: FontsConfig::default(),
            client_hints: ClientHintsConfig::default(),
            battery: BatteryConfig::default(),
            network: NetworkConfig::default(),
            privacy: PrivacyConfig::default(),
            device: DeviceConfig::default(),
        }
    }
}

// ============================================================================
// bm_cloud.json Schema
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
    pub mode: String,  // "cloud" | "local" | "both" | "none"
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
        // 1. 确保目录存在
        fs::create_dir_all(user_data_dir)
            .map_err(|e| format!("创建目录失败: {}", e))?;
        
        // 2. 序列化为 JSON
        let json = serde_json::to_string_pretty(config)
            .map_err(|e| format!("序列化失败: {}", e))?;
        
        // 3. 写入文件
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
        // 1. 确保目录存在
        fs::create_dir_all(user_data_dir)
            .map_err(|e| format!("创建目录失败: {}", e))?;
        
        // 2. 序列化为 JSON
        let json = serde_json::to_string_pretty(config)
            .map_err(|e| format!("序列化失败: {}", e))?;
        
        // 3. 写入文件
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
        // 1. 生成指纹配置
        let fp_config = Self::build_fingerprint_config(profile_id, fingerprint);
        Self::write_fingerprint_config(user_data_dir, &fp_config)?;
        
        // 2. 生成云端配置
        let cloud_config = CloudFileConfig::new(profile_id, profile_name, group);
        Self::write_cloud_config(user_data_dir, &cloud_config)?;
        
        Ok(())
    }
    
    /// 从 Profile Fingerprint 构建完整的配置文件
    fn build_fingerprint_config(
        profile_id: &str,
        fp: &crate::modules::profile::Fingerprint,
    ) -> FingerprintFileConfig {
        // 解析屏幕分辨率
        let (width, height) = Self::parse_screen_resolution(&fp.screen_resolution);
        
        FingerprintFileConfig {
            schema: "bm_fingerprint_v2".to_string(),
            schema_version: 2,
            profile_id: profile_id.to_string(),
            created_at: Utc::now().to_rfc3339(),
            signature: None,
            
            seed: SeedConfig {
                master: fp.seed,
                canvas: fp.seed,
                webgl: fp.seed,
                audio: fp.seed,
            },
            
            navigator: NavigatorConfig {
                user_agent: fp.user_agent.clone(),
                platform: Self::platform_to_string(&fp.platform),
                hardware_concurrency: fp.hardware_concurrency as u32,
                device_memory: fp.device_memory as u32,
                language: fp.language.clone(),
                languages: vec![fp.language.clone(), "en-US".to_string()],
                ..Default::default()
            },
            
            screen: ScreenConfig {
                width,
                height,
                avail_width: width,
                avail_height: height.saturating_sub(40),
                ..Default::default()
            },
            
            timezone: TimezoneConfig {
                id: fp.timezone.clone(),
                offset_minutes: Self::timezone_to_offset(&fp.timezone),
            },
            
            canvas: CanvasConfig {
                mode: if fp.canvas_noise { "noise".to_string() } else { "off".to_string() },
                noise_level: 0.0001,
            },
            
            webgl: WebGLConfig {
                noise_enabled: fp.webgl_noise,
                ..Default::default()
            },
            
            audio: AudioConfig {
                mode: if fp.audio_noise { "noise".to_string() } else { "off".to_string() },
                ..Default::default()
            },
            
            ..Default::default()
        }
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
    
    /// 平台字符串转换
    fn platform_to_string(platform: &str) -> String {
        match platform.to_lowercase().as_str() {
            "windows" => "Win32".to_string(),
            "macos" | "mac" => "MacIntel".to_string(),
            "linux" => "Linux x86_64".to_string(),
            _ => "Win32".to_string(),
        }
    }
    
    /// 时区转偏移量（分钟）
    fn timezone_to_offset(timezone: &str) -> i32 {
        match timezone {
            "Asia/Shanghai" | "Asia/Hong_Kong" | "Asia/Taipei" => -480,
            "Asia/Tokyo" => -540,
            "America/New_York" => 300,
            "America/Los_Angeles" => 480,
            "Europe/London" => 0,
            "Europe/Paris" | "Europe/Berlin" => -60,
            _ => -480, // 默认东八区
        }
    }
}
