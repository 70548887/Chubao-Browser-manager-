/**
 * 统一配置管理
 * 
 * 根据编译模式自动切换开发/生产环境配置
 * 避免硬编码，统一管理所有后端 API 地址
 */

// ==================== API 配置 ====================

/// API 基础地址（根据编译模式自动切换）
#[cfg(debug_assertions)]
pub const API_BASE_URL: &str = "http://localhost:8081";  // 开发环境

#[cfg(not(debug_assertions))]
pub const API_BASE_URL: &str = "http://96.126.191.43";  // 生产环境

// ==================== API 端点 ====================

/// 获取完整 API URL
pub fn get_api_url(endpoint: &str) -> String {
    format!("{}{}", API_BASE_URL, endpoint)
}

/// 启动器更新检查端点
pub fn get_launcher_update_url(version: &str, platform: &str, arch: &str) -> String {
    format!(
        "{}/api/v1/updates/launcher?version={}&platform={}&arch={}",
        API_BASE_URL, version, platform, arch
    )
}

/// 内核更新检查端点
pub fn get_kernel_update_url(version: &str, platform: &str, arch: &str, launcher_version: &str) -> String {
    format!(
        "{}/api/v1/updates/kernel?version={}&platform={}&arch={}&launcher_version={}",
        API_BASE_URL, version, platform, arch, launcher_version
    )
}

/// 内核下载信息端点
pub fn get_kernel_download_info_url(platform: &str, arch: &str, launcher_version: &str) -> String {
    format!(
        "{}/api/v1/kernel/download-info?platform={}&arch={}&launcher_version={}",
        API_BASE_URL, platform, arch, launcher_version
    )
}

// ==================== 其他配置 ====================

/// 请求超时时间（秒）
pub const REQUEST_TIMEOUT_SECS: u64 = 10;

/// WebSocket 心跳间隔（秒）
pub const WS_HEARTBEAT_INTERVAL_SECS: u64 = 30;

/// 下载重试次数
pub const DOWNLOAD_RETRY_COUNT: usize = 3;

// ==================== 辅助函数 ====================

/// 获取当前环境名称
pub fn get_environment() -> &'static str {
    if cfg!(debug_assertions) {
        "development"
    } else {
        "production"
    }
}

/// 打印配置信息（用于调试）
pub fn print_config() {
    println!("=== 应用配置 ===");
    println!("环境: {}", get_environment());
    println!("API 基础地址: {}", API_BASE_URL);
    println!("请求超时: {}秒", REQUEST_TIMEOUT_SECS);
    println!("================");
}

// ==================== 第三方服务地址 ====================

/// IP 地理位置查询 API（第三方服务）
pub const IP_GEO_API_URL: &str = "http://ip-api.com/json";
