/**
 * @description 代理管理业务逻辑 Composable
 * @author DeepAgent
 */

import { ref, reactive, computed, watch, onMounted, onUnmounted } from 'vue'
import { ElMessageBox } from 'element-plus'
import { Message } from '@/utils/message'
import { 
  getProxies, 
  createProxy, 
  updateProxy, 
  deleteProxy, 
  testProxy, 
  testProxyConfig,
  batchTestProxies,
  type Proxy, 
  type ProxyTestResult 
} from '@/api/proxyApi'
import { listen, type UnlistenFn } from '@tauri-apps/api/event'
import { useUIStore } from '@/stores/ui.store'

// 表单数据类型
export interface ProxyFormData {
  type: 'http' | 'https' | 'socks5' | 'direct'
  source: string
  tag: string
  host: string
  port: string
  username: string
  password: string
  remark: string
  ipCheckChannel: string
}

// IP查询渠道选项
export const IP_CHECK_CHANNELS = [
  { label: 'DB-IP', value: 'db-ip' },
  { label: 'IP2Location', value: 'ip2location' },
  { label: 'MaxMind', value: 'maxmind' },
  { label: 'IP-API', value: 'ip-api' },
  { label: 'IP138', value: 'ip138' }
]

export function useProxyManagement() {
  // ==================== 状态 ====================
  const proxies = ref<Proxy[]>([])
  const isLoading = ref(false)
  const selectedIds = ref<string[]>([])
  const uiStore = useUIStore()

  // 筛选状态
  const filterType = ref('')
  const filterSource = ref('')
  const filterStatus = ref('')

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
  const dialogTitle = ref('添加代理')
  const isEditMode = ref(false)
  const editingId = ref<string | null>(null)
  const importMode = ref<'single' | 'batch'>('single')
  const batchInput = ref('')
  const testStatus = ref<'idle' | 'testing' | 'success' | 'error'>('idle')

  // 表单数据
  const formData = reactive<ProxyFormData>({
    type: 'socks5',
    source: 'custom',
    tag: '',
    host: '',
    port: '',
    username: '',
    password: '',
    remark: '',
    ipCheckChannel: 'db-ip'
  })

  // 解析批量输入的代理
  const parseBatchInput = (): ProxyFormData[] => {
    const lines = batchInput.value.trim().split('\n').filter(line => line.trim())
    const proxies: ProxyFormData[] = []

    console.log('批量导入：开始解析', lines.length, '行')

    for (const line of lines) {
      const trimmedLine = line.trim()
      if (!trimmedLine) continue

      console.log('正在解析行:', trimmedLine)

      // 只支持标准格式: protocol://username:password@host:port
      // 例如: http://user123:pass456@proxy.example.com:8080
      //       socks5://myuser:mypass@192.168.1.1:1080
      const match = trimmedLine.match(/^(\w+):\/\/([^:]+):([^@]+)@([^\s:]+):(\d+)$/)
      
      if (match) {
        const [, type, username, password, host, port] = match
        
        console.log('解析成功:', { type, username, password: '***', host, port })
        
        // 验证协议类型
        const validTypes = ['http', 'https', 'socks5']
        const normalizedType = type.toLowerCase()
        
        if (!validTypes.includes(normalizedType)) {
          console.warn(`无效的协议类型: ${type}, 跳过该行: ${trimmedLine}`)
          continue
        }
        
        proxies.push({
          type: normalizedType as 'http' | 'https' | 'socks5',
          source: 'custom',
          tag: '',
          host,
          port,
          username,
          password,
          remark: '',
          ipCheckChannel: 'db-ip'
        })
      } else {
        console.warn(`格式不正确，跳过该行: ${trimmedLine}`)
        console.warn('字符串长度:', trimmedLine.length, '字符编码:', Array.from(trimmedLine).map(c => c.charCodeAt(0)))
      }
    }

    console.log('批量导入：解析完成，成功解析', proxies.length, '个代理')
    return proxies
  }

  // ==================== 计算属性 ====================
  const activeCount = computed(() => proxies.value.filter(p => p.status === 'active').length)
  const errorCount = computed(() => proxies.value.filter(p => p.status === 'error').length)

  const filteredProxies = computed(() => {
    let result = proxies.value
    
    if (filterType.value) {
      result = result.filter(p => p.type === filterType.value)
    }
    
    if (filterSource.value) {
      result = result.filter(p => p.source === filterSource.value)
    }
    
    if (filterStatus.value) {
      result = result.filter(p => p.status === filterStatus.value)
    }
    
    if (uiStore.searchKeyword) {
      const keyword = uiStore.searchKeyword.toLowerCase()
      result = result.filter(p => 
        p.host.toLowerCase().includes(keyword) ||
        (p.ipAddress?.toLowerCase().includes(keyword) ?? false) ||
        (p.name?.toLowerCase().includes(keyword) ?? false)
      )
    }
    
    return result
  })

  const paginatedProxies = computed(() => {
    const start = (currentPage.value - 1) * pageSize.value
    const end = start + pageSize.value
    return filteredProxies.value.slice(start, end).map((proxy, idx) => ({
      ...proxy,
      index: start + idx + 1
    }))
  })

  const totalCount = computed(() => filteredProxies.value.length)

  const isAllSelected = computed(() => {
    return paginatedProxies.value.length > 0 && selectedIds.value.length === paginatedProxies.value.length
  })

  // ==================== 方法 ====================
  const handleSelectAll = (e: Event) => {
    const target = e.target as HTMLInputElement
    if (target.checked) {
      selectedIds.value = paginatedProxies.value.map((p: Proxy & { index: number }) => p.id)
    } else {
      selectedIds.value = []
    }
  }

  const loadProxies = async () => {
    isLoading.value = true
    try {
      proxies.value = await getProxies()
    } catch (error) {
      console.error('Failed to load proxies:', error)
      Message.error('加载代理列表失败')
    } finally {
      isLoading.value = false
    }
  }

  const handleAdd = () => {
    dialogTitle.value = '添加代理'
    isEditMode.value = false
    editingId.value = null
    importMode.value = 'single'
    testStatus.value = 'idle'
    formData.type = 'socks5'
    formData.source = 'custom'
    formData.tag = ''
    formData.host = ''
    formData.port = ''
    formData.username = ''
    formData.password = ''
    formData.remark = ''
    formData.ipCheckChannel = 'db-ip'
    dialogVisible.value = true
  }

  const handleEdit = (proxy: Proxy) => {
    dialogTitle.value = '编辑代理配置'
    isEditMode.value = true
    editingId.value = proxy.id
    testStatus.value = 'idle'
    formData.type = proxy.type
    formData.source = proxy.source
    formData.tag = proxy.tag
    formData.host = proxy.host
    formData.port = proxy.port
    formData.username = proxy.username || ''
    formData.password = proxy.password || ''
    formData.remark = proxy.remark || ''
    dialogVisible.value = true
  }

  const handleCheck = async (proxy: Proxy) => {
    const loadingMsg = Message.info({
      message: `正在检测代理 ${proxy.host}:${proxy.port}...`,
      duration: 0
    })
    
    try {
      const result: ProxyTestResult = await testProxy(proxy.id)
      loadingMsg.close()
      
      if (result.success) {
        Message.success({
          message: `代理连接正常 - 延迟: ${result.latency}ms`,
          duration: 3000
        })
        
        const index = proxies.value.findIndex(p => p.id === proxy.id)
        if (index !== -1) {
          proxies.value[index].status = 'active'
          if (result.ip) proxies.value[index].ipAddress = result.ip
          if (result.location) proxies.value[index].location = result.location
          if (result.latency !== undefined) proxies.value[index].latency = result.latency
        }
      } else {
        Message.error({
          message: `代理连接失败: ${result.error || '未知错误'}`,
          duration: 5000
        })
        
        const index = proxies.value.findIndex(p => p.id === proxy.id)
        if (index !== -1) {
          proxies.value[index].status = 'error'
        }
      }
    } catch (error: unknown) {
      loadingMsg.close()
      console.error('Failed to test proxy:', error)
      const errorMsg = error instanceof Error ? error.message : '未知错误'
      Message.error(`检测代理失败: ${errorMsg}`)
    }
  }

  const handleDelete = async (proxy: Proxy) => {
    try {
      await ElMessageBox.confirm(
        `确定要删除代理「${proxy.host}:${proxy.port}」吗？`,
        '删除确认',
        { type: 'warning', confirmButtonText: '删除', cancelButtonText: '取消' }
      )
      await deleteProxy(proxy.id)
      proxies.value = proxies.value.filter(p => p.id !== proxy.id)
      Message.success('删除成功')
    } catch (error) {
      if (error !== 'cancel') {
        console.error('Failed to delete proxy:', error)
        Message.error('删除代理失败')
      }
    }
  }

  const handleBatchCheck = async () => {
    if (selectedIds.value.length === 0) {
      Message.warning('请先选择要检查的代理')
      return
    }
    
    const loadingMsg = Message.info({
      message: `正在批量检测 ${selectedIds.value.length} 个代理...`,
      duration: 0
    })
    
    try {
      const results = await batchTestProxies(selectedIds.value)
      loadingMsg.close()
      
      let successCount = 0
      let failCount = 0
      
      for (const result of results) {
        const index = proxies.value.findIndex(p => p.id === result.proxy_id)
        if (index !== -1) {
          if (result.success) {
            successCount++
            proxies.value[index].status = 'active'
            if (result.ip) proxies.value[index].ipAddress = result.ip
            if (result.location) proxies.value[index].location = result.location
            if (result.latency !== undefined) proxies.value[index].latency = result.latency
          } else {
            failCount++
            proxies.value[index].status = 'error'
          }
        }
      }
      
      if (failCount === 0) {
        Message.success(`批量检测完成：全部 ${successCount} 个代理连接正常`)
      } else if (successCount === 0) {
        Message.error(`批量检测完成：全部 ${failCount} 个代理连接失败`)
      } else {
        Message.warning(`批量检测完成：${successCount} 个正常，${failCount} 个失败`)
      }
    } catch (error: unknown) {
      loadingMsg.close()
      console.error('Batch test failed:', error)
      const errorMsg = error instanceof Error ? error.message : '未知错误'
      Message.error(`批量检测失败: ${errorMsg}`)
    }
  }

  const handleBatchDelete = async () => {
    if (selectedIds.value.length === 0) {
      Message.warning('请先选择要删除的代理')
      return
    }
    
    try {
      await ElMessageBox.confirm(
        `确定要删除选中的 ${selectedIds.value.length} 个代理吗？此操作不可恢复！`,
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
          await deleteProxy(id)
          successCount++
        } catch {
          failCount++
        }
      }
      
      await loadProxies()
      selectedIds.value = []
      
      if (failCount === 0) {
        Message.success(`成功删除 ${successCount} 个代理`)
      } else {
        Message.warning(`删除完成：成功 ${successCount} 个，失败 ${failCount} 个`)
      }
    } catch {
      // 用户取消删除
    }
  }

  const handleTestConnection = async () => {
    if (!formData.host.trim() || !formData.port.trim()) {
      Message.warning('请先输入代理主机和端口')
      return
    }
    
    testStatus.value = 'testing'
    
    try {
      const result = await testProxyConfig(
        formData.type,
        formData.host, 
        formData.port,
        formData.username || undefined,
        formData.password || undefined
      )
      if (result.success) {
        testStatus.value = 'success'
        Message.success(`连接成功 - 延迟: ${result.latency}ms${result.ip ? ` - IP: ${result.ip}` : ''}`)
      } else {
        testStatus.value = 'error'
        Message.error(`连接失败: ${result.error || '未知错误'}`)
      }
    } catch (error: unknown) {
      testStatus.value = 'error'
      const errorMsg = error instanceof Error ? error.message : '未知错误'
      Message.error(`测试连接出错: ${errorMsg}`)
    }
  }

  const handleSubmit = async () => {
    if (!formData.host.trim() && importMode.value === 'single') {
      Message.warning('请输入代理主机')
      return
    }
    
    if (!formData.port.trim() && importMode.value === 'single') {
      Message.warning('请输入代理端口')
      return
    }
    
    if (importMode.value === 'single') {
      // 单个添加模式
      const proxyName = formData.tag || `${formData.host}:${formData.port}`
      const existingProxy = proxies.value.find(
        p => (p.name === proxyName || `${p.host}:${p.port}` === `${formData.host}:${formData.port}`) && p.id !== editingId.value
      )
      
      if (existingProxy) {
        Message.error('代理配置已存在')
        return
      }
    }
    
    try {
      if (isEditMode.value && editingId.value) {
        // 编辑模式
        const updated = await updateProxy(editingId.value, {
          name: formData.tag || `${formData.host}:${formData.port}`,
          type: formData.type,
          host: formData.host,
          port: formData.port,
          source: formData.source,
          tag: formData.tag,
          username: formData.username || undefined,
          password: formData.password || undefined,
          remark: formData.remark || undefined
        })
        const index = proxies.value.findIndex(p => p.id === editingId.value)
        if (index !== -1) {
          proxies.value[index] = updated
        }
        Message.success('保存成功')
      } else if (importMode.value === 'single') {
        // 单个添加模式
        const newProxy = await createProxy({
          name: formData.tag || `${formData.host}:${formData.port}`,
          type: formData.type,
          host: formData.host,
          port: formData.port,
          source: formData.source,
          tag: formData.tag,
          username: formData.username || undefined,
          password: formData.password || undefined,
          remark: formData.remark || undefined
        })
        proxies.value.unshift(newProxy)
        Message.success('添加成功')
      } else {
        // 批量导入模式
        const proxiesToImport = parseBatchInput()
        if (proxiesToImport.length === 0) {
          Message.warning('请输入有效的代理配置')
          return
        }

        let successCount = 0
        let failCount = 0
        let skipCount = 0
        
        for (const proxy of proxiesToImport) {
          try {
            // 检查是否已存在相同的 host:port 组合
            const existingProxy = proxies.value.find(
              p => p.host === proxy.host && p.port === proxy.port
            )
            
            if (existingProxy) {
              console.log(`代理已存在，跳过: ${proxy.host}:${proxy.port}`)
              skipCount++
              continue
            }
            
            await createProxy({
              name: proxy.tag || `${proxy.host}:${proxy.port}`,
              type: proxy.type,
              host: proxy.host,
              port: proxy.port,
              source: proxy.source,
              tag: proxy.tag,
              username: proxy.username || undefined,
              password: proxy.password || undefined,
              remark: proxy.remark || undefined
            })
            successCount++
          } catch (error) {
            console.error('Failed to import proxy:', error)
            failCount++
          }
        }
        
        // 构建提示消息
        const messages: string[] = []
        if (successCount > 0) messages.push(`成功 ${successCount} 个`)
        if (skipCount > 0) messages.push(`跳过 ${skipCount} 个（已存在）`)
        if (failCount > 0) messages.push(`失败 ${failCount} 个`)
        
        if (failCount === 0 && skipCount === 0) {
          Message.success(`成功批量导入 ${successCount} 个代理`)
        } else if (successCount === 0 && failCount > 0) {
          Message.error(`批量导入失败：${messages.join('，')}`)
        } else {
          Message.warning(`批量导入完成：${messages.join('，')}`)
        }
      }
      
      dialogVisible.value = false
    } catch (error: unknown) {
      if (importMode.value !== 'batch') {
        // 只在非批量模式下显示错误，批量模式已经在循环中处理了
        console.error('Failed to save proxy:', error)
        const errorMessage = error instanceof Error ? error.message : (isEditMode.value ? '保存失败' : '添加失败')
        const detailMessage = errorMessage.includes(':') 
          ? errorMessage.split(':').slice(1).join(':').trim() 
          : errorMessage
        Message.error(detailMessage)
      }
    }
  }

  // ==================== 辅助函数 ====================
  const getProxyIcon = (proxy: Proxy) => {
    if (proxy.status === 'error') return 'wifi_off'
    if (proxy.source === 'api') return 'public'
    if (proxy.type === 'socks5') return 'vpn_lock'
    if (proxy.usedCount && proxy.usedCount > 5) return 'speed'
    return 'dns'
  }

  const getProxyIconClass = (proxy: Proxy) => {
    if (proxy.status === 'error') return 'error'
    if (proxy.source === 'api') return 'purple'
    if (proxy.type === 'socks5') return 'indigo'
    if (proxy.usedCount && proxy.usedCount > 5) return 'orange'
    return 'default'
  }

  const getCountryFlag = (location: string) => {
    const loc = location.toLowerCase()
    if (loc.includes('united states') || loc.includes('us')) return 'https://flagcdn.com/w40/us.png'
    if (loc.includes('united kingdom') || loc.includes('gb') || loc.includes('uk')) return 'https://flagcdn.com/w40/gb.png'
    if (loc.includes('germany') || loc.includes('de')) return 'https://flagcdn.com/w40/de.png'
    if (loc.includes('japan') || loc.includes('jp')) return 'https://flagcdn.com/w40/jp.png'
    if (loc.includes('singapore') || loc.includes('sg')) return 'https://flagcdn.com/w40/sg.png'
    if (loc.includes('china') || loc.includes('cn')) return 'https://flagcdn.com/w40/cn.png'
    return 'https://flagcdn.com/w40/un.png'
  }

  const getStatusClass = (proxy: Proxy) => {
    if (proxy.status === 'active') return 'active'
    if (proxy.status === 'error') return 'error'
    return 'pending'
  }

  const getStatusText = (proxy: Proxy) => {
    if (proxy.status === 'active') return proxy.latency ? `${proxy.latency}ms` : '在线'
    if (proxy.status === 'error') return '超时'
    return '未检查'
  }

  const getTypeClass = (type: string) => {
    const classes: Record<string, string> = {
      http: 'type-http',
      https: 'type-https',
      socks5: 'type-socks5'
    }
    return classes[type] || ''
  }

  // ==================== 生命周期 ====================
  let unlistenCreated: UnlistenFn | null = null
  let unlistenUpdated: UnlistenFn | null = null
  let unlistenDeleted: UnlistenFn | null = null

  const setupEventListeners = async () => {
    unlistenCreated = await listen('proxy:created', () => {
      loadProxies()
    })
    
    unlistenUpdated = await listen('proxy:updated', () => {
      loadProxies()
    })
    
    unlistenDeleted = await listen('proxy:deleted', () => {
      loadProxies()
    })
  }

  const cleanupEventListeners = () => {
    unlistenCreated?.()
    unlistenUpdated?.()
    unlistenDeleted?.()
  }

  onMounted(async () => {
    loadProxies()
    await setupEventListeners()
  })

  onUnmounted(() => {
    cleanupEventListeners()
  })

  return {
    // 状态
    proxies,
    isLoading,
    selectedIds,
    filterType,
    filterSource,
    filterStatus,
    currentPage,
    pageSize,
    pageSizes,
    dialogVisible,
    dialogTitle,
    isEditMode,
    editingId,
    importMode,
    batchInput,
    testStatus,
    formData,
    
    // 计算属性
    activeCount,
    errorCount,
    filteredProxies,
    paginatedProxies,
    totalCount,
    isAllSelected,
    
    // 方法
    handleSelectAll,
    loadProxies,
    handleAdd,
    handleEdit,
    handleCheck,
    handleDelete,
    handleBatchCheck,
    handleBatchDelete,
    handleTestConnection,
    handleSubmit,
    
    // 辅助函数
    getProxyIcon,
    getProxyIconClass,
    getCountryFlag,
    getStatusClass,
    getStatusText,
    getTypeClass
  }
}

