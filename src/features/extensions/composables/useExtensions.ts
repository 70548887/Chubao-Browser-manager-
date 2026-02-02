/**
 * @description 扩展中心业务逻辑 Composable
 * @author DeepAgent
 */

import { ref, computed, onMounted } from 'vue'
import * as extensionApi from '@/api/extensionApi'
import type { Extension } from '@/api/extensionApi'

// 分类标签配置
export const CATEGORIES = [
  { id: 'all', name: '全部' },
  { id: 'web3', name: 'Web3 钱包' },
  { id: 'adblock', name: '广告拦截' },
  { id: 'productivity', name: '生产力工具' },
  { id: 'social', name: '社交媒体' },
  { id: 'developer', name: '开发者工具' },
]

// 重导出类型
export type { Extension }

export function useExtensions() {
  // ==================== 状态 ====================
  const activeCategory = ref('all')
  const extensions = ref<Extension[]>([])
  const loading = ref(false)
  const error = ref<string | null>(null)
  const scanning = ref(false)

  // ==================== 计算属性 ====================
  const installedExtensions = computed(() => {
    return extensions.value.filter(ext => ext.installed)
  })

  const filteredExtensions = computed(() => {
    if (activeCategory.value === 'all') {
      return extensions.value
    }
    const categoryName = CATEGORIES.find(c => c.id === activeCategory.value)?.name
    return extensions.value.filter(ext => ext.category === categoryName)
  })

  // 统计数据
  const stats = computed(() => ({
    total: extensions.value.length,
    installed: installedExtensions.value.length,
    enabled: installedExtensions.value.filter(ext => ext.enabled).length
  }))

  // ==================== 数据加载 ====================
  const loadExtensions = async () => {
    loading.value = true
    error.value = null
    try {
      extensions.value = await extensionApi.getExtensions()
    } catch (e) {
      error.value = e instanceof Error ? e.message : '加载扩展列表失败'
      console.error('Failed to load extensions:', e)
    } finally {
      loading.value = false
    }
  }

  // ==================== 方法 ====================
  const handleCategoryChange = (categoryId: string) => {
    activeCategory.value = categoryId
  }

  /**
   * 扫描扩展目录，自动注册新扩展
   */
  const handleScanExtensions = async () => {
    scanning.value = true
    try {
      const newExtensions = await extensionApi.scanExtensions()
      if (newExtensions.length > 0) {
        // 刷新列表
        await loadExtensions()
        return newExtensions.length
      }
      return 0
    } catch (e) {
      error.value = e instanceof Error ? e.message : '扫描扩展失败'
      console.error('Failed to scan extensions:', e)
      throw e
    } finally {
      scanning.value = false
    }
  }

  /**
   * 切换扩展启用状态
   */
  const handleToggleEnable = async (ext: Extension) => {
    try {
      const updated = await extensionApi.toggleExtension(ext.id, !ext.enabled)
      // 更新本地状态
      const index = extensions.value.findIndex(e => e.id === ext.id)
      if (index >= 0) {
        extensions.value[index] = updated
      }
    } catch (e) {
      error.value = e instanceof Error ? e.message : '切换扩展状态失败'
      console.error('Failed to toggle extension:', e)
      throw e
    }
  }

  /**
   * 删除扩展
   */
  const handleDelete = async (ext: Extension) => {
    try {
      await extensionApi.deleteExtension(ext.id)
      // 从本地列表移除
      extensions.value = extensions.value.filter(e => e.id !== ext.id)
    } catch (e) {
      error.value = e instanceof Error ? e.message : '删除扩展失败'
      console.error('Failed to delete extension:', e)
      throw e
    }
  }

  /**
   * 上传扩展（打开文件选择对话框）
   * TODO: 实现文件上传逻辑
   */
  const handleUpload = async () => {
    console.log('上传扩展 - TODO: 实现文件选择和解压逻辑')
    // 扫描目录作为替代
    return handleScanExtensions()
  }

  /**
   * 管理扩展（查看详情）
   */
  const handleManage = (ext: Extension) => {
    console.log('管理扩展:', ext.name)
    // TODO: 打开扩展详情弹窗
  }

  /**
   * 安装扩展（从商店下载或本地导入）
   * TODO: 实现安装逻辑
   */
  const handleInstall = (ext: Extension) => {
    console.log('安装扩展:', ext.name)
    // TODO: 实现从商店下载或本地导入
  }

  // 初始化时加载数据
  onMounted(() => {
    loadExtensions()
  })

  return {
    // 常量
    categories: CATEGORIES,
    
    // 状态
    activeCategory,
    extensions,
    loading,
    error,
    scanning,
    
    // 计算属性 (保持兼容性)
    featuredExtensions: extensions,
    installedExtensions,
    filteredExtensions,
    stats,
    
    // 方法
    loadExtensions,
    handleCategoryChange,
    handleScanExtensions,
    handleInstall,
    handleManage,
    handleToggleEnable,
    handleDelete,
    handleUpload,
  }
}

// ==================== Profile 扩展管理 ====================

/**
 * Profile 扩展管理 Composable
 * 用于管理单个 Profile 启用的扩展
 */
export function useProfileExtensions(profileId: string) {
  const extensions = ref<Extension[]>([])
  const loading = ref(false)
  const error = ref<string | null>(null)

  const loadProfileExtensions = async () => {
    if (!profileId) return
    loading.value = true
    error.value = null
    try {
      extensions.value = await extensionApi.getProfileExtensions(profileId)
    } catch (e) {
      error.value = e instanceof Error ? e.message : '加载Profile扩展失败'
      console.error('Failed to load profile extensions:', e)
    } finally {
      loading.value = false
    }
  }

  const enableExtension = async (extensionId: string) => {
    try {
      await extensionApi.enableExtensionForProfile(profileId, extensionId)
      await loadProfileExtensions()
    } catch (e) {
      error.value = e instanceof Error ? e.message : '启用扩展失败'
      throw e
    }
  }

  const disableExtension = async (extensionId: string) => {
    try {
      await extensionApi.disableExtensionForProfile(profileId, extensionId)
      await loadProfileExtensions()
    } catch (e) {
      error.value = e instanceof Error ? e.message : '禁用扩展失败'
      throw e
    }
  }

  onMounted(() => {
    loadProfileExtensions()
  })

  return {
    extensions,
    loading,
    error,
    loadProfileExtensions,
    enableExtension,
    disableExtension,
  }
}
