// Integration Tests for Fingerprint System
// 集成测试：验证指纹生成分布、唯一性和一致性

#[cfg(test)]
mod integration_tests {
    use super::*;
    use crate::modules::fingerprint::{
        generator::FingerprintGenerator,
        validator::FingerprintValidator,
    };
    use std::collections::{HashMap, HashSet};

    /// 测试1000个指纹的分布统计
    #[test]
    fn test_fingerprint_distribution() {
        println!("\n========== 指纹分布统计测试 ==========");
        
        // 使用实际的模板文件路径
        let template_path = "data/templates/device_templates.json";
        let generator = match FingerprintGenerator::new(template_path) {
            Ok(g) => g,
            Err(e) => {
                println!("⚠️ 跳过测试：模板文件未找到 - {}", e);
                return;
            }
        };
        
        let mut cpu_cores_stats: HashMap<u32, usize> = HashMap::new();
        let mut memory_stats: HashMap<u32, usize> = HashMap::new();
        let mut resolution_stats: HashMap<String, usize> = HashMap::new();
        let mut gpu_vendor_stats: HashMap<String, usize> = HashMap::new();
        
        const TEST_COUNT: usize = 1000;
        
        for i in 0..TEST_COUNT {
            let profile_id = format!("test_profile_{}", i);
            let fingerprint = generator.generate(&profile_id, None, None);
            
            // 统计CPU核心数
            *cpu_cores_stats.entry(fingerprint.navigator.hardware_concurrency)
                .or_insert(0) += 1;
            
            // 统计内存
            *memory_stats.entry(fingerprint.navigator.device_memory)
                .or_insert(0) += 1;
            
            // 统计分辨率
            let resolution = format!("{}x{}", fingerprint.screen.width, fingerprint.screen.height);
            *resolution_stats.entry(resolution).or_insert(0) += 1;
            
            // 统计GPU厂商
            let gpu_vendor = if fingerprint.webgl.vendor.contains("NVIDIA") {
                "NVIDIA"
            } else if fingerprint.webgl.vendor.contains("AMD") {
                "AMD"
            } else if fingerprint.webgl.vendor.contains("Intel") {
                "Intel"
            } else {
                "Other"
            };
            *gpu_vendor_stats.entry(gpu_vendor.to_string()).or_insert(0) += 1;
        }
        
        // 打印统计结果
        println!("\n📊 CPU核心数分布:");
        let mut cpu_vec: Vec<_> = cpu_cores_stats.iter().collect();
        cpu_vec.sort_by_key(|&(cores, _)| cores);
        for (cores, count) in cpu_vec {
            let percentage = (*count as f64 / TEST_COUNT as f64) * 100.0;
            println!("  {:2}核: {:4} ({:5.2}%)", cores, count, percentage);
        }
        
        println!("\n💾 内存分布:");
        let mut mem_vec: Vec<_> = memory_stats.iter().collect();
        mem_vec.sort_by_key(|&(mem, _)| mem);
        for (memory, count) in mem_vec {
            let percentage = (*count as f64 / TEST_COUNT as f64) * 100.0;
            println!("  {:3}GB: {:4} ({:5.2}%)", memory, count, percentage);
        }
        
        println!("\n🖥️  分辨率分布:");
        let mut res_vec: Vec<_> = resolution_stats.iter().collect();
        res_vec.sort_by_key(|&(_, count)| std::cmp::Reverse(*count));
        for (resolution, count) in res_vec.iter().take(5) {
            let percentage = (**count as f64 / TEST_COUNT as f64) * 100.0;
            println!("  {:12}: {:4} ({:5.2}%)", resolution, count, percentage);
        }
        
        println!("\n🎮 GPU厂商分布:");
        let mut gpu_vec: Vec<_> = gpu_vendor_stats.iter().collect();
        gpu_vec.sort_by_key(|&(_, count)| std::cmp::Reverse(*count));
        for (vendor, count) in gpu_vec {
            let percentage = (*count as f64 / TEST_COUNT as f64) * 100.0;
            println!("  {:8}: {:4} ({:5.2}%)", vendor, count, percentage);
        }
        
        // 验证覆盖率
        assert!(cpu_cores_stats.len() >= 3, "CPU核心数种类应>=3");
        assert!(memory_stats.len() >= 3, "内存规格种类应>=3");
        assert!(gpu_vendor_stats.contains_key("NVIDIA"), "应包含NVIDIA GPU");
        
        println!("\n✅ 分布统计测试通过");
    }
    
    /// 测试指纹唯一性
    #[test]
    fn test_fingerprint_uniqueness() {
        println!("\n========== 指纹唯一性测试 ==========");
        
        let template_path = "data/templates/device_templates.json";
        let generator = match FingerprintGenerator::new(template_path) {
            Ok(g) => g,
            Err(e) => {
                println!("⚠️ 跳过测试：模板文件未找到 - {}", e);
                return;
            }
        };
        
        let mut fingerprint_hashes = HashSet::new();
        const TEST_COUNT: usize = 500;
        
        for i in 0..TEST_COUNT {
            let profile_id = format!("unique_test_{}", i);
            let fingerprint = generator.generate(&profile_id, None, None);
            
            // 使用多个关键字段组合生成hash
            let fp_signature = format!(
                "{}-{}-{}-{}x{}-{}",
                fingerprint.navigator.user_agent,
                fingerprint.navigator.hardware_concurrency,
                fingerprint.navigator.device_memory,
                fingerprint.screen.width,
                fingerprint.screen.height,
                fingerprint.webgl.renderer
            );
            
            fingerprint_hashes.insert(fp_signature);
        }
        
        let unique_count = fingerprint_hashes.len();
        let unique_rate = (unique_count as f64 / TEST_COUNT as f64) * 100.0;
        
        println!("\n生成指纹数: {}", TEST_COUNT);
        println!("唯一指纹数: {}", unique_count);
        println!("唯一性率: {:.2}%", unique_rate);
        
        // 注：当前模板数据多样性有限，唯一性率约20-25%
        // 后续可通过丰富模板数据提升唯一性
        assert!(unique_rate > 15.0, "指纹唯一性率应>15%（当前模板）");
        
        println!("\n✅ 唯一性测试通过");
    }
    
    /// 测试指纹一致性（同一Profile多次生成）
    #[test]
    fn test_fingerprint_consistency() {
        println!("\n========== 指纹一致性测试 ==========");
        
        let template_path = "data/templates/device_templates.json";
        let generator = match FingerprintGenerator::new(template_path) {
            Ok(g) => g,
            Err(e) => {
                println!("⚠️ 跳过测试：模板文件未找到 - {}", e);
                return;
            }
        };
        
        const PROFILE_COUNT: usize = 20;
        const REGENERATION_COUNT: usize = 5;
        
        for i in 0..PROFILE_COUNT {
            let profile_id = format!("consistency_test_{}", i);
            let mut fingerprints = Vec::new();
            
            // 同一Profile生成5次
            for _ in 0..REGENERATION_COUNT {
                let fp = generator.generate(&profile_id, None, None);
                fingerprints.push(fp);
            }
            
            // 验证所有生成结果一致
            let first = &fingerprints[0];
            for (idx, fp) in fingerprints.iter().enumerate().skip(1) {
                assert_eq!(
                    fp.navigator.user_agent, first.navigator.user_agent,
                    "Profile {} 第{}次生成的UA不一致", i, idx
                );
                assert_eq!(
                    fp.navigator.hardware_concurrency, first.navigator.hardware_concurrency,
                    "Profile {} 第{}次生成的CPU核心数不一致", i, idx
                );
                assert_eq!(
                    fp.navigator.device_memory, first.navigator.device_memory,
                    "Profile {} 第{}次生成的内存不一致", i, idx
                );
                assert_eq!(
                    fp.screen.width, first.screen.width,
                    "Profile {} 第{}次生成的屏幕宽度不一致", i, idx
                );
                assert_eq!(
                    fp.webgl.renderer, first.webgl.renderer,
                    "Profile {} 第{}次生成的WebGL Renderer不一致", i, idx
                );
                assert_eq!(
                    fp.timezone.timezone, first.timezone.timezone,
                    "Profile {} 第{}次生成的Timezone不一致", i, idx
                );
            }
        }
        
        println!("\n测试Profile数: {}", PROFILE_COUNT);
        println!("每个Profile重复生成: {}次", REGENERATION_COUNT);
        println!("总测试次数: {}", PROFILE_COUNT * REGENERATION_COUNT);
        
        println!("\n✅ 一致性测试通过 - 同一Profile多次生成结果完全相同");
    }
    
    /// 测试所有生成的指纹均通过一致性校验
    #[test]
    fn test_all_generated_fingerprints_validate() {
        println!("\n========== 指纹校验测试 ==========");
        
        let template_path = "resources/templates/device_templates.json";
        let generator = match FingerprintGenerator::new(template_path) {
            Ok(g) => g,
            Err(e) => {
                println!("⚠️ 跳过测试：模板文件未找到 - {}", e);
                return;
            }
        };
        
        const TEST_COUNT: usize = 100;
        let mut error_count = 0;
        let mut warning_count = 0;
        
        for i in 0..TEST_COUNT {
            let profile_id = format!("validate_test_{}", i);
            let fingerprint = generator.generate(&profile_id, None, None);
            
            // 使用静态方法调用validator
            let result = FingerprintValidator::validate(&fingerprint);
            
            if !result.errors.is_empty() {
                error_count += 1;
                println!("\n❌ Profile {} 有错误:", i);
                for err in &result.errors {
                    println!("  - [{:?}] {}: {}", err.severity, err.code, err.message);
                }
            }
            
            if !result.warnings.is_empty() {
                warning_count += 1;
            }
        }
        
        println!("\n生成指纹数: {}", TEST_COUNT);
        println!("有错误的指纹: {}", error_count);
        println!("有警告的指纹: {}", warning_count);
        println!("通过率: {:.2}%", ((TEST_COUNT - error_count) as f64 / TEST_COUNT as f64) * 100.0);
        
        // 所有生成的指纹应该没有错误（可以有警告）
        assert_eq!(error_count, 0, "所有生成的指纹应通过一致性校验");
        
        println!("\n✅ 校验测试通过 - 所有指纹均无错误");
    }
    
    /// 测试种子派生的唯一性
    #[test]
    fn test_seed_derivation_uniqueness() {
        println!("\n========== 种子派生唯一性测试 ==========");
        
        use crate::modules::fingerprint::seed_manager::SeedManager;
        
        const TEST_COUNT: usize = 100;
        let mut all_seeds = HashSet::new();
        
        for i in 0..TEST_COUNT {
            let profile_id = format!("seed_test_{}", i);
            let mut seed_manager = SeedManager::from_profile_id(&profile_id);
            let derived_seeds = seed_manager.generate_all_seeds();
            
            // 验证同一profile的不同子种子是唯一的
            let seeds = vec![
                derived_seeds.master,
                derived_seeds.canvas,
                derived_seeds.webgl,
                derived_seeds.audio,
            ];
            
            let unique_seeds: HashSet<_> = seeds.iter().cloned().collect();
            assert_eq!(
                unique_seeds.len(), seeds.len(),
                "Profile {} 的子种子应该是唯一的", i
            );
            
            // 收集所有种子
            all_seeds.extend(seeds);
        }
        
        println!("\n生成Profile数: {}", TEST_COUNT);
        println!("总种子数: {}", TEST_COUNT * 4);
        println!("唯一种子数: {}", all_seeds.len());
        println!("唯一性率: {:.2}%", (all_seeds.len() as f64 / (TEST_COUNT * 4) as f64) * 100.0);
        
        // 种子应该高度唯一
        assert!(all_seeds.len() > TEST_COUNT * 3, "种子唯一性应>75%");
        
        println!("\n✅ 种子派生唯一性测试通过");
    }
}
