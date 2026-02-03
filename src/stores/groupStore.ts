import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { getGroups, createGroup, updateGroup as updateGroupApi, deleteGroup as deleteGroupApi, type Group } from '@/api'
import { ElMessageBox } from 'element-plus'
import { Message } from '@/utils/message'

// 导出 Group 类型
export type { Group }

export const useGroupStore = defineStore('group', () => {
  // 状态
  const groups = ref<Group[]>([])
  const isLoading = ref(false)
  const error = ref<string | null>(null)

  // 初始化模拟数据
  const mockGroups: Group[] = [
    { id: 'default', name: '默认分组', sort: 0, profileCount: 0, permission: '全部权限', remark: '系统默认分组', icon: 'folder', createdAt: Date.now(), updatedAt: Date.now() },
    { id: '1', name: '电商核心组', sort: 1, profileCount: 45, permission: '全部权限', remark: '亚马逊主账号，包含US/UK站点', icon: 'shopping_bag', createdAt: Date.now() - 86400000 * 30, updatedAt: Date.now() },
    { id: '2', name: '社媒营销组', sort: 2, profileCount: 128, permission: '全部权限', remark: 'FB/Ins/Twitter 广告投放号', icon: 'campaign', createdAt: Date.now() - 86400000 * 20, updatedAt: Date.now() },
    { id: '3', name: '短视频矩阵', sort: 3, profileCount: 18, permission: '全部权限', remark: 'TikTok accounts for SG region', icon: 'movie', createdAt: Date.now() - 86400000 * 15, updatedAt: Date.now() },
    { id: '4', name: '支付账号', sort: 4, profileCount: 5, permission: '只读权限', remark: 'High security PayPal accounts', icon: 'payments', createdAt: Date.now() - 86400000 * 10, updatedAt: Date.now() },
    { id: '5', name: '邮箱养号', sort: 5, profileCount: 28, permission: '全部权限', remark: 'Gmail batch 1-5', icon: 'mail', createdAt: Date.now() - 86400000 * 5, updatedAt: Date.now() },
  ]

  // 计算属性 - 按排序排列的分组
  const sortedGroups = computed(() => {
    return [...groups.value].sort((a, b) => a.sort - b.sort)
  })

  // 计算属性 - 用于下拉选择的分组选项
  const groupOptions = computed(() => {
    return sortedGroups.value.map(g => ({
      label: g.name,
      value: g.id
    }))
  })

  // 方法 - 初始化分组数据
  const initGroups = async () => {
    if (groups.value.length > 0) return

    isLoading.value = true
    error.value = null

    try {
      const data = await getGroups()
      groups.value = data
      console.log('分组列表加载成功:', data.length)
    } catch (err: any) {
      console.error('Failed to load groups from backend:', err)
      error.value = err.message || '获取分组列表失败'
      Message.error({
        message: '无法加载分组列表，请检查后端连接',
        duration: 5000,
      })

      // 仅在开发模式使用 mock（可通过环境变量控制）
      if (import.meta.env.DEV && import.meta.env.VITE_USE_MOCK === 'true') {
        console.warn('Using mock data in DEV mode')
        groups.value = mockGroups
        Message.warning('当前使用模拟数据')
      } else {
        // 生产模式：保持空列表，强制用户修复后端问题
        groups.value = []
      }
    } finally {
      isLoading.value = false
    }
  }

  // 方法 - 添加分组
  const addGroup = async (group: Omit<Group, 'id' | 'createdAt' | 'updatedAt' | 'profileCount'>) => {
    try {
      const created = await createGroup({
        name: group.name,
        sort: group.sort,
        permission: group.permission,
        remark: group.remark,
        icon: group.icon
      })
      console.log('[DEBUG] 后端返回的原始数据:', created)
      console.log('[DEBUG] 创建的分组对象:', created)
      console.log('[DEBUG] createdAt值:', created.createdAt, '类型:', typeof created.createdAt)
      groups.value.push(created)
      Message.success('创建分组成功')
      return created
    } catch (err: any) {
      console.error('Failed to create group via API:', err)
      Message.error(err.message || '创建失败')
      throw err // 抛出错误，不再本地创建
    }
  }

  // 方法 - 更新分组
  const updateGroup = async (id: string, updates: Partial<Omit<Group, 'id' | 'createdAt' | 'updatedAt' | 'profileCount'>>) => {
    try {
      // 调用后端 API
      const updated = await updateGroupApi(id, {
        name: updates.name,
        sort: updates.sort,
        remark: updates.remark,
        icon: updates.icon
      })

      // 更新本地状态
      const index = groups.value.findIndex(g => g.id === id)
      if (index !== -1) {
        groups.value[index] = updated
      }

      Message.success('更新分组成功')
      return updated
    } catch (err: any) {
      console.error('Failed to update group via API:', err)
      Message.error(err.message || '更新失败')
      return null
    }
  }

  // 方法 - 删除分组（必须先清空）
  const deleteGroup = async (id: string) => {
    const group = getGroupById(id)

    // 检查分组是否为空
    if (group && group.profileCount > 0) {
      Message.error(`无法删除分组：分组内还有 ${group.profileCount} 个环境，请先清空分组`)
      return false
    }

    try {
      // 二次确认
      await ElMessageBox.confirm(
        `确定要删除分组"${group?.name}"吗？`,
        '删除确认',
        {
          type: 'warning',
          confirmButtonText: '确定删除',
          cancelButtonText: '取消',
        }
      )

      await deleteGroupApi(id)
      const index = groups.value.findIndex(g => g.id === id)
      if (index !== -1) {
        groups.value.splice(index, 1)
      }
      Message.success('删除分组成功')
      return true
    } catch (err: any) {
      if (err === 'cancel') {
        // 用户取消操作
        return false
      }
      console.error('Failed to delete group via API:', err)
      Message.error(err.message || '删除失败')
      return false
    }
  }

  // 方法 - 根据ID获取分组
  const getGroupById = (id: string) => {
    return groups.value.find(g => g.id === id)
  }

  // 方法 - 增加分组窗口数
  const incrementProfileCount = (id: string) => {
    const group = groups.value.find(g => g.id === id)
    if (group) {
      group.profileCount++
    }
  }

  // 方法 - 减少分组窗口数
  const decrementProfileCount = (id: string) => {
    const group = groups.value.find(g => g.id === id)
    if (group && group.profileCount > 0) {
      group.profileCount--
    }
  }

  return {
    // 状态
    groups,

    // 计算属性
    sortedGroups,
    groupOptions,

    // 方法
    initGroups,
    addGroup,
    updateGroup,
    deleteGroup,
    getGroupById,
    incrementProfileCount,
    decrementProfileCount
  }
})
