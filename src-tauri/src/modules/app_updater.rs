//! 应用自动更新模块
//! 
//! 负责启动器和内核的版本检测、下载、SHA256校验、安装。
//! 所有网络请求通过 Rust reqwest 发起，前端仅通过 Tauri IPC 调用。

use anyhow::{Context, Result};
use futures_util::StreamExt;
use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use std::path::{Path, PathBuf};
use tokio::fs::{self, File};
use tokio::io::AsyncWriteExt;
use tracing::{info, warn, error};

// ==================== 常量 ====================

/// 默认更新服务器地址（生产环境替换）
pub const DEFAULT_UPDATE_SERVER: &str = "http://localhost:8080";

// ==================== 数据结构 ====================

/// 更新组件类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum UpdateComponent {
    Launcher,
    Kernel,
}

/// 下载源
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadSource {
    pub id: i64,
    pub name: String,
    pub url: String,
    pub priority: i32,
    pub region: String,
}

/// 更新信息（后端 API 返回）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateInfo {
    pub has_update: bool,
    #[serde(default)]
    pub version: Option<String>,
    pub current_version: String,
    #[serde(default)]
    pub latest_version: Option<String>,
    #[serde(default)]
    pub release_date: Option<String>,
    #[serde(default)]
    pub release_notes: Option<String>,
    #[serde(default)]
    pub mandatory: Option<bool>,
    #[serde(default)]
    pub min_version: Option<String>,
    #[serde(default)]
    pub file_size: Option<u64>,
    #[serde(default)]
    pub file_hash: Option<String>,
    #[serde(default)]
    pub downloads: Option<Vec<DownloadSource>>,
}

/// API 统一响应
#[derive(Debug, Clone, Deserialize)]
pub struct ApiResponse<T> {
    pub code: i32,
    pub message: String,
    pub data: Option<T>,
}

/// 更新下载进度
#[derive(Debug, Clone, Serialize)]
pub struct UpdateDownloadProgress {
    pub component: String,
    pub downloaded: u64,
    pub total: u64,
    pub speed: u64,
    pub percent: u32,
    pub status: UpdateDownloadStatus,
    pub message: String,
}

/// 下载状态
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum UpdateDownloadStatus {
    Idle,
    Downloading,
    Verifying,
    Extracting,
    Completed,
    Failed,
}

// ==================== 版本检测 ====================

/// 检查启动器更新
pub async fn check_launcher_update(
    server_url: &str,
    current_version: &str,
    platform: &str,
    arch: &str,
) -> Result<UpdateInfo> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()?;

    let url = format!(
        "{}/api/v1/updates/launcher?version={}&platform={}&arch={}",
        server_url, current_version, platform, arch
    );

    info!("Checking launcher update: {}", url);

    let resp: ApiResponse<UpdateInfo> = client
        .get(&url)
        .send()
        .await
        .context("请求更新服务器失败")?
        .json()
        .await
        .context("解析更新响应失败")?;

    if resp.code != 0 {
        return Err(anyhow::anyhow!("服务器返回错误: {}", resp.message));
    }

    resp.data.context("响应数据为空")
}

/// 检查内核更新
pub async fn check_kernel_update(
    server_url: &str,
    current_kernel_version: &str,
    platform: &str,
    arch: &str,
    launcher_version: &str,
) -> Result<UpdateInfo> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()?;

    let url = format!(
        "{}/api/v1/updates/kernel?version={}&platform={}&arch={}&launcher_version={}",
        server_url, current_kernel_version, platform, arch, launcher_version
    );

    info!("Checking kernel update: {}", url);

    let resp: ApiResponse<UpdateInfo> = client
        .get(&url)
        .send()
        .await
        .context("请求更新服务器失败")?
        .json()
        .await
        .context("解析更新响应失败")?;

    if resp.code != 0 {
        return Err(anyhow::anyhow!("服务器返回错误: {}", resp.message));
    }

    resp.data.context("响应数据为空")
}

// ==================== 下载 + SHA256 强制校验 ====================

/// 下载文件并校验 SHA256
/// 返回下载后的文件路径
pub async fn download_and_verify<F>(
    url: &str,
    expected_hash: &str,
    dest_dir: &Path,
    component: UpdateComponent,
    progress_callback: F,
) -> Result<PathBuf>
where
    F: Fn(UpdateDownloadProgress) + Send + 'static,
{
    let client = reqwest::Client::builder()
        .user_agent("ChubaoUpdater/1.0")
        .redirect(reqwest::redirect::Policy::limited(10))
        .timeout(std::time::Duration::from_secs(600))
        .build()?;

    // 提取文件名
    let file_name = url.split('/').last().unwrap_or("download.tmp");
    let dest_path = dest_dir.join(file_name);

    // 确保目录存在
    fs::create_dir_all(dest_dir).await?;

    // 发起请求
    let response = client
        .get(url)
        .send()
        .await
        .context("下载请求失败")?;

    if !response.status().is_success() {
        return Err(anyhow::anyhow!("下载失败，HTTP {}", response.status()));
    }

    let total_size = response.content_length().unwrap_or(0);
    let mut downloaded: u64 = 0;
    let mut stream = response.bytes_stream();
    let mut file = File::create(&dest_path).await?;
    let mut hasher = Sha256::new();
    let start_time = std::time::Instant::now();
    let mut last_emit = std::time::Instant::now();
    let component_str = match component {
        UpdateComponent::Launcher => "launcher",
        UpdateComponent::Kernel => "kernel",
    };

    while let Some(chunk) = stream.next().await {
        let chunk = chunk.context("读取数据块失败")?;
        file.write_all(&chunk).await?;
        hasher.update(&chunk);
        downloaded += chunk.len() as u64;

        // 每 200ms 发送一次进度
        if last_emit.elapsed().as_millis() >= 200 {
            let elapsed = start_time.elapsed().as_secs_f64();
            let speed = if elapsed > 0.0 { (downloaded as f64 / elapsed) as u64 } else { 0 };
            let percent = if total_size > 0 {
                (downloaded as f64 / total_size as f64 * 100.0) as u32
            } else { 0 };

            progress_callback(UpdateDownloadProgress {
                component: component_str.to_string(),
                downloaded,
                total: total_size,
                speed,
                percent,
                status: UpdateDownloadStatus::Downloading,
                message: format!(
                    "下载中 {:.1} MB / {:.1} MB",
                    downloaded as f64 / 1_048_576.0,
                    total_size as f64 / 1_048_576.0
                ),
            });
            last_emit = std::time::Instant::now();
        }
    }

    file.flush().await?;
    drop(file);

    info!("下载完成: {} bytes -> {:?}", downloaded, dest_path);

    // ========== SHA256 强制校验 ==========
    progress_callback(UpdateDownloadProgress {
        component: component_str.to_string(),
        downloaded,
        total: total_size,
        speed: 0,
        percent: 100,
        status: UpdateDownloadStatus::Verifying,
        message: "正在校验文件完整性...".to_string(),
    });

    let hash_result = format!("{:x}", hasher.finalize());
    // 支持 "sha256:xxxx" 和 "xxxx" 两种格式
    let expected = expected_hash
        .strip_prefix("sha256:")
        .unwrap_or(expected_hash);

    if hash_result != expected {
        // 校验失败：删除文件，返回错误
        let _ = fs::remove_file(&dest_path).await;
        error!(
            "SHA256 校验失败！期望: {}, 实际: {}",
            expected, hash_result
        );
        return Err(anyhow::anyhow!(
            "文件完整性校验失败，文件可能已被篡改（期望: {}..., 实际: {}...）",
            &expected[..8.min(expected.len())],
            &hash_result[..8]
        ));
    }

    info!("SHA256 校验通过: {}", hash_result);

    progress_callback(UpdateDownloadProgress {
        component: component_str.to_string(),
        downloaded,
        total: total_size,
        speed: 0,
        percent: 100,
        status: UpdateDownloadStatus::Completed,
        message: "下载完成".to_string(),
    });

    Ok(dest_path)
}

// ==================== 启动器安装 ====================

/// 安装启动器更新
/// 流程：检查运行实例 → 关闭浏览器 → 关闭代理桥 → 启动安装器 → 优雅退出
pub async fn install_launcher_update(
    installer_path: &Path,
    browser_manager: &super::BrowserManager,
    proxy_bridge_manager: &super::ProxyBridgeManager,
    app_handle: &tauri::AppHandle,
) -> Result<()> {
    use tauri::Emitter;

    // Step 1: 检查是否有运行中的浏览器实例
    let running = browser_manager.get_running_profiles().await;
    if !running.is_empty() {
        info!("正在关闭 {} 个运行中的浏览器实例...", running.len());
        for (profile_id, _pid) in &running {
            if let Some(mut child) = browser_manager
                .unregister_process(profile_id)
                .await
                .unwrap_or(None)
            {
                let _ = child.kill();
                let _ = child.wait();
            }
        }
        // 等待进程完全退出
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
    }

    // Step 2: 关闭所有代理桥
    info!("正在关闭代理桥...");
    proxy_bridge_manager.stop_all().await;

    // Step 3: 启动 NSIS 安装器（/S 静默安装）
    info!("启动安装器: {:?}", installer_path);
    #[cfg(target_os = "windows")]
    {
        use std::process::Command;
        Command::new(installer_path)
            .arg("/S")  // NSIS 静默安装参数
            .spawn()
            .context("启动安装器失败")?;
    }

    // Step 4: 通知前端即将退出
    let _ = app_handle.emit("app:will-restart", ());

    // Step 5: 优雅退出（让 Tauri 执行清理流程）
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;
    app_handle.exit(0);

    Ok(())
}

// ==================== 内核更新 ====================

/// 安装内核更新
/// 前置条件：必须无运行中的浏览器实例
/// 流程：解压到临时目录 → 备份旧内核 → 替换 → 写入版本信息
pub async fn install_kernel_update<F>(
    zip_path: &Path,
    kernel_dir: &Path,
    new_version: &str,
    browser_manager: &super::BrowserManager,
    progress_callback: F,
) -> Result<()>
where
    F: Fn(UpdateDownloadProgress) + Send + 'static,
{
    // 前置检查：必须无运行中的浏览器
    let running = browser_manager.get_running_profiles().await;
    if !running.is_empty() {
        return Err(anyhow::anyhow!(
            "仍有 {} 个浏览器实例运行中，请先关闭所有浏览器再更新内核",
            running.len()
        ));
    }

    progress_callback(UpdateDownloadProgress {
        component: "kernel".to_string(),
        downloaded: 0, total: 0, speed: 0, percent: 0,
        status: UpdateDownloadStatus::Extracting,
        message: "正在解压内核文件...".to_string(),
    });

    // Step 1: 备份旧内核（如果存在）
    let backup_dir = kernel_dir.parent()
        .unwrap_or(kernel_dir)
        .join("kernel_backup");

    if kernel_dir.exists() {
        info!("备份旧内核到: {:?}", backup_dir);
        if backup_dir.exists() {
            fs::remove_dir_all(&backup_dir).await?;
        }
        fs::rename(kernel_dir, &backup_dir).await
            .context("备份旧内核失败")?;
    }

    // Step 2: 解压新内核
    fs::create_dir_all(kernel_dir).await?;

    let zip_path_owned = zip_path.to_path_buf();
    let dest_dir_owned = kernel_dir.to_path_buf();
    let extract_result = tokio::task::spawn_blocking(move || {
        let file = std::fs::File::open(&zip_path_owned)?;
        let mut archive = zip::ZipArchive::new(file)?;
        for i in 0..archive.len() {
            let mut entry = archive.by_index(i)?;
            let outpath = match entry.enclosed_name() {
                Some(path) => dest_dir_owned.join(path),
                None => continue,
            };
            if entry.name().ends_with('/') {
                std::fs::create_dir_all(&outpath)?;
            } else {
                if let Some(parent) = outpath.parent() {
                    std::fs::create_dir_all(parent)?;
                }
                let mut outfile = std::fs::File::create(&outpath)?;
                std::io::copy(&mut entry, &mut outfile)?;
            }
        }
        Ok::<_, anyhow::Error>(())
    }).await?;

    if let Err(e) = extract_result {
        // 解压失败：回滚
        error!("解压失败，正在回滚: {}", e);
        let _ = fs::remove_dir_all(kernel_dir).await;
        if backup_dir.exists() {
            let _ = fs::rename(&backup_dir, kernel_dir).await;
        }
        return Err(e.context("解压内核失败，已回滚到旧版本"));
    }

    // Step 3: 写入版本信息
    let version_info = serde_json::json!({
        "version": new_version,
        "build_date": chrono::Utc::now().to_rfc3339(),
        "platform": "win64",
        "source": "auto_update",
    });
    let version_file = kernel_dir.join("kernel_version.json");
    fs::write(&version_file, serde_json::to_string_pretty(&version_info)?)
        .await?;

    // Step 4: 清理备份和临时文件
    if backup_dir.exists() {
        let _ = fs::remove_dir_all(&backup_dir).await;
    }
    let _ = fs::remove_file(zip_path).await;

    info!("内核更新完成: {}", new_version);

    progress_callback(UpdateDownloadProgress {
        component: "kernel".to_string(),
        downloaded: 0, total: 0, speed: 0, percent: 100,
        status: UpdateDownloadStatus::Completed,
        message: format!("内核已更新到 {}", new_version),
    });

    Ok(())
}

// ==================== 辅助函数 ====================

/// 获取临时下载目录
pub fn get_update_temp_dir() -> PathBuf {
    std::env::temp_dir().join("chubao-update")
}

/// 清理临时下载目录
pub async fn cleanup_temp_dir() -> Result<()> {
    let temp_dir = get_update_temp_dir();
    if temp_dir.exists() {
        fs::remove_dir_all(&temp_dir).await?;
    }
    Ok(())
}
