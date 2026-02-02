// Audio Noise Generator - Audio 噪声生成器
// 生成音频噪声因子，用于 AudioContext 指纹混淆

use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

const NOISE_FACTOR_MIN: f32 = 0.0001;
const NOISE_FACTOR_MAX: f32 = 0.001;

/// Audio 噪声配置
#[derive(Debug, Clone)]
pub struct AudioNoiseConfig {
    pub noise_factor: f32,
}

/// Audio 噪声生成器
pub struct AudioNoiseGenerator;

impl AudioNoiseGenerator {
    /// 生成 Audio 噪声配置
    /// 
    /// # Arguments
    /// * `seed` - 随机种子
    pub fn generate(seed: u64) -> AudioNoiseConfig {
        let mut rng = StdRng::seed_from_u64(seed);
        
        let noise_factor = rng.gen_range(NOISE_FACTOR_MIN..=NOISE_FACTOR_MAX);
        
        AudioNoiseConfig {
            noise_factor,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_audio_noise_generation() {
        let config = AudioNoiseGenerator::generate(12345);
        
        // 验证噪声因子在范围内
        assert!(config.noise_factor >= NOISE_FACTOR_MIN);
        assert!(config.noise_factor <= NOISE_FACTOR_MAX);
    }
    
    #[test]
    fn test_deterministic() {
        let config1 = AudioNoiseGenerator::generate(12345);
        let config2 = AudioNoiseGenerator::generate(12345);
        
        assert_eq!(config1.noise_factor, config2.noise_factor);
    }
}
