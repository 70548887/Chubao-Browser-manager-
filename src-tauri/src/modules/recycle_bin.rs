// RecycleBin Module - 回收站管理
use sqlx::{SqlitePool, Row};
use chrono::Utc;
use anyhow::Result;
use serde::{Serialize, Deserialize};
use std::path::PathBuf;
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
    
    /// 获取用户数据目录（从设置读取或使用默认值）
    async fn get_user_data_dir(&self, app_data_dir: &PathBuf) -> Result<PathBuf> {
        // 从数据库读取 user_data_dir 设置
        let user_data_dir_setting = sqlx::query_scalar::<_, String>(
            "SELECT value FROM settings WHERE key = 'user_data_dir'"
        )
        .fetch_optional(&self.pool)
        .await?
        .unwrap_or_default();
        
        let base_dir = if user_data_dir_setting.trim().is_empty() {
            app_data_dir.clone()
        } else {
            PathBuf::from(user_data_dir_setting)
        };
        
        Ok(base_dir)
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

    /// 永久删除窗口（同时删除数据库记录和缓存目录）
    pub async fn permanently_delete(&self, id: &str, app_data_dir: &PathBuf) -> Result<()> {
        // 1. 从数据库删除
        let result = sqlx::query(
            "DELETE FROM profiles WHERE id = ? AND deleted_at IS NOT NULL"
        )
        .bind(id)
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(anyhow::anyhow!("窗口不存在或未在回收站中"));
        }

        // 2. 获取用户数据目录
        let base_user_data_dir = self.get_user_data_dir(app_data_dir).await?;
        
        // 3. 删除缓存目录 (profiles/[profile_id])
        let profile_dir = base_user_data_dir.join("profiles").join(id);
        if profile_dir.exists() {
            if let Err(e) = std::fs::remove_dir_all(&profile_dir) {
                tracing::warn!(
                    profile_id = %id,
                    error = %e,
                    path = ?profile_dir,
                    "删除缓存目录失败（数据库记录已删除）"
                );
                // 不阻断执行，即使目录删除失败也认为成功
            } else {
                tracing::info!(
                    profile_id = %id,
                    path = ?profile_dir,
                    "已删除窗口缓存目录"
                );
            }
        }

        Ok(())
    }

    /// 批量永久删除窗口
    pub async fn batch_permanently_delete(&self, ids: Vec<String>, app_data_dir: &PathBuf) -> Result<crate::modules::BatchResult> {
        let mut results = Vec::new();

        for id in ids {
            let result = match self.permanently_delete(&id, app_data_dir).await {
                Ok(_) => crate::modules::BatchItemResult::success(id.clone()),
                Err(e) => crate::modules::BatchItemResult::failure(id.clone(), e.to_string()),
            };
            results.push(result);
        }

        Ok(crate::modules::BatchResult::from_results(results))
    }

    /// 清空回收站（永久删除所有已删除的窗口）
    pub async fn empty_recycle_bin(&self, app_data_dir: &PathBuf) -> Result<u64> {
        // 1. 获取所有待删除的 profile_id
        let rows = sqlx::query(
            "SELECT id FROM profiles WHERE deleted_at IS NOT NULL"
        )
        .fetch_all(&self.pool)
        .await?;

        let profile_ids: Vec<String> = rows
            .iter()
            .filter_map(|row| row.try_get::<String, _>("id").ok())
            .collect();

        // 2. 从数据库删除
        let result = sqlx::query(
            "DELETE FROM profiles WHERE deleted_at IS NOT NULL"
        )
        .execute(&self.pool)
        .await?;

        let deleted_count = result.rows_affected();

        // 3. 获取用户数据目录
        let base_user_data_dir = self.get_user_data_dir(app_data_dir).await?;
        
        // 4. 删除所有对应的缓存目录
        for profile_id in profile_ids {
            let profile_dir = base_user_data_dir.join("profiles").join(&profile_id);
            if profile_dir.exists() {
                if let Err(e) = std::fs::remove_dir_all(&profile_dir) {
                    tracing::warn!(
                        profile_id = %profile_id,
                        error = %e,
                        path = ?profile_dir,
                        "清空回收站时删除缓存目录失败"
                    );
                }
            }
        }

        tracing::info!(
            count = deleted_count,
            "已清空回收站并删除对应缓存目录"
        );

        Ok(deleted_count)
    }
}

