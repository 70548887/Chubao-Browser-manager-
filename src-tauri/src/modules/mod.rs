pub mod profile;
pub mod browser;
pub mod settings;
pub mod browser_manager;
pub mod group;  // Group module
pub mod fingerprint_merge;
pub mod batch_result;
pub mod window_helper;
pub mod tag;  // Tag management
pub mod recycle_bin;  // Recycle bin
pub mod proxy;  // Proxy management
pub mod proxy_checker;  // Proxy health check
pub mod proxy_bridge;  // Proxy bridge (SOCKS5 auth)
pub mod logger;  // Logger system
pub mod config_writer;  // Config file generation
pub mod fingerprint;  // Fingerprint generation
pub mod extension;  // Extension management
pub mod kernel_downloader;  // Kernel download and management
pub mod app_updater;  // 应用自动更新

pub use profile::ProfileService;
pub use browser::BrowserLauncher;
pub use browser_manager::BrowserManager;
pub use group::GroupService;
pub use fingerprint_merge::FingerprintMerger;
pub use batch_result::{BatchResult, BatchItemResult};
pub use tag::TagService;
pub use recycle_bin::{RecycleBinService, RecycledProfile};  // ✅ V5 导出 RecycledProfile
pub use proxy::ProxyService;
pub use proxy_checker::{ProxyChecker, ProxyCheckResult};  // ✅ 代理健康检查
pub use logger::{Logger, LoggerConfig, LogFileInfo};  // ✅ 日志系统
pub use config_writer::ConfigWriter;  // ✅ P0 导出配置写入器
pub use fingerprint::FingerprintGenerator;  // ✅ 导出指纹生成器
pub use extension::{ExtensionService, Extension, CreateExtensionDto, UpdateExtensionDto};  // ✅ 扩展管理
pub use proxy_bridge::{ProxyBridge, ProxyBridgeConfig, ProxyBridgeManager, BridgeStats};
pub use kernel_downloader::{KernelDownloader, DownloadProgress, DownloadStatus, KernelVersionInfo};
pub use app_updater::{UpdateInfo, UpdateDownloadProgress, UpdateDownloadStatus, DownloadSource, UpdateComponent};
