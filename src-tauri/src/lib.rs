// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod modules;
mod infrastructure;

use modules::{ProfileService, BrowserLauncher, BrowserManager, GroupService, TagService, RecycleBinService, ProxyService, RecycledProfile, ConfigWriter, ProxyBridgeManager, ProxyBridgeConfig, BridgeStats};
use modules::profile::{CreateProfileDto, UpdateProfileDto, Profile, ProfileStatus};
use modules::group::{CreateGroupDto, UpdateGroupDto, Group};
use modules::tag::{CreateTagDto, UpdateTagDto, Tag};  // ✅ V5 解锁
use modules::proxy::{CreateProxyDto, UpdateProxyDto, Proxy};  // ✅ V5 升级
use infrastructure::init_database;
use tauri::{Manager, State, Emitter};
use std::sync::Arc;
use tokio::sync::Mutex;
use sqlx::SqlitePool;
use sqlx::Row;
use std::collections::HashMap;
use std::path::PathBuf;

use std::error::Error;

#[cfg(windows)]
use windows::Win32::Foundation::{HWND, LPARAM, BOOL};

#[cfg(windows)]
use windows::Win32::UI::WindowsAndMessaging::{
    EnumWindows, GetWindowThreadProcessId, ShowWindow, SetWindowPos, GetSystemMetrics,
    SW_HIDE, SW_SHOW, SW_RESTORE,
    HWND_TOP,
    SWP_NOZORDER,
    SM_CXSCREEN, SM_CYSCREEN,
};

/// 应用状态
pub struct AppState {
    profile_service: Arc<Mutex<ProfileService>>,
    group_service: Arc<Mutex<GroupService>>,
    tag_service: Arc<Mutex<TagService>>,  // ✅ V5 解锁
    recycle_bin_service: Arc<Mutex<RecycleBinService>>,  // ✅ V5 解锁
    proxy_service: Arc<Mutex<ProxyService>>,  // ✅ V5 升级
    extension_service: Arc<Mutex<modules::ExtensionService>>,  // ✅ 扩展管理
    proxy_bridge_manager: Arc<ProxyBridgeManager>,  // ✅ P3 代理桥接
    pool: SqlitePool,
    browser_manager: Arc<BrowserManager>,
    app_data_dir: PathBuf,
}

async fn get_setting(pool: &SqlitePool, key: &str) -> Result<Option<String>, String> {
    sqlx::query_scalar::<_, String>("SELECT value FROM settings WHERE key = ?")
        .bind(key)
        .fetch_optional(pool)
        .await
        .map_err(|e| e.to_string())
}

async fn set_setting(pool: &SqlitePool, key: &str, value: &str) -> Result<(), String> {
    sqlx::query(
        "INSERT INTO settings(key, value, updated_at) VALUES(?, ?, ?) \
        ON CONFLICT(key) DO UPDATE SET value = excluded.value, updated_at = excluded.updated_at",
    )
    .bind(key)
    .bind(value)
    .bind(chrono::Utc::now().to_rfc3339())
    .execute(pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(())
}

/// 获取单个设置值
#[tauri::command]
async fn get_setting_value(key: String, state: State<'_, AppState>) -> Result<Option<String>, String> {
    get_setting(&state.pool, &key).await
}

/// 设置单个设置值
#[tauri::command]
async fn set_setting_value(key: String, value: String, state: State<'_, AppState>) -> Result<(), String> {
    // 特殊键值校验
    match key.as_str() {
        "kernel_path" => {
            modules::settings::validate_kernel_path(&value)?;
        },
        "user_data_dir" => {
            modules::settings::validate_user_data_dir(&value)?;
        },
        _ => {}
    }
    
    set_setting(&state.pool, &key, &value).await
}

/// 获取所有设置
#[tauri::command]
async fn get_all_settings(state: State<'_, AppState>) -> Result<HashMap<String, String>, String> {
    let rows = sqlx::query("SELECT key, value FROM settings")
        .fetch_all(&state.pool)
        .await
        .map_err(|e| e.to_string())?;

    let mut map = HashMap::new();
    for row in rows {
        let key: String = row.try_get("key").map_err(|e| e.to_string())?;
        let value: String = row.try_get("value").map_err(|e| e.to_string())?;
        map.insert(key, value);
    }
    Ok(map)
}

fn proxy_to_arg(profile: &Profile) -> Option<String> {
    let proxy = profile.proxy.as_ref()?;
    let scheme = match proxy.r#type {
        modules::profile::ProxyType::Http => "http",
        modules::profile::ProxyType::Https => "https",
        modules::profile::ProxyType::Socks5 => "socks5",
    };
    Some(format!("{}://{}:{}", scheme, proxy.host, proxy.port))
}

fn profile_user_data_dir(base_dir: &PathBuf, profile_id: &str) -> PathBuf {
    base_dir.join("profiles").join(profile_id)
}

async fn do_launch_browser(profile_id: String, state: &AppState) -> Result<(), String> {
    // 检查是否已在运行
    if state.browser_manager.is_running(&profile_id).await {
        return Ok(()); // 幂等性：已在运行则直接返回成功
    }

    // 获取启动许可（限流）
    let semaphore = state.browser_manager.get_launch_permit();
    let _permit = semaphore
        .acquire()
        .await
        .map_err(|e| format!("获取启动许可失败: {}", e))?;

    let kernel_path = get_setting(&state.pool, "kernel_path")
        .await?
        .unwrap_or_default();
    if kernel_path.trim().is_empty() {
        return Err("浏览器内核路径未设置，请先在设置中配置".to_string());
    }

    let user_data_dir_setting = get_setting(&state.pool, "user_data_dir")
        .await?
        .unwrap_or_default();
    let base_user_data_dir = if user_data_dir_setting.trim().is_empty() {
        state.app_data_dir.clone()
    } else {
        PathBuf::from(user_data_dir_setting)
    };

    let profile_dir = profile_user_data_dir(&base_user_data_dir, &profile_id);
    std::fs::create_dir_all(&profile_dir)
        .map_err(|e| format!("创建用户数据目录失败: {}", e))?;

    let profile = {
        let service = state.profile_service.lock().await;
        service.get_profile(&profile_id)
            .await
            .map_err(|e| e.to_string())?
    };

    // ✅ P0: 写入配置文件（bm_fingerprint.json / bm_cloud.json）
    ConfigWriter::setup_profile_configs(
        &profile_dir,
        &profile.id,
        &profile.name,
        &profile.group,
        &profile.fingerprint,
    ).map_err(|e| {
        let error_msg = format!("写入配置文件失败: {}", e);
        state.browser_manager.emit_error(profile_id.clone(), error_msg.clone());
        error_msg
    })?;

    // ✅ 获取 Profile 启用的扩展，生成 --load-extension 参数
    let extensions_dir = state.app_data_dir.join("Extensions");
    let load_extension_arg = {
        let extension_service = state.extension_service.lock().await;
        extension_service.build_load_extension_arg(&extensions_dir, &profile_id)
            .await
            .map_err(|e| format!("获取扩展列表失败: {}", e))?
    };

    // ✅ P3: 代理桥接 - 解决带密码 SOCKS5 代理问题
    let proxy_arg = if let Some(ref proxy_config) = profile.proxy {
        let proxy_type = match proxy_config.r#type {
            modules::profile::ProxyType::Http => "http",
            modules::profile::ProxyType::Https => "https",
            modules::profile::ProxyType::Socks5 => "socks5",
        };
        
        // 检查是否需要代理桥接（带密码的 SOCKS5）
        if ProxyBridgeManager::needs_bridge(proxy_type, &proxy_config.username, &proxy_config.password) {
            // 启动代理桥接
            let bridge_config = ProxyBridgeConfig {
                upstream_host: proxy_config.host.clone(),
                upstream_port: proxy_config.port,
                upstream_type: proxy_type.to_string(),
                username: proxy_config.username.clone(),
                password: proxy_config.password.clone(),
                enable_udp: true, // 启用 UDP 支持（用于 WebRTC）
            };
            
            let local_addr = state.proxy_bridge_manager
                .start_bridge(&profile_id, bridge_config)
                .await
                .map_err(|e| {
                    let error_msg = format!("启动代理桥接失败: {}", e);
                    state.browser_manager.emit_error(profile_id.clone(), error_msg.clone());
                    error_msg
                })?;
            
            tracing::info!(
                profile_id = %profile_id,
                local_addr = %local_addr,
                upstream = %format!("{}:{}", proxy_config.host, proxy_config.port),
                "代理桥接已启动，使用本地代理"
            );
            
            Some(local_addr)
        } else {
            // 直接使用原始代理地址
            Some(format!("{}://{}:{}", proxy_type, proxy_config.host, proxy_config.port))
        }
    } else {
        None
    };

    let launcher = BrowserLauncher::new(PathBuf::from(kernel_path));
    let child = launcher
        .launch(&profile_id, &profile_dir, proxy_arg.as_deref(), load_extension_arg.as_deref())
        .map_err(|e| {
            let error_msg = format!("启动浏览器失败: {}", e);
            state.browser_manager.emit_error(profile_id.clone(), error_msg.clone());
            error_msg
        })?;

    // 注册进程到 BrowserManager
    state.browser_manager.register_process(profile_id.clone(), child).await?;

    // 更新数据库状态
    {
        let service = state.profile_service.lock().await;
        service.update_status(&profile_id, ProfileStatus::Running)
            .await
            .map_err(|e| e.to_string())?;
    }

    Ok(())
}

async fn do_stop_browser(profile_id: String, state: &AppState) -> Result<(), String> {
    use tracing::{info, warn, debug};
    
    info!(profile_id = %profile_id, "开始停止浏览器");
    
    // ✅ P3: 停止代理桥接（如果有）
    if let Err(e) = state.proxy_bridge_manager.stop_bridge(&profile_id).await {
        warn!(profile_id = %profile_id, error = %e, "停止代理桥接失败");
    }
    
    // 从 BrowserManager 注销进程
    let child_opt = state.browser_manager.unregister_process(&profile_id).await?;

    // 如果有 child，则优雅关闭
    if let Some(mut child) = child_opt {
        let pid = child.id();
        debug!(pid = pid, "发送关闭信号");
        
        // 使用 spawn_blocking 包装同步等待
        let wait_result = tokio::task::spawn_blocking(move || {
            // 尝试等待进程，设置超时通过 try_wait 轮询实现
            for _ in 0..30 { // 最多等待3秒 (100ms * 30)
                match child.try_wait() {
                    Ok(Some(status)) => return Ok((child, Some(status))),
                    Ok(None) => {
                        std::thread::sleep(std::time::Duration::from_millis(100));
                    }
                    Err(e) => return Err((child, e)),
                }
            }
            // 超时，返回 child 以便后续强制 kill
            Ok((child, None))
        }).await.map_err(|e| format!("等待任务失败: {}", e))?;

        match wait_result {
            Ok((_, Some(status))) => {
                info!(
                    pid = pid,
                    exit_code = ?status.code(),
                    "浏览器进程已正常退出"
                );
            }
            Ok((mut child, None)) => {
                // 超时，强制kill
                warn!(pid = pid, "进程未在3秒内退出，执行强制终止");
                child.kill().map_err(|e| format!("停止进程失败: {}", e))?;
                let _ = child.wait();
            }
            Err((_, e)) => {
                warn!(pid = pid, error = %e, "等待进程退出失败");
            }
        }
    }

    // 更新数据库状态
    {
        let service = state.profile_service.lock().await;
        service.update_status(&profile_id, ProfileStatus::Stopped)
            .await
            .map_err(|e| e.to_string())?;
    }

    info!(profile_id = %profile_id, "浏览器停止完成");
    Ok(())
}

/// 获取所有环境（前端期望命令名）
#[tauri::command]
async fn get_profiles(state: State<'_, AppState>) -> Result<Vec<Profile>, String> {
    let service = state.profile_service.lock().await;
    service.list_profiles()
        .await
        .map_err(|e| e.to_string())
}

/// 获取所有环境（兼容旧命令名）
#[tauri::command]
async fn list_profiles(state: State<'_, AppState>) -> Result<Vec<Profile>, String> {
    get_profiles(state).await
}

/// 获取单个环境
#[tauri::command]
async fn get_profile(id: String, state: State<'_, AppState>) -> Result<Profile, String> {
    let service = state.profile_service.lock().await;
    service.get_profile(&id)
        .await
        .map_err(|e| e.to_string())
}

/// 创建环境
#[tauri::command]
async fn create_profile(
    data: CreateProfileDto,
    state: State<'_, AppState>,
    app: tauri::AppHandle,
) -> Result<Profile, String> {
    let service = state.profile_service.lock().await;
    let profile = service.create_profile(data)
        .await
        .map_err(|e| e.to_string())?;
    
    // 发射 profile:created 事件
    let _ = app.emit("profile:created", &profile);
    
    Ok(profile)
}

/// 更新环境
#[tauri::command]
async fn update_profile(
    id: String,
    data: UpdateProfileDto,
    state: State<'_, AppState>,
    app: tauri::AppHandle,
) -> Result<Profile, String> {
    let service = state.profile_service.lock().await;
    let profile = service.update_profile(&id, data)
        .await
        .map_err(|e| e.to_string())?;
    
    // 发射 profile:updated 事件
    let _ = app.emit("profile:updated", &profile);
    
    Ok(profile)
}

/// 删除环境
#[tauri::command]
async fn delete_profile(
    id: String,
    state: State<'_, AppState>,
    app: tauri::AppHandle,
) -> Result<(), String> {
    let service = state.profile_service.lock().await;
    service.delete_profile(&id)
        .await
        .map_err(|e| e.to_string())?;
    
    // 发射 profile:deleted 事件
    #[derive(serde::Serialize, Clone)]
    struct DeletedEvent {
        id: String,
    }
    let _ = app.emit("profile:deleted", DeletedEvent { id });
    
    Ok(())
}

/// 批量删除环境
#[tauri::command]
async fn batch_delete_profiles(ids: Vec<String>, state: State<'_, AppState>) -> Result<modules::BatchResult, String> {
    let service = state.profile_service.lock().await;
    let mut results = Vec::new();

    for id in ids {
        let result = match service.delete_profile(&id).await {
            Ok(_) => modules::BatchItemResult::success(id.clone()),
            Err(e) => modules::BatchItemResult::failure(id.clone(), e.to_string()),
        };
        results.push(result);
    }

    Ok(modules::BatchResult::from_results(results))
}

/// 启动浏览器
#[tauri::command]
async fn launch_browser(profile_id: String, state: State<'_, AppState>) -> Result<(), String> {
    do_launch_browser(profile_id, state.inner()).await
}

/// 停止浏览器
#[tauri::command]
async fn stop_browser(profile_id: String, state: State<'_, AppState>) -> Result<(), String> {
    do_stop_browser(profile_id, state.inner()).await
}

/// 批量启动浏览器
#[tauri::command]
async fn batch_launch_browsers(profile_ids: Vec<String>, state: State<'_, AppState>) -> Result<modules::BatchResult, String> {
    let mut results = Vec::new();

    for id in profile_ids {
        let result = match do_launch_browser(id.clone(), state.inner()).await {
            Ok(_) => modules::BatchItemResult::success(id.clone()),
            Err(e) => modules::BatchItemResult::failure(id.clone(), e),
        };
        results.push(result);
    }

    Ok(modules::BatchResult::from_results(results))
}

/// 批量停止浏览器
#[tauri::command]
async fn batch_stop_browsers(profile_ids: Vec<String>, state: State<'_, AppState>) -> Result<modules::BatchResult, String> {
    let mut results = Vec::new();

    for id in profile_ids {
        let result = match do_stop_browser(id.clone(), state.inner()).await {
            Ok(_) => modules::BatchItemResult::success(id.clone()),
            Err(e) => modules::BatchItemResult::failure(id.clone(), e),
        };
        results.push(result);
    }

    Ok(modules::BatchResult::from_results(results))
}

/// 批量移动环境到指定分组
#[tauri::command]
async fn batch_move_to_group(
    profile_ids: Vec<String>,
    target_group_id: String,
    state: State<'_, AppState>
) -> Result<modules::BatchResult, String> {
    let mut results = Vec::new();

    for id in profile_ids {
        let result = {
            let service = state.profile_service.lock().await;
            match service.update_profile(&id, UpdateProfileDto {
                name: None,
                group: Some(target_group_id.clone()),
                fingerprint: None,
                proxy: None,
                remark: None,
                preferences: None,
            }).await {
                Ok(_) => modules::BatchItemResult::success(id.clone()),
                Err(e) => modules::BatchItemResult::failure(id.clone(), e.to_string()),
            }
        };
        results.push(result);
    }

    Ok(modules::BatchResult::from_results(results))
}

/// 批量复制环境
#[tauri::command]
async fn batch_duplicate_profiles(
    profile_ids: Vec<String>,
    state: State<'_, AppState>
) -> Result<modules::BatchResult, String> {
    let mut results = Vec::new();

    for id in profile_ids {
        let result = {
            let service = state.profile_service.lock().await;
            match service.get_profile(&id).await {
                Ok(source_profile) => {
                    let copy_name = format!("{} (副本)", source_profile.name);
                    match service.create_profile(CreateProfileDto {
                        name: copy_name,
                        group: source_profile.group,
                        fingerprint: source_profile.fingerprint,
                        proxy: source_profile.proxy,
                        remark: source_profile.remark,
                        preferences: source_profile.preferences,
                    }).await {
                        Ok(new_profile) => modules::BatchItemResult::success(new_profile.id),
                        Err(e) => modules::BatchItemResult::failure(id.clone(), e.to_string()),
                    }
                },
                Err(e) => modules::BatchItemResult::failure(id.clone(), e.to_string()),
            }
        };
        results.push(result);
    }

    Ok(modules::BatchResult::from_results(results))
}

/// 获取所有分组
#[tauri::command]
async fn get_groups(state: State<'_, AppState>) -> Result<Vec<Group>, String> {
    let service = state.group_service.lock().await;
    service.list_groups().await
}

/// 获取单个分组
#[tauri::command]
async fn get_group(id: String, state: State<'_, AppState>) -> Result<Group, String> {
    let service = state.group_service.lock().await;
    service.get_group(&id).await
}

/// 创建分组
#[tauri::command]
async fn create_group(
    data: CreateGroupDto,
    state: State<'_, AppState>,
    app: tauri::AppHandle,
) -> Result<Group, String> {
    let service = state.group_service.lock().await;
    let group = service.create_group(data).await?;
    
    // 发射 group:created 事件
    let _ = app.emit("group:created", &group);
    
    Ok(group)
}

/// 更新分组
#[tauri::command]
async fn update_group(
    id: String,
    data: UpdateGroupDto,
    state: State<'_, AppState>,
    app: tauri::AppHandle,
) -> Result<Group, String> {
    let service = state.group_service.lock().await;
    let group = service.update_group(&id, data).await?;
    
    // 发射 group:updated 事件
    let _ = app.emit("group:updated", &group);
    
    Ok(group)
}

/// 删除分组
#[tauri::command]
async fn delete_group(
    id: String,
    state: State<'_, AppState>,
    app: tauri::AppHandle,
) -> Result<(), String> {
    let service = state.group_service.lock().await;
    service.delete_group(&id).await?;
    
    // 发射 group:deleted 事件
    #[derive(serde::Serialize, Clone)]
    struct DeletedEvent {
        id: String,
    }
    let _ = app.emit("group:deleted", DeletedEvent { id });
    
    Ok(())
}

// ============================================================================
// Fingerprint Commands - 指纹生成
// ============================================================================

/// 生成随机指纹
#[tauri::command]
async fn generate_random_fingerprint(
    profile_id: String,
    platform: Option<String>,
    browser_version: Option<String>,
    state: State<'_, AppState>,
) -> Result<modules::config_writer::FingerprintFileConfig, String> {
    use modules::FingerprintGenerator;
    
    // 获取模板文件路径
    let template_path = state.app_data_dir.join("data").join("templates").join("device_templates.json");
    
    // 如果模板文件不存在，从内嵌资源复制
    if !template_path.exists() {
        std::fs::create_dir_all(template_path.parent().unwrap())
            .map_err(|e| format!("创建模板目录失败: {}", e))?;
        
        // 这里应该从资源文件复制，暂时返回错误提示
        return Err(format!("模板文件不存在: {:?}", template_path));
    }
    
    // 创建生成器并生成指纹
    let generator = FingerprintGenerator::new(template_path)
        .map_err(|e| format!("创建指纹生成器失败: {}", e))?;
    
    // 传递平台和浏览器版本参数
    let platform_str = platform.as_deref();
    let browser_version_str = browser_version.as_deref();
    
    let fingerprint = generator.generate(&profile_id, platform_str, browser_version_str);
    
    Ok(fingerprint)
}

/// 获取设备模板列表
#[tauri::command]
async fn get_template_list(
    state: State<'_, AppState>,
) -> Result<Vec<TemplateInfo>, String> {
    use modules::fingerprint::TemplateManager;
    
    let template_path = state.app_data_dir.join("data").join("templates").join("device_templates.json");
    
    if !template_path.exists() {
        return Err(format!("模板文件不存在: {:?}", template_path));
    }
    
    let manager = TemplateManager::load_from_file(template_path)
        .map_err(|e| format!("加载模板失败: {}", e))?;
    
    let templates = manager.get_all_templates();
    let template_infos: Vec<TemplateInfo> = templates
        .iter()
        .map(|t| TemplateInfo {
            id: t.id.clone(),
            description: t.description.clone(),
            weight: t.weight,
            os_name: t.os.name.clone(),
            os_version: t.os.version.clone(),
            cpu_vendor: t.cpu.vendor.clone(),
            gpu_vendor: t.gpu.vendor.clone(),
        })
        .collect();
    
    Ok(template_infos)
}

/// 校验指纹配置
#[tauri::command]
async fn validate_fingerprint(
    config: modules::config_writer::FingerprintFileConfig,
) -> Result<modules::fingerprint::ValidationResult, String> {
    use modules::fingerprint::FingerprintValidator;
    
    let result = FingerprintValidator::validate(&config);
    Ok(result)
}

/// 模板信息（简化版，用于前端显示）
#[derive(serde::Serialize, Clone)]
struct TemplateInfo {
    id: String,
    description: String,
    weight: f32,
    os_name: String,
    os_version: String,
    cpu_vendor: String,
    gpu_vendor: String,
}

// ==================== Tag IPC Commands ====================
// ✅ V5 解锁 - Tag 功能已实现

/// 获取所有标签
#[tauri::command]
async fn get_tags(state: State<'_, AppState>) -> Result<Vec<Tag>, String> {
    let service = state.tag_service.lock().await;
    service.list_tags().await.map_err(|e| e.to_string())
}

/// 创建标签
#[tauri::command]
async fn create_tag(
    data: CreateTagDto,
    state: State<'_, AppState>,
    app: tauri::AppHandle,
) -> Result<Tag, String> {
    let service = state.tag_service.lock().await;
    let tag = service.create_tag(data).await.map_err(|e| e.to_string())?;
    
    // 发射 tag:created 事件
    let _ = app.emit("tag:created", &tag);
    
    Ok(tag)
}

/// 更新标签
#[tauri::command]
async fn update_tag(
    id: String,
    data: UpdateTagDto,
    state: State<'_, AppState>,
    app: tauri::AppHandle,
) -> Result<Tag, String> {
    let service = state.tag_service.lock().await;
    let tag = service.update_tag(&id, data).await.map_err(|e| e.to_string())?;
    
    // 发射 tag:updated 事件
    let _ = app.emit("tag:updated", &tag);
    
    Ok(tag)
}

/// 删除标签
#[tauri::command]
async fn delete_tag(
    id: String,
    state: State<'_, AppState>,
    app: tauri::AppHandle,
) -> Result<(), String> {
    let service = state.tag_service.lock().await;
    service.delete_tag(&id).await.map_err(|e| e.to_string())?;
    
    // 发射 tag:deleted 事件
    #[derive(serde::Serialize, Clone)]
    struct DeletedEvent {
        id: String,
    }
    let _ = app.emit("tag:deleted", DeletedEvent { id });
    
    Ok(())
}

// ==================== RecycleBin IPC Commands ====================
// ✅ V5 解锁 - RecycleBin 功能已实现

/// 获取回收站列表
#[tauri::command]
async fn get_recycle_bin(state: State<'_, AppState>) -> Result<Vec<RecycledProfile>, String> {
    let service = state.recycle_bin_service.lock().await;
    service.list_recycled().await.map_err(|e| e.to_string())
}

/// 恢复窗口
#[tauri::command]
async fn restore_profile(id: String, state: State<'_, AppState>) -> Result<(), String> {
    let service = state.recycle_bin_service.lock().await;
    service.restore_profile(&id).await.map_err(|e| e.to_string())
}

/// 批量恢复窗口
#[tauri::command]
async fn batch_restore_profiles(ids: Vec<String>, state: State<'_, AppState>) -> Result<modules::BatchResult, String> {
    let service = state.recycle_bin_service.lock().await;
    service.batch_restore_profiles(ids).await.map_err(|e| e.to_string())
}

/// 永久删除窗口
#[tauri::command]
async fn permanently_delete_profile(id: String, state: State<'_, AppState>) -> Result<(), String> {
    let service = state.recycle_bin_service.lock().await;
    service.permanently_delete(&id).await.map_err(|e| e.to_string())
}

/// 批量永久删除窗口
#[tauri::command]
async fn batch_permanently_delete_profiles(ids: Vec<String>, state: State<'_, AppState>) -> Result<modules::BatchResult, String> {
    let service = state.recycle_bin_service.lock().await;
    service.batch_permanently_delete(ids).await.map_err(|e| e.to_string())
}

/// 清空回收站
#[tauri::command]
async fn empty_recycle_bin(state: State<'_, AppState>) -> Result<u64, String> {
    let service = state.recycle_bin_service.lock().await;
    service.empty_recycle_bin().await.map_err(|e| e.to_string())
}

// ==================== Proxy IPC Commands ====================
// ✅ V5 升级 - Proxy 功能已实现（基于数据库表）

/// 获取代理列表
#[tauri::command]
async fn get_proxies(state: State<'_, AppState>) -> Result<Vec<Proxy>, String> {
    let service = state.proxy_service.lock().await;
    service.list_proxies().await.map_err(|e| e.to_string())
}

/// 获取单个代理
#[tauri::command]
async fn get_proxy(id: String, state: State<'_, AppState>) -> Result<Proxy, String> {
    let service = state.proxy_service.lock().await;
    service.get_proxy(&id).await.map_err(|e| e.to_string())
}

/// 创建代理
#[tauri::command]
async fn create_proxy(
    data: CreateProxyDto,
    state: State<'_, AppState>,
    app: tauri::AppHandle,
) -> Result<Proxy, String> {
    let service = state.proxy_service.lock().await;
    let proxy = service.create_proxy(data).await.map_err(|e| e.to_string())?;
    
    // 发射 proxy:created 事件
    let _ = app.emit("proxy:created", &proxy);
    
    Ok(proxy)
}

/// 更新代理
#[tauri::command]
async fn update_proxy(
    id: String,
    data: UpdateProxyDto,
    state: State<'_, AppState>,
    app: tauri::AppHandle,
) -> Result<Proxy, String> {
    let service = state.proxy_service.lock().await;
    let proxy = service.update_proxy(&id, data).await.map_err(|e| e.to_string())?;
    
    // 发射 proxy:updated 事件
    let _ = app.emit("proxy:updated", &proxy);
    
    Ok(proxy)
}

/// 删除代理
#[tauri::command]
async fn delete_proxy(
    id: String,
    state: State<'_, AppState>,
    app: tauri::AppHandle,
) -> Result<(), String> {
    let service = state.proxy_service.lock().await;
    service.delete_proxy(&id).await.map_err(|e| e.to_string())?;
    
    // 发射 proxy:deleted 事件
    #[derive(serde::Serialize, Clone)]
    struct DeletedEvent {
        id: String,
    }
    let _ = app.emit("proxy:deleted", DeletedEvent { id });
    
    Ok(())
}

/// 检测代理 - 使用新的 ProxyChecker（支持 HTTP/HTTPS/SOCKS5 + IP 地理位置）
#[tauri::command]
async fn test_proxy(id: String, state: State<'_, AppState>) -> Result<modules::ProxyCheckResult, String> {
    let proxy = {
        let service = state.proxy_service.lock().await;
        service.get_proxy(&id).await.map_err(|e| e.to_string())?
    };
    
    let checker = modules::ProxyChecker::new().with_timeout(10);
    let result = checker.check_proxy(
        &id,
        &proxy.proxy_type,
        &proxy.host,
        &proxy.port,
        proxy.username.as_deref(),
        proxy.password.as_deref(),
    ).await;
    
    // 更新数据库中的检测结果
    {
        let service = state.proxy_service.lock().await;
        let status = if result.success { "active" } else { "error" };
        let _ = service.update_test_result(
            &id,
            result.ip.as_deref(),
            result.location.as_deref(),
            status,
        ).await;
    }
    
    Ok(result)
}

/// 直接检测代理配置 (未保存前)
#[tauri::command]
async fn test_proxy_config(
    proxy_type: String,
    host: String, 
    port: String,
    username: Option<String>,
    password: Option<String>,
) -> Result<modules::ProxyCheckResult, String> {
    let checker = modules::ProxyChecker::new().with_timeout(10);
    let result = checker.check_proxy(
        "temp",
        &proxy_type,
        &host,
        &port,
        username.as_deref(),
        password.as_deref(),
    ).await;
    
    Ok(result)
}

/// 批量检测代理
#[tauri::command]
async fn batch_test_proxies(
    ids: Vec<String>,
    state: State<'_, AppState>,
) -> Result<Vec<modules::ProxyCheckResult>, String> {
    let proxies: Vec<_> = {
        let service = state.proxy_service.lock().await;
        let mut result = Vec::new();
        for id in &ids {
            if let Ok(proxy) = service.get_proxy(id).await {
                result.push(proxy);
            }
        }
        result
    };
    
    let checker = modules::ProxyChecker::new().with_timeout(10);
    
    // 逐个检测（避免生命周期问题）
    let mut results = Vec::new();
    for proxy in &proxies {
        let result = checker.check_proxy(
            &proxy.id,
            &proxy.proxy_type,
            &proxy.host,
            &proxy.port,
            proxy.username.as_deref(),
            proxy.password.as_deref(),
        ).await;
        results.push(result);
    }
    
    // 更新数据库中的检测结果
    {
        let service = state.proxy_service.lock().await;
        for result in &results {
            let status = if result.success { "active" } else { "error" };
            let _ = service.update_test_result(
                &result.proxy_id,
                result.ip.as_deref(),
                result.location.as_deref(),
                status,
            ).await;
        }
    }
    
    Ok(results)
}

/// 检测所有代理
#[tauri::command]
async fn test_all_proxies(state: State<'_, AppState>) -> Result<Vec<modules::ProxyCheckResult>, String> {
    let proxies = {
        let service = state.proxy_service.lock().await;
        service.list_proxies().await.map_err(|e| e.to_string())?
    };
    
    let checker = modules::ProxyChecker::new().with_timeout(10);
    
    // 逐个检测
    let mut results = Vec::new();
    for proxy in &proxies {
        let result = checker.check_proxy(
            &proxy.id,
            &proxy.proxy_type,
            &proxy.host,
            &proxy.port,
            proxy.username.as_deref(),
            proxy.password.as_deref(),
        ).await;
        results.push(result);
    }
    
    // 更新数据库中的检测结果
    {
        let service = state.proxy_service.lock().await;
        for result in &results {
            let status = if result.success { "active" } else { "error" };
            let _ = service.update_test_result(
                &result.proxy_id,
                result.ip.as_deref(),
                result.location.as_deref(),
                status,
            ).await;
        }
    }
    
    Ok(results)
}

/// 设置代理自动检测状态
#[tauri::command]
async fn set_proxy_auto_check(
    id: String,
    enabled: bool,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let service = state.proxy_service.lock().await;
    service.update_proxy(&id, modules::proxy::UpdateProxyDto {
        name: None,
        proxy_type: None,
        host: None,
        port: None,
        source: None,
        tag: None,
        username: None,
        password: None,
        ip_address: None,
        location: None,
        remark: None,
        auto_check: Some(enabled),
        expire_at: None,
        bind_window: None,
        status: None,
    }).await.map_err(|e| e.to_string())?;
    
    Ok(())
}

// ==================== Logger IPC Commands ====================
// ✅ 日志系统 - 提供前端日志查看功能

/// 获取日志文件列表
#[tauri::command]
async fn get_log_files() -> Result<Vec<modules::LogFileInfo>, String> {
    Ok(modules::Logger::list_log_files())
}

/// 读取日志文件内容
#[tauri::command]
async fn read_log_file(filename: String, lines: Option<usize>) -> Result<Vec<String>, String> {
    modules::Logger::read_log_file(&filename, lines)
}

/// 读取最新日志文件的最后 n 行
#[tauri::command]
async fn tail_log(lines: u32) -> Result<Vec<String>, String> {
    modules::Logger::tail_latest_log(lines as usize)
}

/// 清理旧日志文件
#[tauri::command]
async fn cleanup_logs(keep_days: u32) -> Result<usize, String> {
    modules::Logger::cleanup_old_logs(keep_days as usize)
}

// ==================== Extension IPC Commands ====================
// ✅ 扩展管理 - 共享目录模式

/// 获取所有扩展
#[tauri::command]
async fn get_extensions(state: State<'_, AppState>) -> Result<Vec<modules::Extension>, String> {
    let service = state.extension_service.lock().await;
    service.list_extensions().await.map_err(|e| e.to_string())
}

/// 获取已安装的扩展
#[tauri::command]
async fn get_installed_extensions(state: State<'_, AppState>) -> Result<Vec<modules::Extension>, String> {
    let service = state.extension_service.lock().await;
    service.list_installed_extensions().await.map_err(|e| e.to_string())
}

/// 获取单个扩展
#[tauri::command]
async fn get_extension(id: String, state: State<'_, AppState>) -> Result<modules::Extension, String> {
    let service = state.extension_service.lock().await;
    service.get_extension(&id).await.map_err(|e| e.to_string())
}

/// 创建/注册扩展
#[tauri::command]
async fn create_extension(
    data: modules::CreateExtensionDto,
    state: State<'_, AppState>,
) -> Result<modules::Extension, String> {
    let service = state.extension_service.lock().await;
    service.create_extension(data).await.map_err(|e| e.to_string())
}

/// 更新扩展
#[tauri::command]
async fn update_extension(
    id: String,
    data: modules::UpdateExtensionDto,
    state: State<'_, AppState>,
) -> Result<modules::Extension, String> {
    let service = state.extension_service.lock().await;
    service.update_extension(&id, data).await.map_err(|e| e.to_string())
}

/// 删除扩展
#[tauri::command]
async fn delete_extension(id: String, state: State<'_, AppState>) -> Result<(), String> {
    let service = state.extension_service.lock().await;
    service.delete_extension(&id).await.map_err(|e| e.to_string())
}

/// 切换扩展启用状态
#[tauri::command]
async fn toggle_extension(
    id: String,
    enabled: bool,
    state: State<'_, AppState>,
) -> Result<modules::Extension, String> {
    let service = state.extension_service.lock().await;
    service.toggle_extension(&id, enabled).await.map_err(|e| e.to_string())
}

/// 扫描扩展目录
#[tauri::command]
async fn scan_extensions(state: State<'_, AppState>) -> Result<Vec<modules::Extension>, String> {
    let extensions_dir = state.app_data_dir.join("Extensions");
    let service = state.extension_service.lock().await;
    service.scan_extensions_dir(&extensions_dir).await.map_err(|e| e.to_string())
}

/// 获取 Profile 启用的扩展
#[tauri::command]
async fn get_profile_extensions(
    profile_id: String,
    state: State<'_, AppState>,
) -> Result<Vec<modules::Extension>, String> {
    let service = state.extension_service.lock().await;
    service.get_profile_extensions(&profile_id).await.map_err(|e| e.to_string())
}

/// 为 Profile 启用扩展
#[tauri::command]
async fn enable_extension_for_profile(
    profile_id: String,
    extension_id: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let service = state.extension_service.lock().await;
    service.enable_extension_for_profile(&profile_id, &extension_id).await.map_err(|e| e.to_string())
}

/// 为 Profile 禁用扩展
#[tauri::command]
async fn disable_extension_for_profile(
    profile_id: String,
    extension_id: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let service = state.extension_service.lock().await;
    service.disable_extension_for_profile(&profile_id, &extension_id).await.map_err(|e| e.to_string())
}


#[cfg(windows)]
struct EnumWindowsContext {
    pids: Vec<u32>,
    hwnds: Vec<HWND>,
}

#[cfg(windows)]
unsafe extern "system" fn enum_windows_callback(hwnd: HWND, lparam: LPARAM) -> BOOL {
    let ctx = &mut *(lparam.0 as *mut EnumWindowsContext);
    let mut pid: u32 = 0;
    let _ = GetWindowThreadProcessId(hwnd, Some(&mut pid));
    if ctx.pids.contains(&pid) {
        ctx.hwnds.push(hwnd);
    }
    BOOL(1)
}

#[cfg(windows)]
fn collect_hwnds_for_pids(pids: &[u32]) -> Vec<HWND> {
    let mut ctx = EnumWindowsContext {
        pids: pids.to_vec(),
        hwnds: Vec::new(),
    };
    unsafe {
        let _ = EnumWindows(
            Some(enum_windows_callback),
            LPARAM((&mut ctx as *mut EnumWindowsContext) as isize),
        );
    }
    ctx.hwnds
}

/// 宫格排列窗口
#[tauri::command]
async fn arrange_windows_grid(columns: u32, state: State<'_, AppState>) -> Result<(), String> {
    #[cfg(not(windows))]
    {
        let _ = (columns, state);
        return Err("windows only".to_string());
    }

    #[cfg(windows)]
    {
        let pids = state.browser_manager.get_running_pids().await;

        if pids.is_empty() {
            return Ok(());
        }

        // 在 spawn_blocking 中执行窗口操作（避免 Send 问题）
        tokio::task::spawn_blocking(move || {
            use modules::window_helper::collect_main_windows_for_pids;
            use std::thread::sleep;
            use std::time::Duration;

            let mut hwnds = Vec::new();
            
            // 简单的重试逻辑
            for _retry in 0..3 {
                hwnds = collect_main_windows_for_pids(&pids);
                if !hwnds.is_empty() {
                    break;
                }
                sleep(Duration::from_secs(1));
            }
            
            if hwnds.is_empty() {
                return Err("未找到可操作的浏览器窗口".to_string());
            }

            let cols = columns.max(1);
            let count = hwnds.len() as u32;
            let rows = (count + cols - 1) / cols;

            let screen_w = unsafe { GetSystemMetrics(SM_CXSCREEN) } as u32;
            let screen_h = unsafe { GetSystemMetrics(SM_CYSCREEN) } as u32;

            let cell_w = (screen_w / cols).max(1);
            let cell_h = (screen_h / rows).max(1);

            for (i, hwnd) in hwnds.iter_mut().enumerate() {
                let i = i as u32;
                let row = i / cols;
                let col = i % cols;
                let x = (col * cell_w) as i32;
                let y = (row * cell_h) as i32;
                unsafe {
                    let _ = ShowWindow(*hwnd, SW_RESTORE);
                    let _ = ShowWindow(*hwnd, SW_SHOW);
                    let _ = SetWindowPos(
                        *hwnd,
                        HWND_TOP,
                        x,
                        y,
                        cell_w as i32,
                        cell_h as i32,
                        SWP_NOZORDER,
                    );
                }
            }
            
            Ok(())
        })
        .await
        .map_err(|e| format!("窗口排列任务失败: {}", e))?
    }
}

/// 高级排列窗口配置
#[derive(Debug, Clone, serde::Deserialize)]
struct ArrangeWindowsConfig {
    monitor_id: i32,
    start_x: i32,
    start_y: i32,
    window_width: i32,
    window_height: i32,
    columns: u32,
    gap_x: i32,
    gap_y: i32,
    order: String,  // "asc" | "desc"
    mode: String,   // "grid" | "diagonal"
}

/// 获取显示器信息
#[tauri::command]
async fn get_monitors() -> Result<Vec<serde_json::Value>, String> {
    #[cfg(not(windows))]
    {
        return Ok(vec![serde_json::json!({
            "id": 0,
            "name": "主显示器",
            "width": 1920,
            "height": 1080,
            "x": 0,
            "y": 0
        })]);
    }

    #[cfg(windows)]
    {
        use windows::Win32::Graphics::Gdi::{
            EnumDisplayDevicesW, EnumDisplaySettingsW, DEVMODEW, DISPLAY_DEVICEW,
            ENUM_CURRENT_SETTINGS,
        };
        use windows::core::PCWSTR;
        
        let mut monitors = Vec::new();
        let mut device_index: u32 = 0;
        
        loop {
            let mut display_device: DISPLAY_DEVICEW = unsafe { std::mem::zeroed() };
            display_device.cb = std::mem::size_of::<DISPLAY_DEVICEW>() as u32;
            
            let result = unsafe {
                EnumDisplayDevicesW(
                    PCWSTR::null(),
                    device_index,
                    &mut display_device,
                    0,
                )
            };
            
            if !result.as_bool() {
                break;
            }
            
            // 只处理活动的显示设备
            if (display_device.StateFlags & 0x00000001) != 0 {  // DISPLAY_DEVICE_ATTACHED_TO_DESKTOP
                // 获取显示设置
                let mut dev_mode: DEVMODEW = unsafe { std::mem::zeroed() };
                dev_mode.dmSize = std::mem::size_of::<DEVMODEW>() as u16;
                
                let settings_result = unsafe {
                    EnumDisplaySettingsW(
                        PCWSTR(display_device.DeviceName.as_ptr()),
                        ENUM_CURRENT_SETTINGS,
                        &mut dev_mode,
                    )
                };
                
                if settings_result.as_bool() {
                    // 获取友好名称（显示器型号）
                    let friendly_name: Vec<u16> = display_device.DeviceString.iter()
                        .take_while(|&&c| c != 0)
                        .copied()
                        .collect();
                    let friendly_name_str = String::from_utf16_lossy(&friendly_name);
                    
                    let display_name = if friendly_name_str.is_empty() {
                        format!("显示器 {}", device_index + 1)
                    } else {
                        friendly_name_str
                    };
                    
                    let (pos_x, pos_y) = unsafe {
                        (
                            dev_mode.Anonymous1.Anonymous2.dmPosition.x,
                            dev_mode.Anonymous1.Anonymous2.dmPosition.y
                        )
                    };
                    
                    monitors.push(serde_json::json!({
                        "id": device_index,
                        "name": display_name,
                        "width": dev_mode.dmPelsWidth,
                        "height": dev_mode.dmPelsHeight,
                        "x": pos_x,
                        "y": pos_y
                    }));
                }
            }
            
            device_index += 1;
            if device_index > 16 {
                break;  // 最多16个显示器
            }
        }
        
        // 如果没有检测到显示器，返回默认
        if monitors.is_empty() {
            let width = unsafe { GetSystemMetrics(SM_CXSCREEN) };
            let height = unsafe { GetSystemMetrics(SM_CYSCREEN) };
            return Ok(vec![serde_json::json!({
                "id": 0,
                "name": "主显示器",
                "width": width,
                "height": height,
                "x": 0,
                "y": 0
            })]);
        }
        
        Ok(monitors)
    }
}

/// 高级排列窗口
#[tauri::command]
async fn arrange_windows_advanced(
    config: ArrangeWindowsConfig,
    state: State<'_, AppState>,
) -> Result<(), String> {
    #[cfg(not(windows))]
    {
        let _ = (config, state);
        return Err("windows only".to_string());
    }

    #[cfg(windows)]
    {
        let pids = state.browser_manager.get_running_pids().await;

        if pids.is_empty() {
            return Ok(());
        }

        tokio::task::spawn_blocking(move || {
            use modules::window_helper::collect_main_windows_for_pids;
            use std::thread::sleep;
            use std::time::Duration;

            let mut hwnds = Vec::new();
            
            for _retry in 0..3 {
                hwnds = collect_main_windows_for_pids(&pids);
                if !hwnds.is_empty() {
                    break;
                }
                sleep(Duration::from_secs(1));
            }
            
            if hwnds.is_empty() {
                return Err("未找到可操作的浏览器窗口".to_string());
            }

            // 根据排序方式调整顺序
            if config.order == "desc" {
                hwnds.reverse();
            }

            let cols = config.columns.max(1) as i32;
            let w = config.window_width.max(120);
            let h = config.window_height.max(40);
            let gap_x = config.gap_x.max(0);
            let gap_y = config.gap_y.max(0);

            if config.mode == "diagonal" {
                // 对角线排列
                let offset = 30; // 每个窗口偏移量
                for (i, hwnd) in hwnds.iter_mut().enumerate() {
                    let x = config.start_x + (i as i32 * offset);
                    let y = config.start_y + (i as i32 * offset);
                    unsafe {
                        let _ = ShowWindow(*hwnd, SW_RESTORE);
                        let _ = ShowWindow(*hwnd, SW_SHOW);
                        let _ = SetWindowPos(
                            *hwnd,
                            HWND_TOP,
                            x,
                            y,
                            w,
                            h,
                            SWP_NOZORDER,
                        );
                    }
                }
            } else {
                // 宫格排列
                for (i, hwnd) in hwnds.iter_mut().enumerate() {
                    let i = i as i32;
                    let row = i / cols;
                    let col = i % cols;
                    let x = config.start_x + col * (w + gap_x);
                    let y = config.start_y + row * (h + gap_y);
                    unsafe {
                        let _ = ShowWindow(*hwnd, SW_RESTORE);
                        let _ = ShowWindow(*hwnd, SW_SHOW);
                        let _ = SetWindowPos(
                            *hwnd,
                            HWND_TOP,
                            x,
                            y,
                            w,
                            h,
                            SWP_NOZORDER,
                        );
                    }
                }
            }
            
            Ok(())
        })
        .await
        .map_err(|e| format!("窗口排列任务失败: {}", e))?
    }
}

/// 隐藏所有窗口 (老板键)
#[tauri::command]
async fn hide_all_windows(state: State<'_, AppState>) -> Result<(), String> {
    #[cfg(not(windows))]
    {
        let _ = state;
        return Err("windows only".to_string());
    }

    #[cfg(windows)]
    {
        let pids = state.browser_manager.get_running_pids().await;

        let hwnds = modules::window_helper::collect_main_windows_for_pids(&pids);
        for hwnd in hwnds {
            unsafe {
                let _ = ShowWindow(hwnd, SW_HIDE);
            }
        }
        Ok(())
    }
}

/// 显示所有窗口
#[tauri::command]
async fn show_all_windows(state: State<'_, AppState>) -> Result<(), String> {
    #[cfg(not(windows))]
    {
        let _ = state;
        return Err("windows only".to_string());
    }

    #[cfg(windows)]
    {
        let pids = state.browser_manager.get_running_pids().await;

        let hwnds = modules::window_helper::collect_main_windows_for_pids(&pids);
        for hwnd in hwnds {
            unsafe {
                let _ = ShowWindow(hwnd, SW_RESTORE);
                let _ = ShowWindow(hwnd, SW_SHOW);
            }
        }
        Ok(())
    }
}

// ==================== Window Management Commands ====================
// ✅ P2 完善 - 窗口管理增强功能

/// 获取所有运行中浏览器的窗口详情
#[tauri::command]
async fn get_window_details(state: State<'_, AppState>) -> Result<Vec<modules::window_helper::WindowDetail>, String> {
    #[cfg(not(windows))]
    {
        let _ = state;
        return Err("窗口管理仅支持 Windows 平台".to_string());
    }

    #[cfg(windows)]
    {
        let pids = state.browser_manager.get_running_pids().await;
        
        // 使用重试机制获取窗口详情
        let details = modules::window_helper::collect_window_details_with_retry(
            &pids, 
            3,    // 最多重试3次
            500   // 每次延迟500ms
        ).await;
        
        Ok(details)
    }
}

/// 获取指定 Profile 的窗口详情
#[tauri::command]
async fn get_profile_window_details(
    profile_id: String,
    state: State<'_, AppState>,
) -> Result<Option<modules::window_helper::WindowDetail>, String> {
    #[cfg(not(windows))]
    {
        let _ = (profile_id, state);
        return Err("窗口管理仅支持 Windows 平台".to_string());
    }

    #[cfg(windows)]
    {
        let pid = match state.browser_manager.get_pid(&profile_id).await {
            Some(p) => p,
            None => return Ok(None),
        };
        
        let details = modules::window_helper::collect_window_details_with_retry(
            &[pid],
            3,
            500
        ).await;
        
        Ok(details.into_iter().next())
    }
}

/// 重命名浏览器窗口标题
#[tauri::command]
async fn rename_browser_window(
    profile_id: String,
    new_title: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    #[cfg(not(windows))]
    {
        let _ = (profile_id, new_title, state);
        return Err("窗口管理仅支持 Windows 平台".to_string());
    }

    #[cfg(windows)]
    {
        let pid = state.browser_manager.get_pid(&profile_id).await
            .ok_or_else(|| format!("Profile {} 未运行", profile_id))?;
        
        let details = modules::window_helper::collect_window_details_with_retry(
            &[pid],
            3,
            500
        ).await;
        
        if let Some(detail) = details.first() {
            modules::window_helper::rename_window(detail.hwnd_ptr, &new_title)?;
            Ok(())
        } else {
            Err(format!("未找到 Profile {} 的窗口", profile_id))
        }
    }
}

/// 置顶浏览器窗口
#[tauri::command]
async fn bring_browser_to_front(
    profile_id: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    #[cfg(not(windows))]
    {
        let _ = (profile_id, state);
        return Err("窗口管理仅支持 Windows 平台".to_string());
    }

    #[cfg(windows)]
    {
        let pid = state.browser_manager.get_pid(&profile_id).await
            .ok_or_else(|| format!("Profile {} 未运行", profile_id))?;
        
        let details = modules::window_helper::collect_window_details_with_retry(
            &[pid],
            3,
            500
        ).await;
        
        if let Some(detail) = details.first() {
            modules::window_helper::bring_window_to_front(detail.hwnd_ptr)?;
            Ok(())
        } else {
            Err(format!("未找到 Profile {} 的窗口", profile_id))
        }
    }
}

/// 最小化浏览器窗口
#[tauri::command]
async fn minimize_browser_window(
    profile_id: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    #[cfg(not(windows))]
    {
        let _ = (profile_id, state);
        return Err("窗口管理仅支持 Windows 平台".to_string());
    }

    #[cfg(windows)]
    {
        let pid = state.browser_manager.get_pid(&profile_id).await
            .ok_or_else(|| format!("Profile {} 未运行", profile_id))?;
        
        let details = modules::window_helper::collect_window_details_with_retry(
            &[pid],
            3,
            500
        ).await;
        
        if let Some(detail) = details.first() {
            modules::window_helper::minimize_window(detail.hwnd_ptr)?;
            Ok(())
        } else {
            Err(format!("未找到 Profile {} 的窗口", profile_id))
        }
    }
}

/// 恢复浏览器窗口
#[tauri::command]
async fn restore_browser_window(
    profile_id: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    #[cfg(not(windows))]
    {
        let _ = (profile_id, state);
        return Err("窗口管理仅支持 Windows 平台".to_string());
    }

    #[cfg(windows)]
    {
        let pid = state.browser_manager.get_pid(&profile_id).await
            .ok_or_else(|| format!("Profile {} 未运行", profile_id))?;
        
        let details = modules::window_helper::collect_window_details_with_retry(
            &[pid],
            3,
            500
        ).await;
        
        if let Some(detail) = details.first() {
            modules::window_helper::restore_window(detail.hwnd_ptr)?;
            Ok(())
        } else {
            Err(format!("未找到 Profile {} 的窗口", profile_id))
        }
    }
}

/// 移动浏览器窗口到指定位置
#[tauri::command]
async fn move_browser_window(
    profile_id: String,
    x: i32,
    y: i32,
    state: State<'_, AppState>,
) -> Result<(), String> {
    #[cfg(not(windows))]
    {
        let _ = (profile_id, x, y, state);
        return Err("窗口管理仅支持 Windows 平台".to_string());
    }

    #[cfg(windows)]
    {
        let pid = state.browser_manager.get_pid(&profile_id).await
            .ok_or_else(|| format!("Profile {} 未运行", profile_id))?;
        
        let details = modules::window_helper::collect_window_details_with_retry(
            &[pid],
            3,
            500
        ).await;
        
        if let Some(detail) = details.first() {
            modules::window_helper::move_window(detail.hwnd_ptr, x, y)?;
            Ok(())
        } else {
            Err(format!("未找到 Profile {} 的窗口", profile_id))
        }
    }
}

/// 调整浏览器窗口大小
#[tauri::command]
async fn resize_browser_window(
    profile_id: String,
    width: i32,
    height: i32,
    state: State<'_, AppState>,
) -> Result<(), String> {
    #[cfg(not(windows))]
    {
        let _ = (profile_id, width, height, state);
        return Err("窗口管理仅支持 Windows 平台".to_string());
    }

    #[cfg(windows)]
    {
        let pid = state.browser_manager.get_pid(&profile_id).await
            .ok_or_else(|| format!("Profile {} 未运行", profile_id))?;
        
        let details = modules::window_helper::collect_window_details_with_retry(
            &[pid],
            3,
            500
        ).await;
        
        if let Some(detail) = details.first() {
            modules::window_helper::resize_window(detail.hwnd_ptr, width, height)?;
            Ok(())
        } else {
            Err(format!("未找到 Profile {} 的窗口", profile_id))
        }
    }
}

/// 设置浏览器窗口位置和大小
#[tauri::command]
async fn set_browser_window_bounds(
    profile_id: String,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    state: State<'_, AppState>,
) -> Result<(), String> {
    #[cfg(not(windows))]
    {
        let _ = (profile_id, x, y, width, height, state);
        return Err("窗口管理仅支持 Windows 平台".to_string());
    }

    #[cfg(windows)]
    {
        let pid = state.browser_manager.get_pid(&profile_id).await
            .ok_or_else(|| format!("Profile {} 未运行", profile_id))?;
        
        let details = modules::window_helper::collect_window_details_with_retry(
            &[pid],
            3,
            500
        ).await;
        
        if let Some(detail) = details.first() {
            modules::window_helper::set_window_bounds(detail.hwnd_ptr, x, y, width, height)?;
            Ok(())
        } else {
            Err(format!("未找到 Profile {} 的窗口", profile_id))
        }
    }
}

// ==================== Proxy Bridge IPC Commands ====================
// ✅ P3 - 代理桥接状态查询

/// 获取代理桥接统计信息
#[tauri::command]
async fn get_proxy_bridge_stats(
    profile_id: String,
    state: State<'_, AppState>,
) -> Result<Option<BridgeStats>, String> {
    Ok(state.proxy_bridge_manager.get_bridge_stats(&profile_id).await)
}

/// 获取所有代理桥接统计信息
#[tauri::command]
async fn get_all_proxy_bridge_stats(
    state: State<'_, AppState>,
) -> Result<HashMap<String, BridgeStats>, String> {
    Ok(state.proxy_bridge_manager.get_all_stats().await)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // 使用新的 Logger 模块初始化日志系统
    let log_config = modules::LoggerConfig::default();
    let _guard = modules::Logger::init(log_config);
    
    tracing::info!("Browser Manager starting...");

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            let app_data_dir = app.path().app_data_dir()?;
            std::fs::create_dir_all(&app_data_dir)?;

            // 初始化模板文件：如果不存在则从内嵌资源复制
            let template_dir = app_data_dir.join("data").join("templates");
            let template_file = template_dir.join("device_templates.json");
            if !template_file.exists() {
                std::fs::create_dir_all(&template_dir)?;
                // 内嵌模板文件内容
                const EMBEDDED_TEMPLATE: &str = include_str!("../data/templates/device_templates.json");
                std::fs::write(&template_file, EMBEDDED_TEMPLATE)?;
                tracing::info!("已初始化设备模板文件: {:?}", template_file);
            }

            let db_path = app_data_dir.join("browser-manager.db");
            let db = tauri::async_runtime::block_on(async { init_database(&db_path).await })
                .map_err(|e| Box::<dyn Error>::from(e))?;

            let pool = db.pool().clone();
            let profile_service = ProfileService::new(pool.clone());
            let group_service = GroupService::new(pool.clone());
            let tag_service = TagService::new(pool.clone());  // ✅ V5 解锁
            let recycle_bin_service = RecycleBinService::new(pool.clone());  // ✅ V5 解锁
            let proxy_service = ProxyService::new(pool.clone());  // ✅ V5 升级
            let extension_service = modules::ExtensionService::new(pool.clone());  // ✅ 扩展管理
            let proxy_bridge_manager = Arc::new(ProxyBridgeManager::new());  // ✅ P3 代理桥接
            
            // 创建 BrowserManager
            let browser_manager = Arc::new(BrowserManager::new(app.handle().clone()));
            
            // 启动进程监控任务
            let pool_arc = Arc::new(pool.clone());
            let manager_clone = Arc::clone(&browser_manager);
            tauri::async_runtime::spawn(async move {
                modules::browser_manager::start_process_monitor(manager_clone, pool_arc).await;
            });

            app.manage(AppState {
                profile_service: Arc::new(Mutex::new(profile_service)),
                group_service: Arc::new(Mutex::new(group_service)),
                tag_service: Arc::new(Mutex::new(tag_service)),  // ✅ V5 解锁
                recycle_bin_service: Arc::new(Mutex::new(recycle_bin_service)),  // ✅ V5 解锁
                proxy_service: Arc::new(Mutex::new(proxy_service)),  // ✅ V5 升级
                extension_service: Arc::new(Mutex::new(extension_service)),  // ✅ 扩展管理
                proxy_bridge_manager,  // ✅ P3 代理桥接
                pool,
                browser_manager,
                app_data_dir,
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Profile commands
            get_profiles,
            list_profiles,
            get_profile,
            create_profile,
            update_profile,
            delete_profile,
            batch_duplicate_profiles,
            // Fingerprint commands - 指纹生成
            generate_random_fingerprint,
            get_template_list,
            validate_fingerprint,
            // Group commands
            get_groups,
            get_group,
            create_group,
            update_group,
            delete_group,
            // Tag commands - ✅ V5 解锁
            get_tags,
            create_tag,
            update_tag,
            delete_tag,
            // RecycleBin commands - ✅ V5 解锁
            get_recycle_bin,
            restore_profile,
            batch_restore_profiles,
            permanently_delete_profile,
            batch_permanently_delete_profiles,
            empty_recycle_bin,
            // Proxy commands - ✅ V5 解锁
            get_proxies,
            get_proxy,
            create_proxy,
            update_proxy,
            delete_proxy,
            test_proxy,
            test_proxy_config,
            batch_test_proxies,
            test_all_proxies,
            set_proxy_auto_check,
            // Settings commands
            get_setting_value,
            set_setting_value,
            get_all_settings,
            // Browser commands
            launch_browser,
            stop_browser,
            batch_launch_browsers,
            batch_stop_browsers,
            batch_move_to_group,
            batch_duplicate_profiles,
            batch_delete_profiles,
            // Window commands
            arrange_windows_grid,
            arrange_windows_advanced,
            get_monitors,
            hide_all_windows,
            show_all_windows,
            // ✅ P2 窗口管理增强
            get_window_details,
            get_profile_window_details,
            rename_browser_window,
            bring_browser_to_front,
            minimize_browser_window,
            restore_browser_window,
            move_browser_window,
            resize_browser_window,
            set_browser_window_bounds,
            // Logger commands - ✅ 日志系统
            get_log_files,
            read_log_file,
            tail_log,
            cleanup_logs,
            // Extension commands - ✅ 扩展管理
            get_extensions,
            get_installed_extensions,
            get_extension,
            create_extension,
            update_extension,
            delete_extension,
            toggle_extension,
            scan_extensions,
            get_profile_extensions,
            enable_extension_for_profile,
            disable_extension_for_profile,
            // Proxy Bridge commands - ✅ P3 代理桥接
            get_proxy_bridge_stats,
            get_all_proxy_bridge_stats,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
