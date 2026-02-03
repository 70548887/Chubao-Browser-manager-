/**
 * @description 分组管理业务逻辑 Composable
 * @author DeepAgent
 */

import { ref, reactive, computed, onMounted, watch } from 'vue'
import { ElMessageBox } from 'element-plus'
import { Message } from '@/utils/message'
import { useGroupStore, type Group } from '@/stores/groupStore'
import { useUIStore } from '@/stores/ui.store'

// 表单数据类型
export interface GroupFormData {
  name: string
  sort: number
  remark: string
  icon: string
}

// 排序选项
export const SORT_OPTIONS = [
  { label: '最近更新', value: 'updated' },
  { label: '按名称', value: 'name' },
  { label: '按排序值', value: 'sort' }
]

export function useGroupManagement() {
  // ==================== Stores ====================
  const groupStore = useGroupStore()
  const uiStore = useUIStore()

  // ==================== 状态 ====================
  const isLoading = ref(false)

  // 监听全局搜索关键词
  watch(() => uiStore.searchKeyword, () => {
    currentPage.value = 1
  })

  // 分页状态
  const currentPage = ref(1)
  const pageSize = ref(10)
  const pageSizes = [10, 20, 50, 100]

  // 弹窗状态
  const dialogVisible = ref(false)
  const drawerVisible = ref(false)
  const dialogTitle = ref('添加分组')
  const isEditMode = ref(false)
  const editingId = ref<string | null>(null)

  // 排序状态
  const sortBy = ref<'updated' | 'name' | 'sort'>('sort')

  // 选中状态
  const selectedIds = ref<string[]>([])

  // 表单数据
  const formData = reactive<GroupFormData>({
    name: '',
    sort: 0,
    remark: '',
    icon: 'folder'
  })

  // ==================== 计算属性 ====================
  const filteredGroups = computed(() => {
    let result = groupStore.groups
    
    if (uiStore.searchKeyword) {
      const keyword = uiStore.searchKeyword.toLowerCase()
      result = result.filter((g: Group) => 
        g.name.toLowerCase().includes(keyword) || 
        (g.remark?.toLowerCase().includes(keyword))
      )
    }
    
    // 根据sortBy排序
    result = [...result].sort((a, b) => {
      if (sortBy.value === 'updated') {
        return b.updatedAt - a.updatedAt
      } else if (sortBy.value === 'name') {
        return a.name.localeCompare(b.name, 'zh-CN')
      } else {
        if (a.sort !== b.sort) return a.sort - b.sort
        return b.createdAt - a.createdAt
      }
    })
    
    return result
  })

  const paginatedGroups = computed(() => {
    const start = (currentPage.value - 1) * pageSize.value
    const end = start + pageSize.value
    return filteredGroups.value.slice(start, end).map((group, idx) => ({
      ...group,
      index: start + idx + 1
    }))
  })

  const totalCount = computed(() => filteredGroups.value.length)

  const isAllSelected = computed(() => {
    return paginatedGroups.value.length > 0 && selectedIds.value.length === paginatedGroups.value.length
  })

  // 当前编辑分组的关联窗口数
  const editingGroupProfileCount = computed(() => {
    if (!editingId.value) return 0
    const group = groupStore.groups.find(g => g.id === editingId.value)
    return group?.profileCount || 0
  })

  // ==================== 方法 ====================
  const handleSelectAll = (e: Event) => {
    const target = e.target as HTMLInputElement
    if (target.checked) {
      selectedIds.value = paginatedGroups.value.map(g => g.id)
    } else {
      selectedIds.value = []
    }
  }

  const formatDate = (timestamp: number) => {
    if (!timestamp) return '-'
    const date = new Date(timestamp)
    return `${date.getFullYear()}-${String(date.getMonth() + 1).padStart(2, '0')}-${String(date.getDate()).padStart(2, '0')} ${String(date.getHours()).padStart(2, '0')}:${String(date.getMinutes()).padStart(2, '0')}`
  }

  const formatShortId = (id: string | null) => {
    if (!id) return '-'
    if (id.length <= 16) return id
    return `${id.slice(0, 12)}...${id.slice(-4)}`
  }

  const copyIdToClipboard = async () => {
    if (!editingId.value) return
    try {
      await navigator.clipboard.writeText(editingId.value)
      Message.success('已复制到剪切板')
    } catch {
      Message.error('复制失败')
    }
  }

  const getGroupIcon = (icon?: string) => {
    const iconMap: Record<string, string> = {
      'shopping_bag': 'shopping_bag',
      'campaign': 'campaign',
      'movie': 'movie',
      'payments': 'payments',
      'mail': 'mail',
      'folder': 'folder_open'
    }
    return iconMap[icon || 'folder'] || 'folder_open'
  }

  const getIconClass = (icon?: string) => {
    const classMap: Record<string, string> = {
      'shopping_bag': 'icon-indigo',
      'campaign': 'icon-blue',
      'movie': 'icon-slate',
      'payments': 'icon-orange',
      'mail': 'icon-emerald',
      'folder': 'icon-blue'
    }
    return classMap[icon || 'folder'] || 'icon-blue'
  }

  const handleAdd = () => {
    dialogTitle.value = '新建分组'
    isEditMode.value = false
    editingId.value = null
    formData.name = ''
    formData.sort = 1
    formData.remark = ''
    formData.icon = 'folder'
    dialogVisible.value = true
  }

  const handleEdit = (group: Group) => {
    isEditMode.value = true
    editingId.value = group.id
    formData.name = group.name
    formData.sort = group.sort
    formData.remark = group.remark || ''
    formData.icon = group.icon || 'folder'
    drawerVisible.value = true
  }

  const handleDelete = async (group: Group) => {
    if (group.id === 'default') {
      Message.warning('默认分组不可删除')
      return
    }
    
    const success = await groupStore.deleteGroup(group.id)
    if (success) {
      selectedIds.value = selectedIds.value.filter(id => id !== group.id)
    }
  }

  const handleBatchDelete = async () => {
    if (selectedIds.value.length === 0) {
      Message.warning('请先选择要删除的分组')
      return
    }
    
    if (selectedIds.value.includes('default')) {
      Message.error('选中的分组包含默认分组，默认分组不可删除')
      return
    }
    
    const groupsToDelete = selectedIds.value.map(id => groupStore.groups.find(g => g.id === id)).filter(Boolean)
    const nonEmptyGroups = groupsToDelete.filter(g => g && g.profileCount > 0)
    
    if (nonEmptyGroups.length > 0) {
      Message.error(`无法批量删除：${nonEmptyGroups.length} 个分组内还有环境，请先清空`)
      return
    }
    
    try {
      await ElMessageBox.confirm(
        `确定要删除选中的 ${selectedIds.value.length} 个分组吗？此操作不可恢复！`,
        '批量删除确认',
        {
          confirmButtonText: '确定删除',
          cancelButtonText: '取消',
          type: 'warning',
          confirmButtonClass: 'el-button--danger'
        }
      )
      
      const { deleteGroup: deleteGroupApi } = await import('@/api/groupApi')
      let successCount = 0
      let failCount = 0
      
      for (const id of selectedIds.value) {
        try {
          await deleteGroupApi(id)
          const index = groupStore.groups.findIndex(g => g.id === id)
          if (index !== -1) {
            groupStore.groups.splice(index, 1)
          }
          successCount++
        } catch (err) {
          console.error(`Failed to delete group ${id}:`, err)
          failCount++
        }
      }
      
      selectedIds.value = []
      
      if (failCount === 0) {
        Message.success(`成功删除 ${successCount} 个分组`)
      } else {
        Message.warning(`删除完成：成功 ${successCount} 个，失败 ${failCount} 个`)
      }
    } catch {
      // 用户取消删除
    }
  }

  const handleSubmit = async () => {
    if (!formData.name.trim()) {
      Message.warning('请输入分组名称')
      return
    }
    
    const payload = {
      name: formData.name,
      sort: formData.sort,
      permission: 'editable',
      remark: formData.remark,
      icon: formData.icon
    }
    
    if (isEditMode.value && editingId.value) {
      await groupStore.updateGroup(editingId.value, payload)
      drawerVisible.value = false
    } else {
      await groupStore.addGroup(payload)
      dialogVisible.value = false
    }
  }

  // ==================== 生命周期 ====================
  onMounted(() => {
    isLoading.value = true
    groupStore.initGroups()
    setTimeout(() => {
      isLoading.value = false
    }, 300)
  })

  return {
    // 状态
    isLoading,
    currentPage,
    pageSize,
    pageSizes,
    dialogVisible,
    drawerVisible,
    dialogTitle,
    isEditMode,
    editingId,
    sortBy,
    selectedIds,
    formData,
    
    // 计算属性
    filteredGroups,
    paginatedGroups,
    totalCount,
    isAllSelected,
    editingGroupProfileCount,
    
    // 方法
    handleSelectAll,
    formatDate,
    formatShortId,
    copyIdToClipboard,
    getGroupIcon,
    getIconClass,
    handleAdd,
    handleEdit,
    handleDelete,
    handleBatchDelete,
    handleSubmit,
    
    // 常量
    sortOptions: SORT_OPTIONS
  }
}
