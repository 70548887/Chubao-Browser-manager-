pub mod profile;
pub mod browser;
pub mod settings;
pub mod browser_manager;
pub mod group;  // ✅ 已解锁 - Group 模块完整实现
pub mod fingerprint_merge;
pub mod batch_result;
pub mod window_helper;
pub mod tag;  // ✅ V5 解锁 - 标签管理
pub mod recycle_bin;  // ✅ V5 解锁 - 窗口回收站
pub mod proxy;  // ✅ V5 升级 - 代理管理（数据库表）
pub mod proxy_checker;  // ✅ 代理健康检查
pub mod proxy_bridge;  // ✅ P3 - 代理桥接（解决 SOCKS5 认证问题）
pub mod logger;  // ✅ 日志系统管理
pub mod config_writer;  // ✅ P0 - 配置文件生成（bm_fingerprint.json / bm_cloud.json）
pub mod fingerprint;  // ✅ 指纹生成系统
pub mod extension;  // ✅ 扩展管理（共享目录模式）

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
pub use proxy_bridge::{ProxyBridge, ProxyBridgeConfig, ProxyBridgeManager, BridgeStats};  // ✅ 代理桥接
// 兼容旧代码的类型别名
