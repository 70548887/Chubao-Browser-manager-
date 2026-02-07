/**
 * 应用更新 Composable
 * 
 * 封装启动器和内核的更新检测、下载、安装逻辑。
 */

import { ref, onMounted, onUnmounted } from 'vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import * as updateApi from '@/api/updateApi'
import { useUIStore } from '@/stores/ui.store'

export function useAppUpdate() {
  const uiStore = useUIStore()
  
  // ========== 状态 ==========
  const launcherUpdate = ref<updateApi.UpdateInfo | null>(null)
  const kernelUpdate = ref<updateApi.UpdateInfo | null>(null)
  const isChecking = ref(false)
  const isDownloading = ref(false)
  const downloadProgress = ref<updateApi.UpdateDownloadProgress | null>(null)
  const downloadedFilePath = ref('')
  const selectedSource = ref<updateApi.DownloadSource | null>(null)

  let unlistenProgress: (() => void) | null = null
  let unlistenRestart: (() => void) | null = null

  // ========== 检查更新 ==========
  const checkAllUpdates = async () => {
    isChecking.value = true
    uiStore.setCheckingUpdate(true)
    try {
      const [launcher, kernel] = await Promise.allSettled([
        updateApi.checkAppUpdate(),
        updateApi.checkKernelUpdate(),
      ])

      if (launcher.status === 'fulfilled') {
        launcherUpdate.value = launcher.value
        uiStore.setLauncherUpdate(launcher.value)
      } else {
        console.error('检查启动器更新失败:', launcher.reason)
      }
      
      if (kernel.status === 'fulfilled') {
        kernelUpdate.value = kernel.value
        uiStore.setKernelUpdate(kernel.value)
      } else {
        console.error('检查内核更新失败:', kernel.reason)
      }

      const hasAny =
        launcherUpdate.value?.has_update || kernelUpdate.value?.has_update
      if (hasAny) {
        ElMessage.success('发现新版本可用')
      } else {
        ElMessage.info('当前已是最新版本')
      }
    } catch (error) {
      ElMessage.error(`检查更新失败: ${error}`)
    } finally {
      isChecking.value = false
      uiStore.setCheckingUpdate(false)
    }
  }

  /** 仅检查启动器更新 */
  const checkLauncherUpdate = async () => {
    isChecking.value = true
    try {
      launcherUpdate.value = await updateApi.checkAppUpdate()
      if (launcherUpdate.value.has_update) {
        ElMessage.success(`发现启动器新版本 v${launcherUpdate.value.version}`)
      } else {
        ElMessage.info('启动器已是最新版本')
      }
    } catch (error) {
      ElMessage.error(`检查启动器更新失败: ${error}`)
    } finally {
      isChecking.value = false
    }
  }

  /** 仅检查内核更新 */
  const checkKernelUpdateOnly = async () => {
    isChecking.value = true
    try {
      kernelUpdate.value = await updateApi.checkKernelUpdate()
      if (kernelUpdate.value.has_update) {
        ElMessage.success(`发现内核新版本 v${kernelUpdate.value.version}`)
      } else {
        ElMessage.info('内核已是最新版本')
      }
    } catch (error) {
      ElMessage.error(`检查内核更新失败: ${error}`)
    } finally {
      isChecking.value = false
    }
  }

  // ========== 下载并安装启动器 ==========
  const handleLauncherUpdate = async (source: updateApi.DownloadSource) => {
    if (!launcherUpdate.value?.file_hash) {
      ElMessage.error('缺少文件校验信息，无法安全更新')
      return
    }

    selectedSource.value = source
    isDownloading.value = true
    
    try {
      const filePath = await updateApi.downloadAppUpdate(
        source.url,
        launcherUpdate.value.file_hash
      )
      downloadedFilePath.value = filePath

      await ElMessageBox.confirm(
        '下载完成且校验通过，是否立即安装？安装时程序将自动关闭并重启。',
        '安装启动器更新',
        { confirmButtonText: '立即安装', cancelButtonText: '稍后安装', type: 'success' }
      )

      await updateApi.installAppUpdate(filePath)
    } catch (error: any) {
      if (error !== 'cancel') {
        ElMessage.error(`更新失败: ${error}`)
      }
    } finally {
      isDownloading.value = false
      selectedSource.value = null
    }
  }

  // ========== 下载并安装内核 ==========
  const handleKernelUpdate = async (source: updateApi.DownloadSource) => {
    if (!kernelUpdate.value?.file_hash || !kernelUpdate.value?.version) {
      ElMessage.error('缺少文件校验信息，无法安全更新')
      return
    }

    // 检查运行中的浏览器
    const runningCount = await updateApi.getRunningBrowserCount()
    if (runningCount > 0) {
      ElMessage.warning(`请先关闭所有运行中的浏览器（当前 ${runningCount} 个）`)
      return
    }

    selectedSource.value = source
    isDownloading.value = true
    
    try {
      const filePath = await updateApi.downloadKernelUpdate(
        source.url,
        kernelUpdate.value.file_hash
      )

      await updateApi.installKernelUpdate(filePath, kernelUpdate.value.version)
      ElMessage.success(`内核已更新到 ${kernelUpdate.value.version}`)
      kernelUpdate.value = null
    } catch (error) {
      ElMessage.error(`内核更新失败: ${error}`)
    } finally {
      isDownloading.value = false
      selectedSource.value = null
    }
  }

  // ========== 生命周期 ==========
  onMounted(async () => {
    // 监听下载进度
    const unlisten1 = await updateApi.onUpdateProgress((progress) => {
      downloadProgress.value = progress
    })
    unlistenProgress = unlisten1

    // 监听重启事件
    const unlisten2 = await updateApi.onAppWillRestart(() => {
      ElMessage.info('应用即将重启...')
    })
    unlistenRestart = unlisten2
  })

  onUnmounted(() => {
    unlistenProgress?.()
    unlistenRestart?.()
  })

  return {
    // 状态
    launcherUpdate,
    kernelUpdate,
    isChecking,
    isDownloading,
    downloadProgress,
    downloadedFilePath,
    selectedSource,
    // 方法
    checkAllUpdates,
    checkLauncherUpdate,
    checkKernelUpdateOnly,
    handleLauncherUpdate,
    handleKernelUpdate,
    // 工具函数
    formatFileSize: updateApi.formatFileSize,
    formatSpeed: updateApi.formatSpeed,
  }
}
