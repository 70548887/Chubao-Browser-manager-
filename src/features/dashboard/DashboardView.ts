/**
 * @file DashboardView.ts
 * @description 仪表盘主视图 - 业务逻辑�?
 */

import { ref, computed, onMounted } from 'vue'
import type { Profile } from '@/types'
import { mockProfiles } from './mock.data'
import { ElMessageBox } from 'element-plus'
import { Message } from '@/utils/message'

// ==================== 状态管�?====================
export const profiles = ref<Profile[]>([])
export const selectedIds = ref<Set<string>>(new Set())
export const isLoading = ref(false)
export const searchKeyword = ref('')
export const filterStatus = ref<string>('all')
export const sortField = ref('updatedAt')
export const sortOrder = ref<'asc' | 'desc'>('desc')

// 抽屉状�?
export const drawerVisible = ref(false)
export const editingProfile = ref<Profile | undefined>(undefined)

// ==================== 计算属�?====================
/**
 * 过滤后的环境列表
 */
export const filteredProfiles = computed(() => {
  let result = profiles.value
  
  // 状态筛�?
  if (filterStatus.value !== 'all') {
    result = result.filter(p => p.status === filterStatus.value)
  }
  
  // 关键词搜�?
  if (searchKeyword.value) {
    const keyword = searchKeyword.value.toLowerCase()
    result = result.filter(p => 
      p.name.toLowerCase().includes(keyword) ||
      p.id.toLowerCase().includes(keyword)
    )
  }
  
  // 排序
  result = [...result].sort((a, b) => {
    const aVal = a[sortField.value as keyof Profile]
    const bVal = b[sortField.value as keyof Profile]
    const order = sortOrder.value === 'asc' ? 1 : -1
    if (typeof aVal === 'string' && typeof bVal === 'string') {
      return aVal.localeCompare(bVal) * order
    }
    return ((aVal as number) - (bVal as number)) * order
  })
  
  return result
})

/**
 * 是否全�?
 */
export const isAllSelected = computed(() => 
  filteredProfiles.value.length > 0 && 
  filteredProfiles.value.every(p => selectedIds.value.has(p.id))
)

/**
 * 运行中的环境数量
 */
export const runningCount = computed(() => 
  profiles.value.filter(p => p.status === 'running').length
)

// ==================== 选择操作 ====================
/**
 * 全�?取消全�?
 */
export const handleSelectAll = (value: boolean) => {
  if (value) {
    filteredProfiles.value.forEach(p => selectedIds.value.add(p.id))
  } else {
    selectedIds.value.clear()
  }
}

/**
 * 单个选择
 */
export const handleSelect = (profile: Profile) => {
  if (selectedIds.value.has(profile.id)) {
    selectedIds.value.delete(profile.id)
  } else {
    selectedIds.value.add(profile.id)
  }
}

// ==================== 排序操作 ====================
/**
 * 处理排序
 */
export const handleSort = (field: string) => {
  if (sortField.value === field) {
    sortOrder.value = sortOrder.value === 'asc' ? 'desc' : 'asc'
  } else {
    sortField.value = field
    sortOrder.value = 'asc'
  }
}

// ==================== 浏览器操�?====================
/**
 * 启动环境
 */
export const handleLaunch = (id: string) => {
  Message.success(`启动环境: ${id}`)
  const profile = profiles.value.find(p => p.id === id)
  if (profile) {
    profile.status = 'running'
  }
}

/**
 * 停止环境
 */
export const handleStop = (id: string) => {
  Message.info(`停止环境: ${id}`)
  const profile = profiles.value.find(p => p.id === id)
  if (profile) {
    profile.status = 'stopped'
  }
}

/**
 * 编辑环境
 */
export const handleEdit = (profile: Profile) => {
  editingProfile.value = profile
  drawerVisible.value = true
}

/**
 * 删除环境
 */
export const handleDelete = async (id: string) => {
  try {
    await ElMessageBox.confirm('确定要删除此环境吗？', '删除确认', {
      type: 'warning',
    })
    profiles.value = profiles.value.filter(p => p.id !== id)
    selectedIds.value.delete(id)
    Message.success('删除成功')
  } catch {
    // 取消删除
  }
}

/**
 * 创建新环�?
 */
export const handleCreateNew = () => {
  editingProfile.value = undefined
  drawerVisible.value = true
}

/**
 * 抽屉保存成功回调
 */
export const handleDrawerSuccess = () => {
  // 刷新列表
  isLoading.value = true
  setTimeout(() => {
    profiles.value = mockProfiles
    isLoading.value = false
  }, 300)
}

// ==================== 批量操作 ====================
/**
 * 批量启动
 */
export const handleBatchLaunch = () => {
  if (selectedIds.value.size === 0) {
    Message.warning('请先选择环境')
    return
  }
  selectedIds.value.forEach(id => handleLaunch(id))
}

/**
 * 批量停止
 */
export const handleBatchStop = () => {
  if (selectedIds.value.size === 0) {
    Message.warning('请先选择环境')
    return
  }
  selectedIds.value.forEach(id => handleStop(id))
}

// ==================== 生命周期 ====================
/**
 * 组件挂载时初始化数据
 */
export const initDashboard = () => {
  onMounted(() => {
    isLoading.value = true
    // 模拟加载
    setTimeout(() => {
      profiles.value = mockProfiles
      isLoading.value = false
    }, 500)
  })
}
