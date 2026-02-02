// Browser Manager - 进程生命周期管理与状态事件
use std::collections::HashMap;
use std::process::Child;
use std::sync::Arc;
use tokio::sync::{Mutex, Semaphore};
use tauri::Emitter;
use serde::Serialize;
use chrono::{DateTime, Utc};
use tracing::{info, warn, error, debug};

/// 进程状态
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ProcessStatus {
    Starting,
    Running,
    Stopping,
    Stopped,
    Error(String),
}

/// 进程信息
#[derive(Debug)]
pub struct ProcessInfo {
    pub profile_id: String,
    pub pid: u32,
    pub status: ProcessStatus,
    pub started_at: DateTime<Utc>,
    pub stopped_at: Option<DateTime<Utc>>,
    pub exit_reason: Option<String>, // 退出原因：manual_stop, crashed, normal_exit
    pub child: Option<Child>,
}

/// Profile 状态变化事件
#[derive(Debug, Clone, Serialize)]
pub struct ProfileStatusChangedEvent {
    #[serde(rename = "profileId")]
    pub profile_id: String,
    pub status: String, // "running" | "stopped" | "error"
    pub timestamp: i64,
}

/// 浏览器错误事件
#[derive(Debug, Clone, Serialize)]
pub struct BrowserErrorEvent {
    #[serde(rename = "profileId")]
    pub profile_id: String,
    pub error: String,
    pub timestamp: i64,
}

/// 事件发射器
pub struct EventEmitter {
    app_handle: tauri::AppHandle,
}

impl EventEmitter {
    pub fn new(app_handle: tauri::AppHandle) -> Self {
        Self { app_handle }
    }
    
    /// 发送状态变化事件
    pub fn emit_status_changed(&self, profile_id: String, status: &str) {
        let event = ProfileStatusChangedEvent {
            profile_id,
            status: status.to_string(),
            timestamp: Utc::now().timestamp_millis(),
        };
        
        if let Err(e) = self.app_handle.emit("profile:status_changed", event) {
            error!(error = %e, "Failed to emit status_changed event");
        }
    }
    
    /// 发送浏览器错误事件
    pub fn emit_browser_error(&self, profile_id: String, error: String) {
        let event = BrowserErrorEvent {
            profile_id,
            error,
            timestamp: Utc::now().timestamp_millis(),
        };
        
        if let Err(e) = self.app_handle.emit("browser:error", event) {
            error!(error = %e, "Failed to emit browser_error event");
        }
    }
}

/// 浏览器管理器
pub struct BrowserManager {
    processes: Arc<Mutex<HashMap<String, ProcessInfo>>>,
    /// 限制同时启动的浏览器数量（防止系统卡死）
    launch_semaphore: Arc<Semaphore>,
    event_emitter: Arc<EventEmitter>,
}

impl BrowserManager {
    pub fn new(app_handle: tauri::AppHandle) -> Self {
        Self {
            processes: Arc::new(Mutex::new(HashMap::new())),
            // 最多同时启动 3 个浏览器
            launch_semaphore: Arc::new(Semaphore::new(3)),
            event_emitter: Arc::new(EventEmitter::new(app_handle)),
        }
    }
    
    /// 检查进程是否已在运行
    pub async fn is_running(&self, profile_id: &str) -> bool {
        let processes = self.processes.lock().await;
        if let Some(info) = processes.get(profile_id) {
            matches!(info.status, ProcessStatus::Running | ProcessStatus::Starting)
        } else {
            false
        }
    }
    
    /// 注册进程（启动时调用）
    pub async fn register_process(
        &self,
        profile_id: String,
        child: Child,
    ) -> Result<(), String> {
        let pid = child.id();
        let mut processes = self.processes.lock().await;
        
        processes.insert(profile_id.clone(), ProcessInfo {
            profile_id: profile_id.clone(),
            pid,
            status: ProcessStatus::Running,
            started_at: Utc::now(),
            stopped_at: None,
            exit_reason: None,
            child: Some(child),
        });
        
        // 发送状态变化事件
        self.event_emitter.emit_status_changed(profile_id, "running");
        
        Ok(())
    }
    
    /// 注销进程（停止时调用）
    pub async fn unregister_process(&self, profile_id: &str) -> Result<Option<Child>, String> {
        let mut processes = self.processes.lock().await;
        
        if let Some(mut info) = processes.remove(profile_id) {
            info.status = ProcessStatus::Stopped;
            info.stopped_at = Some(Utc::now());
            info.exit_reason = Some("manual_stop".to_string());
            
            // 发送状态变化事件
            self.event_emitter.emit_status_changed(profile_id.to_string(), "stopped");
            
            Ok(info.child)
        } else {
            Ok(None)
        }
    }
    
    /// 更新进程状态
    pub async fn update_status(&self, profile_id: &str, status: ProcessStatus) {
        let mut processes = self.processes.lock().await;
        
        if let Some(info) = processes.get_mut(profile_id) {
            info.status = status.clone();
            
            // 发送状态变化事件
            let status_str = match status {
                ProcessStatus::Running => "running",
                ProcessStatus::Stopped => "stopped",
                ProcessStatus::Error(_) => "error",
                _ => return,
            };
            
            self.event_emitter.emit_status_changed(profile_id.to_string(), status_str);
        }
    }
    
    /// 获取启动信号量（用于限流）
    pub fn get_launch_permit(&self) -> Arc<Semaphore> {
        Arc::clone(&self.launch_semaphore)
    }
    
    /// 获取所有运行中的进程 PID
    pub async fn get_running_pids(&self) -> Vec<u32> {
        let processes = self.processes.lock().await;
        processes
            .values()
            .filter(|info| info.status == ProcessStatus::Running)
            .map(|info| info.pid)
            .collect()
    }
    
    /// 获取指定 Profile 的进程 PID
    pub async fn get_pid(&self, profile_id: &str) -> Option<u32> {
        let processes = self.processes.lock().await;
        processes
            .get(profile_id)
            .filter(|info| info.status == ProcessStatus::Running)
            .map(|info| info.pid)
    }
    
    /// 获取所有运行中的 Profile ID 和 PID 映射
    pub async fn get_running_profiles(&self) -> HashMap<String, u32> {
        let processes = self.processes.lock().await;
        processes
            .iter()
            .filter(|(_, info)| info.status == ProcessStatus::Running)
            .map(|(id, info)| (id.clone(), info.pid))
            .collect()
    }
    
    /// 发送错误事件
    pub fn emit_error(&self, profile_id: String, error: String) {
        self.event_emitter.emit_browser_error(profile_id, error);
    }
}

/// 启动进程监控任务（后台任务，监控进程退出）
pub async fn start_process_monitor(
    manager: Arc<BrowserManager>,
    pool: Arc<sqlx::SqlitePool>,
) {
    use tokio::time::{interval, Duration};
    
    tokio::spawn(async move {
        let mut ticker = interval(Duration::from_secs(5));
        
        loop {
            ticker.tick().await;
            
            let mut processes = manager.processes.lock().await;
            let mut to_remove = Vec::new();
            
            for (profile_id, info) in processes.iter_mut() {
                if let Some(child) = &mut info.child {
                    // 检查进程是否退出
                    match child.try_wait() {
                        Ok(Some(status)) => {
                            // 进程已退出
                            let exit_code = status.code();
                            let exit_reason = if let Some(code) = exit_code {
                                if code == 0 {
                                    "normal_exit".to_string()
                                } else {
                                    format!("crashed_with_code_{}", code)
                                }
                            } else {
                                "crashed_unknown".to_string()
                            };
                            
                            info!(
                                profile_id = %profile_id,
                                exit_code = ?exit_code,
                                exit_reason = %exit_reason,
                                "浏览器进程已退出"
                            );
                            
                            // 记录退出信息
                            info.status = ProcessStatus::Stopped;
                            info.stopped_at = Some(Utc::now());
                            info.exit_reason = Some(exit_reason.clone());
                            info.child = None;
                            
                            // 更新数据库状态
                            let profile_id_clone = profile_id.clone();
                            let pool_clone = Arc::clone(&pool);
                            let stopped_at = Utc::now().to_rfc3339();
                            tokio::spawn(async move {
                                let _ = sqlx::query(
                                    "UPDATE profiles SET status = 'stopped', updated_at = ? WHERE id = ?"
                                )
                                .bind(&stopped_at)
                                .bind(&profile_id_clone)
                                .execute(&*pool_clone)
                                .await;
                            });
                            
                            // 发送事件
                            manager.event_emitter.emit_status_changed(
                                profile_id.clone(),
                                "stopped"
                            );
                            
                            // 如果是崩溃，发送错误事件
                            if exit_reason.starts_with("crashed") {
                                manager.event_emitter.emit_browser_error(
                                    profile_id.clone(),
                                    format!("浏览器异常退出: {}", exit_reason)
                                );
                            }
                            
                            to_remove.push(profile_id.clone());
                        }
                        Ok(None) => {
                            // 进程仍在运行
                        }
                        Err(e) => {
                            // 检查失败
                            error!(profile_id = %profile_id, error = %e, "检查进程状态失败");
                        }
                    }
                }
            }
            
            // 清理已退出的进程
            for profile_id in to_remove {
                processes.remove(&profile_id);
            }
        }
    });
}
