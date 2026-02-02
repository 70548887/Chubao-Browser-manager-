// 指纹一致性校验器
// 检查生成的指纹配置是否符合真实设备特征

use crate::modules::config_writer::*;
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
    pub severity: IssueSeverity,
    pub field: String,
}

/// 问题严重程度
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum IssueSeverity {
    Error,   // 致命错误，会导致检测
    Warning, // 警告，不太真实但可接受
    Info,    // 信息提示
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
            severity: IssueSeverity::Error,
            field: field.to_string(),
        });
    }
    
    pub fn add_warning(&mut self, code: &str, message: String, field: &str) {
        self.warnings.push(ValidationIssue {
            code: code.to_string(),
            message,
            severity: IssueSeverity::Warning,
            field: field.to_string(),
        });
    }
}

/// 指纹一致性校验器
pub struct FingerprintValidator;

impl FingerprintValidator {
    /// 校验指纹配置
    pub fn validate(config: &FingerprintFileConfig) -> ValidationResult {
        let mut result = ValidationResult::new();
        
        // 1. 设备档次一致性
        Self::validate_device_tier(&config, &mut result);
        
        // 2. 版本一致性
        Self::validate_version_consistency(&config, &mut result);
        
        // 3. 地理位置一致性
        Self::validate_geo_consistency(&config, &mut result);
        
        // 4. 技术参数合理性
        Self::validate_technical_params(&config, &mut result);
        
        result
    }
    
    /// 校验设备档次一致性
    fn validate_device_tier(config: &FingerprintFileConfig, result: &mut ValidationResult) {
        let cores = config.navigator.hardware_concurrency;
        let memory = config.navigator.device_memory;
        let resolution_pixels = config.screen.width * config.screen.height;
        
        // 规则 1: 高端 CPU (12+ 核) + 低端内存 (8GB)
        if cores >= 12 && memory <= 8 {
            result.add_warning(
                "TIER_MISMATCH_CPU_MEMORY",
                format!("高端CPU ({} 核) 但内存较低 ({} GB)，不太常见", cores, memory),
                "hardware_concurrency,device_memory"
            );
        }
        
        // 规则 2: 低端 CPU (4 核) + 高端内存 (32GB+)
        if cores <= 4 && memory >= 32 {
            result.add_warning(
                "TIER_MISMATCH_CPU_MEMORY",
                format!("低端CPU ({} 核) 但内存较高 ({} GB)，不太常见", cores, memory),
                "hardware_concurrency,device_memory"
            );
        }
        
        // 规则 3: 低端设备 + 4K 分辨率
        if cores <= 4 && memory <= 8 && resolution_pixels >= 3840 * 2160 {
            result.add_warning(
                "TIER_MISMATCH_LOW_END_4K",
                format!("低端设备 ({} 核, {} GB) 但使用 4K 分辨率，不常见", cores, memory),
                "screen"
            );
        }
        
        // 规则 4: 高端设备 + 低分辨率
        if cores >= 12 && memory >= 32 && resolution_pixels <= 1366 * 768 {
            result.add_warning(
                "TIER_MISMATCH_HIGH_END_LOW_RES",
                format!("高端设备 ({} 核, {} GB) 但使用低分辨率 ({}x{})，不常见", 
                    cores, memory, config.screen.width, config.screen.height),
                "screen"
            );
        }
    }
    
    /// 校验版本一致性
    fn validate_version_consistency(config: &FingerprintFileConfig, result: &mut ValidationResult) {
        // 提取 User-Agent 中的 Chrome 版本
        let ua = &config.navigator.user_agent;
        let chrome_version = Self::extract_chrome_version(ua);
        
        // 提取 Client Hints 中的版本
        let ch_version = &config.client_hints.full_version;
        let ch_major = ch_version.split('.').next().unwrap_or("0");
        
        // 规则 1: User-Agent 和 Client Hints 版本不匹配
        if let Some(ref ua_major) = chrome_version {
            if ua_major != ch_major {
                result.add_error(
                    "VERSION_MISMATCH_UA_CH",
                    format!("User-Agent 版本 ({}) 与 Client Hints 版本 ({}) 不一致", 
                        ua_major, ch_major),
                    "navigator.user_agent,client_hints.full_version"
                );
            }
        }
        
        // 规则 2: 过旧版本检查（Chrome < 100 在 2026 年不合理）
        if let Some(ref major) = chrome_version {
            if let Ok(version_num) = major.parse::<u32>() {
                if version_num < 100 {
                    result.add_warning(
                        "VERSION_TOO_OLD",
                        format!("Chrome 版本过旧 ({}), 2026年应该 >= 130", major),
                        "navigator.user_agent"
                    );
                }
            }
        }
        
        // 规则 3: Platform 和 platformVersion 一致性
        let platform = &config.navigator.platform;
        let platform_version = &config.client_hints.platform_version;
        
        if platform.contains("Win") && !platform_version.contains("10.0") && !platform_version.contains("13.0") {
            result.add_warning(
                "VERSION_PLATFORM_MISMATCH",
                format!("Windows 平台但 platformVersion 不合理: {}", platform_version),
                "client_hints.platform_version"
            );
        }
    }
    
    /// 校验地理位置一致性
    fn validate_geo_consistency(config: &FingerprintFileConfig, result: &mut ValidationResult) {
        let timezone = &config.timezone.id;
        let language = &config.navigator.language;
        
        // 规则 1: 中国时区但非中文语言
        if timezone.contains("Shanghai") || timezone.contains("Hong_Kong") {
            if !language.starts_with("zh") {
                result.add_warning(
                    "GEO_MISMATCH_TIMEZONE_LANG",
                    format!("中国时区 ({}) 但语言不是中文 ({})", timezone, language),
                    "timezone.id,navigator.language"
                );
            }
        }
        
        // 规则 2: 美国时区但非英文
        if timezone.contains("America/") && !language.starts_with("en") {
            result.add_warning(
                "GEO_MISMATCH_TIMEZONE_LANG",
                format!("美国时区 ({}) 但语言不是英文 ({})", timezone, language),
                "timezone.id,navigator.language"
            );
        }
        
        // 规则 3: 欧洲时区但语言不匹配
        if timezone.contains("Europe/Paris") || timezone.contains("Europe/Berlin") {
            if !language.starts_with("fr") && !language.starts_with("de") && !language.starts_with("en") {
                result.add_warning(
                    "GEO_MISMATCH_TIMEZONE_LANG",
                    format!("欧洲时区 ({}) 但语言不常见 ({})", timezone, language),
                    "timezone.id,navigator.language"
                );
            }
        }
    }
    
    /// 校验技术参数合理性
    fn validate_technical_params(config: &FingerprintFileConfig, result: &mut ValidationResult) {
        // 规则 1: 硬件并发数合理性（1-32）
        let cores = config.navigator.hardware_concurrency;
        if cores == 0 || cores > 128 {
            result.add_error(
                "INVALID_CORES",
                format!("CPU 核心数不合理: {}", cores),
                "navigator.hardware_concurrency"
            );
        }
        
        // 规则 2: 内存合理性
        let memory = config.navigator.device_memory;
        let valid_memory = [2, 4, 8, 16, 32, 64, 128];
        if !valid_memory.contains(&memory) {
            result.add_warning(
                "UNUSUAL_MEMORY",
                format!("内存容量不常见: {} GB（常见值：2/4/8/16/32/64）", memory),
                "navigator.device_memory"
            );
        }
        
        // 规则 3: 屏幕分辨率合理性
        let width = config.screen.width;
        let height = config.screen.height;
        
        if width < 800 || height < 600 {
            result.add_error(
                "INVALID_RESOLUTION",
                format!("屏幕分辨率过低: {}x{}", width, height),
                "screen"
            );
        }
        
        if width > 7680 || height > 4320 {
            result.add_warning(
                "UNUSUAL_RESOLUTION",
                format!("屏幕分辨率过高: {}x{} (8K+)", width, height),
                "screen"
            );
        }
        
        // 规则 4: 色深合理性
        if config.screen.color_depth != 24 && config.screen.color_depth != 32 {
            result.add_warning(
                "UNUSUAL_COLOR_DEPTH",
                format!("色深不常见: {} (常见：24或32)", config.screen.color_depth),
                "screen.color_depth"
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
        config.navigator.hardware_concurrency = 8;
        config.navigator.device_memory = 16;
        config.screen.width = 1920;
        config.screen.height = 1080;
        config.navigator.user_agent = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/139.0.0.0 Safari/537.36".to_string();
        config.client_hints.full_version = "139.0.0.0".to_string();
        config.timezone.id = "Asia/Shanghai".to_string();
        config.navigator.language = "zh-CN".to_string();
        
        let result = FingerprintValidator::validate(&config);
        assert!(result.valid);
        assert_eq!(result.errors.len(), 0);
    }
    
    #[test]
    fn test_tier_mismatch_high_cpu_low_memory() {
        let mut config = FingerprintFileConfig::default();
        config.navigator.hardware_concurrency = 16;
        config.navigator.device_memory = 8;
        
        let result = FingerprintValidator::validate(&config);
        assert!(result.warnings.iter().any(|w| w.code == "TIER_MISMATCH_CPU_MEMORY"));
    }
    
    #[test]
    fn test_version_mismatch() {
        let mut config = FingerprintFileConfig::default();
        config.navigator.user_agent = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36".to_string();
        config.client_hints.full_version = "139.0.0.0".to_string();
        
        let result = FingerprintValidator::validate(&config);
        assert!(!result.valid);
        assert!(result.errors.iter().any(|e| e.code == "VERSION_MISMATCH_UA_CH"));
    }
    
    #[test]
    fn test_geo_mismatch() {
        let mut config = FingerprintFileConfig::default();
        config.timezone.id = "Asia/Shanghai".to_string();
        config.navigator.language = "en-US".to_string();
        
        let result = FingerprintValidator::validate(&config);
        assert!(result.warnings.iter().any(|w| w.code == "GEO_MISMATCH_TIMEZONE_LANG"));
    }
}
