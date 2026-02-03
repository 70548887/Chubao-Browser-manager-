// Fingerprint Validator - 指纹配置一致性校验器（适配新结构体）

use crate::modules::config_writer::FingerprintFileConfig;
use serde::{Serialize, Deserialize};

/// 校验结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    pub valid: bool,
    pub errors: Vec<ValidationIssue>,
    pub warnings: Vec<ValidationIssue>,
}

/// 校验问题
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationIssue {
    pub code: String,
    pub message: String,
    pub field: String,
}

impl ValidationResult {
    pub fn new() -> Self {
        Self {
            valid: true,
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }
    
    pub fn add_error(&mut self, code: &str, message: String, field: &str) {
        self.valid = false;
        self.errors.push(ValidationIssue {
            code: code.to_string(),
            message,
            field: field.to_string(),
        });
    }
    
    pub fn add_warning(&mut self, code: &str, message: String, field: &str) {
        self.warnings.push(ValidationIssue {
            code: code.to_string(),
            message,
            field: field.to_string(),
        });
    }
}

/// 指纹校验器
pub struct FingerprintValidator;

impl FingerprintValidator {
    /// 执行完整校验
    pub fn validate(config: &FingerprintFileConfig) -> ValidationResult {
        let mut result = ValidationResult::new();
        
        // 执行各项校验
        Self::validate_device_tier(config, &mut result);
        Self::validate_version_consistency(config, &mut result);
        Self::validate_geo_consistency(config, &mut result);
        Self::validate_technical_params(config, &mut result);
        
        result
    }
    
    /// 校验设备档次一致性
    fn validate_device_tier(config: &FingerprintFileConfig, result: &mut ValidationResult) {
        let cores = config.resource_info.cpu;
        let memory = config.resource_info.memory as i32;
        let resolution_pixels = config.resolution.monitor_width * config.resolution.monitor_height;
        
        // 规则 1: 高端 CPU (12+ 核) + 低端内存 (8GB)
        if cores >= 12 && memory <= 8 {
            result.add_warning(
                "TIER_MISMATCH_CPU_MEMORY",
                format!("高端CPU ({} 核) 但内存较低 ({} GB)，不太常见", cores, memory),
                "resource_info.cpu,resource_info.memory"
            );
        }
        
        // 规则 2: 低端 CPU (4 核) + 高端内存 (32GB+)
        if cores <= 4 && memory >= 32 {
            result.add_warning(
                "TIER_MISMATCH_CPU_MEMORY",
                format!("低端CPU ({} 核) 但内存较高 ({} GB)，不太常见", cores, memory),
                "resource_info.cpu,resource_info.memory"
            );
        }
        
        // 规则 3: 低端设备 + 4K 分辨率
        if cores <= 4 && memory <= 8 && resolution_pixels >= 3840 * 2160 {
            result.add_warning(
                "TIER_MISMATCH_LOW_END_4K",
                format!("低端设备 ({} 核, {} GB) 但使用 4K 分辨率，不常见", cores, memory),
                "resolution"
            );
        }
        
        // 规则 4: 高端设备 + 低分辨率
        if cores >= 12 && memory >= 32 && resolution_pixels <= 1366 * 768 {
            result.add_warning(
                "TIER_MISMATCH_HIGH_END_LOW_RES",
                format!("高端设备 ({} 核, {} GB) 但使用低分辨率 ({}x{})，不常见", 
                    cores, memory, config.resolution.monitor_width, config.resolution.monitor_height),
                "resolution"
            );
        }
    }
    
    /// 校验版本一致性（简化版，新结构体暂无 Client Hints）
    fn validate_version_consistency(config: &FingerprintFileConfig, result: &mut ValidationResult) {
        // 提取 User-Agent 中的 Chrome 版本
        let ua = &config.ua.user_agent;
        let chrome_version = Self::extract_chrome_version(ua);
        
        // 规则: 过旧版本检查（Chrome < 100 在 2026 年不合理）
        if let Some(ref major) = chrome_version {
            if let Ok(version_num) = major.parse::<u32>() {
                if version_num < 100 {
                    result.add_warning(
                        "VERSION_TOO_OLD",
                        format!("Chrome 版本过旧 ({}), 2026年应该 >= 130", major),
                        "ua.user_agent"
                    );
                }
            }
        }
    }
    
    /// 校验地理位置一致性
    fn validate_geo_consistency(config: &FingerprintFileConfig, result: &mut ValidationResult) {
        let timezone = &config.time_zone.gmt;
        let language = &config.language.interface_language;
        
        // 规则 1: 中国时区但非中文语言
        if timezone.contains("Shanghai") || timezone.contains("Hong_Kong") {
            if !language.starts_with("zh") {
                result.add_warning(
                    "GEO_MISMATCH_TIMEZONE_LANG",
                    format!("中国时区 ({}) 但语言不是中文 ({})，不常见", timezone, language),
                    "time_zone.gmt,language.interface_language"
                );
            }
        }
        
        // 规则 2: 英语语言但亚洲时区
        if language.starts_with("en") && (timezone.contains("Shanghai") || timezone.contains("Tokyo")) {
            result.add_warning(
                "GEO_MISMATCH_LANG_TIMEZONE",
                format!("英语语言 ({}) 但亚洲时区 ({})，可能不常见", language, timezone),
                "language.interface_language,time_zone.gmt"
            );
        }
    }
    
    /// 校验技术参数合理性
    fn validate_technical_params(config: &FingerprintFileConfig, result: &mut ValidationResult) {
        // 规则 1: 硬件并发数合理性（1-128）
        let cores = config.resource_info.cpu;
        if cores == 0 || cores > 128 {
            result.add_error(
                "INVALID_CORES",
                format!("CPU 核心数不合理: {}", cores),
                "resource_info.cpu"
            );
        }
        
        // 规则 2: 内存合理性
        let memory = config.resource_info.memory as i32;
        let valid_memory = [2, 4, 8, 16, 32, 64, 128];
        if !valid_memory.contains(&memory) {
            result.add_warning(
                "UNUSUAL_MEMORY",
                format!("内存大小不常见: {} GB", memory),
                "resource_info.memory"
            );
        }
        
        // 规则 3: 屏幕分辨率合理性
        let width = config.resolution.monitor_width;
        let height = config.resolution.monitor_height;
        
        if width < 800 || height < 600 {
            result.add_error(
                "INVALID_RESOLUTION",
                format!("屏幕分辨率过低: {}x{}", width, height),
                "resolution"
            );
        }
        
        if width > 7680 || height > 4320 {
            result.add_warning(
                "UNUSUAL_RESOLUTION",
                format!("屏幕分辨率过高: {}x{} (8K+)", width, height),
                "resolution"
            );
        }
        
        // 规则 4: 色深合理性
        if config.resolution.color_depth != 24 && config.resolution.color_depth != 32 {
            result.add_warning(
                "UNUSUAL_COLOR_DEPTH",
                format!("色深不常见: {} (常见：24或32)", config.resolution.color_depth),
                "resolution.color_depth"
            );
        }
    }
    
    /// 从 User-Agent 提取 Chrome 版本号（主版本）
    fn extract_chrome_version(user_agent: &str) -> Option<String> {
        // 示例: "Chrome/139.0.0.0"
        if let Some(start) = user_agent.find("Chrome/") {
            let version_str = &user_agent[start + 7..];
            if let Some(end) = version_str.find('.') {
                return Some(version_str[..end].to_string());
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_valid_fingerprint() {
        let mut config = FingerprintFileConfig::default();
        config.resource_info.cpu = 8;
        config.resource_info.memory = 16.0;
        config.resolution.monitor_width = 1920;
        config.resolution.monitor_height = 1080;
        config.ua.user_agent = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/146.0.0.0 Safari/537.36".to_string();
        config.time_zone.gmt = "Asia/Shanghai".to_string();
        config.language.interface_language = "zh-CN".to_string();
        
        let result = FingerprintValidator::validate(&config);
        assert!(result.valid);
        assert_eq!(result.errors.len(), 0);
    }
    
    #[test]
    fn test_tier_mismatch() {
        let mut config = FingerprintFileConfig::default();
        config.resource_info.cpu = 2;  // 低端 CPU
        config.resource_info.memory = 64.0;  // 高端内存
        
        let result = FingerprintValidator::validate(&config);
        assert!(result.warnings.iter().any(|w| w.code == "TIER_MISMATCH_CPU_MEMORY"));
    }
}
