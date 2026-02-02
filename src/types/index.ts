// 类型统一导出
export * from './profile.types'

// 批量操作结果类型
export interface BatchItemResult {
  profileId: string
  ok: boolean
  error?: string
}

export interface BatchResult {
  results: BatchItemResult[]
  total: number
  successCount: number
  failureCount: number
}
