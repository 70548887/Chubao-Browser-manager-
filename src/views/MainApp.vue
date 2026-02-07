<script setup lang="ts">
/**
 * @description 主应用视图 - 登录后的主界面
 * @author DeepAgent
 */
import { ref, provide, computed, watch, onMounted } from 'vue'
import MainLayout from '@/layouts/MainLayout.vue'
import DashboardView from '@/features/dashboard/DashboardView.vue'
import GroupManagement from '@/features/groups/GroupManagement.vue'
import RecycleBin from '@/features/recycle/RecycleBin.vue'
import ProxyManagement from '@/features/proxy/ProxyManagement.vue'
import TagManagement from '@/features/tags/TagManagement.vue'
import SettingsView from '@/features/settings/SettingsView.vue'
import RPAManagement from '@/features/rpa/RPAManagement.vue'
import ExtensionsCenter from '@/features/extensions/ExtensionsCenter.vue'
import CreateWindowDialog from '@/features/dashboard/components/CreateWindowDialog.vue'
import UpdateDialog from '@/components/common/UpdateDialog.vue'
import { useUIStore } from '@/stores/ui.store'
import { createProfile } from '@/api/profileApi'
import { Message } from '@/utils/message'
import { triggerKernelExtraction, isKernelInstalled, getKernelPath } from '@/api/kernelApi'
import { getSettingValue, setSettingValue } from '@/api/settingsApi'

// 当前页面
const currentPage = ref<'dashboard' | 'groups' | 'recycle' | 'proxy' | 'tags' | 'settings' | 'rpa' | 'extensions'>('dashboard')
const uiStore = useUIStore()

// 监听页面切换，清空搜索框
watch(currentPage, () => {
  uiStore.clearSearchKeyword()
})

const dashboardRef = ref()

// 新建窗口对话框状态
const createWindowDialogVisible = ref(false)
const isCreatingProfile = ref(false) // 防止重复提交

const handleCreateNew = () => {
  console.log('🔵 [MainApp] handleCreateNew 被调用，打开新建窗口对话框')
  createWindowDialogVisible.value = true
}

const handleCreateWindowSubmit = async (formData: any) => {
  // 防止重复提交
  if (isCreatingProfile.value) {
    console.log('❗ [MainApp] 正在创建中，忽略重复提交')
    return
  }

  isCreatingProfile.value = true
  console.log('✅ [MainApp] 新建窗口表单数据:', formData)

  try {
    // 构建代理配置（仅在填写了代理信息时才传递）
    let proxyConfig: { type: 'http' | 'https' | 'socks5'; host: string; port: number; username?: string; password?: string } | undefined = undefined
    if (formData.proxyHost && formData.proxyPort) {
      // 转换代理类型
      const proxyTypeMap: Record<string, 'http' | 'https' | 'socks5'> = {
        'http': 'http',
        'https': 'https',
        'socks5': 'socks5',
        'ssh': 'socks5'  // SSH 暂时映射为 socks5
      }
      proxyConfig = {
        type: proxyTypeMap[formData.proxyProtocol] || 'socks5',
        host: formData.proxyHost,
        port: parseInt(formData.proxyPort, 10),
        username: formData.proxyUsername || undefined,
        password: formData.proxyPassword || undefined,
      }
    }

    // 调用后端API创建窗口
    const profileData = {
      name: formData.name || `窗口_${Date.now()}`,
      group: formData.groupId || 'default',
      remark: formData.remark || '',
      fingerprint: formData.fingerprint,
      proxy: proxyConfig,
    }

    console.log('📤 [MainApp] 调用 createProfile API:', profileData)
    const newProfile = await createProfile(profileData)
    console.log('✅ [MainApp] 创建成功:', newProfile)

    // 注意：不在这里显示成功消息，eventListener.ts 会监听 profile:created 事件并显示

    // 关闭对话框（API 成功后才关闭）
    createWindowDialogVisible.value = false

    // 刷新Dashboard列表
    dashboardRef.value?.handleDrawerSuccess?.()
  } catch (error) {
    console.error('❌ [MainApp] 创建窗口失败:', error)
    Message.error(`创建失败: ${error}`)
  } finally {
    isCreatingProfile.value = false
  }
}

// 页面导航
const navigateTo = (page: 'dashboard' | 'groups' | 'recycle' | 'proxy' | 'tags' | 'settings' | 'rpa' | 'extensions') => {
  currentPage.value = page
}

// 提供给子组件
provide('currentPage', computed(() => currentPage.value))
provide('navigateTo', navigateTo)

// 组件挂载后触发内核检查 (异步后台执行,不阻塞UI)
onMounted(async () => {
  // 延迟触发内核检查和解压 (给用户登录动画一点时间)
  setTimeout(async () => {
    try {
      console.log('🔍 [MainApp] 触发延迟内核检查...')
      const triggered = await triggerKernelExtraction()
      if (triggered) {
        console.log('✅ [MainApp] 内核解压已在后台触发')
      } else {
        console.log('ℹ️ [MainApp] 内核已存在,无需解压')
      }
      
      // 自动保存内核路径到数据库（如果内核存在但数据库为空）
      await autoSaveKernelPath()
    } catch (error) {
      console.error('❌ [MainApp] 触发内核检查失败:', error)
      // 静默失败,不影响用户使用
    }
  }, 1000) // 登录后1秒触发
})

/**
 * 自动检测并保存内核路径到数据库
 * 解决：内核文件存在但数据库 kernel_path 为空的问题
 */
async function autoSaveKernelPath() {
  try {
    // 检查数据库中是否已有路径
    const savedPath = await getSettingValue('kernel_path')
    if (savedPath && savedPath.trim() !== '') {
      console.log('✅ [MainApp] 内核路径已配置:', savedPath)
      return
    }
    
    // 数据库为空，检查内核是否已安装
    const installed = await isKernelInstalled()
    if (!installed) {
      console.log('ℹ️ [MainApp] 内核未安装，跳过路径配置')
      return
    }
    
    // 内核已安装，获取路径并保存
    const kernelPath = await getKernelPath()
    if (kernelPath) {
      await setSettingValue('kernel_path', kernelPath)
      console.log('✅ [MainApp] 内核路径已自动保存到数据库:', kernelPath)
    }
  } catch (error) {
    console.error('❌ [MainApp] 自动保存内核路径失败:', error)
  }
}
</script>

<template>
  <MainLayout @create-new="handleCreateNew">
    <DashboardView v-if="currentPage === 'dashboard'" ref="dashboardRef" @create-new-window="handleCreateNew" />
    <GroupManagement v-else-if="currentPage === 'groups'" />
    <RecycleBin v-else-if="currentPage === 'recycle'" />
    <ProxyManagement v-else-if="currentPage === 'proxy'" />
    <TagManagement v-else-if="currentPage === 'tags'" />
    <SettingsView v-else-if="currentPage === 'settings'" />
    <RPAManagement v-else-if="currentPage === 'rpa'" />
    <ExtensionsCenter v-else-if="currentPage === 'extensions'" />
  </MainLayout>

  <!-- 新建窗口对话框 -->
  <CreateWindowDialog :visible="createWindowDialogVisible" @close="createWindowDialogVisible = false"
    @submit="handleCreateWindowSubmit" />

  <!-- 版本更新弹窗 -->
  <UpdateDialog />
</template>
