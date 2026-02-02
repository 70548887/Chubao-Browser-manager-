// Fingerprint Generator - 指纹生成器核心模块
// 整合所有组件生成完整的设备指纹配置

use super::templates::{TemplateManager, ResolutionOption};
use super::seed_manager::{SeedManager};
use super::noise::{WebGLNoiseGenerator, CanvasNoiseGenerator, AudioNoiseGenerator};
use crate::modules::config_writer::*;
use rand::{SeedableRng};
use rand::rngs::StdRng;
use std::path::Path;

/// 指纹生成器
pub struct FingerprintGenerator {
    template_manager: TemplateManager,
}

impl FingerprintGenerator {
    /// 创建指纹生成器
    /// 
    /// # Arguments
    /// * `template_path` - 设备模板文件路径
    pub fn new<P: AsRef<Path>>(template_path: P) -> Result<Self, String> {
        let template_manager = TemplateManager::load_from_file(template_path)?;
        Ok(Self { template_manager })
    }
    
    /// 生成完整指纹配置
    /// 
    /// # Arguments
    /// * `profile_id` - Profile 唯一标识（必须由外部提供）
    /// * `platform` - 目标平台（可选：windows/macos/android/ios）
    /// * `browser_version` - 浏览器版本（可选：146/145/144等）
    /// 
    /// # Returns
    /// 完整的指纹配置，可直接写入 bm_fingerprint.json
    pub fn generate(&self, profile_id: &str, platform: Option<&str>, browser_version: Option<&str>) -> FingerprintFileConfig {
        // 获取平台和版本参数
        let target_platform = platform.unwrap_or("windows");
        let target_version = browser_version.unwrap_or("146");
        
        // 1. 创建种子管理器
        let mut seed_manager = SeedManager::from_profile_id(profile_id);
        let derived_seeds = seed_manager.generate_all_seeds();
        
        // 2. 创建随机数生成器
        let mut rng = StdRng::seed_from_u64(derived_seeds.master);
        
        // 3. 选择设备模板
        let template = self.template_manager.pick_template(&mut rng);
        
        // 4. 从模板中随机选择具体配置
        let cores = self.pick_random(&template.cpu.cores, &mut rng);
        let memory = self.pick_random(&template.memory.options_gb, &mut rng);
        let gpu_model = self.pick_random(&template.gpu.models, &mut rng);
        let resolution = self.template_manager.pick_weighted_resolution(
            &template.screen.resolutions, 
            &mut rng
        );
        
        // 根据平台选择一致的时区和语言（保持地理一致性）
        let (timezone, language_primary, language_fallback) = self.get_locale_for_platform(target_platform);
        
        // 根据平台和版本生成 User-Agent
        let user_agent = self.generate_user_agent(target_platform, target_version, &resolution);
        
        // 5. 生成噪声
        let _webgl_noise = WebGLNoiseGenerator::generate(derived_seeds.webgl);
        let _canvas_noise = CanvasNoiseGenerator::generate_compact(derived_seeds.canvas);
        let audio_noise = AudioNoiseGenerator::generate(derived_seeds.audio);
        
        // 6. 构建完整配置
        FingerprintFileConfig {
            schema: "bm_fingerprint_v2".to_string(),
            schema_version: 2,
            profile_id: profile_id.to_string(),
            created_at: chrono::Utc::now().to_rfc3339(),
            signature: None,
            
            seed: SeedConfig {
                master: derived_seeds.master as i64,
                canvas: derived_seeds.canvas as i64,
                webgl: derived_seeds.webgl as i64,
                audio: derived_seeds.audio as i64,
            },
            
            navigator: NavigatorConfig {
                user_agent: user_agent.clone(),
                platform: self.get_navigator_platform(target_platform),
                hardware_concurrency: cores,
                device_memory: memory,
                language: language_primary.clone(),
                languages: {
                    let mut langs = vec![language_primary.clone()];
                    langs.extend(language_fallback.clone());
                    langs
                },
                vendor: "Google Inc.".to_string(),
                app_version: format!("5.0 ({}) AppleWebKit/537.36", self.get_navigator_platform(target_platform)),
                max_touch_points: self.get_max_touch_points(target_platform),
                do_not_track: None,
                webdriver: false,
                pdf_viewer_enabled: true,
                cookie_enabled: true,
            },
            
            screen: ScreenConfig {
                width: resolution.width,
                height: resolution.height,
                avail_width: resolution.width,
                avail_height: resolution.height - 40,
                color_depth: 24,
                pixel_depth: 24,
                device_pixel_ratio: 1.0,
                orientation_type: "landscape-primary".to_string(),
                orientation_angle: 0,
            },
            
            webgl: WebGLConfig {
                vendor: gpu_model.webgl_vendor.clone(),
                renderer: gpu_model.webgl_renderer.clone(),
                unmasked_vendor: gpu_model.unmasked_vendor.clone(),
                unmasked_renderer: gpu_model.unmasked_renderer.clone(),
                noise_enabled: true,
                version: "WebGL 1.0 (OpenGL ES 2.0 Chromium)".to_string(),
                shading_language_version: "WebGL GLSL ES 1.0 (OpenGL ES GLSL ES 1.0 Chromium)".to_string(),
                max_texture_size: 16384,
                max_vertex_attribs: 16,
            },
            
            timezone: TimezoneConfig {
                id: timezone.clone(),
                offset_minutes: self.timezone_to_offset(&timezone),
            },
            
            fonts: FontsConfig {
                mode: template.fonts.mode.clone(),
                list: template.fonts.common_fonts.clone(),
            },
            
            canvas: CanvasConfig {
                mode: "noise".to_string(),
                noise_level: audio_noise.noise_factor as f64,
            },
            
            audio: AudioConfig {
                mode: "noise".to_string(),
                sample_rate: 44100,
                max_channel_count: 2,
            },
            
            webrtc: WebRTCConfig {
                mode: "disabled".to_string(),
                public_ip: None,
                local_ip: None,
            },
            
            geolocation: GeolocationConfig::default(),
            client_hints: Self::build_client_hints(&user_agent, target_platform),
            battery: BatteryConfig::default(),
            network: NetworkConfig::default(),
            privacy: PrivacyConfig::default(),
            device: DeviceConfig::default(),
        }
    }
    
    /// 从 User-Agent 构建 Client Hints（保持版本一致性）
    fn build_client_hints(user_agent: &str, platform: &str) -> ClientHintsConfig {
        // 提取 Chrome 版本号
        let chrome_version = Self::extract_chrome_version(user_agent).unwrap_or("146".to_string());
        let full_version = format!("{}.0.0.0", chrome_version);
        
        // 根据平台参数设置
        let (platform_name, platform_version, is_mobile, arch, bitness) = match platform {
            "windows" => ("Windows".to_string(), "10.0.0".to_string(), false, "x86".to_string(), "64".to_string()),
            "macos" => ("macOS".to_string(), "13.0.0".to_string(), false, "arm".to_string(), "64".to_string()),
            "android" => ("Android".to_string(), "14.0.0".to_string(), true, "arm".to_string(), "64".to_string()),
            "ios" => ("iOS".to_string(), "17.4.0".to_string(), true, "arm".to_string(), "64".to_string()),
            "linux" => ("Linux".to_string(), "6.0.0".to_string(), false, "x86".to_string(), "64".to_string()),
            _ => ("Windows".to_string(), "10.0.0".to_string(), false, "x86".to_string(), "64".to_string()),
        };
        
        ClientHintsConfig {
            brands: vec![
                ClientHintsBrand { brand: "Not_A Brand".to_string(), version: "8".to_string() },
                ClientHintsBrand { brand: "Chromium".to_string(), version: chrome_version.clone() },
                ClientHintsBrand { brand: "Google Chrome".to_string(), version: chrome_version },
            ],
            full_version,
            platform: platform_name,
            platform_version,
            architecture: arch,
            bitness,
            model: if is_mobile { "SM-S928B".to_string() } else { "".to_string() },
            mobile: is_mobile,
            wow64: false,
        }
    }
    
    /// 从 User-Agent 提取 Chrome 版本号
    fn extract_chrome_version(user_agent: &str) -> Option<String> {
        // 示例: "Chrome/134.0.0.0"
        if let Some(start) = user_agent.find("Chrome/") {
            let version_str = &user_agent[start + 7..];
            if let Some(end) = version_str.find('.') {
                return Some(version_str[..end].to_string());
            }
        }
        None
    }
    
    /// 从模板随机选择
    fn pick_random<T: Clone>(&self, options: &[T], rng: &mut StdRng) -> T {
        self.template_manager.pick_random(options, rng)
    }
    
    /// 时区 ID 转偏移量（分钟）
    fn timezone_to_offset(&self, timezone: &str) -> i32 {
        match timezone {
            "Asia/Shanghai" | "Asia/Hong_Kong" => -480,
            "Asia/Tokyo" => -540,
            "Asia/Seoul" => -540,
            "America/New_York" => 300,
            "America/Los_Angeles" => 480,
            "America/Chicago" => 360,
            "Europe/London" => 0,
            "Europe/Paris" | "Europe/Berlin" => -60,
            "UTC" => 0,
            _ => -480, // 默认 UTC+8
        }
    }
    
    /// 根据平台和版本生成 User-Agent
    fn generate_user_agent(&self, platform: &str, version: &str, _resolution: &ResolutionOption) -> String {
        match platform {
            "windows" => {
                format!(
                    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/{}.0.0.0 Safari/537.36",
                    version
                )
            }
            "macos" => {
                format!(
                    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/{}.0.0.0 Safari/537.36",
                    version
                )
            }
            "android" => {
                // Android 使用移动版 User-Agent
                format!(
                    "Mozilla/5.0 (Linux; Android 14; SM-S928B) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/{}.0.0.0 Mobile Safari/537.36",
                    version
                )
            }
            "ios" => {
                // iOS 使用 Safari 或 Chrome
                format!(
                    "Mozilla/5.0 (iPhone; CPU iPhone OS 17_4 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) CriOS/{}.0.0.0 Mobile/15E148 Safari/604.1",
                    version
                )
            }
            "linux" => {
                format!(
                    "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/{}.0.0.0 Safari/537.36",
                    version
                )
            }
            _ => {
                // 默认 Windows
                format!(
                    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/{}.0.0.0 Safari/537.36",
                    version
                )
            }
        }
    }
    
    /// 获取 navigator.platform 值
    fn get_navigator_platform(&self, platform: &str) -> String {
        match platform {
            "windows" => "Win32".to_string(),
            "macos" => "MacIntel".to_string(),
            "android" => "Linux armv81".to_string(),
            "ios" => "iPhone".to_string(),
            "linux" => "Linux x86_64".to_string(),
            _ => "Win32".to_string(),
        }
    }
    
    /// 获取触摸点数量
    fn get_max_touch_points(&self, platform: &str) -> u32 {
        match platform {
            "android" | "ios" => 5,  // 移动设备支持多点触摸
            _ => 0,  // 桌面设备默认不支持
        }
    }
    
    /// 根据平台获取一致的时区和语言配置
    /// 确保时区、语言、平台三者地理一致，避免检测
    fn get_locale_for_platform(&self, platform: &str) -> (String, String, Vec<String>) {
        match platform {
            "windows" => (
                "America/New_York".to_string(),
                "en-US".to_string(),
                vec!["en".to_string()]
            ),
            "macos" => (
                "America/Los_Angeles".to_string(),
                "en-US".to_string(),
                vec!["en".to_string()]
            ),
            "android" => (
                "Asia/Shanghai".to_string(),
                "zh-CN".to_string(),
                vec!["zh".to_string(), "en-US".to_string()]
            ),
            "ios" => (
                "Asia/Shanghai".to_string(),
                "zh-CN".to_string(),
                vec!["zh".to_string(), "en-US".to_string()]
            ),
            "linux" => (
                "Europe/London".to_string(),
                "en-GB".to_string(),
                vec!["en".to_string()]
            ),
            _ => (
                "America/New_York".to_string(),
                "en-US".to_string(),
                vec!["en".to_string()]
            ),
        }
    }
}
