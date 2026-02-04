// 配置文件写入器 - 生成 bm_fingerprint.json 和 bm_cloud.json
// 格式与 Chromium 内核 fingerprint_browser 模块兼容

use serde::{Deserialize, Serialize};
use std::path::Path;
use std::fs;
use chrono::{DateTime, Utc};

// ============================================================================
// bm_fingerprint.json Schema - 匹配内核实际期望的格式
// ============================================================================

/// UA 配置 - 匹配内核 ua 字段
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KernelUaConfig {
    #[serde(rename = "type")]
    pub config_type: i32,
    pub user_agent: String,
}

impl Default for KernelUaConfig {
    fn default() -> Self {
        Self {
            config_type: 2,
            user_agent: "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/139.0.0.0 Safari/537.36".to_string(),
        }
    }
}

/// ResourceInfo 配置 - 匹配内核 resourceInfo 字段
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KernelResourceInfoConfig {
    #[serde(rename = "type")]
    pub config_type: i32,
    pub cpu: u32,
    pub memory: f32,
}

impl Default for KernelResourceInfoConfig {
    fn default() -> Self {
        Self {
            config_type: 2,
            cpu: 8,
            memory: 8.0,
        }
    }
}

/// Resolution 配置 - 匹配内核 resolution 字段
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KernelResolutionConfig {
    #[serde(rename = "type")]
    pub config_type: i32,
    pub monitor_width: u32,
    pub monitor_height: u32,
    pub color_depth: u32,
    pub avail_width: u32,
    pub avail_height: u32,
}

impl Default for KernelResolutionConfig {
    fn default() -> Self {
        Self {
            config_type: 2,
            monitor_width: 1920,
            monitor_height: 1080,
            color_depth: 24,
            avail_width: 1920,
            avail_height: 1040,
        }
    }
}

/// TimeZone 配置 - 匹配内核 timeZone 字段
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KernelTimeZoneConfig {
    #[serde(rename = "type")]
    pub config_type: i32,
    pub gmt: String,
}

impl Default for KernelTimeZoneConfig {
    fn default() -> Self {
        Self {
            config_type: 2,
            gmt: "Asia/Shanghai".to_string(),
        }
    }
}

/// Language 配置 - 匹配内核 language 字段
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KernelLanguageConfig {
    #[serde(rename = "type")]
    pub config_type: i32,
    pub interface_language: String,
    pub languages: Vec<String>,
}

impl Default for KernelLanguageConfig {
    fn default() -> Self {
        Self {
            config_type: 2,
            interface_language: "en-US".to_string(),
            languages: vec!["en-US".to_string(), "en".to_string()],
        }
    }
}

/// WebGLDevice 配置 - 匹配内核 webGLDevice 字段
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KernelWebGLDeviceConfig {
    #[serde(rename = "type")]
    pub config_type: i32,
    pub vendors: String,
    pub renderer: String,
}

impl Default for KernelWebGLDeviceConfig {
    fn default() -> Self {
        Self {
            config_type: 2,
            vendors: "Google Inc. (Intel)".to_string(),
            renderer: "ANGLE (Intel, Intel(R) UHD Graphics 630 Direct3D11 vs_5_0 ps_5_0, D3D11)".to_string(),
        }
    }
}

/// Canvas 配置 - 匹配内核 canvas 字段
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KernelCanvasConfig {
    #[serde(rename = "type")]
    pub config_type: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub noise_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub noise_factor: Option<f64>,
}

impl Default for KernelCanvasConfig {
    fn default() -> Self {
        Self {
            config_type: 2,
            noise_enabled: Some(true),
            noise_factor: Some(0.001),
        }
    }
}

/// AudioContext 配置 - 匹配内核 audioContext 字段
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KernelAudioContextConfig {
    #[serde(rename = "type")]
    pub config_type: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub noise_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub noise: Option<Vec<f64>>,
}

impl Default for KernelAudioContextConfig {
    fn default() -> Self {
        Self {
            config_type: 2,
            noise_enabled: Some(true),
            noise: None,
        }
    }
}

/// Font 配置 - 匹配内核 font 字段
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KernelFontConfig {
    #[serde(rename = "type")]
    pub config_type: i32,
    pub fonts: Vec<String>,
}

impl Default for KernelFontConfig {
    fn default() -> Self {
        Self {
            config_type: 2,
            fonts: vec![
                "Arial".to_string(),
                "Calibri".to_string(),
                "Cambria".to_string(),
                "Consolas".to_string(),
                "Microsoft YaHei".to_string(),
                "SimSun".to_string(),
                "Times New Roman".to_string(),
                "Verdana".to_string(),
            ],
        }
    }
}

/// ClientRects 配置 - 匹配内核 clientRects 字段
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KernelClientRectsConfig {
    #[serde(rename = "type")]
    pub config_type: i32,
}

impl Default for KernelClientRectsConfig {
    fn default() -> Self {
        Self {
            config_type: 2,
        }
    }
}

/// MediaDevices 配置 - 匹配内核 mediaDevices 字段
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KernelMediaDevicesConfig {
    #[serde(rename = "type")]
    pub config_type: i32,
    pub video_input: u32,
    pub audio_input: u32,
    pub audio_output: u32,
}

impl Default for KernelMediaDevicesConfig {
    fn default() -> Self {
        Self {
            config_type: 2,
            video_input: 1,
            audio_input: 1,
            audio_output: 2,
        }
    }
}

/// 完整指纹配置 (bm_fingerprint.json) - 匹配内核期望的格式
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FingerprintFileConfig {
    pub init: i32,
    pub ua: KernelUaConfig,
    pub resource_info: KernelResourceInfoConfig,
    pub resolution: KernelResolutionConfig,
    #[serde(rename = "timeZone")]
    pub time_zone: KernelTimeZoneConfig,
    pub language: KernelLanguageConfig,
    #[serde(rename = "webGLDevice")]
    pub webgl_device: KernelWebGLDeviceConfig,
    pub canvas: KernelCanvasConfig,
    #[serde(rename = "audioContext")]
    pub audio_context: KernelAudioContextConfig,
    pub font: KernelFontConfig,
    #[serde(rename = "clientRects")]
    pub client_rects: KernelClientRectsConfig,
    #[serde(rename = "mediaDevices")]
    pub media_devices: KernelMediaDevicesConfig,
}

impl Default for FingerprintFileConfig {
    fn default() -> Self {
        Self {
            init: 2,
            ua: KernelUaConfig::default(),
            resource_info: KernelResourceInfoConfig::default(),
            resolution: KernelResolutionConfig::default(),
            time_zone: KernelTimeZoneConfig::default(),
            language: KernelLanguageConfig::default(),
            webgl_device: KernelWebGLDeviceConfig::default(),
            canvas: KernelCanvasConfig::default(),
            audio_context: KernelAudioContextConfig::default(),
            font: KernelFontConfig::default(),
            client_rects: KernelClientRectsConfig::default(),
            media_devices: KernelMediaDevicesConfig::default(),
        }
    }
}

// ============== 以下保留原来的结构用于内部处理（但不输出到配置文件）==============

/// 种子配置 (启动器内部使用，不写入配置文件)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SeedConfig {
    pub master: i64,
    pub canvas: i64,
    pub webgl: i64,
    pub audio: i64,
}

/// Navigator 配置 - camelCase 格式与内核匹配
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NavigatorConfig {
    pub user_agent: String,
    pub platform: String,
    pub vendor: String,
    pub app_version: String,
    pub hardware_concurrency: u32,
    pub device_memory: u32,
    pub languages: Vec<String>,
    pub do_not_track: bool,
}

impl Default for NavigatorConfig {
    fn default() -> Self {
        Self {
            user_agent: "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/139.0.0.0 Safari/537.36".to_string(),
            platform: "Win32".to_string(),
            vendor: "Google Inc.".to_string(),
            app_version: "5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/139.0.0.0 Safari/537.36".to_string(),
            hardware_concurrency: 8,
            device_memory: 8,
            languages: vec!["zh-CN".to_string(), "zh".to_string(), "en-US".to_string(), "en".to_string()],
            do_not_track: false,
        }
    }
}

/// Screen 配置 - camelCase 格式
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScreenConfig {
    pub width: u32,
    pub height: u32,
    pub avail_width: u32,
    pub avail_height: u32,
    pub color_depth: u32,
    pub pixel_depth: u32,
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
        }
    }
}

/// WebGL 配置 - 匹配内核格式
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebGLConfig {
    pub vendor: String,
    pub renderer: String,
    pub unmasked_vendor_enabled: bool,
    pub unmasked_renderer_enabled: bool,
}

impl Default for WebGLConfig {
    fn default() -> Self {
        Self {
            vendor: "Google Inc. (Intel)".to_string(),
            renderer: "ANGLE (Intel, Intel(R) UHD Graphics 630 Direct3D11 vs_5_0 ps_5_0, D3D11)".to_string(),
            unmasked_vendor_enabled: true,
            unmasked_renderer_enabled: true,
        }
    }
}

/// Canvas 配置 - 匹配内核格式
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CanvasConfig {
    pub noise_enabled: bool,
    pub noise_factor: f64,
}

impl Default for CanvasConfig {
    fn default() -> Self {
        Self {
            noise_enabled: true,
            noise_factor: 0.0001,
        }
    }
}

/// Audio 配置 - 匹配内核格式
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AudioConfig {
    pub noise_enabled: bool,
    pub noise_factor: f64,
}

impl Default for AudioConfig {
    fn default() -> Self {
        Self {
            noise_enabled: true,
            noise_factor: 0.0001,
        }
    }
}

/// Timezone 配置 - 匹配内核格式
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimezoneConfig {
    pub timezone: String,
    pub timezone_offset: i32,
}

impl Default for TimezoneConfig {
    fn default() -> Self {
        Self {
            timezone: "Asia/Shanghai".to_string(),
            timezone_offset: -480,
        }
    }
}

/// Geolocation 配置 - 匹配内核格式
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeolocationConfig {
    pub latitude: f64,
    pub longitude: f64,
    pub accuracy: f64,
    pub enabled: bool,
}

impl Default for GeolocationConfig {
    fn default() -> Self {
        Self {
            latitude: 31.230416,
            longitude: 121.473701,
            accuracy: 100.0,
            enabled: false,
        }
    }
}

/// MediaDevices 配置 - 匹配内核格式
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaDevicesConfig {
    pub video_inputs: u32,
    pub audio_inputs: u32,
    pub audio_outputs: u32,
    pub enumerate_devices_enabled: bool,
}

impl Default for MediaDevicesConfig {
    fn default() -> Self {
        Self {
            video_inputs: 1,
            audio_inputs: 1,
            audio_outputs: 2,
            enumerate_devices_enabled: true,
        }
    }
}

/// Font 配置 - 匹配内核格式 (font 而不是 fonts)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FontConfig {
    pub available_fonts: Vec<String>,
    pub randomize: bool,
}

impl Default for FontConfig {
    fn default() -> Self {
        Self {
            available_fonts: vec![
                "Arial".to_string(),
                "Calibri".to_string(),
                "Cambria".to_string(),
                "Consolas".to_string(),
                "Microsoft YaHei".to_string(),
                "SimSun".to_string(),
                "Times New Roman".to_string(),
                "Verdana".to_string(),
            ],
            randomize: false,
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

// 旧的 FingerprintFileConfig 已删除，使用新的内核兼容格式（第240行）

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
    
    /// 从 Profile Fingerprint 构建完整的配置文件 - 匹配内核格式
    fn build_fingerprint_config(
        _profile_id: &str,
        fp: &crate::modules::profile::Fingerprint,
    ) -> FingerprintFileConfig {
        // 解析屏幕分辨率
        let (width, height) = Self::parse_screen_resolution(&fp.screen_resolution);
        
        FingerprintFileConfig {
            init: 2,
            
            ua: KernelUaConfig {
                config_type: 2,
                user_agent: fp.user_agent.clone(),
            },
            
            resource_info: KernelResourceInfoConfig {
                config_type: 2,
                cpu: fp.hardware_concurrency as u32,
                memory: fp.device_memory as f32,
            },
            
            resolution: KernelResolutionConfig {
                config_type: 2,
                monitor_width: width,
                monitor_height: height,
                color_depth: 24,
                avail_width: width,
                avail_height: height.saturating_sub(40),
            },
            
            time_zone: KernelTimeZoneConfig {
                config_type: 2,
                gmt: fp.timezone.clone(),
            },
            
            language: KernelLanguageConfig {
                config_type: 2,
                interface_language: fp.language.clone(),
                languages: vec![fp.language.clone(), "en".to_string()],
            },
            
            webgl_device: KernelWebGLDeviceConfig::default(),
            
            canvas: KernelCanvasConfig {
                config_type: 2,
                noise_enabled: Some(fp.canvas_noise),
                noise_factor: Some(0.001),
            },
            
            audio_context: KernelAudioContextConfig {
                config_type: 2,
                noise_enabled: Some(fp.audio_noise),
                noise: None,
            },
            
            font: KernelFontConfig::default(),
            client_rects: KernelClientRectsConfig::default(),
            media_devices: KernelMediaDevicesConfig::default(),
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
