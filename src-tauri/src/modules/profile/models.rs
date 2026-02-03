use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// 环境配置状态
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ProfileStatus {
    Stopped,
    Running,
}

/// 代理类型
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ProxyType {
    Http,
    Https,
    Socks5,
}

/// 代理配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProxyConfig {
    pub r#type: ProxyType,
    pub host: String,
    pub port: u16,
    pub username: Option<String>,
    pub password: Option<String>,
}

/// 指纹配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fingerprint {
    pub seed: i64,
    pub platform: String,
    pub browser: String,
    pub user_agent: String,
    pub hardware_concurrency: i32,
    pub device_memory: i32,
    pub screen_resolution: String,
    pub timezone: String,
    pub language: String,
    pub canvas_noise: bool,
    pub webgl_noise: bool,
    pub audio_noise: bool,

    // --- 新增字段，与前端 FingerprintConfig 对齐 ---
    pub navigator_platform: Option<String>,
    pub os_version: Option<String>,
    pub browser_version: Option<String>,
    pub screen_width: Option<u32>,
    pub screen_height: Option<u32>,
    pub resolution: Option<String>,
    pub fonts: Option<Vec<String>>,
    pub webrtc: Option<String>, // 'real' | 'fake' | 'disabled'
    pub webrtc_public_ip: Option<String>,
    pub webrtc_local_ip: Option<String>,
    pub webgl_image: Option<String>,
    pub webgl_vendor: Option<String>,
    pub webgl_renderer: Option<String>,
    pub webgpu: Option<bool>,
    pub canvas: Option<String>, // 'noise' | 'block' | 'off'
    pub audio_context: Option<String>, // 'noise' | 'block' | 'off'
    pub speech_voices: Option<Vec<String>>,
    pub do_not_track: Option<String>, // '1' | '0' | 'unspecified'
    pub client_rects: Option<bool>,
    pub media_devices: Option<String>, // 'real' | 'fake' | 'disabled'
    pub device_name: Option<String>,
    pub mac_address: Option<String>,
    pub ssl_fingerprint: Option<String>,
    pub port_scan_protection: Option<bool>,
    pub port_scan_whitelist: Option<String>,
    pub custom_fonts: Option<Vec<String>>,
    pub ignore_cert_errors: Option<bool>,
    pub custom_plugins: Option<bool>,
    pub cloudflare_optimize: Option<bool>,
    pub hardware_acceleration: Option<bool>,
    pub disable_sandbox: Option<bool>,
    pub launch_args: Option<String>,
    
    // --- 字体配置 ---
    pub fonts_mode: Option<String>,       // 'subset' | 'real' | 'custom' | 'random'
    pub fonts_list: Option<Vec<String>>,  // 选中的字体列表
    
    // --- Variations 配置 ---
    pub variations_enabled: Option<bool>,
    pub variations_seed_id: Option<String>,
    
    // --- 地理位置配置 ---
    pub geolocation_mode: Option<String>,      // 'auto' | 'custom' | 'disabled'
    pub geolocation_latitude: Option<f64>,
    pub geolocation_longitude: Option<f64>,
    pub geolocation_accuracy: Option<f64>,
    pub geolocation_prompt: Option<String>,    // 'ask' | 'allow' | 'block'
}

/// 偏好设置配置
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PreferencesConfig {
    // 扩展管理
    #[serde(default)]
    pub extensions: Vec<String>,
    #[serde(default)]
    pub custom_extensions: Vec<String>,
    
    // 启动设置
    pub window_name: Option<bool>,
    pub custom_bookmarks: Option<bool>,
    pub startup_page: Option<String>, // 'blank' | 'url'
    pub startup_url: Option<String>,

    // 退出自动清理
    #[serde(default)]
    pub clear_history_on_exit: bool,
    #[serde(default)]
    pub clear_cookies_on_exit: bool,
    #[serde(default)]
    pub clear_cache_on_exit: bool,
    
    // 启动前清理
    #[serde(default)]
    pub clear_cache_on_start: bool,
    #[serde(default)]
    pub clear_cookies_on_start: bool,
    #[serde(default)]
    pub clear_local_storage_on_start: bool,
    
    // 同步选项
    #[serde(default)]
    pub sync_bookmarks: bool,
    #[serde(default)]
    pub sync_history: bool,
    #[serde(default)]
    pub sync_tabs: bool,
    #[serde(default)]
    pub sync_cookies: bool,
    #[serde(default)]
    pub sync_extensions: bool,
    #[serde(default)]
    pub sync_passwords: bool,
    #[serde(default)]
    pub sync_indexed_db: bool,
    #[serde(default)]
    pub sync_local_storage: bool,
    #[serde(default)]
    pub sync_session_storage: bool,
    
    // 云端同步
    #[serde(default)]
    pub cloud_sync: bool,
    #[serde(default)]
    pub cloud_sync_extensions: bool,
    #[serde(default)]
    pub cloud_sync_bookmarks: bool,
    
    // 其他选项
    #[serde(default)]
    pub random_fingerprint_on_start: bool,
    #[serde(default)]
    pub show_password_save_prompt: bool,
    #[serde(default)]
    pub stop_on_network_error: bool,
    #[serde(default)]
    pub stop_on_ip_change: bool,
    #[serde(default)]
    pub stop_on_country_change: bool,
    #[serde(default)]
    pub open_workbench: bool,
    #[serde(default)]
    pub ip_change_notification: bool,
    #[serde(default)]
    pub enable_google_login: bool,
    
    // 网址访问控制
    pub url_blacklist: Option<String>,
    pub url_whitelist: Option<String>,
}

/// 环境配置（完整）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    pub id: String,
    pub name: String,
    pub group: String,
    pub remark: String,
    pub status: ProfileStatus,
    pub fingerprint: Fingerprint,
    pub proxy: Option<ProxyConfig>,
    pub preferences: Option<PreferencesConfig>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// 创建环境 DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateProfileDto {
    pub name: String,
    pub group: String,
    pub remark: String,
    pub fingerprint: Fingerprint,
    pub proxy: Option<ProxyConfig>,
    pub preferences: Option<PreferencesConfig>,
}

/// 更新环境 DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateProfileDto {
    pub name: Option<String>,
    pub group: Option<String>,
    pub remark: Option<String>,
    pub fingerprint: Option<Fingerprint>,
    pub proxy: Option<ProxyConfig>,
    pub preferences: Option<PreferencesConfig>,
}
