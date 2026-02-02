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
}

/// 偏好设置配置
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PreferencesConfig {
    // 扩展管理
    #[serde(default)]
    pub extensions: Vec<String>,
    
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
