// Fingerprint Module - 设备指纹生成系统

pub mod seed_manager;
pub mod templates;
pub mod generator;
pub mod noise;
pub mod validator;  // 重新启用，已适配新结构体

// 导出主要类型
pub use seed_manager::SeedManager;
pub use templates::TemplateManager;
pub use generator::FingerprintGenerator;
pub use validator::{FingerprintValidator, ValidationResult};

#[cfg(test)]
pub mod integration_tests;
