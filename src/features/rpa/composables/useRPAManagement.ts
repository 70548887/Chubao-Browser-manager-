/**
 * @description RPA 机器人管理业务逻辑 Composable
 * @author DeepAgent
 */

import { ref, computed } from 'vue'
import { useUIStore } from '@/stores/ui.store'

// RPA 机器人类型
export interface RPARobot {
  id: string
  name: string
  icon: string
  status: 'running' | 'idle' | 'error'
  runningCount?: number
  totalCount?: number
  successRate: number
  lastRun: string
  steps: number
  group: string
  accountCount: number
  gradient: string
  iconBg: string
  iconColor: string
}

// 初始机器人数据
const INITIAL_ROBOTS: RPARobot[] = [
  {
    id: 'FB-2023-01',
    name: 'Facebook Auto-post',
    icon: 'public',
    status: 'running',
    runningCount: 3,
    totalCount: 15,
    successRate: 98,
    lastRun: '刚刚',
    steps: 15,
    group: '社媒推广组',
    accountCount: 20,
    gradient: 'from-blue-500 to-cyan-400',
    iconBg: 'bg-blue-50 dark:bg-blue-900/20',
    iconColor: 'text-blue-600 dark:text-blue-400'
  },
  {
    id: 'AMZ-DATA-09',
    name: 'Amazon Scraper',
    icon: 'shopping_cart',
    status: 'idle',
    successRate: 100,
    lastRun: '2小时前',
    steps: 42,
    group: '电商核心组',
    accountCount: 50,
    gradient: 'from-orange-400 to-yellow-400',
    iconBg: 'bg-orange-50 dark:bg-orange-900/20',
    iconColor: 'text-orange-600 dark:text-orange-400'
  },
  {
    id: 'TK-FARM-22',
    name: 'TikTok Interaction',
    icon: 'music_note',
    status: 'running',
    runningCount: 8,
    totalCount: 10,
    successRate: 85,
    lastRun: '5分钟前',
    steps: 10,
    group: '短视频矩阵',
    accountCount: 100,
    gradient: 'from-pink-500 to-rose-500',
    iconBg: 'bg-black/5 dark:bg-white/10',
    iconColor: 'text-slate-700 dark:text-slate-200'
  },
  {
    id: 'GM-WARM-01',
    name: 'Gmail Warm-up',
    icon: 'mail',
    status: 'idle',
    successRate: 92,
    lastRun: '1天前',
    steps: 8,
    group: '邮箱养号',
    accountCount: 200,
    gradient: 'from-purple-500 to-indigo-500',
    iconBg: 'bg-purple-50 dark:bg-purple-900/20',
    iconColor: 'text-purple-600 dark:text-purple-400'
  },
  {
    id: 'PP-CHK-88',
    name: 'PayPal Audit',
    icon: 'payments',
    status: 'error',
    successRate: 45,
    lastRun: '超时',
    steps: 25,
    group: '支付账号',
    accountCount: 5,
    gradient: 'from-red-500 to-orange-500',
    iconBg: 'bg-indigo-50 dark:bg-indigo-900/20',
    iconColor: 'text-indigo-600 dark:text-indigo-400'
  }
]

export function useRPAManagement() {
  const uiStore = useUIStore()

  // ==================== 状态 ====================
  const createDialogVisible = ref(false)
  const robots = ref<RPARobot[]>([...INITIAL_ROBOTS])

  // ==================== 计算属性 ====================
  const filteredRobots = computed(() => {
    if (!uiStore.searchKeyword) return robots.value
    const keyword = uiStore.searchKeyword.toLowerCase()
    return robots.value.filter(r => 
      r.name.toLowerCase().includes(keyword) || 
      r.id.toLowerCase().includes(keyword)
    )
  })

  // 统计数据
  const stats = computed(() => ({
    total: robots.value.length,
    running: robots.value.filter(r => r.status === 'running').length,
    idle: robots.value.filter(r => r.status === 'idle').length,
    error: robots.value.filter(r => r.status === 'error').length
  }))

  // ==================== 方法 ====================
  const handleCreate = () => {
    createDialogVisible.value = true
  }

  const handleCreateSubmit = (data: any) => {
    console.log('提交创建', data)
    createDialogVisible.value = false
  }

  const handleImport = () => {
    console.log('导入流程')
  }

  const handleFilter = () => {
    console.log('筛选')
  }

  const handleEdit = (robot: RPARobot) => {
    console.log('编辑', robot.name)
  }

  const handleStop = (robot: RPARobot) => {
    console.log('停止', robot.name)
  }

  const handleStart = (robot: RPARobot) => {
    console.log('启动', robot.name)
  }

  const handleLogs = (robot: RPARobot) => {
    console.log('查看日志', robot.name)
  }

  const handleViewError = (robot: RPARobot) => {
    console.log('查看错误', robot.name)
  }

  return {
    // 状态
    createDialogVisible,
    robots,
    
    // 计算属性
    filteredRobots,
    stats,
    
    // 方法
    handleCreate,
    handleCreateSubmit,
    handleImport,
    handleFilter,
    handleEdit,
    handleStop,
    handleStart,
    handleLogs,
    handleViewError,
  }
}
