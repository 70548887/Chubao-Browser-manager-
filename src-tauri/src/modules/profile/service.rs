use anyhow::Result;
use sqlx::{Sqlite, SqlitePool, QueryBuilder, Row};
use uuid::Uuid;
use chrono::Utc;

use super::models::{Profile, CreateProfileDto, UpdateProfileDto, ProfileStatus};

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

/// 环境配置服务
pub struct ProfileService {
    pool: SqlitePool,
}

impl ProfileService {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    /// 创建环境
    pub async fn create_profile(&self, dto: CreateProfileDto) -> Result<Profile> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();
        
        let fingerprint_json = serde_json::to_string(&dto.fingerprint)?;
        let proxy_json = dto.proxy.as_ref().map(|p| serde_json::to_string(p)).transpose()?;
        let preferences_json = dto.preferences.as_ref().map(|p| serde_json::to_string(p)).transpose()?;

        // 开始事务
        let mut tx = self.pool.begin().await?;

        // 插入主表
        sqlx::query(
            r#"
            INSERT INTO profiles (id, name, "group", remark, status, fingerprint, proxy, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&id)
        .bind(dto.name)
        .bind(dto.group)
        .bind(dto.remark)
        .bind("stopped")
        .bind(fingerprint_json)
        .bind(proxy_json)
        .bind(&now)
        .bind(&now)
        .execute(&mut *tx)
        .await?;

        // 如果有偏好设置，则插入偏好设置表
        if let Some(pref) = preferences_json {
            sqlx::query(
                r#"INSERT INTO profile_preferences (profile_id, preferences, created_at, updated_at) VALUES (?, ?, ?, ?)"#
            )
            .bind(&id)
            .bind(pref)
            .bind(&now)
            .bind(&now)
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await?;

        self.get_profile(&id).await
    }

    /// 获取单个环境
    pub async fn get_profile(&self, id: &str) -> Result<Profile> {
        // 获取主表信息
        let row = sqlx::query(
            r#"
            SELECT id, name, "group", remark, status, fingerprint, proxy, created_at, updated_at
            FROM profiles
            WHERE id = ?
            "#,
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await?;

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

        // 获取偏好设置
        let preferences_row = sqlx::query(
            r#"SELECT preferences FROM profile_preferences WHERE profile_id = ? LIMIT 1"#
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        let preferences = match preferences_row {
            Some(row) => {
                let pref_str: String = row.try_get("preferences")?;
                Some(serde_json::from_str(&pref_str)?)
            },
            None => None,
        };

        Ok(Profile {
            id: row.try_get("id")?,
            name: row.try_get("name")?,
            group: row.try_get("group")?,
            remark: row.try_get("remark")?,
            status,
            fingerprint,
            proxy,
            preferences,
            created_at: parse_datetime(created_at)?,
            updated_at: parse_datetime(updated_at)?,
        })
    }

    /// 获取所有环境（不包含已删除）
    pub async fn list_profiles(&self) -> Result<Vec<Profile>> {
        let rows = sqlx::query(
            r#"
            SELECT p.id, p.name, p."group", p.remark, p.status, p.fingerprint, p.proxy, p.created_at, p.updated_at,
                   pp.preferences as preferences_json
            FROM profiles p
            LEFT JOIN profile_preferences pp ON p.id = pp.profile_id
            WHERE p.deleted_at IS NULL
            ORDER BY p.created_at DESC
            "#,
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
            let preferences_json_str: Option<String> = row.try_get("preferences_json")?;
            let fingerprint = serde_json::from_str(&fingerprint_str)?;
            let proxy = proxy_str.as_ref().map(|p| serde_json::from_str(p)).transpose()?;
            let preferences = match preferences_json_str {
                Some(pref_str) => Some(serde_json::from_str(&pref_str)?),
                None => None,
            };

            let created_at: String = row.try_get("created_at")?;
            let updated_at: String = row.try_get("updated_at")?;

            profiles.push(Profile {
                id: row.try_get("id")?,
                name: row.try_get("name")?,
                group: row.try_get("group")?,
                remark: row.try_get("remark")?,
                status,
                fingerprint,
                proxy,
                preferences,
                created_at: parse_datetime(created_at)?,
                updated_at: parse_datetime(updated_at)?,
            });
        }

        Ok(profiles)
    }

    /// 更新环境
    pub async fn update_profile(&self, id: &str, dto: UpdateProfileDto) -> Result<Profile> {
        let now = Utc::now();
        
        // 处理 fingerprint merge
        let fingerprint_json = if let Some(patch_fp) = dto.fingerprint {
            // 获取现有的 fingerprint
            let existing_profile = self.get_profile(id).await?;
            let existing_fp_value = serde_json::to_value(&existing_profile.fingerprint)?;
            let patch_fp_value = serde_json::to_value(&patch_fp)?;
            
            // 使用 FingerprintMerger 合并
            let merger = crate::modules::FingerprintMerger::new();
            let merged_value = merger.merge(&existing_fp_value, &patch_fp_value)
                .map_err(|e| anyhow::anyhow!(e))?;
            
            Some(serde_json::to_string(&merged_value)?)
        } else {
            None
        };
        
        let proxy_json = dto.proxy.as_ref().map(|p| serde_json::to_string(p)).transpose()?;
        let preferences_json = dto.preferences.as_ref().map(|p| serde_json::to_string(p)).transpose()?;

        // 开始事务
        let mut tx = self.pool.begin().await?;

        // 动态构建 UPDATE 语句
        let mut qb = QueryBuilder::<Sqlite>::new("UPDATE profiles SET ");
        let mut separated = qb.separated(", ");

        if let Some(name) = dto.name {
            separated.push("name = ");
            separated.push_bind(name);
        }
        if let Some(group) = dto.group {
            separated.push(r#""group" = "#);
            separated.push_bind(group);
        }
        if let Some(remark) = dto.remark {
            separated.push("remark = ");
            separated.push_bind(remark);
        }
        if let Some(fingerprint) = fingerprint_json {
            separated.push("fingerprint = ");
            separated.push_bind(fingerprint);
        }
        if let Some(proxy) = proxy_json {
            separated.push("proxy = ");
            separated.push_bind(proxy);
        }

        separated.push("updated_at = ");
        separated.push_bind(now.to_rfc3339());

        qb.push(" WHERE id = ");
        qb.push_bind(id);

        qb.build().execute(&mut *tx).await?;

        // 如果有偏好设置，则更新偏好设置表
        if let Some(pref_json) = preferences_json {
            sqlx::query(
                r#"INSERT OR REPLACE INTO profile_preferences (profile_id, preferences, created_at, updated_at) VALUES (?, ?, ?, ?)"#
            )
            .bind(id)
            .bind(pref_json)
            .bind(now.to_rfc3339())
            .bind(now.to_rfc3339())
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await?;

        self.get_profile(id).await
    }

    /// 删除环境（软删除，移入回收站）
    pub async fn delete_profile(&self, id: &str) -> Result<()> {
        let now = chrono::Utc::now().to_rfc3339();
        
        let result = sqlx::query(
            "UPDATE profiles SET deleted_at = ?, updated_at = ? WHERE id = ? AND deleted_at IS NULL"
        )
        .bind(&now)
        .bind(&now)
        .bind(id)
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(anyhow::anyhow!("窗口不存在或已被删除"));
        }

        Ok(())
    }

    /// 更新环境状态
    pub async fn update_status(&self, id: &str, status: ProfileStatus) -> Result<()> {
        let status_str = match status {
            ProfileStatus::Running => "running",
            ProfileStatus::Stopped => "stopped",
        };

        sqlx::query("UPDATE profiles SET status = ?, updated_at = ? WHERE id = ?")
            .bind(status_str)
            .bind(Utc::now().to_rfc3339())
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
