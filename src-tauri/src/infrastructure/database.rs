use anyhow::{Result, Context};
use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};
use std::path::Path;
use tracing::info;

/// 数据库管理器
pub struct Database {
    pool: SqlitePool,
}

impl Database {
    /// 初始化数据库连接
    pub async fn new(database_url: &str) -> Result<Self> {
        info!("Connecting to database: {}", database_url);
        
        // 创建连接池
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(database_url)
            .await
            .context(format!("Failed to connect to database at: {}", database_url))?;
        
        info!("Database connected successfully");
        
        Ok(Self { pool })
    }

    /// 运行迁移
    pub async fn migrate(&self) -> Result<()> {
        info!("Running database migrations");
        
        sqlx::migrate!("./migrations")
            .run(&self.pool)
            .await
            .context("Failed to run migrations")?;
        
        info!("Migrations completed successfully");
        
        Ok(())
    }

    /// 获取连接池
    pub fn pool(&self) -> &SqlitePool {
        &self.pool
    }

    /// 关闭数据库连接
    pub async fn close(self) -> Result<()> {
        self.pool.close().await;
        info!("Database connection closed");
        Ok(())
    }
}

/// 初始化数据库（创建文件并运行迁移）
pub async fn init_database(db_path: &Path) -> Result<Database> {
    // 确保父目录存在
    if let Some(parent) = db_path.parent() {
        std::fs::create_dir_all(parent)
            .context("Failed to create database directory")?;
    }

    // 使用绝对路径格式（Windows 兼容）
    let database_url = if cfg!(windows) {
        // Windows: sqlite:///C:/path/to/db.db?mode=rwc (三个斜杠，mode=rwc 表示读写创建)
        format!("sqlite:///{}?mode=rwc", db_path.display().to_string().replace("\\", "/"))
    } else {
        // Unix: sqlite:///path/to/db.db?mode=rwc
        format!("sqlite://{}?mode=rwc", db_path.display())
    };
    
    info!("Database URL: {}", database_url);
    
    let db = Database::new(&database_url).await?;
    db.migrate().await?;
    
    Ok(db)
}
