// Seed Manager - 种子管理系统
// 基于 SHA256 主种子派生，确保指纹的确定性和稳定性

use sha2::{Sha256, Digest};
use std::collections::HashMap;

/// 种子管理器
pub struct SeedManager {
    master_seed: u64,
    derived_seeds: HashMap<String, u64>,
}

impl SeedManager {
    /// 从 Profile ID 创建种子管理器
    /// 使用 SHA256 哈希确保稳定性
    pub fn from_profile_id(profile_id: &str) -> Self {
        let master_seed = Self::derive_master_seed(profile_id);
        
        Self {
            master_seed,
            derived_seeds: HashMap::new(),
        }
    }
    
    /// 派生主种子
    /// profile_id -> SHA256 -> u64
    fn derive_master_seed(profile_id: &str) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(profile_id.as_bytes());
        let hash = hasher.finalize();
        
        // 取前8个字节作为 u64
        let mut seed_bytes = [0u8; 8];
        seed_bytes.copy_from_slice(&hash[0..8]);
        u64::from_le_bytes(seed_bytes)
    }
    
    /// 获取主种子
    pub fn master_seed(&self) -> u64 {
        self.master_seed
    }
    
    /// 派生子种子
    /// 公式: derived_seed = SHA256(master_seed || context)
    pub fn derive(&mut self, context: &str) -> u64 {
        // 检查缓存
        if let Some(&seed) = self.derived_seeds.get(context) {
            return seed;
        }
        
        // 生成新的子种子
        let mut hasher = Sha256::new();
        hasher.update(self.master_seed.to_le_bytes());
        hasher.update(context.as_bytes());
        let hash = hasher.finalize();
        
        // 取前8个字节
        let mut seed_bytes = [0u8; 8];
        seed_bytes.copy_from_slice(&hash[0..8]);
        let derived_seed = u64::from_le_bytes(seed_bytes);
        
        // 缓存
        self.derived_seeds.insert(context.to_string(), derived_seed);
        
        derived_seed
    }
    
    /// 生成所有常用子种子
    pub fn generate_all_seeds(&mut self) -> DerivedSeeds {
        DerivedSeeds {
            master: self.master_seed,
            canvas: self.derive("canvas"),
            webgl: self.derive("webgl"),
            audio: self.derive("audio"),
            fonts: self.derive("fonts"),
            client_rects: self.derive("client_rects"),
        }
    }
}

/// 派生的种子集合
#[derive(Debug, Clone)]
pub struct DerivedSeeds {
    pub master: u64,
    pub canvas: u64,
    pub webgl: u64,
    pub audio: u64,
    pub fonts: u64,
    pub client_rects: u64,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_deterministic_seed() {
        // 相同的 profile_id 应该生成相同的种子
        let id = "test-profile-123";
        let manager1 = SeedManager::from_profile_id(id);
        let manager2 = SeedManager::from_profile_id(id);
        
        assert_eq!(manager1.master_seed(), manager2.master_seed());
    }
    
    #[test]
    fn test_derived_seeds() {
        let mut manager = SeedManager::from_profile_id("test");
        let canvas_seed1 = manager.derive("canvas");
        let canvas_seed2 = manager.derive("canvas");
        
        // 相同上下文应该返回相同的子种子
        assert_eq!(canvas_seed1, canvas_seed2);
        
        // 不同上下文应该返回不同的子种子
        let webgl_seed = manager.derive("webgl");
        assert_ne!(canvas_seed1, webgl_seed);
    }
}
