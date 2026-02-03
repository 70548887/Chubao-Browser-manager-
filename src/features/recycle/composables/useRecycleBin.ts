/**
 * @description 回收站业务逻辑 Composable
 * @author DeepAgent
 */

import { ref, computed, watch } from 'vue'
import { ElMessageBox } from 'element-plus'
import { Message } from '@/utils/message'
import { 
  getRecycleBin, 
  restoreProfile, 
  batchRestoreProfiles, 
  permanentlyDeleteProfile, 
  batchPermanentlyDeleteProfiles, 
  emptyRecycleBin,
  type RecycledProfile 
} from '@/api/recycleBinApi'
import { getGroups } from '@/api/groupApi'
import { useUIStore } from '@/stores/ui.store'
import type { BatchResult } from '@/types'
import { listen, type UnlistenFn } from '@tauri-apps/api/event'

// 回收站项目类型
export interface RecycleItem {
  id: string
  index: number
  groupName: string
  windowName: string
  windowId: string
  remark: string
  operator: string
  operatorAvatar: string
  deletedAt: number
  icon: string
  iconColor: string
}

export function useRecycleBin() {
  // ==================== 状态 ====================
  const items = ref<RecycleItem[]>([])
  const selectedIds = ref<Set<string>>(new Set())
  const isLoading = ref(false)
  const uiStore = useUIStore()

  const groupFilter = ref('')
  const groupOptions = ref<{ label: string, value: string }[]>([])

  // 分页状态
  const currentPage = ref(1)
  const pageSize = ref(10)
  const pageSizes = [10, 20, 50, 100]

  // 批量结果对话框
  const batchResultVisible = ref(false)
  const batchResultData = ref<BatchResult | null>(null)

  // 监听全局搜索关键词
  watch(() => uiStore.searchKeyword, () => {
    currentPage.value = 1
    selectedIds.value.clear()  // 搜索时清空选择
  })

  // 翻页时清空选择
  watch(currentPage, () => {
    selectedIds.value.clear()
  })

  // 切换每页数量时重置页码并清空选择
  watch(pageSize, () => {
    currentPage.value = 1
    selectedIds.value.clear()
  })

  // ==================== 计算属性 ====================
  const filteredItems = computed(() => {
    let result = items.value
    
    // 分组筛选
    if (groupFilter.value) {
      result = result.filter(item => item.groupName === groupFilter.value)
    }
    
    // 关键词搜索
    if (uiStore.searchKeyword) {
      const keyword = uiStore.searchKeyword.toLowerCase()
      result = result.filter(item => 
        item.windowName.toLowerCase().includes(keyword) ||
        item.remark.toLowerCase().includes(keyword)
      )
    }
    
    return result
  })

  const paginatedItems = computed(() => {
    const start = (currentPage.value - 1) * pageSize.value
    const end = start + pageSize.value
    return filteredItems.value.slice(start, end)
  })

  const totalCount = computed(() => filteredItems.value.length)

  const isAllSelected = computed(() => 
    paginatedItems.value.length > 0 && 
    paginatedItems.value.every(item => selectedIds.value.has(item.id))
  )

  // ==================== 辅助方法 ====================
  const formatDate = (timestamp: number) => {
    const date = new Date(timestamp)
    const year = date.getFullYear()
    const month = String(date.getMonth() + 1).padStart(2, '0')
    const day = String(date.getDate()).padStart(2, '0')
    const hours = String(date.getHours()).padStart(2, '0')
    const minutes = String(date.getMinutes()).padStart(2, '0')
    return `${year}-${month}-${day} ${hours}:${minutes}`
  }

  // 将后端 Profile 转换为前端 RecycleItem
  const profileToItem = (profile: RecycledProfile, index: number, groupMap: Map<string, string>): RecycleItem => {
    const icons = ['public', 'account_circle', 'shopping_cart', 'mail', 'work', 'person']
    const colors = ['blue', 'purple', 'orange', 'green', 'indigo', 'pink']
    const randomIcon = icons[Math.floor(Math.random() * icons.length)]
    const randomColor = colors[Math.floor(Math.random() * colors.length)]
    
    const operators = [
      { name: 'Admin', avatar: 'A', color: 'indigo' },
      { name: 'Liang', avatar: 'L', color: 'green' },
      { name: 'Sarah', avatar: 'S', color: 'pink' },
      { name: 'David', avatar: 'D', color: 'blue' }
    ]
    const randomOperator = operators[Math.floor(Math.random() * operators.length)]
    
    return {
      id: profile.id,
      index: index + 1,
      groupName: groupMap.get(profile.group || '') || profile.group || '未分组',
      windowName: profile.name,
      windowId: profile.id.substring(0, 5),
      remark: profile.remark || '',
      operator: randomOperator.name,
      operatorAvatar: randomOperator.avatar,
      deletedAt: profile.deletedAt,
      icon: randomIcon,
      iconColor: randomColor
    }
  }

  // ==================== 数据加载 ====================
  const loadData = async () => {
    isLoading.value = true
    try {
      const [profiles, groups] = await Promise.all([
        getRecycleBin(),
        getGroups()
      ])
      
      const groupMap = new Map<string, string>()
      groups.forEach(g => groupMap.set(g.id, g.name))
      
      groupOptions.value = [
        { label: '全部分组', value: '' },
        ...groups.map(g => ({ label: g.name, value: g.name }))
      ]
      
      items.value = profiles.map((p, idx) => profileToItem(p, idx, groupMap))
    } catch (error) {
      console.error('Failed to load recycle bin:', error)
      Message.error('加载回收站失败')
    } finally {
      isLoading.value = false
    }
  }

  // ==================== 选择操作 ====================
  const handleSelectAll = (checked: boolean) => {
    if (checked) {
      // 先清空之前的选择，然后选择当前页
      selectedIds.value.clear()
      paginatedItems.value.forEach(item => selectedIds.value.add(item.id))
    } else {
      // 取消全选时清空所有选择
      selectedIds.value.clear()
    }
  }

  const handleSelect = (item: RecycleItem) => {
    if (selectedIds.value.has(item.id)) {
      selectedIds.value.delete(item.id)
    } else {
      selectedIds.value.add(item.id)
    }
  }

  // ==================== 单项操作 ====================
  const handleRestore = async (item: RecycleItem) => {
    try {
      await ElMessageBox.confirm(
        `确定要恢复窗口「${item.windowName}」吗？`,
        '恢复确认',
        { type: 'info', confirmButtonText: '恢复', cancelButtonText: '取消' }
      )
      await restoreProfile(item.id)
      items.value = items.value.filter(i => i.id !== item.id)
      selectedIds.value.delete(item.id)
      Message.success('恢复成功')
    } catch (error) {
      if (error !== 'cancel') {
        console.error('Failed to restore profile:', error)
        Message.error('恢复窗口失败')
      }
    }
  }

  const handleDelete = async (item: RecycleItem) => {
    try {
      await ElMessageBox.confirm(
        `确定要永久删除窗口「${item.windowName}」吗？此操作无法恢复！`,
        '删除确认',
        { type: 'warning', confirmButtonText: '永久删除', cancelButtonText: '取消' }
      )
      await permanentlyDeleteProfile(item.id)
      items.value = items.value.filter(i => i.id !== item.id)
      selectedIds.value.delete(item.id)
      Message.success('删除成功')
    } catch (error) {
      if (error !== 'cancel') {
        console.error('Failed to delete profile:', error)
        Message.error('删除窗口失败')
      }
    }
  }

  // ==================== 批量操作 ====================
  const handleBatchRestore = async () => {
    if (selectedIds.value.size === 0) {
      Message.warning('请先选择要恢复的窗口')
      return
    }
    
    try {
      await ElMessageBox.confirm(
        `确定要恢复选中的 ${selectedIds.value.size} 个窗口吗？`,
        '批量恢复',
        { type: 'info', confirmButtonText: '恢复', cancelButtonText: '取消' }
      )
      const result = await batchRestoreProfiles(Array.from(selectedIds.value))
      
      items.value = items.value.filter(i => !selectedIds.value.has(i.id))
      selectedIds.value.clear()
      
      batchResultData.value = result
      batchResultVisible.value = true
      
      if (result.failureCount === 0) {
        Message.success(`批量恢复成功：${result.successCount}/${result.total}`)
      } else {
        Message.warning(`批量恢复完成：成功 ${result.successCount}，失败 ${result.failureCount}`)
      }
    } catch (error) {
      if (error !== 'cancel') {
        console.error('Failed to batch restore:', error)
        Message.error('批量恢复失败')
      }
    }
  }

  const handleBatchDelete = async () => {
    if (selectedIds.value.size === 0) {
      Message.warning('请先选择要删除的窗口')
      return
    }
    
    try {
      await ElMessageBox.confirm(
        `确定要永久删除选中的 ${selectedIds.value.size} 个窗口吗？此操作无法恢复！`,
        '批量删除',
        { type: 'warning', confirmButtonText: '永久删除', cancelButtonText: '取消' }
      )
      const result = await batchPermanentlyDeleteProfiles(Array.from(selectedIds.value))
      
      items.value = items.value.filter(i => !selectedIds.value.has(i.id))
      selectedIds.value.clear()
      
      batchResultData.value = result
      batchResultVisible.value = true
      
      if (result.failureCount === 0) {
        Message.success(`批量删除成功：${result.successCount}/${result.total}`)
      } else {
        Message.warning(`批量删除完成：成功 ${result.successCount}，失败 ${result.failureCount}`)
      }
    } catch (error) {
      if (error !== 'cancel') {
        console.error('Failed to batch delete:', error)
        Message.error('批量删除失败')
      }
    }
  }

  const handleDeleteAll = async () => {
    if (items.value.length === 0) {
      Message.warning('回收站为空')
      return
    }
    
    try {
      await ElMessageBox.confirm(
        `确定要清空回收站吗？共 ${items.value.length} 个窗口将被永久删除，此操作无法恢复！`,
        '清空回收站',
        { type: 'error', confirmButtonText: '清空', cancelButtonText: '取消' }
      )
      const count = await emptyRecycleBin()
      items.value = []
      selectedIds.value.clear()
      Message.success(`回收站已清空，删除了 ${count} 个窗口`)
    } catch (error) {
      if (error !== 'cancel') {
        console.error('Failed to empty recycle bin:', error)
        Message.error('清空回收站失败')
      }
    }
  }

  // ==================== 生命周期 ====================
  let unlistenDeleted: UnlistenFn | null = null

  const initRecycleBin = async () => {
    await loadData()
    
    // 监听 profile:deleted 事件实现多标签页同步
    unlistenDeleted = await listen('profile:deleted', () => {
      loadData()
    })
  }

  const cleanupRecycleBin = () => {
    unlistenDeleted?.()
  }

  return {
    // 状态
    items,
    selectedIds,
    isLoading,
    groupFilter,
    groupOptions,
    currentPage,
    pageSize,
    pageSizes,
    batchResultVisible,
    batchResultData,
    
    // 计算属性
    filteredItems,
    paginatedItems,
    totalCount,
    isAllSelected,
    
    // 方法
    formatDate,
    loadData,
    handleSelectAll,
    handleSelect,
    handleRestore,
    handleDelete,
    handleBatchRestore,
    handleBatchDelete,
    handleDeleteAll,
    initRecycleBin,
    cleanupRecycleBin,
  }
}

