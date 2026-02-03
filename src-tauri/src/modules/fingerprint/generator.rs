// Fingerprint Generator - 指纹生成器核心模块
// 整合所有组件生成完整的设备指纹配置（适配内核格式）

use super::templates::{TemplateManager, ResolutionOption};
use super::seed_manager::SeedManager;
use super::noise::{WebGLNoiseGenerator, CanvasNoiseGenerator, AudioNoiseGenerator};
use crate::modules::config_writer::*;
use rand::SeedableRng;
use rand::rngs::StdRng;
use std::path::Path;

/// 指纹生成器
pub struct FingerprintGenerator {
    template_manager: TemplateManager,
}

impl FingerprintGenerator {
    /// 创建指纹生成器
    pub fn new<P: AsRef<Path>>(template_path: P) -> Result<Self, String> {
        let template_manager = TemplateManager::load_from_file(template_path)?;
        Ok(Self { template_manager })
    }
    
    /// 生成完整指纹配置（内核格式）
    pub fn generate(&self, profile_id: &str, platform: Option<&str>, browser_version: Option<&str>) -> FingerprintFileConfig {
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
        
        // 根据平台选择一致的时区和语言
        let (timezone, language_primary, language_fallback) = self.get_locale_for_platform(target_platform);
        
        // 根据平台和版本生成 User-Agent
        let user_agent = self.generate_user_agent(target_platform, target_version, &resolution);
        
        // 5. 生成噪声
        let _webgl_noise = WebGLNoiseGenerator::generate(derived_seeds.webgl);
        let _canvas_noise = CanvasNoiseGenerator::generate_compact(derived_seeds.canvas);
        let audio_noise = AudioNoiseGenerator::generate(derived_seeds.audio);
        
        // 6. 构建内核格式配置
        FingerprintFileConfig {
            init: 2,  // 启用指纹功能
            
            ua: UaConfig {
                r#type: 2,
                user_agent,
            },
            
            resource_info: ResourceInfoConfig {
                r#type: 2,
                cpu: cores,
                memory: memory as f64,
            },
            
            resolution: ResolutionConfig {
                r#type: 2,
                monitor_width: resolution.width,
                monitor_height: resolution.height,
                avail_width: resolution.width,
                avail_height: resolution.height - 40,
                color_depth: 24,
            },
            
            time_zone: TimeZoneConfig {
                r#type: 2,
                gmt: timezone,
            },
            
            language: LanguageConfig {
                r#type: 2,
                interface_language: language_primary.clone(),
                languages: {
                    let mut langs = vec![language_primary];
                    langs.extend(language_fallback);
                    langs
                },
            },
            
            web_gl_device: WebGLDeviceConfig {
                r#type: 2,
                vendors: gpu_model.webgl_vendor.clone(),
                renderer: gpu_model.webgl_renderer.clone(),
            },
            
            canvas: CanvasConfig {
                r#type: 2,
                noise_enabled: true,
                noise_factor: audio_noise.noise_factor as f64,
            },
            
            audio_context: AudioContextConfig {
                r#type: 2,
                noise: vec![0.0001, 0.0002, 0.0001],
            },
            
            client_rects: ClientRectsConfig {
                r#type: 2,
                x: 0.0001,
                y: 0.0001,
                width: 0.0001,
                height: 0.0001,
            },
            
            location: LocationConfig {
                r#type: 0,  // 默认禁用
                permissions: false,
                latitude: None,
                longitude: None,
                accuracy: None,
            },
            
            webrtc: None,
            fonts: Some(FontsConfig {
                r#type: 2,
                mode: template.fonts.mode.clone(),
                list: template.fonts.common_fonts.clone(),
            }),
            variations: Some(VariationsConfig {
                r#type: 2,
                enabled: true,
                seed_id: derived_seeds.master.to_string(),
                seed_type: "profile_based".to_string(),
                field_trials: vec![],
                enabled_features: vec![],
                disabled_features: vec!["AutofillServerCommunication".to_string()],
            }),
        }
    }
    
    /// 从模板随机选择
    fn pick_random<T: Clone>(&self, options: &[T], rng: &mut StdRng) -> T {
        self.template_manager.pick_random(options, rng)
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
                format!(
                    "Mozilla/5.0 (Linux; Android 14; SM-S928B) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/{}.0.0.0 Mobile Safari/537.36",
                    version
                )
            }
            "ios" => {
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
                format!(
                    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/{}.0.0.0 Safari/537.36",
                    version
                )
            }
        }
    }
    
    /// 根据平台获取一致的时区和语言配置
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
