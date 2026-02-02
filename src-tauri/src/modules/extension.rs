// Extension Service - 扩展管理模块
// 采用共享目录模式：所有扩展存放在全局 Extensions/ 目录
// 每个 Profile 可选择启用哪些扩展

use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use chrono::Utc;
use std::path::{Path, PathBuf};
use std::fs;

// ============================================================================
// 数据模型
// ============================================================================

/// 扩展信息
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Extension {
    pub id: String,
    pub name: String,
    pub extension_id: String,      // Chrome扩展ID
    pub version: String,
    pub category: String,
    pub description: String,
    pub icon: String,
    pub source: String,            // local/store/upload
    pub file_path: String,         // 相对于 Extensions 目录
    pub manifest_json: String,     // manifest.json 内容
    pub enabled: bool,
    pub created_at: String,
    pub updated_at: String,
}

/// 创建扩展 DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateExtensionDto {
    pub name: String,
    pub extension_id: String,
    pub version: Option<String>,
    pub category: Option<String>,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub source: Option<String>,
    pub file_path: String,
    pub manifest_json: Option<String>,
}

/// 更新扩展 DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateExtensionDto {
    pub name: Option<String>,
    pub category: Option<String>,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub enabled: Option<bool>,
}

/// Profile-扩展关联
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ProfileExtension {
    pub profile_id: String,
    pub extension_id: String,
    pub enabled: bool,
    pub created_at: String,
}

/// Manifest.json 解析结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtensionManifest {
    pub name: Option<String>,
    pub version: Option<String>,
    pub description: Option<String>,
    #[serde(default)]
    pub icons: std::collections::HashMap<String, String>,
}

// ============================================================================
// 扩展服务
// ============================================================================

pub struct ExtensionService {
    pool: SqlitePool,
}

impl ExtensionService {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    /// 获取所有扩展
    pub async fn list_extensions(&self) -> Result<Vec<Extension>, anyhow::Error> {
        let extensions = sqlx::query_as::<_, Extension>(
            r#"
            SELECT id, name, extension_id, version, category, description, 
                   icon, source, file_path, manifest_json, 
                   enabled = 1 as enabled, created_at, updated_at
            FROM extensions
            ORDER BY created_at DESC
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(extensions)
    }

    /// 获取已安装的扩展（file_path 非空）
    pub async fn list_installed_extensions(&self) -> Result<Vec<Extension>, anyhow::Error> {
        let extensions = sqlx::query_as::<_, Extension>(
            r#"
            SELECT id, name, extension_id, version, category, description, 
                   icon, source, file_path, manifest_json,
                   enabled = 1 as enabled, created_at, updated_at
            FROM extensions
            WHERE file_path != '' AND file_path IS NOT NULL
            ORDER BY created_at DESC
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(extensions)
    }

    /// 获取单个扩展
    pub async fn get_extension(&self, id: &str) -> Result<Extension, anyhow::Error> {
        let extension = sqlx::query_as::<_, Extension>(
            r#"
            SELECT id, name, extension_id, version, category, description, 
                   icon, source, file_path, manifest_json,
                   enabled = 1 as enabled, created_at, updated_at
            FROM extensions WHERE id = ?
            "#
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await?;

        Ok(extension)
    }

    /// 通过 Chrome 扩展ID 获取
    pub async fn get_by_extension_id(&self, extension_id: &str) -> Result<Option<Extension>, anyhow::Error> {
        let extension = sqlx::query_as::<_, Extension>(
            r#"
            SELECT id, name, extension_id, version, category, description, 
                   icon, source, file_path, manifest_json,
                   enabled = 1 as enabled, created_at, updated_at
            FROM extensions WHERE extension_id = ?
            "#
        )
        .bind(extension_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(extension)
    }

    /// 创建/注册扩展
    pub async fn create_extension(&self, data: CreateExtensionDto) -> Result<Extension, anyhow::Error> {
        let id = uuid::Uuid::new_v4().to_string().replace("-", "");
        let now = Utc::now().to_rfc3339();

        sqlx::query(
            r#"
            INSERT INTO extensions (id, name, extension_id, version, category, description, 
                                   icon, source, file_path, manifest_json, enabled, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, 1, ?, ?)
            "#
        )
        .bind(&id)
        .bind(&data.name)
        .bind(&data.extension_id)
        .bind(data.version.as_deref().unwrap_or(""))
        .bind(data.category.as_deref().unwrap_or(""))
        .bind(data.description.as_deref().unwrap_or(""))
        .bind(data.icon.as_deref().unwrap_or("extension"))
        .bind(data.source.as_deref().unwrap_or("local"))
        .bind(&data.file_path)
        .bind(data.manifest_json.as_deref().unwrap_or("{}"))
        .bind(&now)
        .bind(&now)
        .execute(&self.pool)
        .await?;

        self.get_extension(&id).await
    }

    /// 更新扩展
    pub async fn update_extension(&self, id: &str, data: UpdateExtensionDto) -> Result<Extension, anyhow::Error> {
        let now = Utc::now().to_rfc3339();
        let existing = self.get_extension(id).await?;

        sqlx::query(
            r#"
            UPDATE extensions SET
                name = ?,
                category = ?,
                description = ?,
                icon = ?,
                enabled = ?,
                updated_at = ?
            WHERE id = ?
            "#
        )
        .bind(data.name.as_deref().unwrap_or(&existing.name))
        .bind(data.category.as_deref().unwrap_or(&existing.category))
        .bind(data.description.as_deref().unwrap_or(&existing.description))
        .bind(data.icon.as_deref().unwrap_or(&existing.icon))
        .bind(data.enabled.unwrap_or(existing.enabled))
        .bind(&now)
        .bind(id)
        .execute(&self.pool)
        .await?;

        self.get_extension(id).await
    }

    /// 删除扩展
    pub async fn delete_extension(&self, id: &str) -> Result<(), anyhow::Error> {
        sqlx::query("DELETE FROM extensions WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    /// 切换扩展启用状态
    pub async fn toggle_extension(&self, id: &str, enabled: bool) -> Result<Extension, anyhow::Error> {
        let now = Utc::now().to_rfc3339();
        
        sqlx::query("UPDATE extensions SET enabled = ?, updated_at = ? WHERE id = ?")
            .bind(enabled)
            .bind(&now)
            .bind(id)
            .execute(&self.pool)
            .await?;

        self.get_extension(id).await
    }

    // ========================================================================
    // Profile-扩展关联
    // ========================================================================

    /// 获取 Profile 启用的扩展
    pub async fn get_profile_extensions(&self, profile_id: &str) -> Result<Vec<Extension>, anyhow::Error> {
        let extensions = sqlx::query_as::<_, Extension>(
            r#"
            SELECT e.id, e.name, e.extension_id, e.version, e.category, e.description, 
                   e.icon, e.source, e.file_path, e.manifest_json,
                   pe.enabled = 1 as enabled, e.created_at, e.updated_at
            FROM extensions e
            INNER JOIN profile_extensions pe ON e.id = pe.extension_id
            WHERE pe.profile_id = ? AND pe.enabled = 1 AND e.file_path != ''
            "#
        )
        .bind(profile_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(extensions)
    }

    /// 为 Profile 启用扩展
    pub async fn enable_extension_for_profile(&self, profile_id: &str, extension_id: &str) -> Result<(), anyhow::Error> {
        let now = Utc::now().to_rfc3339();
        
        sqlx::query(
            r#"
            INSERT INTO profile_extensions (profile_id, extension_id, enabled, created_at)
            VALUES (?, ?, 1, ?)
            ON CONFLICT(profile_id, extension_id) DO UPDATE SET enabled = 1
            "#
        )
        .bind(profile_id)
        .bind(extension_id)
        .bind(&now)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// 为 Profile 禁用扩展
    pub async fn disable_extension_for_profile(&self, profile_id: &str, extension_id: &str) -> Result<(), anyhow::Error> {
        sqlx::query(
            "UPDATE profile_extensions SET enabled = 0 WHERE profile_id = ? AND extension_id = ?"
        )
        .bind(profile_id)
        .bind(extension_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    // ========================================================================
    // 扩展文件操作
    // ========================================================================

    /// 扫描扩展目录，自动注册新扩展
    pub async fn scan_extensions_dir(&self, extensions_dir: &Path) -> Result<Vec<Extension>, anyhow::Error> {
        let mut new_extensions = Vec::new();

        if !extensions_dir.exists() {
            fs::create_dir_all(extensions_dir)?;
            return Ok(new_extensions);
        }

        for entry in fs::read_dir(extensions_dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if !path.is_dir() {
                continue;
            }

            let manifest_path = path.join("manifest.json");
            if !manifest_path.exists() {
                continue;
            }

            // 读取 manifest.json
            let manifest_content = fs::read_to_string(&manifest_path)?;
            let manifest: ExtensionManifest = serde_json::from_str(&manifest_content)
                .unwrap_or(ExtensionManifest {
                    name: None,
                    version: None,
                    description: None,
                    icons: std::collections::HashMap::new(),
                });

            // 获取扩展ID（文件夹名）
            let ext_id = path.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("unknown")
                .to_string();

            // 检查是否已注册
            if self.get_by_extension_id(&ext_id).await?.is_some() {
                continue;
            }

            // 注册新扩展
            let extension = self.create_extension(CreateExtensionDto {
                name: manifest.name.unwrap_or_else(|| ext_id.clone()),
                extension_id: ext_id.clone(),
                version: manifest.version,
                category: None,
                description: manifest.description,
                icon: Some("extension".to_string()),
                source: Some("local".to_string()),
                file_path: ext_id,
                manifest_json: Some(manifest_content),
            }).await?;

            new_extensions.push(extension);
        }

        Ok(new_extensions)
    }

    /// 获取扩展的完整路径
    pub fn get_extension_full_path(&self, extensions_dir: &Path, extension: &Extension) -> PathBuf {
        extensions_dir.join(&extension.file_path)
    }

    /// 生成浏览器启动参数 --load-extension
    pub async fn build_load_extension_arg(&self, extensions_dir: &Path, profile_id: &str) -> Result<Option<String>, anyhow::Error> {
        let extensions = self.get_profile_extensions(profile_id).await?;
        
        if extensions.is_empty() {
            return Ok(None);
        }

        let paths: Vec<String> = extensions
            .iter()
            .filter(|e| !e.file_path.is_empty())
            .map(|e| {
                let full_path = extensions_dir.join(&e.file_path);
                full_path.to_string_lossy().to_string()
            })
            .collect();

        if paths.is_empty() {
            return Ok(None);
        }

        Ok(Some(paths.join(",")))
    }
}
