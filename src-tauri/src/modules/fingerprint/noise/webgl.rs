// WebGL Noise Generator - WebGL 噪声生成器
// 生成64个浮点数噪声数组，用于 WebGL 指纹混淆

use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

const NOISE_ARRAY_SIZE: usize = 64;
const NOISE_MIN: f32 = 0.0001;
const NOISE_MAX: f32 = 0.001;

/// WebGL 噪声生成器
pub struct WebGLNoiseGenerator;

impl WebGLNoiseGenerator {
    /// 生成 WebGL 噪声数组
    /// 
    /// # Arguments
    /// * `seed` - 随机种子
    /// 
    /// # Returns
    /// 64个浮点数组成的噪声数组，范围在 [0.0001, 0.001]
    pub fn generate(seed: u64) -> Vec<f32> {
        let mut rng = StdRng::seed_from_u64(seed);
        let mut noise_array = Vec::with_capacity(NOISE_ARRAY_SIZE);
        
        for _ in 0..NOISE_ARRAY_SIZE {
            let noise = rng.gen_range(NOISE_MIN..=NOISE_MAX);
            noise_array.push(noise);
        }
        
        noise_array
    }
    
    /// 生成带符号的 WebGL 噪声数组（可正可负）
    pub fn generate_signed(seed: u64) -> Vec<f32> {
        let mut rng = StdRng::seed_from_u64(seed);
        let mut noise_array = Vec::with_capacity(NOISE_ARRAY_SIZE);
        
        for _ in 0..NOISE_ARRAY_SIZE {
            let magnitude = rng.gen_range(NOISE_MIN..=NOISE_MAX);
            let sign = if rng.gen_bool(0.5) { 1.0 } else { -1.0 };
            noise_array.push(magnitude * sign);
        }
        
        noise_array
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_noise_generation() {
        let noise = WebGLNoiseGenerator::generate(12345);
        
        // 验证数组大小
        assert_eq!(noise.len(), NOISE_ARRAY_SIZE);
        
        // 验证所有值在范围内
        for &value in &noise {
            assert!(value >= NOISE_MIN && value <= NOISE_MAX);
        }
    }
    
    #[test]
    fn test_deterministic() {
        // 相同种子应该生成相同的噪声
        let noise1 = WebGLNoiseGenerator::generate(12345);
        let noise2 = WebGLNoiseGenerator::generate(12345);
        
        assert_eq!(noise1, noise2);
    }
    
    #[test]
    fn test_signed_noise() {
        let noise = WebGLNoiseGenerator::generate_signed(12345);
        
        // 应该有正数和负数
        let has_positive = noise.iter().any(|&x| x > 0.0);
        let has_negative = noise.iter().any(|&x| x < 0.0);
        
        assert!(has_positive);
        assert!(has_negative);
    }
}
