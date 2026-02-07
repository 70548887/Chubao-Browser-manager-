//! 用户认证模块
//!
//! 负责用户登录、注册、Token 管理
//! 所有云服务器请求通过 Rust 后端发起，前端仅通过 IPC 调用

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use tracing::{info, warn, error};
use super::config;

// ==================== 数据结构 ====================

/// 登录请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginRequest {
    pub account: String,
    pub password: String,
    pub remember: bool,
}

/// 注册请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
    pub confirm_password: String,
    pub invite_code: Option<String>,
}

/// 用户信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    pub id: String,
    pub username: String,
    pub email: String,
    pub avatar: Option<String>,
    pub role: String,
    pub created_at: String,
}

/// 登录响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginResponse {
    pub user: UserInfo,
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: i64,
}

/// API 通用响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub code: i32,
    pub message: String,
    pub data: Option<T>,
}

// ==================== Token 安全存储 ====================

/// Token 存储服务名称
const TOKEN_SERVICE_NAME: &str = "com.chubao.browser-manager";
const ACCESS_TOKEN_KEY: &str = "access_token";
const REFRESH_TOKEN_KEY: &str = "refresh_token";

/// 保存 Token 到系统密钥链
pub fn save_tokens(access_token: &str, refresh_token: &str) -> Result<()> {
    let entry_access = keyring::Entry::new(TOKEN_SERVICE_NAME, ACCESS_TOKEN_KEY)?;
    entry_access.set_password(access_token)?;
    
    let entry_refresh = keyring::Entry::new(TOKEN_SERVICE_NAME, REFRESH_TOKEN_KEY)?;
    entry_refresh.set_password(refresh_token)?;
    
    info!("Tokens saved to system keychain");
    Ok(())
}

/// 从系统密钥链获取 Access Token
pub fn get_access_token() -> Result<String> {
    let entry = keyring::Entry::new(TOKEN_SERVICE_NAME, ACCESS_TOKEN_KEY)?;
    entry.get_password().context("Access token not found")
}

/// 从系统密钥链获取 Refresh Token
pub fn get_refresh_token() -> Result<String> {
    let entry = keyring::Entry::new(TOKEN_SERVICE_NAME, REFRESH_TOKEN_KEY)?;
    entry.get_password().context("Refresh token not found")
}

/// 清除存储的 Tokens（登出时使用）
pub fn clear_tokens() -> Result<()> {
    if let Ok(entry) = keyring::Entry::new(TOKEN_SERVICE_NAME, ACCESS_TOKEN_KEY) {
        let _ = entry.delete_credential();
    }
    if let Ok(entry) = keyring::Entry::new(TOKEN_SERVICE_NAME, REFRESH_TOKEN_KEY) {
        let _ = entry.delete_credential();
    }
    info!("Tokens cleared from system keychain");
    Ok(())
}

/// 检查是否已登录
pub fn is_logged_in() -> bool {
    get_access_token().is_ok()
}

// ==================== API 请求 ====================

/// 用户登录
pub async fn login(server_url: &str, request: LoginRequest) -> Result<LoginResponse> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(config::REQUEST_TIMEOUT_SECS))
        .build()?;

    let url = format!("{}/api/v1/auth/login", server_url);
    info!("User login request: {}", url);

    let resp: ApiResponse<LoginResponse> = client
        .post(&url)
        .json(&request)
        .send()
        .await
        .context("登录请求失败")?
        .json()
        .await
        .context("解析登录响应失败")?;

    if resp.code != 0 {
        return Err(anyhow::anyhow!("{}", resp.message));
    }

    let login_data = resp.data.context("登录响应数据为空")?;

    // 保存 Token 到系统密钥链
    if request.remember {
        save_tokens(&login_data.access_token, &login_data.refresh_token)?;
    }

    Ok(login_data)
}

/// 用户注册
pub async fn register(server_url: &str, request: RegisterRequest) -> Result<UserInfo> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(config::REQUEST_TIMEOUT_SECS))
        .build()?;

    let url = format!("{}/api/v1/auth/register", server_url);
    info!("User register request: {}", url);

    let resp: ApiResponse<UserInfo> = client
        .post(&url)
        .json(&request)
        .send()
        .await
        .context("注册请求失败")?
        .json()
        .await
        .context("解析注册响应失败")?;

    if resp.code != 0 {
        return Err(anyhow::anyhow!("{}", resp.message));
    }

    resp.data.context("注册响应数据为空")
}

/// 用户登出
pub async fn logout(server_url: &str) -> Result<()> {
    // 尝试通知服务器登出
    if let Ok(token) = get_access_token() {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(config::REQUEST_TIMEOUT_SECS))
            .build()?;

        let url = format!("{}/api/v1/auth/logout", server_url);
        let _ = client
            .post(&url)
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await;
    }

    // 无论服务器响应如何，都清除本地 Token
    clear_tokens()?;
    info!("User logged out");
    Ok(())
}

/// 刷新 Token
pub async fn refresh_token(server_url: &str) -> Result<LoginResponse> {
    let refresh_token = get_refresh_token()?;

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(config::REQUEST_TIMEOUT_SECS))
        .build()?;

    let url = format!("{}/api/v1/auth/refresh", server_url);
    info!("Refreshing token: {}", url);

    #[derive(Serialize)]
    struct RefreshRequest {
        refresh_token: String,
    }

    let resp: ApiResponse<LoginResponse> = client
        .post(&url)
        .json(&RefreshRequest { refresh_token })
        .send()
        .await
        .context("刷新 Token 请求失败")?
        .json()
        .await
        .context("解析刷新响应失败")?;

    if resp.code != 0 {
        // Token 刷新失败，清除本地存储
        clear_tokens()?;
        return Err(anyhow::anyhow!("{}", resp.message));
    }

    let login_data = resp.data.context("刷新响应数据为空")?;

    // 更新存储的 Token
    save_tokens(&login_data.access_token, &login_data.refresh_token)?;

    Ok(login_data)
}

/// 获取当前用户信息
pub async fn get_user_info(server_url: &str) -> Result<UserInfo> {
    let token = get_access_token()?;

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(config::REQUEST_TIMEOUT_SECS))
        .build()?;

    let url = format!("{}/api/v1/auth/user", server_url);

    let resp: ApiResponse<UserInfo> = client
        .get(&url)
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .context("获取用户信息失败")?
        .json()
        .await
        .context("解析用户信息失败")?;

    if resp.code != 0 {
        return Err(anyhow::anyhow!("{}", resp.message));
    }

    resp.data.context("用户信息为空")
}

// ==================== 许可证验证 ====================

/// 许可证信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicenseInfo {
    pub license_key: String,
    pub status: String,  // "active", "expired", "revoked"
    pub plan: String,    // "free", "pro", "enterprise"
    pub expires_at: Option<String>,
    pub max_profiles: i32,
    pub features: Vec<String>,
}

/// 验证许可证
pub async fn validate_license(server_url: &str) -> Result<LicenseInfo> {
    let token = get_access_token()?;

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(config::REQUEST_TIMEOUT_SECS))
        .build()?;

    let url = format!("{}/api/v1/license/validate", server_url);

    let resp: ApiResponse<LicenseInfo> = client
        .get(&url)
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .context("验证许可证失败")?
        .json()
        .await
        .context("解析许可证信息失败")?;

    if resp.code != 0 {
        return Err(anyhow::anyhow!("{}", resp.message));
    }

    resp.data.context("许可证信息为空")
}

/// 激活许可证
pub async fn activate_license(server_url: &str, license_key: &str) -> Result<LicenseInfo> {
    let token = get_access_token()?;

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(config::REQUEST_TIMEOUT_SECS))
        .build()?;

    let url = format!("{}/api/v1/license/activate", server_url);

    #[derive(Serialize)]
    struct ActivateRequest {
        license_key: String,
    }

    let resp: ApiResponse<LicenseInfo> = client
        .post(&url)
        .header("Authorization", format!("Bearer {}", token))
        .json(&ActivateRequest {
            license_key: license_key.to_string(),
        })
        .send()
        .await
        .context("激活许可证失败")?
        .json()
        .await
        .context("解析激活响应失败")?;

    if resp.code != 0 {
        return Err(anyhow::anyhow!("{}", resp.message));
    }

    resp.data.context("激活响应数据为空")
}
