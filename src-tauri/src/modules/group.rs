// Group 模块 - 分组管理
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use chrono::Utc;

/// 分组权限
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum GroupPermission {
    Editable,
    Readonly,
}

impl std::fmt::Display for GroupPermission {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GroupPermission::Editable => write!(f, "editable"),
            GroupPermission::Readonly => write!(f, "readonly"),
        }
    }
}

/// 分组模型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Group {
    pub id: String,
    pub name: String,
    pub sort: i32,
    pub permission: GroupPermission,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

/// 创建分组 DTO
#[derive(Debug, Deserialize)]
pub struct CreateGroupDto {
    pub name: String,
    #[serde(default)]
    pub sort: i32,
    pub permission: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
}

/// 更新分组 DTO
#[derive(Debug, Deserialize)]
pub struct UpdateGroupDto {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
}

/// 分组服务
pub struct GroupService {
    pool: SqlitePool,
}

impl GroupService {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    /// 获取所有分组（带 profileCount）
    pub async fn list_groups(&self) -> Result<Vec<Group>, String> {
        let rows = sqlx::query!(
            r#"
            SELECT 
                g.id as "id!: String",
                g.name as "name!: String",
                g.sort as "sort!: i32",
                g.permission as "permission!: String",
                g.remark as "remark: String",
                g.icon as "icon: String",
                g.created_at as "created_at!: String",
                g.updated_at as "updated_at!: String",
                COUNT(p.id) as "profile_count!: i32"
            FROM groups g
            LEFT JOIN profiles p ON p.group_id = g.id AND p.deleted_at IS NULL
            GROUP BY g.id
            ORDER BY g.sort ASC, g.created_at ASC
            "#
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("获取分组列表失败: {}", e))?;

        let groups = rows
            .into_iter()
            .map(|row| {
                let permission = match row.permission.as_str() {
                    "readonly" => GroupPermission::Readonly,
                    _ => GroupPermission::Editable,
                };

                Group {
                    id: row.id,
                    name: row.name,
                    sort: row.sort,
                    permission,
                    profile_count: Some(row.profile_count),
                    remark: row.remark,
                    icon: row.icon,
                    created_at: row.created_at,
                    updated_at: row.updated_at,
                }
            })
            .collect();

        Ok(groups)
    }

    /// 获取单个分组
    pub async fn get_group(&self, id: &str) -> Result<Group, String> {
        let row = sqlx::query!(
            r#"
            SELECT 
                g.id as "id!: String",
                g.name as "name!: String",
                g.sort as "sort!: i32",
                g.permission as "permission!: String",
                g.remark as "remark: String",
                g.icon as "icon: String",
                g.created_at as "created_at!: String",
                g.updated_at as "updated_at!: String",
                COUNT(p.id) as "profile_count!: i32"
            FROM groups g
            LEFT JOIN profiles p ON p.group_id = g.id AND p.deleted_at IS NULL
            WHERE g.id = ?
            GROUP BY g.id
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| format!("获取分组失败: {}", e))?
        .ok_or_else(|| format!("分组不存在: {}", id))?;

        let permission = match row.permission.as_str() {
            "readonly" => GroupPermission::Readonly,
            _ => GroupPermission::Editable,
        };

        Ok(Group {
            id: row.id,
            name: row.name,
            sort: row.sort,
            permission,
            profile_count: Some(row.profile_count),
            remark: row.remark,
            icon: row.icon,
            created_at: row.created_at,
            updated_at: row.updated_at,
        })
    }

    /// 创建分组
    pub async fn create_group(&self, data: CreateGroupDto) -> Result<Group, String> {
        // 生成 ID（使用名称的 slug 化版本）
        let id = self.generate_group_id(&data.name);

        // 检查名称是否已存在
        let exists = sqlx::query_scalar::<_, i32>("SELECT COUNT(*) FROM groups WHERE name = ?")
            .bind(&data.name)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| format!("检查分组名称失败: {}", e))?;

        if exists > 0 {
            return Err(format!("分组名称已存在: {}", data.name));
        }

        let now = Utc::now().to_rfc3339();

        sqlx::query!(
            r#"
            INSERT INTO groups (id, name, sort, permission, remark, icon, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?)
            "#,
            id,
            data.name,
            data.sort,
            data.permission,
            data.remark,
            data.icon,
            now,
            now
        )
        .execute(&self.pool)
        .await
        .map_err(|e| format!("创建分组失败: {}", e))?;

        self.get_group(&id).await
    }

    /// 更新分组
    pub async fn update_group(&self, id: &str, data: UpdateGroupDto) -> Result<Group, String> {
        // 检查分组是否存在
        let group = self.get_group(id).await?;

        // 检查是否为只读分组
        if group.permission == GroupPermission::Readonly {
            return Err("默认分组不可编辑".to_string());
        }

        let now = Utc::now().to_rfc3339();

        // 更新名称
        if let Some(name) = data.name {
            // 检查名称是否已被其他分组使用
            let exists = sqlx::query_scalar::<_, i32>(
                "SELECT COUNT(*) FROM groups WHERE name = ? AND id != ?"
            )
            .bind(&name)
            .bind(id)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| format!("检查分组名称失败: {}", e))?;

            if exists > 0 {
                return Err(format!("分组名称已存在: {}", name));
            }

            sqlx::query!("UPDATE groups SET name = ?, updated_at = ? WHERE id = ?", name, now, id)
                .execute(&self.pool)
                .await
                .map_err(|e| format!("更新分组名称失败: {}", e))?;
        }

        // 更新排序
        if let Some(sort) = data.sort {
            sqlx::query!("UPDATE groups SET sort = ?, updated_at = ? WHERE id = ?", sort, now, id)
                .execute(&self.pool)
                .await
                .map_err(|e| format!("更新分组排序失败: {}", e))?;
        }

        // 更新备注
        if let Some(remark) = data.remark {
            sqlx::query!("UPDATE groups SET remark = ?, updated_at = ? WHERE id = ?", remark, now, id)
                .execute(&self.pool)
                .await
                .map_err(|e| format!("更新分组备注失败: {}", e))?;
        }

        // 更新图标
        if let Some(icon) = data.icon {
            sqlx::query!("UPDATE groups SET icon = ?, updated_at = ? WHERE id = ?", icon, now, id)
                .execute(&self.pool)
                .await
                .map_err(|e| format!("更新分组图标失败: {}", e))?;
        }

        self.get_group(id).await
    }

    /// 删除分组
    pub async fn delete_group(&self, id: &str) -> Result<(), String> {
        // 检查分组是否存在
        let group = self.get_group(id).await?;

        // 检查是否为只读分组
        if group.permission == GroupPermission::Readonly {
            return Err("默认分组不可删除".to_string());
        }

        // 检查是否有窗口使用此分组
        let profile_count = group.profile_count.unwrap_or(0);
        if profile_count > 0 {
            return Err(format!(
                "该分组下还有 {} 个窗口，无法删除。请先将窗口移动到其他分组。",
                profile_count
            ));
        }

        sqlx::query!("DELETE FROM groups WHERE id = ?", id)
            .execute(&self.pool)
            .await
            .map_err(|e| format!("删除分组失败: {}", e))?;

        Ok(())
    }

    /// 生成分组 ID（基于名称）
    fn generate_group_id(&self, _name: &str) -> String {
        use uuid::Uuid;
        
        // 使用 UUID 生成唯一 ID
        let uuid = Uuid::new_v4();
        format!("group_{}", uuid.simple())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_group_permission_display() {
        assert_eq!(GroupPermission::Editable.to_string(), "editable");
        assert_eq!(GroupPermission::Readonly.to_string(), "readonly");
    }
}
