//! Logger Module - 日志系统管理
//! 
//! 功能：
//! - 开发环境：控制台 + 文件双输出
//! - 生产环境：仅文件输出
//! - 日志文件读取
//! - 支持 JSON 格式日志

use std::path::{Path, PathBuf};
use std::fs;
use std::io::{BufRead, BufReader};
use serde::Serialize;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::{
    layer::SubscriberExt,
    util::SubscriberInitExt,
    EnvFilter,
    fmt,
};

/// 日志文件信息
#[derive(Debug, Clone, Serialize)]
pub struct LogFileInfo {
    pub name: String,
    pub path: String,
    pub size: u64,
    pub modified_at: String,
}

/// 日志配置
pub struct LoggerConfig {
    pub log_dir: PathBuf,
    pub max_files: usize,
    pub enable_console: bool,
    pub enable_json: bool,
}

impl Default for LoggerConfig {
    fn default() -> Self {
        Self {
            log_dir: std::env::current_dir()
                .unwrap_or_else(|_| PathBuf::from("."))
                .join("logs"),
            max_files: 30,
            enable_console: cfg!(debug_assertions), // 开发环境启用控制台
            enable_json: false,
        }
    }
}

/// 日志管理器
pub struct Logger;

impl Logger {
    /// 初始化日志系统
    /// 
    /// 返回 guard 需要保持在 main 函数的生命周期内
    pub fn init(config: LoggerConfig) -> tracing_appender::non_blocking::WorkerGuard {
        // 创建日志目录
        let _ = fs::create_dir_all(&config.log_dir);
        
        // 文件追加器：每天轮转
        let file_appender = RollingFileAppender::builder()
            .rotation(Rotation::DAILY)
            .filename_prefix("browser-manager")
            .filename_suffix("log")
            .max_log_files(config.max_files)
            .build(&config.log_dir)
            .expect("Failed to create log appender");
        
        let (non_blocking_file, guard) = tracing_appender::non_blocking(file_appender);
        
        // 环境过滤器
        let env_filter = EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| EnvFilter::new("info,browser_manager=debug"));
        
        if config.enable_console {
            // 开发环境：控制台 + 文件双输出
            let console_layer = fmt::layer()
                .with_target(true)
                .with_thread_ids(false)
                .with_line_number(true)
                .with_ansi(true)  // 控制台启用颜色
                .pretty();  // 美化输出
            
            let file_layer = fmt::layer()
                .with_writer(non_blocking_file)
                .with_target(true)
                .with_thread_ids(true)
                .with_line_number(true)
                .with_ansi(false);  // 文件禁用 ANSI 颜色码
            
            tracing_subscriber::registry()
                .with(env_filter)
                .with(console_layer)
                .with(file_layer)
                .init();
        } else {
            // 生产环境：仅文件输出
            let file_layer = fmt::layer()
                .with_writer(non_blocking_file)
                .with_target(true)
                .with_thread_ids(true)
                .with_line_number(true)
                .with_ansi(false);
            
            tracing_subscriber::registry()
                .with(env_filter)
                .with(file_layer)
                .init();
        }
        
        tracing::info!("Logger initialized, log_dir: {:?}, console: {}", 
                       config.log_dir, config.enable_console);
        
        guard
    }
    
    /// 获取日志目录
    pub fn get_log_dir() -> PathBuf {
        std::env::current_dir()
            .unwrap_or_else(|_| PathBuf::from("."))
            .join("logs")
    }
    
    /// 获取所有日志文件列表
    pub fn list_log_files() -> Vec<LogFileInfo> {
        let log_dir = Self::get_log_dir();
        let mut files = Vec::new();
        
        if let Ok(entries) = fs::read_dir(&log_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() && path.extension().map_or(false, |e| e == "log") {
                    if let Ok(metadata) = fs::metadata(&path) {
                        let modified_at = metadata.modified()
                            .map(|t| {
                                let datetime: chrono::DateTime<chrono::Utc> = t.into();
                                datetime.to_rfc3339()
                            })
                            .unwrap_or_default();
                        
                        files.push(LogFileInfo {
                            name: path.file_name()
                                .map(|n| n.to_string_lossy().to_string())
                                .unwrap_or_default(),
                            path: path.to_string_lossy().to_string(),
                            size: metadata.len(),
                            modified_at,
                        });
                    }
                }
            }
        }
        
        // 按修改时间倒序排序
        files.sort_by(|a, b| b.modified_at.cmp(&a.modified_at));
        files
    }
    
    /// 读取日志文件内容
    /// 
    /// - `filename`: 日志文件名
    /// - `lines`: 读取的行数，None 表示全部
    pub fn read_log_file(filename: &str, lines: Option<usize>) -> Result<Vec<String>, String> {
        let log_dir = Self::get_log_dir();
        let file_path = log_dir.join(filename);
        
        // 安全检查：确保文件在日志目录内
        if !file_path.starts_with(&log_dir) {
            return Err("非法的文件路径".to_string());
        }
        
        if !file_path.exists() {
            return Err(format!("日志文件不存在: {}", filename));
        }
        
        let file = fs::File::open(&file_path)
            .map_err(|e| format!("打开日志文件失败: {}", e))?;
        
        let reader = BufReader::new(file);
        let all_lines: Vec<String> = reader.lines()
            .filter_map(|l| l.ok())
            .collect();
        
        match lines {
            Some(n) => {
                // 返回最后 n 行
                let start = if all_lines.len() > n { all_lines.len() - n } else { 0 };
                Ok(all_lines[start..].to_vec())
            }
            None => Ok(all_lines),
        }
    }
    
    /// 读取最新日志文件的最后 n 行（tail 功能）
    pub fn tail_latest_log(lines: usize) -> Result<Vec<String>, String> {
        let files = Self::list_log_files();
        
        if files.is_empty() {
            return Err("没有找到日志文件".to_string());
        }
        
        // 获取最新的日志文件
        let latest = &files[0];
        Self::read_log_file(&latest.name, Some(lines))
    }
    
    /// 清理旧日志文件（保留最近 n 天）
    pub fn cleanup_old_logs(keep_days: usize) -> Result<usize, String> {
        let log_dir = Self::get_log_dir();
        let mut deleted = 0;
        
        let cutoff = chrono::Utc::now() - chrono::Duration::days(keep_days as i64);
        
        if let Ok(entries) = fs::read_dir(&log_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() && path.extension().map_or(false, |e| e == "log") {
                    if let Ok(metadata) = fs::metadata(&path) {
                        if let Ok(modified) = metadata.modified() {
                            let modified_time: chrono::DateTime<chrono::Utc> = modified.into();
                            if modified_time < cutoff {
                                if fs::remove_file(&path).is_ok() {
                                    deleted += 1;
                                    tracing::info!("Deleted old log file: {:?}", path);
                                }
                            }
                        }
                    }
                }
            }
        }
        
        Ok(deleted)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_log_files() {
        let files = Logger::list_log_files();
        println!("Found {} log files", files.len());
        for f in &files {
            println!("  - {} ({} bytes)", f.name, f.size);
        }
    }
}
