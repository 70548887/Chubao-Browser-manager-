// Fingerprint Diff - 指纹字段变更检测
import type { FingerprintConfig } from '@/types'

/**
 * 计算指纹字段的变更（patch）
 * @param original 原始指纹配置
 * @param updated 更新后的指纹配置
 * @returns 只包含变更字段的 patch 对象
 */
export function calculateFingerprintPatch(
  original: Partial<FingerprintConfig> | undefined,
  updated: Partial<FingerprintConfig> | undefined
): Partial<FingerprintConfig> | undefined {
  // 如果没有更新，返回 undefined
  if (!updated) {
    return undefined
  }

  // 如果是新建（没有原始数据），返回全部
  if (!original) {
    return updated
  }

  const patch: Partial<FingerprintConfig> = {}
  let hasChanges = false

  // 遍历所有字段，找出变更
  for (const key in updated) {
    const k = key as keyof FingerprintConfig
    const originalValue = original[k]
    const updatedValue = updated[k]

    // 比较值是否变化
    if (JSON.stringify(originalValue) !== JSON.stringify(updatedValue)) {
      // @ts-ignore
      patch[k] = updatedValue
      hasChanges = true
    }
  }

  // 如果没有任何变更，返回 undefined
  return hasChanges ? patch : undefined
}

/**
 * 深度比较两个对象是否相等
 */
export function deepEqual(a: any, b: any): boolean {
  if (a === b) return true
  if (a == null || b == null) return false
  if (typeof a !== 'object' || typeof b !== 'object') return false

  const keysA = Object.keys(a)
  const keysB = Object.keys(b)

  if (keysA.length !== keysB.length) return false

  for (const key of keysA) {
    if (!keysB.includes(key)) return false
    if (!deepEqual(a[key], b[key])) return false
  }

  return true
}
