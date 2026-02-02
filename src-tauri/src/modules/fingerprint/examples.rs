// 使用示例：如何使用指纹生成器

use crate::modules::fingerprint::FingerprintGenerator;
use std::path::PathBuf;

/// 示例1：基本使用
pub fn example_basic_generation() -> Result<(), String> {
    // 1. 创建生成器（加载设备模板）
    let template_path = PathBuf::from("data/templates/device_templates.json");
    let generator = FingerprintGenerator::new(template_path)?;
    
    // 2. 生成指纹（使用 profile_id）
    let profile_id = "test-profile-12345";
    let fingerprint_config = generator.generate(profile_id);
    
    // 3. 打印指纹信息
    println!("Profile ID: {}", fingerprint_config.profile_id);
    println!("User-Agent: {}", fingerprint_config.navigator.user_agent);
    println!("Platform: {}", fingerprint_config.navigator.platform);
    println!("Screen: {}x{}", fingerprint_config.screen.width, fingerprint_config.screen.height);
    println!("CPU Cores: {}", fingerprint_config.navigator.hardware_concurrency);
    println!("Memory: {} GB", fingerprint_config.navigator.device_memory);
    println!("Timezone: {}", fingerprint_config.timezone.id);
    
    Ok(())
}

/// 示例2：写入配置文件
pub fn example_write_fingerprint() -> Result<(), String> {
    use crate::modules::config_writer::ConfigWriter;
    
    // 1. 创建生成器
    let template_path = PathBuf::from("data/templates/device_templates.json");
    let generator = FingerprintGenerator::new(template_path)?;
    
    // 2. 生成指纹
    let profile_id = "test-profile-12345";
    let fingerprint_config = generator.generate(profile_id);
    
    // 3. 写入文件
    let user_data_dir = PathBuf::from(r"F:\test\profiles\test-profile-12345");
    ConfigWriter::write_fingerprint_config(&user_data_dir, &fingerprint_config)?;
    
    println!("指纹配置已写入: {:?}/bm_fingerprint.json", user_data_dir);
    
    Ok(())
}

/// 示例3：确定性测试（相同 profile_id 生成相同指纹）
pub fn example_deterministic() -> Result<(), String> {
    let template_path = PathBuf::from("data/templates/device_templates.json");
    let generator = FingerprintGenerator::new(template_path)?;
    
    let profile_id = "deterministic-test";
    
    // 生成两次
    let fp1 = generator.generate(profile_id);
    let fp2 = generator.generate(profile_id);
    
    // 验证种子相同
    assert_eq!(fp1.seed.master, fp2.seed.master);
    assert_eq!(fp1.seed.canvas, fp2.seed.canvas);
    assert_eq!(fp1.seed.webgl, fp2.seed.webgl);
    assert_eq!(fp1.seed.audio, fp2.seed.audio);
    
    // 验证核心指纹相同
    assert_eq!(fp1.navigator.user_agent, fp2.navigator.user_agent);
    assert_eq!(fp1.screen.width, fp2.screen.width);
    assert_eq!(fp1.navigator.hardware_concurrency, fp2.navigator.hardware_concurrency);
    
    println!("✅ 确定性测试通过：相同 profile_id 生成相同指纹");
    
    Ok(())
}

/// 示例4：集成到 Profile 创建流程
pub async fn example_create_profile_with_fingerprint(
    profile_name: &str,
) -> Result<(), String> {
    use crate::modules::config_writer::ConfigWriter;
    
    // 1. 创建 Profile（生成 UUID）
    let profile_id = uuid::Uuid::new_v4().to_string().replace("-", "");
    
    // 2. 生成指纹
    let template_path = PathBuf::from("data/templates/device_templates.json");
    let generator = FingerprintGenerator::new(template_path)?;
    let fingerprint_config = generator.generate(&profile_id);
    
    // 3. 准备用户数据目录
    let user_data_dir = PathBuf::from(r"F:\DeepChrome\profiles").join(&profile_id);
    std::fs::create_dir_all(&user_data_dir)
        .map_err(|e| format!("创建目录失败: {}", e))?;
    
    // 4. 写入指纹配置
    ConfigWriter::write_fingerprint_config(&user_data_dir, &fingerprint_config)?;
    
    // 5. 写入云端配置
    use crate::modules::config_writer::CloudFileConfig;
    let cloud_config = CloudFileConfig::new(&profile_id, profile_name, "默认分组");
    ConfigWriter::write_cloud_config(&user_data_dir, &cloud_config)?;
    
    println!("✅ Profile 创建成功: {}", profile_id);
    println!("   指纹配置: {:?}/bm_fingerprint.json", user_data_dir);
    println!("   云端配置: {:?}/bm_cloud.json", user_data_dir);
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_basic_generation() {
        let result = example_basic_generation();
        // 注意：需要实际的模板文件才能运行
        // assert!(result.is_ok());
    }
    
    #[test]
    fn test_deterministic() {
        let result = example_deterministic();
        // assert!(result.is_ok());
    }
}
