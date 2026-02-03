// Fingerprint Merge - 受控合并策略
use serde_json::{Value, Map};
use std::collections::HashSet;

/// Fingerprint 白名单字段（允许前端修改的字段）
const WHITELIST_FIELDS: &[&str] = &[
    "platform",
    "browser",
    "user_agent",
    "hardware_concurrency",
    "device_memory",
    "screen_resolution",
    "timezone",
    "language",
    "canvas_noise",
    "webgl_noise",
    "audio_noise",
    // --- 新增字段 ---
    "navigator_platform",
    "os_version",
    "browser_version",
    "screen_width",
    "screen_height",
    "resolution",
    "fonts",
    "webrtc",
    "webrtc_public_ip",
    "webrtc_local_ip",
    "webgl_image",
    "webgl_vendor",
    "webgl_renderer",
    "webgpu",
    "canvas",
    "audio_context",
    "speech_voices",
    "do_not_track",
    "client_rects",
    "media_devices",
    "device_name",
    "mac_address",
    "ssl_fingerprint",
    "port_scan_protection",
    "port_scan_whitelist",
    "custom_fonts",
    "ignore_cert_errors",
    "custom_plugins",
    "cloudflare_optimize",
    "hardware_acceleration",
    "disable_sandbox",
    "launch_args",
];

/// Fingerprint 黑名单字段（禁止前端注入的危险字段）
const BLACKLIST_FIELDS: &[&str] = &[
    "startup_args",           // 启动参数
    "command_line",           // 命令行
    "debug_port",             // 调试端口
    "remote_debugging_port",  // 远程调试端口
    "user_data_dir",          // 用户数据目录
    "disk_cache_dir",         // 磁盘缓存目录
    "extensions_path",        // 扩展路径
    "load_extension",         // 加载扩展
    "disable_extensions",     // 禁用扩展
    "proxy_server",           // 代理服务器（应通过 proxy 字段设置）
    "proxy_bypass_list",      // 代理绕过列表
];

/// Fingerprint 合并策略
pub struct FingerprintMerger {
    whitelist: HashSet<String>,
    blacklist: HashSet<String>,
}

impl FingerprintMerger {
    pub fn new() -> Self {
        Self {
            whitelist: WHITELIST_FIELDS.iter().map(|s| s.to_string()).collect(),
            blacklist: BLACKLIST_FIELDS.iter().map(|s| s.to_string()).collect(),
        }
    }

    /// 合并 fingerprint
    /// 
    /// 策略：
    /// 1. 白名单字段：从 patch 覆盖 existing
    /// 2. 非白名单字段：保持 existing（透传不丢）
    /// 3. 黑名单字段：若 patch 包含则拒绝
    pub fn merge(
        &self,
        existing: &Value,
        patch: &Value,
    ) -> Result<Value, String> {
        // 检查黑名单字段
        if let Some(patch_obj) = patch.as_object() {
            for field in &self.blacklist {
                if patch_obj.contains_key(field) {
                    return Err(format!(
                        "禁止修改危险字段: {}。此字段可能影响浏览器安全性。",
                        field
                    ));
                }
            }
        }

        // 合并逻辑
        let mut result = if let Some(existing_obj) = existing.as_object() {
            existing_obj.clone()
        } else {
            Map::new()
        };

        if let Some(patch_obj) = patch.as_object() {
            for (key, value) in patch_obj {
                if self.whitelist.contains(key) {
                    // 白名单字段：允许覆盖
                    result.insert(key.clone(), value.clone());
                }
                // 非白名单字段：忽略（保持 existing，实现透传）
            }
        }

        Ok(Value::Object(result))
    }

    /// 验证 fingerprint 是否包含黑名单字段
    pub fn validate(&self, fingerprint: &Value) -> Result<(), String> {
        if let Some(obj) = fingerprint.as_object() {
            for field in &self.blacklist {
                if obj.contains_key(field) {
                    return Err(format!(
                        "Fingerprint 包含禁止字段: {}",
                        field
                    ));
                }
            }
        }
        Ok(())
    }
}

impl Default for FingerprintMerger {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_whitelist_field_can_update() {
        let merger = FingerprintMerger::new();
        
        let existing = json!({
            "platform": "Windows",
            "browser": "Chrome",
            "user_agent": "Mozilla/5.0...",
            "seed": 12345,
            "custom_field": "should_keep"
        });

        let patch = json!({
            "platform": "Linux",
            "user_agent": "Mozilla/5.0 (Linux)..."
        });

        let result = merger.merge(&existing, &patch).unwrap();

        assert_eq!(result["platform"], "Linux");
        assert_eq!(result["user_agent"], "Mozilla/5.0 (Linux)...");
        assert_eq!(result["browser"], "Chrome"); // 未修改的保持
        assert_eq!(result["seed"], 12345); // 非白名单字段保持
        assert_eq!(result["custom_field"], "should_keep"); // 自定义字段保持
    }

    #[test]
    fn test_non_whitelist_field_not_lost() {
        let merger = FingerprintMerger::new();
        
        let existing = json!({
            "platform": "Windows",
            "seed": 12345,
            "internal_config": {"key": "value"}
        });

        let patch = json!({
            "platform": "Linux",
            "seed": 99999  // 尝试修改非白名单字段
        });

        let result = merger.merge(&existing, &patch).unwrap();

        assert_eq!(result["platform"], "Linux"); // 白名单字段更新
        assert_eq!(result["seed"], 12345); // 非白名单字段不变（透传）
        assert_eq!(result["internal_config"]["key"], "value"); // 保留
    }

    #[test]
    fn test_blacklist_field_rejected() {
        let merger = FingerprintMerger::new();
        
        let existing = json!({
            "platform": "Windows"
        });

        let patch = json!({
            "platform": "Linux",
            "debug_port": 9222  // 黑名单字段
        });

        let result = merger.merge(&existing, &patch);

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("禁止修改危险字段"));
    }

    #[test]
    fn test_validate_blacklist() {
        let merger = FingerprintMerger::new();

        let valid = json!({
            "platform": "Windows",
            "browser": "Chrome"
        });

        let invalid = json!({
            "platform": "Windows",
            "remote_debugging_port": 9222
        });

        assert!(merger.validate(&valid).is_ok());
        assert!(merger.validate(&invalid).is_err());
    }

    #[test]
    fn test_empty_patch() {
        let merger = FingerprintMerger::new();
        
        let existing = json!({
            "platform": "Windows",
            "seed": 12345
        });

        let patch = json!({});

        let result = merger.merge(&existing, &patch).unwrap();

        assert_eq!(result, existing); // 空 patch 不改变任何内容
    }
}
