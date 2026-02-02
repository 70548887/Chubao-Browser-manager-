/**
 * @description 标签管理业务逻辑 Composable
 * @author DeepAgent
 */

import { ref, reactive, computed, watch } from 'vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import { getTags, createTag, updateTag, deleteTag, type Tag } from '@/api/tagApi'
import { listen, type UnlistenFn } from '@tauri-apps/api/event'
import { useUIStore } from '@/stores/ui.store'

// 颜色选项配置
export const COLOR_OPTIONS = [
  { value: 'blue', bg: '#dbeafe', text: '#1e40af', border: '#3b82f6' },
  { value: 'green', bg: '#d1fae5', text: '#065f46', border: '#10b981' },
  { value: 'purple', bg: '#e9d5ff', text: '#6b21a8', border: '#a855f7' },
  { value: 'orange', bg: '#fed7aa', text: '#9a3412', border: '#f97316' },
  { value: 'pink', bg: '#fce7f3', text: '#9f1239', border: '#ec4899' },
  { value: 'slate', bg: '#f1f5f9', text: '#334155', border: '#64748b' }
]

// 排序选项
export const SORT_OPTIONS = [
  { label: '按排序值', value: 'sort' },
  { label: '按名称', value: 'name' },
  { label: '最近更新', value: 'updated' }
]

export function useTagManagement() {
  // ==================== 状态 ====================
  const tags = ref<Tag[]>([])
  const isLoading = ref(false)
  const uiStore = useUIStore()

  // 分页状态
  const currentPage = ref(1)
  const pageSize = ref(10)

  // 弹窗状态
  const dialogVisible = ref(false)
  const dialogTitle = ref('添加标签')
  const isEditMode = ref(false)
  const editingId = ref<string | null>(null)

  // 排序状态
  const sortBy = ref<'sort' | 'name' | 'updated'>('sort')

  // 表单数据
  const formData = reactive({
    name: '',
    sort: 0,
    color: 'blue',
    remark: ''
  })

  // 多选状态
  const selectedIds = ref<string[]>([])

  // 监听全局搜索关键词
  watch(() => uiStore.searchKeyword, () => {
    currentPage.value = 1
  })

  // ==================== 计算属性 ====================
  const filteredTags = computed(() => {
    let result = tags.value
    
    if (uiStore.searchKeyword) {
      const keyword = uiStore.searchKeyword.toLowerCase()
      result = result.filter(t => t.name.toLowerCase().includes(keyword))
    }
    
    // 根据sortBy排序
    result = [...result].sort((a, b) => {
      if (sortBy.value === 'updated') {
        return b.updatedAt - a.updatedAt
      } else if (sortBy.value === 'name') {
        return a.name.localeCompare(b.name, 'zh-CN')
      } else {
        return a.sort - b.sort
      }
    })
    
    return result
  })

  const paginatedTags = computed(() => {
    const start = (currentPage.value - 1) * pageSize.value
    const end = start + pageSize.value
    return filteredTags.value.slice(start, end).map((tag, idx) => ({
      ...tag,
      index: (currentPage.value - 1) * pageSize.value + idx + 1
    }))
  })

  const isAllSelected = computed(() => {
    return paginatedTags.value.length > 0 && selectedIds.value.length === paginatedTags.value.length
  })

  // ==================== 辅助方法 ====================
  const formatDate = (timestamp: number) => {
    const date = new Date(timestamp)
    return date.toLocaleDateString('zh-CN', {
      year: 'numeric',
      month: '2-digit',
      day: '2-digit',
      hour: '2-digit',
      minute: '2-digit',
      second: '2-digit'
    }).replace(/\//g, '-')
  }

  const getColorStyle = (colorValue: string) => {
    const color = COLOR_OPTIONS.find(c => c.value === colorValue)
    return color || COLOR_OPTIONS[0]
  }

  // ==================== 数据加载 ====================
  const loadTags = async () => {
    isLoading.value = true
    try {
      tags.value = await getTags()
    } catch (error) {
      console.error('Failed to load tags:', error)
      ElMessage.error('加载标签列表失败')
    } finally {
      isLoading.value = false
    }
  }

  // ==================== 选择操作 ====================
  const handleSelectAll = (e: any) => {
    if (e.target.checked) {
      selectedIds.value = paginatedTags.value.map(t => t.id)
    } else {
      selectedIds.value = []
    }
  }

  // ==================== 弹窗操作 ====================
  const handleAdd = () => {
    dialogTitle.value = '新建标签'
    isEditMode.value = false
    editingId.value = null
    formData.name = ''
    formData.sort = 0
    formData.color = 'blue'
    formData.remark = ''
    dialogVisible.value = true
  }

  const handleEdit = (tag: Tag & { index: number }) => {
    dialogTitle.value = '编辑标签'
    isEditMode.value = true
    editingId.value = tag.id
    formData.name = tag.name
    formData.sort = tag.sort
    formData.color = (tag as any).color || 'blue'
    formData.remark = tag.remark || ''
    dialogVisible.value = true
  }

  // ==================== CRUD 操作 ====================
  const handleDelete = async (tag: Tag & { index: number }) => {
    try {
      await ElMessageBox.confirm(
        `确定要删除标签「${tag.name}」吗？`,
        '删除确认',
        { type: 'warning', confirmButtonText: '删除', cancelButtonText: '取消' }
      )
      await deleteTag(tag.id)
      tags.value = tags.value.filter(t => t.id !== tag.id)
      selectedIds.value = selectedIds.value.filter(id => id !== tag.id)
      ElMessage.success('删除成功')
    } catch (error) {
      if (error !== 'cancel') {
        console.error('Failed to delete tag:', error)
        ElMessage.error('删除标签失败')
      }
    }
  }

  const handleBatchDelete = async () => {
    if (selectedIds.value.length === 0) {
      ElMessage.warning('请先选择要删除的标签')
      return
    }
    
    try {
      await ElMessageBox.confirm(
        `确定要删除选中的 ${selectedIds.value.length} 个标签吗？此操作不可恢复！`,
        '批量删除确认',
        {
          confirmButtonText: '确定删除',
          cancelButtonText: '取消',
          type: 'warning',
          confirmButtonClass: 'el-button--danger'
        }
      )
      
      let successCount = 0
      let failCount = 0
      
      for (const id of selectedIds.value) {
        try {
          await deleteTag(id)
          successCount++
        } catch {
          failCount++
        }
      }
      
      await loadTags()
      selectedIds.value = []
      
      if (failCount === 0) {
        ElMessage.success(`成功删除 ${successCount} 个标签`)
      } else {
        ElMessage.warning(`删除完成：成功 ${successCount} 个，失败 ${failCount} 个`)
      }
    } catch {
      // 用户取消删除
    }
  }

  const handleSubmit = async () => {
    if (!formData.name.trim()) {
      ElMessage.warning('请输入标签名称')
      return
    }
    
    try {
      if (isEditMode.value && editingId.value) {
        const updated = await updateTag(editingId.value, {
          name: formData.name,
          sort: formData.sort,
          remark: formData.remark
        })
        const index = tags.value.findIndex(t => t.id === editingId.value)
        if (index !== -1) {
          tags.value[index] = { ...updated, color: formData.color } as any
        }
        ElMessage.success('修改成功')
      } else {
        const newTag = await createTag({
          name: formData.name,
          sort: formData.sort,
          remark: formData.remark
        })
        tags.value.push({ ...newTag, color: formData.color } as any)
        ElMessage.success('添加成功')
      }
      
      dialogVisible.value = false
    } catch (error: any) {
      console.error('Failed to save tag:', error)
      const errorMessage = error?.message || (isEditMode.value ? '修改标签失败' : '添加标签失败')
      const detailMessage = errorMessage.includes(':') 
        ? errorMessage.split(':').slice(1).join(':').trim() 
        : errorMessage
      ElMessage.error(detailMessage)
    }
  }

  // ==================== 生命周期 ====================
  let unlistenCreated: UnlistenFn | null = null
  let unlistenUpdated: UnlistenFn | null = null
  let unlistenDeleted: UnlistenFn | null = null

  const initTagManagement = async () => {
    await loadTags()
    
    unlistenCreated = await listen('tag:created', () => {
      loadTags()
    })
    
    unlistenUpdated = await listen('tag:updated', () => {
      loadTags()
    })
    
    unlistenDeleted = await listen('tag:deleted', () => {
      loadTags()
    })
  }

  const cleanupTagManagement = () => {
    unlistenCreated?.()
    unlistenUpdated?.()
    unlistenDeleted?.()
  }

  return {
    // 状态
    tags,
    isLoading,
    currentPage,
    pageSize,
    dialogVisible,
    dialogTitle,
    isEditMode,
    sortBy,
    formData,
    selectedIds,
    
    // 计算属性
    filteredTags,
    paginatedTags,
    isAllSelected,
    
    // 常量
    colorOptions: COLOR_OPTIONS,
    sortOptions: SORT_OPTIONS,
    
    // 方法
    formatDate,
    getColorStyle,
    loadTags,
    handleSelectAll,
    handleAdd,
    handleEdit,
    handleDelete,
    handleBatchDelete,
    handleSubmit,
    initTagManagement,
    cleanupTagManagement,
  }
}

