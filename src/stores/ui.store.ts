import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import type { UpdateInfo, UpdateDownloadProgress, DownloadSource } from '@/api/updateApi'

export const useUIStore = defineStore('ui', () => {
  const searchKeyword = ref('')
  
  // ========== 更新相关状态 ==========
  const showUpdateDialog = ref(false)
  
  // 更新信息
  const launcherUpdate = ref<UpdateInfo | null>(null)
  const kernelUpdate = ref<UpdateInfo | null>(null)
  
  // 更新状态
  const isCheckingUpdate = ref(false)
  const isDownloadingUpdate = ref(false)
  const downloadProgress = ref<UpdateDownloadProgress | null>(null)
  const selectedDownloadSource = ref<DownloadSource | null>(null)
  
  // 是否有可用更新
  const hasUpdate = computed(() => {
    return launcherUpdate.value?.has_update || kernelUpdate.value?.has_update
  })

  // ========== 搜索相关 ==========
  const setSearchKeyword = (keyword: string) => {
    searchKeyword.value = keyword
  }

  const clearSearchKeyword = () => {
    searchKeyword.value = ''
  }

  // ========== 更新相关方法 ==========
  const setUpdateDialogVisible = (visible: boolean) => {
    showUpdateDialog.value = visible
  }

  const setLauncherUpdate = (info: UpdateInfo | null) => {
    launcherUpdate.value = info
  }

  const setKernelUpdate = (info: UpdateInfo | null) => {
    kernelUpdate.value = info
  }

  const setCheckingUpdate = (checking: boolean) => {
    isCheckingUpdate.value = checking
  }

  const setDownloadingUpdate = (downloading: boolean) => {
    isDownloadingUpdate.value = downloading
  }

  const setDownloadProgress = (progress: UpdateDownloadProgress | null) => {
    downloadProgress.value = progress
  }

  const setSelectedDownloadSource = (source: DownloadSource | null) => {
    selectedDownloadSource.value = source
  }

  const clearUpdateState = () => {
    launcherUpdate.value = null
    kernelUpdate.value = null
    isCheckingUpdate.value = false
    isDownloadingUpdate.value = false
    downloadProgress.value = null
    selectedDownloadSource.value = null
  }

  return {
    // 搜索
    searchKeyword,
    setSearchKeyword,
    clearSearchKeyword,
    // 更新状态
    showUpdateDialog,
    launcherUpdate,
    kernelUpdate,
    hasUpdate,
    isCheckingUpdate,
    isDownloadingUpdate,
    downloadProgress,
    selectedDownloadSource,
    // 更新方法
    setUpdateDialogVisible,
    setLauncherUpdate,
    setKernelUpdate,
    setCheckingUpdate,
    setDownloadingUpdate,
    setDownloadProgress,
    setSelectedDownloadSource,
    clearUpdateState,
  }
})
