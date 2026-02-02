/**
 * @description 仪表盘业务逻辑 Composable
 * @author DeepAgent
 */

import { ref, computed, watch } from 'vue'
import { useProfileStore } from '@/stores/profile.store'
import { useGroupStore } from '@/stores/groupStore'
import { useUIStore } from '@/stores/ui.store'
import type { Profile } from '@/types/profile.types'
import type { BatchResult } from '@/types'
import { ElMessage, ElMessageBox } from 'element-plus'
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
          '浏览器内核路径未设置，无法启动环境。是否前往设置页面？',
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
      ElMessage.error('检查系统设置失败')
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
  const handleLaunch = async (id: string) => {
    const isSettingOk = await checkSystemSettings()
    if (!isSettingOk) return
    
    try {
      await profileStore.startProfile(id)
      ElMessage.success('启动成功')
    } catch (e: any) {
      ElMessage.error(e.message || '启动失败')
    }
  }

  const handleStop = async (id: string) => {
    try {
      await profileStore.stopProfile(id)
      ElMessage.success('停止成功')
    } catch (e: any) {
      ElMessage.error(e.message || '停止失败')
    }
  }

  const handleEdit = (profile: Profile) => {
    // 运行中的窗口不允许编辑
    if (profile.status === 'running') {
      ElMessage.warning('窗口运行中，请先停止后再编辑')
      return
    }
    editingProfile.value = profile
    drawerVisible.value = true
  }

  const handleDelete = async (id: string) => {
    try {
      await ElMessageBox.confirm('确定要删除此环境吗？', '删除确认', {
        type: 'warning',
      })
      await profileStore.deleteProfile(id)
      selectedIds.value.delete(id)
      ElMessage.success('删除成功')
    } catch (e: any) {
      if (e !== 'cancel') {
        ElMessage.error('删除失败')
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
      ElMessage.success('更新成功')
      drawerVisible.value = false
      editingProfile.value = undefined
      await profileStore.fetchProfiles()
    } catch (error: any) {
      console.error('更新失败:', error)
      ElMessage.error(error.message || '更新失败')
    }
  }

  // ==================== 批量操作 ====================
  const handleBatchLaunch = async () => {
    if (selectedIds.value.size === 0) {
      ElMessage.warning('请先选择环境')
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
        ElMessage.success(`批量启动成功（${result.successCount} 个环境）`)
      }
    } catch (e: any) {
      ElMessage.error(e.message || '批量启动失败')
    }
  }

  const handleBatchStop = async () => {
    if (selectedIds.value.size === 0) {
      ElMessage.warning('请先选择环境')
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
        ElMessage.success(`批量停止成功（${result.successCount} 个环境）`)
      }
    } catch (e: any) {
      ElMessage.error(e.message || '批量停止失败')
    }
  }

  const handleBatchDelete = async () => {
    if (selectedIds.value.size === 0) {
      ElMessage.warning('请先选择环境')
      return
    }
    
    try {
      await ElMessageBox.confirm(
        `确定要删除选中的 ${selectedIds.value.size} 个环境吗？此操作不可恢复。`,
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
        ElMessage.success(`批量删除成功（${result.successCount} 个环境）`)
      }
    } catch (e: any) {
      if (e !== 'cancel') {
        ElMessage.error(e.message || '批量删除失败')
      }
    }
  }

  const handleBatchMoveGroup = () => {
    if (selectedIds.value.size === 0) {
      ElMessage.warning('请先选择环境')
      return
    }
    moveGroupDialogVisible.value = true
    targetGroupId.value = ''
  }

  const confirmMoveGroup = async () => {
    if (!targetGroupId.value) {
      ElMessage.warning('请选择目标分组')
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
        ElMessage.success(`批量移动成功（${result.successCount} 个环境）`)
        await groupStore.initGroups()
      }
    } catch (e: any) {
      ElMessage.error(e.message || '批量移动分组失败')
    }
  }

  const handleBatchDuplicate = async () => {
    if (selectedIds.value.size === 0) {
      ElMessage.warning('请先选择环境')
      return
    }
    
    try {
      await ElMessageBox.confirm(
        `确定要复制选中的 ${selectedIds.value.size} 个环境吗？`,
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
        ElMessage.success(`批量复制成功（${result.successCount} 个环境）`)
      }
    } catch (e: any) {
      if (e !== 'cancel') {
        ElMessage.error(e.message || '批量复制失败')
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

  // ==================== 窗口排列 ====================
  const handleArrangeWindows = async () => {
    const runningProfiles = profiles.value.filter(p => p.status === 'running')
    if (runningProfiles.length === 0) {
      ElMessage.warning('没有运行中的窗口')
      return
    }
    
    try {
      // 根据窗口数量自动计算列数
      const count = runningProfiles.length
      const columns = count <= 2 ? count : count <= 4 ? 2 : count <= 9 ? 3 : 4
      await arrangeWindowsGrid(columns)
      ElMessage.success(`已排列 ${count} 个窗口`)
    } catch (e: any) {
      ElMessage.error(e.message || '排列窗口失败')
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
    handleArrangeWindows,
    initDashboard,
  }
}
