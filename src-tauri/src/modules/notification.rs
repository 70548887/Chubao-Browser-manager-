//! 消息通知模块
//!
//! 负责获取消息列表、未读数量、标记已读等
//! 所有云服务器请求通过 Rust 后端发起

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use tracing::info;
use super::config;
use super::auth;

// ==================== 数据结构 ====================

/// 通知类型
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum NotificationType {
    System,
    Update,
    Warning,
    Promo,
}

/// 通知消息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notification {
    pub id: String,
    pub title: String,
    pub content: String,
    pub notification_type: String,
    pub is_read: bool,
    pub created_at: String,
    pub link: Option<String>,
}

/// 通知列表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationListResponse {
    pub notifications: Vec<Notification>,
    pub total: i32,
    pub unread_count: i32,
}

/// API 通用响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub code: i32,
    pub message: String,
    pub data: Option<T>,
}

// ==================== API 请求 ====================

/// 获取通知列表
pub async fn get_notifications(
    server_url: &str,
    page: i32,
    page_size: i32,
) -> Result<NotificationListResponse> {
    let token = auth::get_access_token()?;

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(config::REQUEST_TIMEOUT_SECS))
        .build()?;

    let url = format!(
        "{}/api/v1/notifications?page={}&page_size={}",
        server_url, page, page_size
    );

    info!("Fetching notifications: {}", url);

    let resp: ApiResponse<NotificationListResponse> = client
        .get(&url)
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .context("获取通知列表失败")?
        .json()
        .await
        .context("解析通知响应失败")?;

    if resp.code != 0 {
        return Err(anyhow::anyhow!("{}", resp.message));
    }

    resp.data.context("通知数据为空")
}

/// 获取未读通知数量
pub async fn get_unread_count(server_url: &str) -> Result<i32> {
    let token = auth::get_access_token()?;

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(config::REQUEST_TIMEOUT_SECS))
        .build()?;

    let url = format!("{}/api/v1/notifications/unread-count", server_url);

    #[derive(Deserialize)]
    struct UnreadCountResponse {
        count: i32,
    }

    let resp: ApiResponse<UnreadCountResponse> = client
        .get(&url)
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .context("获取未读数量失败")?
        .json()
        .await
        .context("解析响应失败")?;

    if resp.code != 0 {
        return Err(anyhow::anyhow!("{}", resp.message));
    }

    Ok(resp.data.map(|d| d.count).unwrap_or(0))
}

/// 标记通知为已读
pub async fn mark_as_read(server_url: &str, notification_ids: Vec<String>) -> Result<()> {
    let token = auth::get_access_token()?;

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(config::REQUEST_TIMEOUT_SECS))
        .build()?;

    let url = format!("{}/api/v1/notifications/read", server_url);

    #[derive(Serialize)]
    struct MarkReadRequest {
        ids: Vec<String>,
    }

    let resp: ApiResponse<()> = client
        .post(&url)
        .header("Authorization", format!("Bearer {}", token))
        .json(&MarkReadRequest { ids: notification_ids })
        .send()
        .await
        .context("标记已读失败")?
        .json()
        .await
        .context("解析响应失败")?;

    if resp.code != 0 {
        return Err(anyhow::anyhow!("{}", resp.message));
    }

    Ok(())
}

/// 标记所有通知为已读
pub async fn mark_all_as_read(server_url: &str) -> Result<()> {
    let token = auth::get_access_token()?;

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(config::REQUEST_TIMEOUT_SECS))
        .build()?;

    let url = format!("{}/api/v1/notifications/read-all", server_url);

    let resp: ApiResponse<()> = client
        .post(&url)
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .context("标记全部已读失败")?
        .json()
        .await
        .context("解析响应失败")?;

    if resp.code != 0 {
        return Err(anyhow::anyhow!("{}", resp.message));
    }

    Ok(())
}

/// 删除通知
pub async fn delete_notification(server_url: &str, notification_id: &str) -> Result<()> {
    let token = auth::get_access_token()?;

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(config::REQUEST_TIMEOUT_SECS))
        .build()?;

    let url = format!("{}/api/v1/notifications/{}", server_url, notification_id);

    let resp: ApiResponse<()> = client
        .delete(&url)
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .context("删除通知失败")?
        .json()
        .await
        .context("解析响应失败")?;

    if resp.code != 0 {
        return Err(anyhow::anyhow!("{}", resp.message));
    }

    Ok(())
}
