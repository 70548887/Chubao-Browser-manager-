<template>
  <div class="settings-view">
    <!-- 设置表单 -->
    <div class="settings-container">
      <!-- 浏览器内核设置 -->
      <div class="setting-section">
        <div class="section-header">
          <h2 class="section-title">浏览器内核</h2>
          <p class="section-description">配置浏览器内核可执行文件路径</p>
        </div>

        <!-- 内核状态卡片 -->
        <div class="kernel-status-card">
          <div class="kernel-info">
            <div class="kernel-icon" :class="{ installed: kernelInstalled }">
              {{ kernelInstalled ? '✓' : '!' }}
            </div>
            <div class="kernel-details">
              <div class="kernel-title">
                {{ kernelInstalled ? '内核已安装' : '内核未安装' }}
              </div>
              <div v-if="kernelVersion" class="kernel-version">
                版本: {{ kernelVersion.version }} | {{ kernelVersion.platform }}
              </div>
              <div v-else-if="!kernelInstalled" class="kernel-version">
                请下载或选择内核文件
              </div>
            </div>
          </div>
          
          <!-- 下载进度条 -->
          <div v-if="isDownloading" class="download-progress">
            <div class="progress-bar">
              <div 
                class="progress-fill" 
                :style="{ width: downloadPercent + '%' }"
              ></div>
            </div>
            <div class="progress-info">
              <span>{{ downloadProgress?.message || '准备下载...' }}</span>
              <span>{{ formatSpeed(downloadProgress?.speed || 0) }}</span>
            </div>
          </div>
          
          <!-- 操作按钮 -->
          <div class="kernel-actions">
            <button 
              v-if="!kernelInstalled && !isDownloading" 
              class="btn-download" 
              @click="handleDownloadKernel"
            >
              <span class="icon">⬇</span>
              下载内核
            </button>
            <button 
              v-if="isDownloading" 
              class="btn-cancel-download" 
              disabled
            >
              <span class="icon loading">⏳</span>
              下载中...
            </button>
            <button 
              v-if="kernelInstalled" 
              class="btn-use-kernel" 
              @click="useInstalledKernel"
            >
              <span class="icon">✓</span>
              使用此内核
            </button>
          </div>
        </div>

        <!-- 自定义内核下载地址 -->
        <div class="setting-item">
          <label class="setting-label">内核下载地址</label>
          <div class="setting-input-group">
            <input
              v-model="customKernelUrl"
              type="text"
              class="setting-input"
              placeholder="输入自定义内核下载地址，或使用默认地址"
            />
            <button class="btn-select" @click="resetKernelUrl">
              <span class="icon">↺</span>
              重置
            </button>
          </div>
          <p class="setting-hint">
            默认从 GitHub Releases 下载，如下载缓慢可使用镜像地址
          </p>
        </div>

        <div class="setting-item">
          <label class="setting-label">
            内核路径
            <span class="required">*</span>
          </label>
          <div class="setting-input-group">
            <input
              v-model="settings.kernelPath"
              type="text"
              class="setting-input"
              placeholder="请选择浏览器内核可执行文件"
              readonly
            />
            <button class="btn-select" @click="selectKernelPath">
              <span class="icon">📁</span>
              选择文件
            </button>
          </div>
          <p class="setting-hint" v-if="bundledKernelPath">
            检测到内嵌内核：{{ bundledKernelPath }}<br/>
            如未配置将自动使用内嵌内核
          </p>
          <p class="setting-hint" v-else>
            例如：C:\Program Files\Google\Chrome\Application\chrome.exe
          </p>
        </div>

        <div class="setting-item">
          <label class="setting-label">用户数据目录</label>
          <div class="setting-input-group">
            <input
              v-model="settings.userDataDir"
              type="text"
              class="setting-input"
              placeholder="请选择用户数据存储目录"
              readonly
            />
            <button class="btn-select" @click="selectUserDataDir">
              <span class="icon">📁</span>
              选择目录
            </button>
          </div>
          <p class="setting-hint">
            用于存储浏览器配置文件和用户数据，留空则使用默认目录
          </p>
        </div>
      </div>

      <!-- 代理设置 -->
      <div class="setting-section">
        <div class="section-header">
          <h2 class="section-title">默认代理</h2>
          <p class="section-description">配置新窗口的默认代理设置</p>
        </div>

        <div class="setting-item">
          <label class="setting-label">代理服务器</label>
          <input
            v-model="settings.defaultProxy"
            type="text"
            class="setting-input"
            placeholder="例如：http://127.0.0.1:8080"
          />
          <p class="setting-hint">
            格式：协议://主机:端口，留空表示不使用代理
          </p>
        </div>
      </div>

      <!-- 操作按钮 -->
      <div class="setting-actions">
        <button class="btn-cancel" @click="handleReset">
          <span class="icon">↺</span>
          重置
        </button>
        <button class="btn-save" @click="handleSave" :disabled="!isValid || isSaving">
          <span v-if="isSaving" class="icon loading">⏳</span>
          <span v-else class="icon">✓</span>
          {{ isSaving ? '保存中...' : '保存设置' }}
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue'
import { ElMessage } from 'element-plus'
import { open as openDialog } from '@tauri-apps/plugin-dialog'
import * as settingsApi from '@/api/settingsApi'
import * as kernelApi from '@/api/kernelApi'
import type { DownloadProgress, KernelVersionInfo } from '@/api/kernelApi'

// Settings data
const settings = ref({
  kernelPath: '',
  userDataDir: '',
  defaultProxy: ''
})

// Original settings (for reset)
const originalSettings = ref({ ...settings.value })

// Loading and saving state
const isLoading = ref(false)
const isSaving = ref(false)

// Kernel download state
const kernelInstalled = ref(false)
const kernelVersion = ref<KernelVersionInfo | null>(null)
const isDownloading = ref(false)
const downloadProgress = ref<DownloadProgress | null>(null)
const customKernelUrl = ref(kernelApi.DEFAULT_KERNEL_URL)
const bundledKernelPath = ref<string | null>(null)

// Event unsubscribe functions
let unlistenProgress: (() => void) | null = null
let unlistenComplete: (() => void) | null = null
let unlistenError: (() => void) | null = null

// Calculate download percentage
const downloadPercent = computed(() => {
  if (!downloadProgress.value || !downloadProgress.value.total) return 0
  return Math.round((downloadProgress.value.downloaded / downloadProgress.value.total) * 100)
})

// Format download speed
const formatSpeed = (bytesPerSec: number): string => {
  if (bytesPerSec < 1024) return `${bytesPerSec} B/s`
  if (bytesPerSec < 1024 * 1024) return `${(bytesPerSec / 1024).toFixed(1)} KB/s`
  return `${(bytesPerSec / (1024 * 1024)).toFixed(1)} MB/s`
}

// Validate settings
const isValid = computed(() => {
  return settings.value.kernelPath.trim() !== ''
})

// Check kernel status
const checkKernelStatus = async () => {
  try {
    kernelInstalled.value = await kernelApi.isKernelInstalled()
    if (kernelInstalled.value) {
      kernelVersion.value = await kernelApi.getKernelVersion()
    }
    // Check bundled kernel
    bundledKernelPath.value = await kernelApi.getBundledKernelPath()
  } catch (error) {
    console.error('Failed to check kernel status:', error)
  }
}

// Reset kernel URL to default
const resetKernelUrl = () => {
  customKernelUrl.value = kernelApi.DEFAULT_KERNEL_URL
  ElMessage.info('已重置为默认下载地址')
}

// Handle kernel download
const handleDownloadKernel = async () => {
  if (!customKernelUrl.value.trim()) {
    ElMessage.warning('请输入内核下载地址')
    return
  }

  isDownloading.value = true
  downloadProgress.value = {
    downloaded: 0,
    total: null,
    speed: 0,
    status: 'Downloading',
    message: '准备下载...'
  }

  try {
    await kernelApi.downloadKernel(customKernelUrl.value)
  } catch (error) {
    isDownloading.value = false
    ElMessage.error('启动下载失败: ' + error)
  }
}

// Use installed kernel
const useInstalledKernel = async () => {
  try {
    const kernelPath = await kernelApi.getKernelPath()
    settings.value.kernelPath = kernelPath
    ElMessage.success('已设置为已安装的内核路径')
  } catch (error) {
    ElMessage.error('获取内核路径失败')
  }
}

// Setup event listeners
const setupEventListeners = async () => {
  unlistenProgress = await kernelApi.onDownloadProgress((progress) => {
    downloadProgress.value = progress
  })

  unlistenComplete = await kernelApi.onDownloadComplete(async () => {
    isDownloading.value = false
    downloadProgress.value = null
    await checkKernelStatus()
    
    // Auto-set kernel path
    if (kernelInstalled.value) {
      const kernelPath = await kernelApi.getKernelPath()
      settings.value.kernelPath = kernelPath
    }
    
    ElMessage.success('内核下载安装完成！')
  })

  unlistenError = await kernelApi.onDownloadError((error) => {
    isDownloading.value = false
    downloadProgress.value = null
    ElMessage.error('下载失败: ' + error)
  })
}

// Cleanup event listeners
const cleanupEventListeners = () => {
  if (unlistenProgress) unlistenProgress()
  if (unlistenComplete) unlistenComplete()
  if (unlistenError) unlistenError()
}

// Select kernel path
const selectKernelPath = async () => {
  try {
    const selected = await openDialog({
      multiple: false,
      directory: false,
      filters: [{
        name: 'Executable',
        extensions: ['exe']
      }]
    })
    
    if (selected) {
      settings.value.kernelPath = selected
    }
  } catch (error) {
    console.error('选择文件失败:', error)
    ElMessage.error('选择文件失败')
  }
}

// Select user data directory
const selectUserDataDir = async () => {
  try {
    const selected = await openDialog({
      multiple: false,
      directory: true
    })
    
    if (selected) {
      settings.value.userDataDir = selected
    }
  } catch (error) {
    console.error('选择目录失败:', error)
    ElMessage.error('选择目录失败')
  }
}

// Load settings
const loadSettings = async () => {
  isLoading.value = true
  try {
    const allSettings = await settingsApi.getAllSettings()
    settings.value = {
      kernelPath: allSettings.kernel_path || '',
      userDataDir: allSettings.user_data_dir || '',
      defaultProxy: allSettings.default_proxy || ''
    }
    originalSettings.value = { ...settings.value }
  } catch (error) {
    console.error('加载设置失败:', error)
    ElMessage.warning('加载设置失败，使用默认值')
  } finally {
    isLoading.value = false
  }
}

// Save settings
const handleSave = async () => {
  if (!isValid.value) {
    ElMessage.warning('请填写必填项')
    return
  }

  isSaving.value = true
  try {
    await settingsApi.setSettingValue('kernel_path', settings.value.kernelPath)
    
    if (settings.value.userDataDir) {
      await settingsApi.setSettingValue('user_data_dir', settings.value.userDataDir)
    }
    
    if (settings.value.defaultProxy) {
      await settingsApi.setSettingValue('default_proxy', settings.value.defaultProxy)
    }

    originalSettings.value = { ...settings.value }
    ElMessage.success('设置保存成功')
  } catch (error) {
    console.error('保存设置失败:', error)
    ElMessage.error('保存设置失败：' + error)
  } finally {
    isSaving.value = false
  }
}

// Reset settings
const handleReset = () => {
  settings.value = { ...originalSettings.value }
  ElMessage.info('已重置为上次保存的设置')
}

// Initialize
onMounted(async () => {
  await loadSettings()
  await checkKernelStatus()
  await setupEventListeners()
})

// Cleanup
onUnmounted(() => {
  cleanupEventListeners()
})
</script>

<style scoped lang="scss">
.settings-view {
  padding: var(--spacing-lg);
  max-width: 800px;
  margin: 0 auto;
}

.page-header {
  margin-bottom: var(--spacing-xl);
}

.page-title {
  font-size: 24px;
  font-weight: 600;
  color: var(--text-primary);
  margin: 0 0 var(--spacing-xs);
}

.page-subtitle {
  font-size: 14px;
  color: var(--text-secondary);
  margin: 0;
}

.settings-container {
  background: var(--bg-primary);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-lg);
  overflow: hidden;
}

.setting-section {
  padding: var(--spacing-lg);
  border-bottom: 1px solid var(--border-color);

  &:last-child {
    border-bottom: none;
  }
}

.section-header {
  margin-bottom: var(--spacing-lg);
}

.section-title {
  font-size: 16px;
  font-weight: 600;
  color: var(--text-primary);
  margin: 0 0 var(--spacing-xs);
}

.section-description {
  font-size: 13px;
  color: var(--text-secondary);
  margin: 0;
}

// Kernel status card styles
.kernel-status-card {
  background: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-md);
  padding: var(--spacing-md);
  margin-bottom: var(--spacing-lg);
}

.kernel-info {
  display: flex;
  align-items: center;
  gap: var(--spacing-md);
  margin-bottom: var(--spacing-md);
}

.kernel-icon {
  width: 40px;
  height: 40px;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 20px;
  font-weight: bold;
  background: var(--color-warning);
  color: white;

  &.installed {
    background: var(--color-success);
  }
}

.kernel-details {
  flex: 1;
}

.kernel-title {
  font-size: 14px;
  font-weight: 600;
  color: var(--text-primary);
}

.kernel-version {
  font-size: 12px;
  color: var(--text-secondary);
  margin-top: 2px;
}

.download-progress {
  margin-bottom: var(--spacing-md);
}

.progress-bar {
  height: 8px;
  background: var(--bg-tertiary);
  border-radius: 4px;
  overflow: hidden;
}

.progress-fill {
  height: 100%;
  background: linear-gradient(90deg, var(--color-primary), var(--color-primary-hover));
  border-radius: 4px;
  transition: width 0.3s ease;
}

.progress-info {
  display: flex;
  justify-content: space-between;
  font-size: 12px;
  color: var(--text-secondary);
  margin-top: var(--spacing-xs);
}

.kernel-actions {
  display: flex;
  gap: var(--spacing-sm);
}

.btn-download,
.btn-cancel-download,
.btn-use-kernel {
  height: 32px;
  padding: 0 var(--spacing-md);
  font-size: 13px;
  font-weight: 500;
  border-radius: var(--radius-md);
  cursor: pointer;
  transition: all 0.2s;
  display: flex;
  align-items: center;
  gap: var(--spacing-xs);
  border: none;

  .icon {
    font-size: 14px;

    &.loading {
      animation: spin 1s linear infinite;
    }
  }
}

.btn-download {
  background: var(--color-primary);
  color: white;

  &:hover {
    background: var(--color-primary-hover);
  }
}

.btn-cancel-download {
  background: var(--bg-tertiary);
  color: var(--text-secondary);
  cursor: not-allowed;
}

.btn-use-kernel {
  background: var(--color-success);
  color: white;

  &:hover {
    filter: brightness(1.1);
  }
}

.setting-item {
  margin-bottom: var(--spacing-lg);

  &:last-child {
    margin-bottom: 0;
  }
}

.setting-label {
  display: block;
  font-size: 14px;
  font-weight: 500;
  color: var(--text-primary);
  margin-bottom: var(--spacing-sm);

  .required {
    color: var(--color-danger);
    margin-left: 2px;
  }
}

.setting-input-group {
  display: flex;
  gap: var(--spacing-sm);
}

.setting-input {
  flex: 1;
  height: 36px;
  padding: 0 var(--spacing-md);
  font-size: 13px;
  color: var(--text-primary);
  background: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-md);
  transition: all 0.2s;

  &:focus {
    outline: none;
    border-color: var(--color-primary);
    background: var(--bg-primary);
  }

  &::placeholder {
    color: var(--text-tertiary);
  }

  &[readonly] {
    cursor: pointer;
  }
}

.btn-select {
  height: 36px;
  padding: 0 var(--spacing-md);
  font-size: 13px;
  font-weight: 500;
  color: var(--text-primary);
  background: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-md);
  cursor: pointer;
  transition: all 0.2s;
  display: flex;
  align-items: center;
  gap: var(--spacing-xs);
  white-space: nowrap;

  &:hover {
    background: var(--bg-hover);
    border-color: var(--color-primary);
  }

  &:active {
    transform: translateY(1px);
  }

  .icon {
    font-size: 16px;
  }
}

.setting-hint {
  font-size: 12px;
  color: var(--text-tertiary);
  margin: var(--spacing-xs) 0 0;
}

.setting-actions {
  padding: var(--spacing-lg);
  background: var(--bg-secondary);
  display: flex;
  justify-content: flex-end;
  gap: var(--spacing-md);
}

.btn-cancel,
.btn-save {
  height: 36px;
  padding: 0 var(--spacing-lg);
  font-size: 14px;
  font-weight: 500;
  border-radius: var(--radius-md);
  cursor: pointer;
  transition: all 0.2s;
  display: flex;
  align-items: center;
  gap: var(--spacing-xs);

  &:active {
    transform: translateY(1px);
  }

  .icon {
    font-size: 16px;

    &.loading {
      animation: spin 1s linear infinite;
    }
  }
}

.btn-cancel {
  color: var(--text-secondary);
  background: transparent;
  border: 1px solid var(--border-color);

  &:hover {
    color: var(--text-primary);
    background: var(--bg-hover);
  }
}

.btn-save {
  color: white;
  background: var(--color-primary);
  border: none;

  &:hover {
    background: var(--color-primary-hover);
  }

  &:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
}

@keyframes spin {
  from {
    transform: rotate(0deg);
  }
  to {
    transform: rotate(360deg);
  }
}
</style>
