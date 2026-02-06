// Kernel Downloader Module - 内核下载与管理
// 负责从远程下载内核压缩包并解压到本地

use anyhow::{Context, Result};
use futures_util::StreamExt;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::fs::{self, File};
use tokio::io::AsyncWriteExt;
use tokio::sync::Mutex;
use tracing::{error, info, warn};

/// 下载进度信息
#[derive(Debug, Clone, serde::Serialize)]
pub struct DownloadProgress {
    /// 已下载字节数
    pub downloaded: u64,
    /// 总字节数 (如果已知)
    pub total: Option<u64>,
    /// 下载速度 (bytes/sec)
    pub speed: u64,
    /// 状态
    pub status: DownloadStatus,
    /// 消息
    pub message: String,
}

/// 下载状态
#[derive(Debug, Clone, serde::Serialize, PartialEq)]
pub enum DownloadStatus {
    Idle,
    Downloading,
    Extracting,
    Completed,
    Failed,
}

/// 内核版本信息
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct KernelVersionInfo {
    pub version: String,
    pub build_date: String,
    pub platform: String,
    pub source: String,
    pub files_count: u32,
    pub total_size_bytes: u64,
}

/// 内核下载器
pub struct KernelDownloader {
    /// 内核存放目录
    kernel_dir: PathBuf,
    /// 当前进度
    progress: Arc<Mutex<DownloadProgress>>,
}

impl KernelDownloader {
    /// 创建新的下载器
    pub fn new(kernel_dir: PathBuf) -> Self {
        Self {
            kernel_dir,
            progress: Arc::new(Mutex::new(DownloadProgress {
                downloaded: 0,
                total: None,
                speed: 0,
                status: DownloadStatus::Idle,
                message: String::new(),
            })),
        }
    }

    /// 获取当前进度
    pub async fn get_progress(&self) -> DownloadProgress {
        self.progress.lock().await.clone()
    }

    /// Check if kernel is installed
    pub fn is_kernel_installed(&self) -> bool {
        self.get_kernel_exe_path().exists()
    }

    /// Get installed kernel version info
    pub fn get_installed_version(&self) -> Option<KernelVersionInfo> {
        // Check version file in kernel directory
        let version_file = self.kernel_dir.join("kernel_version.json");
        if version_file.exists() {
            if let Ok(content) = std::fs::read_to_string(&version_file) {
                if let Ok(info) = serde_json::from_str(&content) {
                    return Some(info);
                }
            }
        }
        
        // If no version file, check if kernel exists and return basic info
        if self.is_kernel_installed() {
            return Some(KernelVersionInfo {
                version: "v139".to_string(),
                build_date: "unknown".to_string(),
                platform: "win64".to_string(),
                source: "local".to_string(),
                files_count: 0,
                total_size_bytes: 0,
            });
        }
        
        None
    }

    /// 下载并安装内核
    pub async fn download_and_install(
        &self,
        download_url: &str,
        progress_callback: impl Fn(DownloadProgress) + Send + 'static,
    ) -> Result<()> {
        info!("Starting kernel download from: {}", download_url);

        // 更新状态
        {
            let mut progress = self.progress.lock().await;
            progress.status = DownloadStatus::Downloading;
            progress.message = "Starting download...".to_string();
            progress_callback(progress.clone());
        }

        // 创建临时下载文件
        let temp_dir = std::env::temp_dir();
        let temp_file = temp_dir.join("chromium-kernel-download.zip");

        // 下载文件
        self.download_file(download_url, &temp_file, &progress_callback)
            .await?;

        // 更新状态为解压中
        {
            let mut progress = self.progress.lock().await;
            progress.status = DownloadStatus::Extracting;
            progress.message = "Extracting kernel files...".to_string();
            progress_callback(progress.clone());
        }

        // 确保内核目录存在
        fs::create_dir_all(&self.kernel_dir)
            .await
            .context("Failed to create kernel directory")?;

        // 解压文件
        self.extract_zip(&temp_file, &self.kernel_dir).await?;

        // 清理临时文件
        let _ = fs::remove_file(&temp_file).await;

        // 更新状态为完成
        {
            let mut progress = self.progress.lock().await;
            progress.status = DownloadStatus::Completed;
            progress.message = "Kernel installation completed!".to_string();
            progress_callback(progress.clone());
        }

        info!("Kernel installation completed successfully");
        Ok(())
    }

    /// Download file with progress callback
    async fn download_file(
        &self,
        url: &str,
        dest: &Path,
        progress_callback: &impl Fn(DownloadProgress),
    ) -> Result<()> {
        // Build client with User-Agent and redirect support
        let client = reqwest::Client::builder()
            .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
            .redirect(reqwest::redirect::Policy::limited(10))
            .build()
            .context("Failed to build HTTP client")?;

        info!("Sending download request to: {}", url);

        let response = client
            .get(url)
            .send()
            .await
            .context("Failed to send download request")?;

        info!("Response status: {}", response.status());

        if !response.status().is_success() {
            return Err(anyhow::anyhow!(
                "Download failed with status: {}",
                response.status()
            ));
        }

        let total_size = response.content_length();

        {
            let mut progress = self.progress.lock().await;
            progress.total = total_size;
        }

        let mut file = File::create(dest)
            .await
            .context("Failed to create download file")?;

        let mut downloaded: u64 = 0;
        let mut stream = response.bytes_stream();
        let start_time = std::time::Instant::now();
        let mut last_update = std::time::Instant::now();

        while let Some(chunk) = stream.next().await {
            let chunk = chunk.context("Failed to read chunk")?;
            file.write_all(&chunk)
                .await
                .context("Failed to write chunk")?;

            downloaded += chunk.len() as u64;

            // 每 100ms 更新一次进度
            if last_update.elapsed().as_millis() >= 100 {
                let elapsed = start_time.elapsed().as_secs_f64();
                let speed = if elapsed > 0.0 {
                    (downloaded as f64 / elapsed) as u64
                } else {
                    0
                };

                let mut progress = self.progress.lock().await;
                progress.downloaded = downloaded;
                progress.speed = speed;
                progress.message = format!(
                    "Downloading... {:.1} MB / {:.1} MB",
                    downloaded as f64 / 1_000_000.0,
                    total_size.unwrap_or(0) as f64 / 1_000_000.0
                );
                progress_callback(progress.clone());

                last_update = std::time::Instant::now();
            }
        }

        file.flush().await?;
        info!("Download completed: {} bytes", downloaded);
        Ok(())
    }

    /// 解压 ZIP 文件
    async fn extract_zip(&self, zip_path: &Path, dest_dir: &Path) -> Result<()> {
        let zip_path = zip_path.to_path_buf();
        let dest_dir = dest_dir.to_path_buf();

        // 在阻塞线程中执行解压
        tokio::task::spawn_blocking(move || {
            let file = std::fs::File::open(&zip_path).context("Failed to open zip file")?;
            let mut archive = zip::ZipArchive::new(file).context("Failed to read zip archive")?;

            info!("Extracting {} files...", archive.len());

            for i in 0..archive.len() {
                let mut file = archive
                    .by_index(i)
                    .context("Failed to read file from archive")?;

                let outpath = match file.enclosed_name() {
                    Some(path) => dest_dir.join(path),
                    None => continue,
                };

                if file.name().ends_with('/') {
                    // 目录
                    std::fs::create_dir_all(&outpath)?;
                } else {
                    // 文件
                    if let Some(parent) = outpath.parent() {
                        if !parent.exists() {
                            std::fs::create_dir_all(parent)?;
                        }
                    }
                    let mut outfile = std::fs::File::create(&outpath)
                        .context(format!("Failed to create file: {:?}", outpath))?;
                    std::io::copy(&mut file, &mut outfile)?;
                }

                // 设置文件权限 (Unix)
                #[cfg(unix)]
                {
                    use std::os::unix::fs::PermissionsExt;
                    if let Some(mode) = file.unix_mode() {
                        std::fs::set_permissions(&outpath, std::fs::Permissions::from_mode(mode))?;
                    }
                }
            }

            info!("Extraction completed");
            Ok::<_, anyhow::Error>(())
        })
        .await??;

        Ok(())
    }

    /// 删除已安装的内核
    pub async fn uninstall(&self) -> Result<()> {
        if self.kernel_dir.exists() {
            fs::remove_dir_all(&self.kernel_dir)
                .await
                .context("Failed to remove kernel directory")?;
            info!("Kernel uninstalled from: {:?}", self.kernel_dir);
        }
        Ok(())
    }

    /// Get kernel executable path
    pub fn get_kernel_exe_path(&self) -> PathBuf {
        // Kernel is in win32 subdirectory after extraction
        #[cfg(target_os = "windows")]
        {
            self.kernel_dir.join("win32").join("chrome.exe")
        }
        #[cfg(target_os = "macos")]
        {
            self.kernel_dir.join("darwin").join("Chromium.app").join("Contents").join("MacOS").join("Chromium")
        }
        #[cfg(target_os = "linux")]
        {
            self.kernel_dir.join("linux").join("chrome")
        }
    }

    /// Get bundled kernel path (from resources)
    /// Returns path to bundled kernel executable if exists
    /// Supports versioned kernel structure: resources/kernel/win32/{version}/chrome.exe
    pub fn get_bundled_kernel_path() -> Option<PathBuf> {
        Self::get_bundled_kernel_path_with_version(None)
    }

    /// Get bundled kernel path with specific version
    /// If version is None, returns the latest version found
    pub fn get_bundled_kernel_path_with_version(version: Option<&str>) -> Option<PathBuf> {
        let exe_path = std::env::current_exe().ok()?;
        let exe_dir = exe_path.parent()?;
        
        #[cfg(target_os = "windows")]
        {
            // Check dev mode path
            let dev_base = exe_dir.parent()?.parent()?.join("resources").join("kernel").join("win32");
            if let Some(path) = Self::find_kernel_in_dir(&dev_base, version) {
                return Some(path);
            }
            
            // Check production path
            let prod_base = exe_dir.join("resources").join("kernel").join("win32");
            if let Some(path) = Self::find_kernel_in_dir(&prod_base, version) {
                return Some(path);
            }
        }
        
        None
    }

    /// Find kernel in directory, supporting versioned subdirectories
    /// Directory structure: base_dir/{version}/chrome.exe
    fn find_kernel_in_dir(base_dir: &Path, version: Option<&str>) -> Option<PathBuf> {
        if !base_dir.exists() {
            return None;
        }

        // If specific version requested
        if let Some(ver) = version {
            let kernel_path = base_dir.join(ver).join("chrome.exe");
            if kernel_path.exists() {
                return Some(kernel_path);
            }
            return None;
        }

        // Find latest version (sort by version number)
        let mut versions = Vec::new();
        if let Ok(entries) = std::fs::read_dir(base_dir) {
            for entry in entries.flatten() {
                if entry.path().is_dir() {
                    if let Some(ver_name) = entry.file_name().to_str() {
                        let kernel_path = entry.path().join("chrome.exe");
                        if kernel_path.exists() {
                            versions.push((ver_name.to_string(), kernel_path));
                        }
                    }
                }
            }
        }

        if versions.is_empty() {
            return None;
        }

        // Sort by version string (descending) and return the latest
        versions.sort_by(|a, b| b.0.cmp(&a.0));
        Some(versions[0].1.clone())
    }

    /// List all bundled kernel versions
    pub fn list_bundled_versions() -> Vec<String> {
        let exe_path = std::env::current_exe().ok();
        if exe_path.is_none() {
            return Vec::new();
        }
        let exe_dir = exe_path.unwrap().parent().map(|p| p.to_path_buf());
        if exe_dir.is_none() {
            return Vec::new();
        }
        let exe_dir = exe_dir.unwrap();
        
        let mut versions = Vec::new();
        
        #[cfg(target_os = "windows")]
        {
            // Check both dev and production paths
            let dev_base = exe_dir.parent()
                .and_then(|p| p.parent())
                .map(|p| p.join("resources").join("kernel").join("win32"));
            if let Some(base) = dev_base {
                if let Ok(entries) = std::fs::read_dir(&base) {
                    for entry in entries.flatten() {
                        if entry.path().is_dir() {
                            if let Some(ver) = entry.file_name().to_str() {
                                if entry.path().join("chrome.exe").exists() {
                                    versions.push(ver.to_string());
                                }
                            }
                        }
                    }
                }
            }
            
            // Also check production path
            let prod_base = exe_dir.join("resources").join("kernel").join("win32");
            if let Ok(entries) = std::fs::read_dir(&prod_base) {
                for entry in entries.flatten() {
                    if entry.path().is_dir() {
                        if let Some(ver) = entry.file_name().to_str() {
                            if entry.path().join("chrome.exe").exists() && !versions.contains(&ver.to_string()) {
                                versions.push(ver.to_string());
                            }
                        }
                    }
                }
            }
        }
        
        versions.sort_by(|a, b| b.cmp(a)); // Sort descending
        versions
    }
}

/// 默认内核下载 URL
pub const DEFAULT_KERNEL_DOWNLOAD_URL: &str = "";

/// 获取平台特定的内核下载 URL
pub fn get_kernel_download_url(base_url: &str, version: &str) -> String {
    #[cfg(target_os = "windows")]
    let platform = "win64";
    #[cfg(target_os = "macos")]
    let platform = "darwin";
    #[cfg(target_os = "linux")]
    let platform = "linux";

    format!("{}/chromium-kernel-{}-v{}.zip", base_url, platform, version)
}
