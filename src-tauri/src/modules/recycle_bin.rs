// RecycleBin Module - 回收站管理
use sqlx::{SqlitePool, Row};
use chrono::Utc;
use anyhow::Result;
use serde::{Serialize, Deserialize};
use crate::modules::profile::models::{ProfileStatus, Fingerprint, ProxyConfig};

/// 回收站中的窗口（包含删除时间）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecycledProfile {
    pub id: String,
    pub name: String,
    pub group: String,
    pub remark: String,
    pub status: ProfileStatus,
    pub fingerprint: Fingerprint,
    pub proxy: Option<ProxyConfig>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub deleted_at: chrono::DateTime<chrono::Utc>,  // ✅ V5 新增：删除时间
}

fn parse_datetime(value: String) -> Result<chrono::DateTime<chrono::Utc>> {
    if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(&value) {
        return Ok(dt.with_timezone(&chrono::Utc));
    }
    if let Ok(naive) = chrono::NaiveDateTime::parse_from_str(&value, "%Y-%m-%d %H:%M:%S") {
        return Ok(chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(naive, chrono::Utc));
    }
    if let Ok(naive) = chrono::NaiveDateTime::parse_from_str(&value, "%Y-%m-%d %H:%M:%S%.f") {
        return Ok(chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(naive, chrono::Utc));
    }
    Err(anyhow::anyhow!("Invalid datetime format: {}", value))
}

/// 回收站服务
pub struct RecycleBinService {
    pool: SqlitePool,
}

impl RecycleBinService {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    /// 获取回收站列表（已删除的窗口）
    pub async fn list_recycled(&self) -> Result<Vec<RecycledProfile>> {
        let rows = sqlx::query(
            r#"
            SELECT id, name, "group", remark, status, fingerprint, proxy, created_at, updated_at, deleted_at
            FROM profiles
            WHERE deleted_at IS NOT NULL
            ORDER BY deleted_at DESC
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        let mut profiles = Vec::new();
        for row in rows {
            let status_str: String = row.try_get("status")?;
            let status = match status_str.as_str() {
                "running" => ProfileStatus::Running,
                _ => ProfileStatus::Stopped,
            };

            let fingerprint_str: String = row.try_get("fingerprint")?;
            let proxy_str: Option<String> = row.try_get("proxy")?;
            let fingerprint = serde_json::from_str(&fingerprint_str)?;
            let proxy = proxy_str.as_ref().map(|p| serde_json::from_str(p)).transpose()?;

            let created_at: String = row.try_get("created_at")?;
            let updated_at: String = row.try_get("updated_at")?;
            let deleted_at: String = row.try_get("deleted_at")?;

            profiles.push(RecycledProfile {
                id: row.try_get("id")?,
                name: row.try_get("name")?,
                group: row.try_get("group")?,
                remark: row.try_get("remark")?,
                status,
                fingerprint,
                proxy,
                created_at: parse_datetime(created_at)?,
                updated_at: parse_datetime(updated_at)?,
                deleted_at: parse_datetime(deleted_at)?,  // ✅ V5 新增
            });
        }

        Ok(profiles)
    }

    /// 恢复窗口（从回收站恢复）
    pub async fn restore_profile(&self, id: &str) -> Result<()> {
        let now = Utc::now().to_rfc3339();

        let result = sqlx::query(
            "UPDATE profiles SET deleted_at = NULL, updated_at = ? WHERE id = ? AND deleted_at IS NOT NULL"
        )
        .bind(&now)
        .bind(id)
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(anyhow::anyhow!("窗口不存在或未在回收站中"));
        }

        Ok(())
    }

    /// 批量恢复窗口
    pub async fn batch_restore_profiles(&self, ids: Vec<String>) -> Result<crate::modules::BatchResult> {
        let mut results = Vec::new();

        for id in ids {
            let result = match self.restore_profile(&id).await {
                Ok(_) => crate::modules::BatchItemResult::success(id.clone()),
                Err(e) => crate::modules::BatchItemResult::failure(id.clone(), e.to_string()),
            };
            results.push(result);
        }

        Ok(crate::modules::BatchResult::from_results(results))
    }

    /// 永久删除窗口
    pub async fn permanently_delete(&self, id: &str) -> Result<()> {
        let result = sqlx::query(
            "DELETE FROM profiles WHERE id = ? AND deleted_at IS NOT NULL"
        )
        .bind(id)
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(anyhow::anyhow!("窗口不存在或未在回收站中"));
        }

        Ok(())
    }

    /// 批量永久删除窗口
    pub async fn batch_permanently_delete(&self, ids: Vec<String>) -> Result<crate::modules::BatchResult> {
        let mut results = Vec::new();

        for id in ids {
            let result = match self.permanently_delete(&id).await {
                Ok(_) => crate::modules::BatchItemResult::success(id.clone()),
                Err(e) => crate::modules::BatchItemResult::failure(id.clone(), e.to_string()),
            };
            results.push(result);
        }

        Ok(crate::modules::BatchResult::from_results(results))
    }

    /// 清空回收站（永久删除所有已删除的窗口）
    pub async fn empty_recycle_bin(&self) -> Result<u64> {
        let result = sqlx::query(
            "DELETE FROM profiles WHERE deleted_at IS NOT NULL"
        )
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected())
    }
}

