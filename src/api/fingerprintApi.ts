import { invoke } from '@tauri-apps/api/core'

/**
 * 设备模板信息
 */
export interface TemplateInfo {
  id: string
  description: string
  weight: number
  os_name: string
  os_version: string
  cpu_vendor: string
  gpu_vendor: string
}

/**
 * 指纹配置
 */
export interface FingerprintConfig {
  schema: string
  schema_version: number
  navigator: {
    user_agent: string
    language: string
    languages: string[]
    hardware_concurrency: number
    device_memory: number
    platform: string
  }
  screen: {
    width: number
    height: number
    color_depth: number
    pixel_depth: number
  }
  webgl: {
    vendor: string
    renderer: string
  }
  canvas: {
    rgb_noise: number[]
  }
  audio: {
    noise_factor: number
  }
  timezone: {
    id: string
    offset_minutes: number
  }
  client_hints: {
    brands: Array<{ brand: string; version: string }>
    mobile: boolean
    platform: string
    platform_version: string
    full_version: string
  }
  [key: string]: any
}

/**
 * 校验结果
 */
export interface ValidationResult {
  valid: boolean
  errors: Array<{
    code: string
    message: string
    severity: 'Error' | 'Warning' | 'Info'
    field: string
  }>
  warnings: Array<{
    code: string
    message: string
    severity: 'Error' | 'Warning' | 'Info'
    field: string
  }>
}

/** 平台类型 */
export type PlatformType = 'windows' | 'macos' | 'linux' | 'android' | 'ios'

/** 浏览器版本类型 */
export type BrowserVersionType = '146' | '145' | '144' | '143'

/** 浏览器版本选项 */
export const BROWSER_VERSION_OPTIONS = [
  { value: '146', label: 'Chrome 146', description: '最新稳定版' },
  // 后续可以添加更多版本
  // { value: '145', label: 'Chrome 145', description: '' },
  // { value: '144', label: 'Chrome 144', description: '' },
]

/**
 * 生成随机指纹
 * @param profileId Profile唯一ID
 * @param platform 目标平台（可选，默认windows）
 * @param browserVersion 浏览器版本（可选，默认146）
 */
export async function generateRandomFingerprint(
  profileId: string, 
  platform?: PlatformType,
  browserVersion?: BrowserVersionType
): Promise<FingerprintConfig> {
  return invoke('generate_random_fingerprint', { 
    profileId, 
    platform: platform || 'windows',
    browserVersion: browserVersion || '146'
  })
}

/**
 * 获取设备模板列表
 */
export async function getTemplateList(): Promise<TemplateInfo[]> {
  return invoke('get_template_list')
}

/**
 * 校验指纹配置
 * @param config 指纹配置对象
 */
export async function validateFingerprint(config: FingerprintConfig): Promise<ValidationResult> {
  return invoke('validate_fingerprint', { config })
}
