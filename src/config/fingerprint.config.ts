/**
 * 指纹配置常量
 * V1 策略：子集 + 透传（受控）
 */

/**
 * V1 白名单字段（允许写入/更新）
 * 这些字段对应前端表单的核心字段，属于 V1 安全可控范围
 */
export const FINGERPRINT_WHITELIST = [
  // 基础字段（必填/强校验）
  'platform',
  'browser',
  'hardwareConcurrency',
  'deviceMemory',
  'screenResolution',
  'timezone',
  'language',
  'userAgent',
  
  // 噪声开关（可选）
  'canvasNoise',
  'webglNoise',
  'audioNoise',
  
  // 高级字段（可选，建议默认隐藏在"高级设置"）
  'webrtc',
  'webrtcPublicIp',
  'webrtcLocalIp',
  'webglVendor',
  'webglRenderer',
  'webgpu',
  'canvas',
  'audioContext',
  'doNotTrack',
  'clientRects',
  'mediaDevices',
  'fonts',
  'resolution',
  'screenWidth',
  'screenHeight',
  'brand',
  'version',
  'seed',
] as const

/**
 * 黑名单字段（任何情况下禁止写入）
 * 这是"更安全"的关键，出现则丢弃/拒绝
 */
export const FINGERPRINT_BLACKLIST = [
  // 命令/参数类
  'launchArgs',
  'args',
  'cmdline',
  
  // 路径/加载类
  'extensionPath',
  'loadExtension',
  'binaryPath',
  'pluginPath',
  
  // 调试/远程控制类
  'remoteDebuggingPort',
  'cdpPort',
  
  // 脚本注入类
  'injectScript',
  'preload',
] as const

/**
 * 指纹配置版本
 */
export const FINGERPRINT_SCHEMA_VERSION = 1

/**
 * 过滤指纹对象，只保留白名单字段
 * @param fingerprint 原始指纹对象
 * @returns 过滤后的指纹对象
 */
export function filterFingerprintWhitelist(fingerprint: Record<string, any>): Record<string, any> {
  const filtered: Record<string, any> = {}
  const whitelist = new Set(FINGERPRINT_WHITELIST)
  
  for (const key of Object.keys(fingerprint)) {
    if (whitelist.has(key as any)) {
      filtered[key] = fingerprint[key]
    }
  }
  
  return filtered
}

/**
 * 检查指纹对象是否包含黑名单字段
 * @param fingerprint 指纹对象
 * @returns 包含的黑名单字段列表
 */
export function detectFingerprintBlacklist(fingerprint: Record<string, any>): string[] {
  const blacklist = new Set(FINGERPRINT_BLACKLIST)
  const detected: string[] = []
  
  for (const key of Object.keys(fingerprint)) {
    if (blacklist.has(key as any)) {
      detected.push(key)
    }
    // 检查以 path/Path 结尾的字段
    if (key.toLowerCase().endsWith('path') && !FINGERPRINT_WHITELIST.includes(key as any)) {
      detected.push(key)
    }
  }
  
  return detected
}

/**
 * 合并指纹对象（patch merge）
 * @param existing 现有指纹对象
 * @param patch 要更新的字段（仅白名单）
 * @returns 合并后的指纹对象
 */
export function mergeFingerprintPatch(
  existing: Record<string, any>,
  patch: Record<string, any>
): Record<string, any> {
  // 过滤 patch 只保留白名单
  const filteredPatch = filterFingerprintWhitelist(patch)
  
  // 合并（patch 覆盖 existing 的对应字段，其他字段保持不变）
  return {
    ...existing,
    ...filteredPatch,
    schemaVersion: FINGERPRINT_SCHEMA_VERSION,
  }
}
