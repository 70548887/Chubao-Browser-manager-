/**
 * @description 仪表盘业务逻辑 Composable
 * @author DeepAgent
 */

import { ref, computed, watch, nextTick } from 'vue'
import { useProfileStore } from '@/stores/profile.store'
import { useGroupStore } from '@/stores/groupStore'
import { useUIStore } from '@/stores/ui.store'
import type { Profile } from '@/types/profile.types'
import type { BatchResult } from '@/types'
import { ElMessageBox } from 'element-plus'
import { Message } from '@/utils/message'
import { getSettingValue } from '@/api/settingsApi'
import { arrangeWindowsGrid } from '@/api/profileApi'

export function useDashboard(navigateTo?: (page: string) => void) {
  // ==================== Stores ====================
  const profileStore = useProfileStore()
  const groupStore = useGroupStore()
  const uiStore = useUIStore()

  // ==================== 状态 ====================
  const isLoading = ref(false)
  const selectedIds = ref<Set<string>>(new Set())
  const filterStatus = ref<string>('all')
  const filterGroup = ref<string>('all')
  const sortField = ref('updatedAt')
  const sortOrder = ref<'asc' | 'desc'>('desc')

  // 分页状态
  const currentPage = ref(1)
  const pageSize = ref(10)
  const pageSizes = [10, 20, 50, 100]

  // 抽屉状态
  const drawerVisible = ref(false)
  const editingProfile = ref<Profile | undefined>(undefined)

  // 通知栏状态
  const showNotification = ref(true)

  // 批量操作结果对话框状态
  const batchResultVisible = ref(false)
  const batchResultTitle = ref('')
  const batchResultData = ref<BatchResult | null>(null)

  // 批量移动分组对话框状态
  const moveGroupDialogVisible = ref(false)
  const targetGroupId = ref('')

  // 监听全局搜索关键词
  watch(() => uiStore.searchKeyword, () => {
    currentPage.value = 1
  })

  // ==================== 辅助方法 ====================
  /**
   * 检查系统设置是否完成
   */
  const checkSystemSettings = async (): Promise<boolean> => {
    try {
      const kernelPath = await getSettingValue('kernel_path')
      if (!kernelPath) {
        const result = await ElMessageBox.confirm(
          '浏览器内核路径未设置，无法启动窗口。是否前往设置页面？',
          '配置提示',
          {
            type: 'warning',
            confirmButtonText: '前往设置',
            cancelButtonText: '取消',
          }
        )
        if (result === 'confirm' && navigateTo) {
          navigateTo('settings')
        }
        return false
      }
      return true
    } catch (e: any) {
      if (e === 'cancel') return false
      Message.error('检查系统设置失败')
      return false
    }
  }

  const checkNotificationVisibility = () => {
    const closedDate = localStorage.getItem('dashboard-notification-closed')
    if (!closedDate) return
    
    const today = new Date().toDateString()
    const lastClosedDate = new Date(closedDate).toDateString()
    
    if (today === lastClosedDate) {
      showNotification.value = false
    }
  }

  const closeNotification = () => {
    showNotification.value = false
    localStorage.setItem('dashboard-notification-closed', new Date().toISOString())
  }

  // ==================== 计算属性 ====================
  const filteredProfiles = computed(() => {
    let result = profileStore.profiles
    
    // 状态筛选
    if (filterStatus.value !== 'all') {
      result = result.filter((p: Profile) => p.status === filterStatus.value)
    }
    
    // 分组筛选
    if (filterGroup.value !== 'all') {
      result = result.filter((p: Profile) => p.group === filterGroup.value)
    }
    
    // 关键词搜索
    if (uiStore.searchKeyword) {
      const keyword = uiStore.searchKeyword.toLowerCase()
      result = result.filter((p: Profile) => 
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

  const paginatedProfiles = computed(() => {
    const start = (currentPage.value - 1) * pageSize.value
    const end = start + pageSize.value
    return filteredProfiles.value.slice(start, end).map((profile, idx) => ({
      ...profile,
      index: start + idx + 1
    }))
  })

  const totalCount = computed(() => filteredProfiles.value.length)

  const isAllSelected = computed(() => 
    filteredProfiles.value.length > 0 && 
    filteredProfiles.value.every((p: Profile) => selectedIds.value.has(p.id))
  )

  const profiles = computed(() => profileStore.profiles)
  const runningCount = computed(() => profileStore.runningCount)

  // ==================== 选择操作 ====================
  const handleSelectAll = (value: boolean) => {
    if (value) {
      filteredProfiles.value.forEach((p: Profile) => selectedIds.value.add(p.id))
    } else {
      selectedIds.value.clear()
    }
  }

  const handleSelect = (profile: Profile) => {
    if (selectedIds.value.has(profile.id)) {
      selectedIds.value.delete(profile.id)
    } else {
      selectedIds.value.add(profile.id)
    }
  }

  const handleSort = (field: string) => {
    if (sortField.value === field) {
      sortOrder.value = sortOrder.value === 'asc' ? 'desc' : 'asc'
    } else {
      sortField.value = field
      sortOrder.value = 'asc'
    }
  }

  // ==================== 单项操作 ====================
  
  // 启动前回调（用于显示进度对话框）
  let onBeforeLaunch: ((profileId: string, profileName: string) => void) | null = null
  
  const setOnBeforeLaunch = (callback: (profileId: string, profileName: string) => void) => {
    onBeforeLaunch = callback
  }
  
  const handleLaunch = async (id: string, profileName?: string) => {
    const isSettingOk = await checkSystemSettings()
    if (!isSettingOk) return
    
    // 触发启动前回调（显示进度对话框）
    const profile = filteredProfiles.value.find(p => p.id === id)
    const name = profileName || profile?.name || ''
    onBeforeLaunch?.(id, name)
    
    try {
      await profileStore.startProfile(id)
      // 不再在这里显示成功消息，进度对话框会显示
    } catch (e: any) {
      Message.error(e.message || '启动失败')
    }
  }

  const handleStop = async (id: string) => {
    try {
      await profileStore.stopProfile(id)
      Message.success('停止成功')
    } catch (e: any) {
      Message.error(e.message || '停止失败')
    }
  }

  const handleEdit = async (profile: Profile) => {
    // 运行中的窗口不允许编辑
    if (profile.status === 'running') {
      Message.warning('窗口运行中，请先停止后再编辑')
      return
    }
    // 先设置数据
    editingProfile.value = profile
    // 等待 Vue 响应式更新完成后再打开抽屉，避免竞态问题
    await nextTick()
    drawerVisible.value = true
  }

  const handleDelete = async (id: string) => {
    // 检查窗口是否在运行中
    const profile = profileStore.profiles.find(p => p.id === id)
    if (profile?.status === 'running') {
      Message.warning('窗口运行中，请先停止后再删除')
      return
    }
    
    try {
      await ElMessageBox.confirm('确定要删除此窗口吗？', '删除确认', {
        type: 'warning',
      })
      await profileStore.deleteProfile(id)
      selectedIds.value.delete(id)
      // 注意：不在这里显示成功消息，eventListener.ts 会监听 profile:deleted 事件并显示
    } catch (e: any) {
      if (e !== 'cancel') {
        Message.error('删除失败')
      }
    }
  }

  const handleCreateNew = () => {
    editingProfile.value = undefined
    drawerVisible.value = true
  }

  const handleDrawerSuccess = async () => {
    await profileStore.fetchProfiles()
  }
  
  // 编辑窗口提交
  const handleEditSubmit = async (formData: any) => {
    if (!editingProfile.value) return
    
    try {
      console.log('✅ [编辑] 提交数据:', formData)
      
      // 构建更新数据
      const updateData = {
        name: formData.name,
        group: formData.groupId || 'default',
        remark: formData.remark || '',
        fingerprint: formData,
      }
      
      await profileStore.updateProfile(editingProfile.value.id, updateData)
      drawerVisible.value = false
      editingProfile.value = undefined
      await profileStore.fetchProfiles()
    } catch (error: any) {
      console.error('更新失败:', error)
      Message.error(error.message || '更新失败')
    }
  }

  // ==================== 批量操作 ====================
  const handleBatchLaunch = async () => {
    if (selectedIds.value.size === 0) {
      Message.warning('请先选择窗口')
      return
    }
    
    const isSettingOk = await checkSystemSettings()
    if (!isSettingOk) return
    
    try {
      const ids = Array.from(selectedIds.value)
      const result = await profileStore.startProfiles(ids)
      
      if (result.failureCount > 0) {
        batchResultTitle.value = '批量启动结果'
        batchResultData.value = result
        batchResultVisible.value = true
      } else {
        Message.success(`批量启动成功（${result.successCount} 个窗口）`)
      }
    } catch (e: any) {
      Message.error(e.message || '批量启动失败')
    }
  }

  const handleBatchStop = async () => {
    if (selectedIds.value.size === 0) {
      Message.warning('请先选择窗口')
      return
    }
    try {
      const ids = Array.from(selectedIds.value)
      const result = await profileStore.stopProfiles(ids)
      
      if (result.failureCount > 0) {
        batchResultTitle.value = '批量停止结果'
        batchResultData.value = result
        batchResultVisible.value = true
      } else {
        Message.success(`批量停止成功（${result.successCount} 个窗口）`)
      }
    } catch (e: any) {
      Message.error(e.message || '批量停止失败')
    }
  }

  const handleBatchDelete = async () => {
    if (selectedIds.value.size === 0) {
      Message.warning('请先选择窗口')
      return
    }
    
    try {
      await ElMessageBox.confirm(
        `确定要删除选中的 ${selectedIds.value.size} 个窗口吗？此操作不可恢复。`,
        '批量删除确认',
        {
          type: 'warning',
          confirmButtonText: '确定删除',
          cancelButtonText: '取消',
          confirmButtonClass: 'el-button--danger'
        }
      )
      
      const ids = Array.from(selectedIds.value)
      const result = await profileStore.deleteProfiles(ids)
      
      selectedIds.value.clear()
      
      if (result.failureCount > 0) {
        batchResultTitle.value = '批量删除结果'
        batchResultData.value = result
        batchResultVisible.value = true
      } else {
        Message.success(`批量删除成功（${result.successCount} 个窗口）`)
      }
    } catch (e: any) {
      if (e !== 'cancel') {
        Message.error(e.message || '批量删除失败')
      }
    }
  }

  const handleBatchMoveGroup = () => {
    if (selectedIds.value.size === 0) {
      Message.warning('请先选择窗口')
      return
    }
    moveGroupDialogVisible.value = true
    targetGroupId.value = ''
  }

  const confirmMoveGroup = async () => {
    if (!targetGroupId.value) {
      Message.warning('请选择目标分组')
      return
    }
    
    try {
      const ids = Array.from(selectedIds.value)
      const result = await profileStore.moveToGroup(ids, targetGroupId.value)
      
      moveGroupDialogVisible.value = false
      selectedIds.value.clear()
      
      if (result.failureCount > 0) {
        batchResultTitle.value = '批量移动分组结果'
        batchResultData.value = result
        batchResultVisible.value = true
      } else {
        Message.success(`批量移动成功（${result.successCount} 个窗口）`)
        await groupStore.initGroups()
      }
    } catch (e: any) {
      Message.error(e.message || '批量移动分组失败')
    }
  }

  const handleBatchDuplicate = async () => {
    if (selectedIds.value.size === 0) {
      Message.warning('请先选择窗口')
      return
    }
    
    try {
      await ElMessageBox.confirm(
        `确定要复制选中的 ${selectedIds.value.size} 个窗口吗？`,
        '批量复制确认',
        {
          type: 'info',
          confirmButtonText: '确定复制',
          cancelButtonText: '取消',
        }
      )
      
      const ids = Array.from(selectedIds.value)
      const result = await profileStore.duplicateProfiles(ids)
      
      selectedIds.value.clear()
      
      if (result.failureCount > 0) {
        batchResultTitle.value = '批量复制结果'
        batchResultData.value = result
        batchResultVisible.value = true
      } else {
        Message.success(`批量复制成功（${result.successCount} 个窗口）`)
      }
    } catch (e: any) {
      if (e !== 'cancel') {
        Message.error(e.message || '批量复制失败')
      }
    }
  }

  const handleBatchCommand = (command: string) => {
    switch (command) {
      case 'delete':
        handleBatchDelete()
        break
      case 'move':
        handleBatchMoveGroup()
        break
      case 'duplicate':
        handleBatchDuplicate()
        break
    }
  }

  // ==================== 生命周期 ====================
  const initDashboard = async () => {
    checkNotificationVisibility()
    await profileStore.initProfiles()
    await groupStore.initGroups()
  }

  // ==================== 单个窗口操作 ====================
  
  // 克隆窗口
  const handleClone = async (id: string) => {
    try {
      const result = await profileStore.duplicateProfiles([id])
      if (result.successCount > 0) {
        Message.success('克隆成功')
      } else {
        Message.error(result.results[0]?.error || '克隆失败')
      }
    } catch (e: any) {
      Message.error(e.message || '克隆失败')
    }
  }
  
  // 导出窗口配置
  const handleExport = async (id: string) => {
    try {
      const profile = profiles.value.find(p => p.id === id)
      if (!profile) {
        Message.error('窗口不存在')
        return
      }
      
      // 构建导出数据
      const exportData = {
        name: profile.name,
        group: profile.group,
        remark: profile.remark,
        fingerprint: profile.fingerprint,
        proxy: profile.proxy,
        preferences: profile.preferences,
        exportedAt: new Date().toISOString(),
        version: '1.0',
      }
      
      // 创建并下载 JSON 文件
      const blob = new Blob([JSON.stringify(exportData, null, 2)], { type: 'application/json' })
      const url = URL.createObjectURL(blob)
      const link = document.createElement('a')
      link.href = url
      link.download = `${profile.name.replace(/[^a-zA-Z0-9\u4e00-\u9fa5-_]/g, '_')}_${Date.now()}.json`
      document.body.appendChild(link)
      link.click()
      document.body.removeChild(link)
      URL.revokeObjectURL(url)
      
      Message.success('导出成功')
    } catch (e: any) {
      Message.error(e.message || '导出失败')
    }
  }
  
  // 导出 Cookie
  const handleExportCookie = async (id: string) => {
    try {
      const profile = profiles.value.find(p => p.id === id)
      if (!profile) {
        Message.error('窗口不存在')
        return
      }
      
      // TODO: 实际的 Cookie 需要从浏览器数据目录中读取
      Message.warning('Cookie 导出功能开发中...')
    } catch (e: any) {
      Message.error(e.message || '导出 Cookie 失败')
    }
  }
  
  // 清空窗口缓存
  const handleClearCache = async (id: string) => {
    try {
      // TODO: 调用后端 API 清除缓存
      Message.warning('清空缓存功能开发中...')
    } catch (e: any) {
      Message.error(e.message || '清空缓存失败')
    }
  }

  // ==================== 窗口排列 ====================
  const handleArrangeWindows = async () => {
    const runningProfiles = profiles.value.filter(p => p.status === 'running')
    if (runningProfiles.length === 0) {
      Message.warning('没有运行中的窗口')
      return
    }
    
    try {
      // 根据窗口数量自动计算列数
      const count = runningProfiles.length
      const columns = count <= 2 ? count : count <= 4 ? 2 : count <= 9 ? 3 : 4
      await arrangeWindowsGrid(columns)
      Message.success(`已排列 ${count} 个窗口`)
    } catch (e: any) {
      Message.error(e.message || '排列窗口失败')
    }
  }

  return {
    // Stores
    groupStore,
    
    // 状态
    isLoading,
    selectedIds,
    filterStatus,
    filterGroup,
    sortField,
    sortOrder,
    currentPage,
    pageSize,
    pageSizes,
    drawerVisible,
    editingProfile,
    showNotification,
    batchResultVisible,
    batchResultTitle,
    batchResultData,
    moveGroupDialogVisible,
    targetGroupId,
    
    // 计算属性
    filteredProfiles,
    paginatedProfiles,
    totalCount,
    isAllSelected,
    profiles,
    runningCount,
    
    // 方法
    closeNotification,
    handleSelectAll,
    handleSelect,
    handleSort,
    handleLaunch,
    handleStop,
    handleEdit,
    handleDelete,
    handleCreateNew,
    handleDrawerSuccess,
    handleEditSubmit,
    handleBatchLaunch,
    handleBatchStop,
    handleBatchDelete,
    handleBatchMoveGroup,
    confirmMoveGroup,
    handleBatchDuplicate,
    handleBatchCommand,
    handleClone,
    handleExport,
    handleExportCookie,
    handleClearCache,
    handleArrangeWindows,
    initDashboard,
    setOnBeforeLaunch,
  }
}
