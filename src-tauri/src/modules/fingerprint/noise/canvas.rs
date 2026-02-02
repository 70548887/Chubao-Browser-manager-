// Canvas Noise Generator - Canvas 噪声生成器
// 生成 RGB 像素噪声和坐标偏移，用于 Canvas 指纹混淆

use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

const RGB_NOISE_RANGE: i32 = 2; // ±2 的 RGB 噪声
const COORD_OFFSET_RANGE: i32 = 1; // ±1 的坐标偏移

/// Canvas 噪声配置
#[derive(Debug, Clone)]
pub struct CanvasNoiseConfig {
    pub rgb_noise: Vec<i32>,
    pub coord_offset: (i32, i32),
}

/// Canvas 噪声生成器
pub struct CanvasNoiseGenerator;

impl CanvasNoiseGenerator {
    /// 生成 Canvas 噪声配置
    /// 
    /// # Arguments
    /// * `seed` - 随机种子
    /// * `pixel_count` - 需要的噪声数量（可选，默认256）
    pub fn generate(seed: u64, pixel_count: Option<usize>) -> CanvasNoiseConfig {
        let mut rng = StdRng::seed_from_u64(seed);
        let count = pixel_count.unwrap_or(256);
        
        // 生成 RGB 噪声数组
        let mut rgb_noise = Vec::with_capacity(count * 3); // R, G, B
        for _ in 0..(count * 3) {
            let noise = rng.gen_range(-RGB_NOISE_RANGE..=RGB_NOISE_RANGE);
            rgb_noise.push(noise);
        }
        
        // 生成坐标偏移
        let coord_offset = (
            rng.gen_range(-COORD_OFFSET_RANGE..=COORD_OFFSET_RANGE),
            rng.gen_range(-COORD_OFFSET_RANGE..=COORD_OFFSET_RANGE),
        );
        
        CanvasNoiseConfig {
            rgb_noise,
            coord_offset,
        }
    }
    
    /// 简化版：生成少量噪声（用于配置文件）
    pub fn generate_compact(seed: u64) -> CanvasNoiseConfig {
        Self::generate(seed, Some(32)) // 仅32个像素的噪声
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_canvas_noise_generation() {
        let config = CanvasNoiseGenerator::generate(12345, Some(100));
        
        // 验证 RGB 噪声数组大小（100个像素 * 3通道）
        assert_eq!(config.rgb_noise.len(), 300);
        
        // 验证所有值在范围内
        for &value in &config.rgb_noise {
            assert!(value >= -RGB_NOISE_RANGE && value <= RGB_NOISE_RANGE);
        }
        
        // 验证坐标偏移在范围内
        assert!(config.coord_offset.0.abs() <= COORD_OFFSET_RANGE);
        assert!(config.coord_offset.1.abs() <= COORD_OFFSET_RANGE);
    }
    
    #[test]
    fn test_deterministic() {
        let config1 = CanvasNoiseGenerator::generate(12345, Some(50));
        let config2 = CanvasNoiseGenerator::generate(12345, Some(50));
        
        assert_eq!(config1.rgb_noise, config2.rgb_noise);
        assert_eq!(config1.coord_offset, config2.coord_offset);
    }
}
