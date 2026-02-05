// Fingerprint Generator - 指纹生成器核心模块
// 整合所有组件生成完整的设备指纹配置
// 格式与 Chromium 内核 fingerprint_browser 模块兼容

use super::templates::{TemplateManager, ResolutionOption};
use super::seed_manager::{SeedManager};
use super::noise::{WebGLNoiseGenerator, CanvasNoiseGenerator, AudioNoiseGenerator};
use crate::modules::config_writer::{
    FingerprintFileConfig,
    KernelUaConfig,
    KernelResourceInfoConfig,
    KernelResolutionConfig,
    KernelTimeZoneConfig,
    KernelLanguageConfig,
    KernelWebGLDeviceConfig,
    KernelCanvasConfig,
    KernelCanvasColoredPoint,
    KernelAudioContextConfig,
    KernelFontConfig,
    KernelClientRectsConfig,
    KernelWebRtcConfig,
    KernelLocationConfig,
    KernelVariationsConfig,
    KernelBatteryConfig,
    KernelNetworkInfoConfig,
    KernelMediaEquipmentConfig,
    KernelDeviceConfig,
    KernelDoNotTrackConfig,
    KernelOpenPortConfig,
    KernelWebGpuConfig,
};
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
    
    /// 生成完整指纹配置（匹配内核 bm_fingerprint.json 格式）
    /// 
    /// # Arguments
    /// * `profile_id` - Profile 唯一标识（必须由外部提供）
    /// * `platform` - 目标平台（可选：windows/macos/android/ios）
    /// * `browser_version` - 浏览器版本（可选：139/146等）
    /// 
    /// # Returns
    /// 完整的指纹配置，可直接写入 bm_fingerprint.json
    pub fn generate(&self, profile_id: &str, platform: Option<&str>, browser_version: Option<&str>) -> FingerprintFileConfig {
        // 获取平台和版本参数
        let target_platform = platform.unwrap_or("windows");
        let target_version = browser_version.unwrap_or("139");
        
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
        
        // 生成完整语言列表
        let mut languages = vec![language_primary.clone()];
        // 添加主语言的简写（如 zh-CN -> zh）
        if let Some(short) = language_primary.split('-').next() {
            if short != language_primary {
                languages.push(short.to_string());
            }
        }
        // 添加备用语言
        languages.extend(language_fallback);
        
        // 根据平台和版本生成 User-Agent
        let user_agent = self.generate_user_agent(target_platform, target_version, &resolution);
        
        // 5. 生成噪声参数
        let _webgl_noise = WebGLNoiseGenerator::generate(derived_seeds.webgl);
        let canvas_noise_config = CanvasNoiseGenerator::generate_compact(derived_seeds.canvas);
        let audio_noise_config = AudioNoiseGenerator::generate(derived_seeds.audio);
        
        // 转换 Audio 噪声为 Vec<f64> 格式
        let audio_noise = vec![
            audio_noise_config.noise_factor as f64,
            -audio_noise_config.noise_factor as f64 * 2.0,
            audio_noise_config.noise_factor as f64,
            -audio_noise_config.noise_factor as f64,
            audio_noise_config.noise_factor as f64 * 2.0,
        ];
        
        // 转换 Canvas 噪声为 KernelCanvasColoredPoint 格式
        let colored_point_list = vec![
            KernelCanvasColoredPoint {
                row: 10,
                column: 10,
                red: canvas_noise_config.rgb_noise.get(0).copied().unwrap_or(1),
                green: canvas_noise_config.rgb_noise.get(1).copied().unwrap_or(-1),
                blue: canvas_noise_config.rgb_noise.get(2).copied().unwrap_or(2),
                alpha: 0,
            },
            KernelCanvasColoredPoint {
                row: 50,
                column: 50,
                red: canvas_noise_config.rgb_noise.get(3).copied().unwrap_or(-2),
                green: canvas_noise_config.rgb_noise.get(4).copied().unwrap_or(1),
                blue: canvas_noise_config.rgb_noise.get(5).copied().unwrap_or(-1),
                alpha: 0,
            },
            KernelCanvasColoredPoint {
                row: 100,
                column: 100,
                red: canvas_noise_config.rgb_noise.get(6).copied().unwrap_or(2),
                green: canvas_noise_config.rgb_noise.get(7).copied().unwrap_or(-2),
                blue: canvas_noise_config.rgb_noise.get(8).copied().unwrap_or(1),
                alpha: 0,
            },
        ];
        
        // 6. 构建完整配置（匹配内核格式）
        FingerprintFileConfig {
            init: 2,
            seed: profile_id.to_string(),
            
            device: KernelDeviceConfig {
                config_type: 2,
                name: format!("DESKTOP-{}", &profile_id[..8].to_uppercase()),
                mac_address: {
                    let mac_bytes: Vec<u8> = (0..6)
                        .map(|i| ((derived_seeds.master >> (i * 8)) & 0xFF) as u8)
                        .collect();
                    format!(
                        "{:02X}-{:02X}-{:02X}-{:02X}-{:02X}-{:02X}",
                        mac_bytes[0], mac_bytes[1], mac_bytes[2],
                        mac_bytes[3], mac_bytes[4], mac_bytes[5]
                    )
                },
            },
            
            ua: KernelUaConfig {
                config_type: 2,
                user_agent: user_agent.clone(),
            },
            
            resource_info: KernelResourceInfoConfig {
                config_type: 2,
                cpu: cores,
                memory: memory as f32,
            },
            
            resolution: KernelResolutionConfig {
                config_type: 2,
                monitor_width: resolution.width,
                monitor_height: resolution.height,
                color_depth: 24,
                avail_width: resolution.width,
                avail_height: resolution.height.saturating_sub(40),
            },
            
            time_zone: KernelTimeZoneConfig {
                config_type: 2,
                gmt: timezone.clone(),
            },
            
            language: KernelLanguageConfig {
                config_type: 2,
                interface_language: language_primary.clone(),
                languages: languages.clone(),
            },
            
            webgl_device: KernelWebGLDeviceConfig {
                config_type: 2,
                vendors: gpu_model.webgl_vendor.clone(),
                renderer: gpu_model.webgl_renderer.clone(),
            },
            
            canvas: KernelCanvasConfig {
                config_type: 2,
                colored_point_list,
            },
            
            audio_context: KernelAudioContextConfig {
                config_type: 2,
                noise: audio_noise,
            },
            
            font: KernelFontConfig::default(),
            
            webrtc: KernelWebRtcConfig::default(),
            
            client_rects: KernelClientRectsConfig::default(),
            
            location: KernelLocationConfig::default(),
            
            variations: KernelVariationsConfig::default(),
            
            battery: KernelBatteryConfig::default(),
            
            network_info: KernelNetworkInfoConfig::default(),
            
            media_equipment: KernelMediaEquipmentConfig::default(),
            
            do_not_track: KernelDoNotTrackConfig::default(),
            
            open_port: KernelOpenPortConfig::default(),
            
            webgpu: KernelWebGpuConfig::default(),
        }
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
