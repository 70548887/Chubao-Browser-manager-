// Proxy Module - 代理管理（基于数据库表）
// V5 升级：从 settings JSON 改为独立 proxies 表

use sqlx::{SqlitePool, Row};
use serde::{Deserialize, Serialize};
use chrono::Utc;
use anyhow::Result;

/// 代理状态
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ProxyStatus {
    Active,
    Expired,
    Error,
}

impl Default for ProxyStatus {
    fn default() -> Self {
        ProxyStatus::Active
    }
}

impl std::fmt::Display for ProxyStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProxyStatus::Active => write!(f, "active"),
            ProxyStatus::Expired => write!(f, "expired"),
            ProxyStatus::Error => write!(f, "error"),
        }
    }
}

/// 代理实体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Proxy {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub proxy_type: String, // "http" | "https" | "socks5" | "direct"
    pub source: String,     // "custom" | "imported"
    pub tag: String,
    pub host: String,
    pub port: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    pub used_count: i32,
    pub auto_check: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bind_window: Option<String>,
    pub remark: String,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
}

/// 创建代理 DTO
#[derive(Debug, Deserialize)]
pub struct CreateProxyDto {
    pub name: String,
    #[serde(rename = "type")]
    pub proxy_type: String,
    pub host: String,
    pub port: String,
    #[serde(default)]
    pub source: Option<String>,
    #[serde(default)]
    pub tag: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub remark: Option<String>,
    pub auto_check: Option<bool>,
    pub expire_at: Option<String>,
}

/// 更新代理 DTO
#[derive(Debug, Deserialize)]
pub struct UpdateProxyDto {
    pub name: Option<String>,
    #[serde(rename = "type")]
    pub proxy_type: Option<String>,
    pub host: Option<String>,
    pub port: Option<String>,
    pub source: Option<String>,
    pub tag: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub ip_address: Option<String>,
    pub location: Option<String>,
    pub remark: Option<String>,
    pub auto_check: Option<bool>,
    pub expire_at: Option<String>,
    pub bind_window: Option<String>,
    pub status: Option<String>,
}

/// 代理服务
pub struct ProxyService {
    pool: SqlitePool,
}

impl ProxyService {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    /// 获取所有代理
    pub async fn list_proxies(&self) -> Result<Vec<Proxy>> {
        let rows = sqlx::query(
            r#"
            SELECT id, name, type, source, tag, host, port, username, password,
                   ip_address, location, used_count, auto_check, expire_at,
                   bind_window, remark, status, created_at, updated_at
            FROM proxies
            ORDER BY updated_at DESC
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        let mut proxies = Vec::new();
        for row in rows {
            proxies.push(self.row_to_proxy(&row)?);
        }

        Ok(proxies)
    }

    /// 获取单个代理
    pub async fn get_proxy(&self, id: &str) -> Result<Proxy> {
        let row = sqlx::query(
            r#"
            SELECT id, name, type, source, tag, host, port, username, password,
                   ip_address, location, used_count, auto_check, expire_at,
                   bind_window, remark, status, created_at, updated_at
            FROM proxies
            WHERE id = ?
            "#
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?
        .ok_or_else(|| anyhow::anyhow!("代理不存在"))?;

        self.row_to_proxy(&row)
    }

    /// 创建代理
    pub async fn create_proxy(&self, dto: CreateProxyDto) -> Result<Proxy> {
        let id = uuid::Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();

        // 检查名称唯一性
        let existing = sqlx::query_scalar::<_, i32>(
            "SELECT COUNT(*) FROM proxies WHERE name = ?"
        )
        .bind(&dto.name)
        .fetch_one(&self.pool)
        .await?;

        if existing > 0 {
            return Err(anyhow::anyhow!("代理名称已存在"));
        }

        let source = dto.source.unwrap_or_else(|| "custom".to_string());
        let tag = dto.tag.unwrap_or_default();
        let remark = dto.remark.unwrap_or_default();
        let auto_check = if dto.auto_check.unwrap_or(false) { 1 } else { 0 };

        sqlx::query(
            r#"
            INSERT INTO proxies (id, name, type, source, tag, host, port, username, password,
                                 ip_address, location, used_count, auto_check, expire_at,
                                 bind_window, remark, status, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, NULL, NULL, 0, ?, ?, NULL, ?, 'active', ?, ?)
            "#
        )
        .bind(&id)
        .bind(&dto.name)
        .bind(&dto.proxy_type)
        .bind(&source)
        .bind(&tag)
        .bind(&dto.host)
        .bind(&dto.port)
        .bind(&dto.username)
        .bind(&dto.password)
        .bind(auto_check)
        .bind(&dto.expire_at)
        .bind(&remark)
        .bind(&now)
        .bind(&now)
        .execute(&self.pool)
        .await?;

        self.get_proxy(&id).await
    }

    /// 更新代理
    pub async fn update_proxy(&self, id: &str, dto: UpdateProxyDto) -> Result<Proxy> {
        let now = Utc::now().to_rfc3339();

        // 检查代理是否存在
        let exists = sqlx::query_scalar::<_, i32>(
            "SELECT COUNT(*) FROM proxies WHERE id = ?"
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await?;

        if exists == 0 {
            return Err(anyhow::anyhow!("代理不存在"));
        }

        // 如果更新名称，检查唯一性
        if let Some(ref name) = dto.name {
            let existing = sqlx::query_scalar::<_, i32>(
                "SELECT COUNT(*) FROM proxies WHERE name = ? AND id != ?"
            )
            .bind(name)
            .bind(id)
            .fetch_one(&self.pool)
            .await?;

            if existing > 0 {
                return Err(anyhow::anyhow!("代理名称已存在"));
            }
        }

        // 动态构建更新语句
        let mut updates = Vec::new();
        let mut values: Vec<String> = Vec::new();

        if let Some(name) = dto.name {
            updates.push("name = ?");
            values.push(name);
        }
        if let Some(proxy_type) = dto.proxy_type {
            updates.push("type = ?");
            values.push(proxy_type);
        }
        if let Some(source) = dto.source {
            updates.push("source = ?");
            values.push(source);
        }
        if let Some(tag) = dto.tag {
            updates.push("tag = ?");
            values.push(tag);
        }
        if let Some(host) = dto.host {
            updates.push("host = ?");
            values.push(host);
        }
        if let Some(port) = dto.port {
            updates.push("port = ?");
            values.push(port);
        }
        if let Some(remark) = dto.remark {
            updates.push("remark = ?");
            values.push(remark);
        }
        if let Some(status) = dto.status {
            updates.push("status = ?");
            values.push(status);
        }
        if let Some(ip_address) = dto.ip_address {
            updates.push("ip_address = ?");
            values.push(ip_address);
        }
        if let Some(location) = dto.location {
            updates.push("location = ?");
            values.push(location);
        }

        // 处理可选字段更新
        let mut extra_binds: Vec<Option<String>> = Vec::new();
        if dto.username.is_some() || updates.is_empty() {
            updates.push("username = ?");
            extra_binds.push(dto.username);
        }
        if dto.password.is_some() {
            updates.push("password = ?");
            extra_binds.push(dto.password);
        }
        if dto.expire_at.is_some() {
            updates.push("expire_at = ?");
            extra_binds.push(dto.expire_at);
        }
        if dto.bind_window.is_some() {
            updates.push("bind_window = ?");
            extra_binds.push(dto.bind_window);
        }

        if let Some(auto_check) = dto.auto_check {
            updates.push("auto_check = ?");
            values.push(if auto_check { "1".to_string() } else { "0".to_string() });
        }

        if updates.is_empty() {
            return self.get_proxy(id).await;
        }

        updates.push("updated_at = ?");
        values.push(now);

        let sql = format!("UPDATE proxies SET {} WHERE id = ?", updates.join(", "));
        
        let mut query = sqlx::query(&sql);
        for value in &values {
            query = query.bind(value);
        }
        for opt_value in &extra_binds {
            query = query.bind(opt_value.as_deref());
        }
        query = query.bind(id);
        
        query.execute(&self.pool).await?;

        self.get_proxy(id).await
    }

    /// 删除代理
    pub async fn delete_proxy(&self, id: &str) -> Result<()> {
        let result = sqlx::query("DELETE FROM proxies WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(anyhow::anyhow!("代理不存在"));
        }

        Ok(())
    }

    /// 增加使用次数
    pub async fn increment_used_count(&self, id: &str) -> Result<()> {
        sqlx::query("UPDATE proxies SET used_count = used_count + 1, updated_at = ? WHERE id = ?")
            .bind(Utc::now().to_rfc3339())
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    /// 更新检测结果
    pub async fn update_test_result(&self, id: &str, ip_address: Option<&str>, location: Option<&str>, status: &str) -> Result<()> {
        sqlx::query("UPDATE proxies SET ip_address = ?, location = ?, status = ?, updated_at = ? WHERE id = ?")
            .bind(ip_address)
            .bind(location)
            .bind(status)
            .bind(Utc::now().to_rfc3339())
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    /// 静态检测函数：不依赖数据库，直接检测配置
    pub async fn test_proxy_config(host: &str, port: &str) -> serde_json::Value {
        use tokio::net::TcpStream;
        use std::time::{Instant, Duration};

        let addr = format!("{}:{}", host, port);
        let start = Instant::now();

        match tokio::time::timeout(
            Duration::from_secs(5),
            TcpStream::connect(&addr)
        ).await {
            Ok(Ok(_)) => {
                let latency = start.elapsed().as_millis() as u64;
                serde_json::json!({
                    "success": true,
                    "latency": latency,
                    "ip": host,
                    "location": "检测成功",
                    "error": null
                })
            }
            Ok(Err(e)) => {
                serde_json::json!({
                    "success": false,
                    "latency": null,
                    "ip": null,
                    "location": null,
                    "error": format!("连接失败: {}", e)
                })
            }
            Err(_) => {
                serde_json::json!({
                    "success": false,
                    "latency": null,
                    "ip": null,
                    "location": null,
                    "error": "连接超时（5秒）"
                })
            }
        }
    }

    /// 辅助函数：从数据库行构造 Proxy
    fn row_to_proxy(&self, row: &sqlx::sqlite::SqliteRow) -> Result<Proxy> {
        let auto_check_int: i32 = row.try_get("auto_check")?;

        Ok(Proxy {
            id: row.try_get("id")?,
            name: row.try_get("name")?,
            proxy_type: row.try_get("type")?,
            source: row.try_get("source")?,
            tag: row.try_get("tag")?,
            host: row.try_get("host")?,
            port: row.try_get("port")?,
            username: row.try_get("username")?,
            password: row.try_get("password")?,
            ip_address: row.try_get("ip_address")?,
            location: row.try_get("location")?,
            used_count: row.try_get("used_count")?,
            auto_check: auto_check_int != 0,
            expire_at: row.try_get("expire_at")?,
            bind_window: row.try_get("bind_window")?,
            remark: row.try_get("remark")?,
            status: row.try_get("status")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
        })
    }
}

// ============================================
// 兼容旧代码的类型别名
// ============================================
pub type ProxyTemplate = Proxy;
pub type CreateProxyTemplateDto = CreateProxyDto;
pub type UpdateProxyTemplateDto = UpdateProxyDto;
pub type ProxyTemplateService = ProxyService;
