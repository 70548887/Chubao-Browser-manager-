use anyhow::{Result, Context};
use std::process::{Command, Child};
use std::path::PathBuf;
use tracing::info;

/// 浏览器启动器
/// 
/// 遵循方案A规范：
/// - 启动参数仅传递 `--user-data-dir`
/// - 不使用 CDP（无 --remote-debugging-port）
/// - 配置通过 bm_fingerprint.json / bm_cloud.json 传递
pub struct BrowserLauncher {
    kernel_path: PathBuf,
}

impl BrowserLauncher {
    pub fn new(kernel_path: PathBuf) -> Self {
        Self { kernel_path }
    }

    /// 启动浏览器实例
    /// 
    /// # 参数
    /// - `profile_id`: Profile 唯一标识
    /// - `user_data_dir`: 用户数据目录（包含 bm_fingerprint.json / bm_cloud.json）
    /// - `proxy`: 可选代理服务器地址
    /// - `load_extension`: 可选扩展路径列表（逗号分隔）
    /// 
    /// # 注意
    /// 启动前需确保已调用 ConfigWriter::setup_profile_configs 写入配置文件
    pub fn launch(
        &self, 
        profile_id: &str, 
        user_data_dir: &PathBuf, 
        proxy: Option<&str>,
        load_extension: Option<&str>,
    ) -> Result<Child> {
        info!("Launching browser for profile: {}", profile_id);

        let mut cmd = Command::new(&self.kernel_path);
        
        // 用户数据目录（核心参数，内核从此目录读取配置文件）
        cmd.arg(format!("--user-data-dir={}", user_data_dir.display()));
        
        // 代理配置（如果有）
        if let Some(proxy_url) = proxy {
            cmd.arg(format!("--proxy-server={}", proxy_url));
        }
        
        // 扩展加载（如果有）
        // 共享目录模式：所有扩展存放在全局 Extensions/ 目录
        if let Some(ext_paths) = load_extension {
            cmd.arg(format!("--load-extension={}", ext_paths));
            info!("Loading extensions: {}", ext_paths);
        }
        
        // 基础启动参数
        cmd.args([
            "--no-first-run",                    // 跳过首次运行向导
            "--disable-background-networking",   // 禁用后台网络请求
            "--disable-sync",                    // 禁用同步功能
            
            // WebRTC IP泄露保护（关键安全参数）
            "--force-webrtc-ip-handling-policy=disable_non_proxied_udp",
            "--enforce-webrtc-ip-permission-check",
        ]);
        
        // ❌ 不添加 --remote-debugging-port（方案A规范：无CDP）
        // 浏览器通过内置WS直连云端，不需要CDP调试接口
        
        // 启动进程
        let child = cmd.spawn()
            .context("Failed to spawn browser process")?;
        
        info!("Browser launched with PID: {:?}", child.id());
        
        Ok(child)
    }

    /// 停止浏览器实例
    pub fn stop(&self, mut child: Child) -> Result<()> {
        info!("Stopping browser with PID: {:?}", child.id());
        
        child.kill()
            .context("Failed to kill browser process")?;
        
        child.wait()
            .context("Failed to wait for browser process")?;
        
        info!("Browser stopped successfully");
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_launcher_creation() {
        let launcher = BrowserLauncher::new(PathBuf::from("chrome.exe"));
        assert_eq!(launcher.kernel_path, PathBuf::from("chrome.exe"));
    }
}
