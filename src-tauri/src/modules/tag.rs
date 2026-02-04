// Tag Module - 标签管理
use sqlx::SqlitePool;
use serde::{Deserialize, Serialize};
use chrono::Utc;
use anyhow::Result;

/// 标签
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Tag {
    pub id: String,
    pub name: String,
    pub sort: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_count: Option<i32>,
    pub created_at: String,
    pub updated_at: String,
}

/// 创建标签 DTO
#[derive(Debug, Deserialize)]
pub struct CreateTagDto {
    pub name: String,
    #[serde(default)]
    pub sort: i32,
    pub remark: Option<String>,
}

/// 更新标签 DTO
#[derive(Debug, Deserialize)]
pub struct UpdateTagDto {
    pub name: Option<String>,
    pub sort: Option<i32>,
    pub remark: Option<String>,
}

/// 标签服务
pub struct TagService {
    pool: SqlitePool,
}

impl TagService {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    /// 获取所有标签（包含 windowCount）
    pub async fn list_tags(&self) -> Result<Vec<Tag>> {
        let tags = sqlx::query_as::<_, Tag>(
            r#"
            SELECT 
                t.id,
                t.name,
                t.sort,
                t.remark,
                COUNT(pt.profile_id) as window_count,
                t.created_at,
                t.updated_at
            FROM tags t
            LEFT JOIN profile_tags pt ON t.id = pt.tag_id
            LEFT JOIN profiles p ON pt.profile_id = p.id AND p.deleted_at IS NULL
            GROUP BY t.id
            ORDER BY t.sort ASC, t.created_at DESC
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(tags)
    }

    /// 创建标签
    pub async fn create_tag(&self, dto: CreateTagDto) -> Result<Tag> {
        let id = uuid::Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();

        // 检查名称唯一性
        let existing = sqlx::query_scalar::<_, i32>(
            "SELECT COUNT(*) FROM tags WHERE name = ?"
        )
        .bind(&dto.name)
        .fetch_one(&self.pool)
        .await?;

        if existing > 0 {
            return Err(anyhow::anyhow!("标签名称已存在"));
        }

        sqlx::query(
            r#"
            INSERT INTO tags (id, name, sort, remark, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?)
            "#
        )
        .bind(&id)
        .bind(&dto.name)
        .bind(dto.sort)
        .bind(&dto.remark)
        .bind(&now)
        .bind(&now)
        .execute(&self.pool)
        .await?;

        Ok(Tag {
            id,
            name: dto.name,
            sort: dto.sort,
            remark: dto.remark,
            window_count: Some(0),
            created_at: now.clone(),
            updated_at: now,
        })
    }

    /// 更新标签
    pub async fn update_tag(&self, id: &str, dto: UpdateTagDto) -> Result<Tag> {
        let now = Utc::now().to_rfc3339();

        // 检查标签是否存在
        let exists = sqlx::query_scalar::<_, i32>(
            "SELECT COUNT(*) FROM tags WHERE id = ?"
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await?;

        if exists == 0 {
            return Err(anyhow::anyhow!("标签不存在"));
        }

        // 如果更新名称，检查唯一性
        if let Some(ref name) = dto.name {
            let existing = sqlx::query_scalar::<_, i32>(
                "SELECT COUNT(*) FROM tags WHERE name = ? AND id != ?"
            )
            .bind(name)
            .bind(id)
            .fetch_one(&self.pool)
            .await?;

            if existing > 0 {
                return Err(anyhow::anyhow!("标签名称已存在"));
            }
        }

        // 动态构建更新语句
        let mut updates = Vec::new();
        let mut values: Vec<String> = Vec::new();

        if let Some(name) = dto.name {
            updates.push("name = ?");
            values.push(name);
        }
        if let Some(sort) = dto.sort {
            updates.push("sort = ?");
            values.push(sort.to_string());
        }
        if let Some(remark) = dto.remark {
            updates.push("remark = ?");
            values.push(remark);
        }

        if updates.is_empty() {
            return self.get_tag(id).await;
        }

        updates.push("updated_at = ?");
        values.push(now.clone());

        let sql = format!("UPDATE tags SET {} WHERE id = ?", updates.join(", "));
        
        let mut query = sqlx::query(&sql);
        for value in &values {
            query = query.bind(value);
        }
        query = query.bind(id);
        
        query.execute(&self.pool).await?;

        self.get_tag(id).await
    }

    /// 删除标签
    pub async fn delete_tag(&self, id: &str) -> Result<()> {
        let result = sqlx::query("DELETE FROM tags WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(anyhow::anyhow!("标签不存在"));
        }

        Ok(())
    }

    /// 获取单个标签
    async fn get_tag(&self, id: &str) -> Result<Tag> {
        let tag = sqlx::query_as::<_, Tag>(
            r#"
            SELECT 
                t.id,
                t.name,
                t.sort,
                t.remark,
                COUNT(pt.profile_id) as window_count,
                t.created_at,
                t.updated_at
            FROM tags t
            LEFT JOIN profile_tags pt ON t.id = pt.tag_id
            LEFT JOIN profiles p ON pt.profile_id = p.id AND p.deleted_at IS NULL
            WHERE t.id = ?
            GROUP BY t.id
            "#
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await?;

        Ok(tag)
    }

    /// 获取窗口的标签
    pub async fn get_profile_tags(&self, profile_id: &str) -> Result<Vec<Tag>> {
        let tags = sqlx::query_as::<_, Tag>(
            r#"
            SELECT t.id, t.name, t.sort, t.remark, NULL as window_count, t.created_at, t.updated_at
            FROM tags t
            INNER JOIN profile_tags pt ON t.id = pt.tag_id
            WHERE pt.profile_id = ?
            ORDER BY t.sort ASC, t.created_at DESC
            "#
        )
        .bind(profile_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(tags)
    }

    /// 为窗口添加标签
    pub async fn add_tag_to_profile(&self, profile_id: &str, tag_id: &str) -> Result<()> {
        let now = Utc::now().to_rfc3339();

        sqlx::query(
            "INSERT OR IGNORE INTO profile_tags (profile_id, tag_id, created_at) VALUES (?, ?, ?)"
        )
        .bind(profile_id)
        .bind(tag_id)
        .bind(&now)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// 移除窗口标签
    pub async fn remove_tag_from_profile(&self, profile_id: &str, tag_id: &str) -> Result<()> {
        sqlx::query(
            "DELETE FROM profile_tags WHERE profile_id = ? AND tag_id = ?"
        )
        .bind(profile_id)
        .bind(tag_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// 设置窗口的所有标签（替换模式）
    pub async fn set_profile_tags(&self, profile_id: &str, tag_ids: Vec<String>) -> Result<()> {
        // 先删除该窗口的所有标签
        sqlx::query("DELETE FROM profile_tags WHERE profile_id = ?")
            .bind(profile_id)
            .execute(&self.pool)
            .await?;

        // 再添加新的标签
        let now = Utc::now().to_rfc3339();
        for tag_id in tag_ids {
            sqlx::query(
                "INSERT OR IGNORE INTO profile_tags (profile_id, tag_id, created_at) VALUES (?, ?, ?)"
            )
            .bind(profile_id)
            .bind(&tag_id)
            .bind(&now)
            .execute(&self.pool)
            .await?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_tag_name_uniqueness() {
        // 这个测试需要实际的数据库连接
        // 这里只是示例结构
    }
}
