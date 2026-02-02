// Device Templates - 设备模板系统
// 管理设备模板的加载、验证和选择

use serde::{Deserialize, Serialize};
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use std::path::Path;

/// 模板文件根结构
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DeviceTemplateFile {
    #[serde(rename = "$schema")]
    pub schema: String,
    pub version: String,
    pub created_at: String,
    pub description: String,
    pub templates: Vec<DeviceTemplate>,
}

/// 设备模板
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DeviceTemplate {
    pub id: String,
    pub description: String,
    pub weight: f32,
    pub os: OsConfig,
    pub cpu: CpuConfig,
    pub memory: MemoryConfig,
    pub gpu: GpuConfig,
    pub screen: ScreenConfig,
    pub browser: BrowserConfig,
    pub fonts: FontsConfig,
    pub locale: LocaleConfig,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OsConfig {
    pub name: String,
    pub version: String,
    pub platform: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CpuConfig {
    pub vendor: String,
    pub cores: Vec<u32>,
    pub frequency_range: [u32; 2],
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MemoryConfig {
    pub options_gb: Vec<u32>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GpuConfig {
    pub vendor: String,
    pub models: Vec<GpuModel>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GpuModel {
    pub name: String,
    pub vram_gb: u32,
    pub webgl_vendor: String,
    pub webgl_renderer: String,
    pub unmasked_vendor: String,
    pub unmasked_renderer: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ScreenConfig {
    pub resolutions: Vec<ResolutionOption>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ResolutionOption {
    pub width: u32,
    pub height: u32,
    pub weight: f32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BrowserConfig {
    pub user_agents: Vec<String>,
    pub chrome_version_range: [u32; 2],
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FontsConfig {
    pub mode: String,
    pub common_fonts: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LocaleConfig {
    pub timezones: Vec<String>,
    pub languages: Vec<LanguageOption>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LanguageOption {
    pub primary: String,
    pub fallback: Vec<String>,
}

/// 模板管理器
pub struct TemplateManager {
    templates: Vec<DeviceTemplate>,
}

impl TemplateManager {
    /// 从文件加载模板
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| format!("读取模板文件失败: {}", e))?;
        
        let file_root: DeviceTemplateFile = serde_json::from_str(&content)
            .map_err(|e| format!("解析模板文件失败: {}", e))?;
        
        // 验证权重总和
        let total_weight: f32 = file_root.templates.iter().map(|t| t.weight).sum();
        if (total_weight - 1.0).abs() > 0.01 {
            return Err(format!("模板权重总和必须为 1.0，当前为 {}", total_weight));
        }
        
        Ok(Self { 
            templates: file_root.templates 
        })
    }
    
    /// 按权重随机选择模板
    pub fn pick_template(&self, rng: &mut StdRng) -> &DeviceTemplate {
        let total_weight: f32 = self.templates.iter().map(|t| t.weight).sum();
        let mut pick = rng.gen::<f32>() * total_weight;
        
        for template in &self.templates {
            pick -= template.weight;
            if pick <= 0.0 {
                return template;
            }
        }
        
        // 回退到第一个模板
        &self.templates[0]
    }
    
    /// 按 ID 获取模板
    pub fn get_template_by_id(&self, id: &str) -> Option<&DeviceTemplate> {
        self.templates.iter().find(|t| t.id == id)
    }
    
    /// 获取所有模板
    pub fn get_all_templates(&self) -> &[DeviceTemplate] {
        &self.templates
    }
    
    /// 从模板随机选择配置项
    pub fn pick_random<T: Clone>(&self, options: &[T], rng: &mut StdRng) -> T {
        if options.is_empty() {
            panic!("Cannot pick from empty options");
        }
        options[rng.gen_range(0..options.len())].clone()
    }
    
    /// 按权重选择分辨率
    pub fn pick_weighted_resolution(
        &self,
        options: &[ResolutionOption],
        rng: &mut StdRng,
    ) -> ResolutionOption {
        let total_weight: f32 = options.iter().map(|r| r.weight).sum();
        let mut pick = rng.gen::<f32>() * total_weight;
        
        for res in options {
            pick -= res.weight;
            if pick <= 0.0 {
                return res.clone();
            }
        }
        
        options[0].clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_pick_template() {
        // 创建测试模板
        let templates = vec![
            DeviceTemplate {
                id: "template1".to_string(),
                description: "Test 1".to_string(),
                weight: 0.5,
                os: OsConfig {
                    name: "Windows".to_string(),
                    version: "10.0".to_string(),
                    platform: "Win32".to_string(),
                },
                cpu: CpuConfig {
                    vendor: "Intel".to_string(),
                    cores: vec![4],
                    frequency_range: [2000, 3000],
                },
                memory: MemoryConfig {
                    options_gb: vec![8, 16],
                },
                gpu: GpuConfig {
                    vendor: "NVIDIA".to_string(),
                    models: vec![],
                },
                screen: ScreenConfig {
                    resolutions: vec![],
                },
                browser: BrowserConfig {
                    user_agents: vec![],
                    chrome_version_range: [131, 134],
                },
                fonts: FontsConfig {
                    mode: "subset".to_string(),
                    common_fonts: vec![],
                },
                locale: LocaleConfig {
                    timezones: vec![],
                    languages: vec![],
                },
            },
        ];
        
        let manager = TemplateManager { templates };
        let mut rng = StdRng::seed_from_u64(12345);
        
        let picked = manager.pick_template(&mut rng);
        assert_eq!(picked.id, "template1");
    }
}
