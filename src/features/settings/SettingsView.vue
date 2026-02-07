<template>
  <div class="settings-view">
    <!-- 页面头部 -->
    <header class="page-header">
      <div class="header-left">
        <div class="header-title-row">
          <h1 class="header-title">全局设置</h1>
          <span class="header-badge">专业版</span>
        </div>
        <p class="header-desc">配置应用程序全局偏好设置和安全默认项。</p>
      </div>
      <div class="header-right">
        <button class="btn-discard" @click="handleReset">丢弃</button>
        <button class="btn-save" @click="handleSave" :disabled="!isValid || isSaving">
          {{ isSaving ? '保存中...' : '保存更改' }}
        </button>
      </div>
    </header>

    <!-- 主体内容区 -->
    <div class="settings-body">
      <div class="settings-layout">
      <!-- 左侧导航栏 -->
      <div class="settings-nav">
        <button 
          v-for="section in sections" 
          :key="section.id"
          class="nav-item"
          :class="{ active: currentSection === section.id }"
          @click="currentSection = section.id"
        >
          <span class="material-symbols-outlined text-lg">{{ section.icon }}</span>
          {{ section.label }}
        </button>
      </div>

      <!-- 右侧内容区 -->
      <div class="settings-content">
        <!-- 基础设置 -->
        <div v-show="currentSection === 'basic'" class="space-y-6">
          <!-- 主题模式 -->
          <div class="settings-card">
            <h2 class="card-title">
              <span class="title-indicator"></span>
              主题模式
            </h2>
            <p class="card-desc">自定义 DeepBrowser 在您设备上的外观显示。</p>

            <div class="theme-grid">
              <label class="theme-option">
                <input type="radio" name="theme" value="light" v-model="themeMode" class="sr-only" />
                <div class="theme-card" :class="{ active: themeMode === 'light' }">
                  <div class="theme-preview light">
                    <div class="preview-header"></div>
                    <div class="preview-content">
                      <div class="preview-line w-3/4"></div>
                      <div class="preview-line w-1/2"></div>
                    </div>
                  </div>
                  <span class="theme-label">浅色模式</span>
                </div>
              </label>

              <label class="theme-option">
                <input type="radio" name="theme" value="dark" v-model="themeMode" class="sr-only" />
                <div class="theme-card" :class="{ active: themeMode === 'dark' }">
                  <div class="theme-preview dark">
                    <div class="preview-header"></div>
                    <div class="preview-content">
                      <div class="preview-line w-3/4"></div>
                      <div class="preview-line w-1/2"></div>
                    </div>
                  </div>
                  <span class="theme-label">深色模式</span>
                </div>
              </label>

              <label class="theme-option">
                <input type="radio" name="theme" value="system" v-model="themeMode" class="sr-only" />
                <div class="theme-card" :class="{ active: themeMode === 'system' }">
                  <div class="theme-preview system">
                    <div class="preview-half light"></div>
                    <div class="preview-half dark"></div>
                    <span class="material-symbols-outlined preview-icon">computer</span>
                  </div>
                  <span class="theme-label">跟随系统</span>
                </div>
              </label>
            </div>
          </div>

          <!-- 语言设置 -->
          <div class="settings-card">
            <div class="setting-row">
              <div class="setting-info">
                <h2 class="setting-title">语言设置</h2>
                <p class="setting-desc">选择您偏好的界面显示语言。</p>
              </div>
              <select v-model="language" class="form-select w-64">
                <option value="zh-CN">简体中文 (Chinese Simplified)</option>
                <option value="en-US">English (United States)</option>
                <option value="ja-JP">日本語 (Japanese)</option>
                <option value="ko-KR">한국어 (Korean)</option>
              </select>
            </div>
          </div>

          <!-- 启动行为 -->
          <div class="settings-card">
            <h2 class="card-title">
              <span class="title-indicator"></span>
              启动行为
            </h2>

            <div class="behavior-list">
              <div class="behavior-item">
                <div class="behavior-icon blue">
                  <span class="material-symbols-outlined">rocket_launch</span>
                </div>
                <div class="behavior-info">
                  <p class="behavior-title">开机自启动</p>
                  <p class="behavior-desc">在您打开电脑时自动运行 DeepBrowser。</p>
                </div>
                <label class="toggle-switch">
                  <input type="checkbox" v-model="autoStart" />
                  <span class="toggle-slider"></span>
                </label>
              </div>

              <div class="behavior-item">
                <div class="behavior-icon green">
                  <span class="material-symbols-outlined">history</span>
                </div>
                <div class="behavior-info">
                  <p class="behavior-title">恢复上次会话</p>
                  <p class="behavior-desc">启动时自动打开上次未关闭的标签页和窗口。</p>
                </div>
                <label class="toggle-switch">
                  <input type="checkbox" v-model="restoreSession" />
                  <span class="toggle-slider"></span>
                </label>
              </div>
            </div>
          </div>

          <!-- 检查更新 -->
          <div class="settings-card">
            <h2 class="card-title">
              <span class="title-indicator"></span>
              检查更新
            </h2>

            <div class="behavior-list">
              <div class="behavior-item">
                <div class="behavior-icon orange">
                  <span class="material-symbols-outlined">update</span>
                </div>
                <div class="behavior-info">
                  <p class="behavior-title">自动更新通知</p>
                  <p class="behavior-desc">当有新版本可供下载时，接收桌面提醒通知。</p>
                </div>
                <label class="toggle-switch">
                  <input type="checkbox" v-model="autoUpdate" />
                  <span class="toggle-slider"></span>
                </label>
              </div>
            </div>

            <!-- 版本信息和检查按钮 -->
            <div class="update-section">
              <div class="current-version">
                <span class="version-label">当前版本</span>
                <span class="version-value">v0.2.0</span>
              </div>
              
              <button 
                class="check-update-btn" 
                @click="handleCheckUpdate"
                :disabled="isCheckingUpdate || isDownloadingUpdate"
              >
                <span class="material-symbols-outlined" :class="{ 'spinning': isCheckingUpdate }">
                  {{ isCheckingUpdate ? 'sync' : 'refresh' }}
                </span>
                {{ isCheckingUpdate ? '检查中...' : '检查更新' }}
              </button>
            </div>

            <!-- 发现新版本提示 -->
            <div v-if="launcherUpdate?.has_update || kernelUpdate?.has_update" class="update-available">
              <div class="update-badge">
                <span class="material-symbols-outlined">celebration</span>
                发现新版本
              </div>
              
              <!-- 启动器更新 -->
              <div v-if="launcherUpdate?.has_update" class="update-item">
                <div class="update-item-header">
                  <span class="update-item-title">启动器</span>
                  <span class="update-item-version">v{{ launcherUpdate.current_version }} → v{{ launcherUpdate.version }}</span>
                </div>
                <p class="update-item-size" v-if="launcherUpdate.file_size">
                  文件大小: {{ formatFileSize(launcherUpdate.file_size) }}
                </p>
                <div class="update-item-actions">
                  <button 
                    v-for="source in launcherUpdate.downloads" 
                    :key="source.id"
                    class="source-btn"
                    :class="{ 'recommended': source.priority === 1 }"
                    @click="handleLauncherUpdate(source)"
                    :disabled="isDownloadingUpdate"
                  >
                    {{ source.name }}
                  </button>
                </div>
              </div>

              <!-- 内核更新 -->
              <div v-if="kernelUpdate?.has_update" class="update-item">
                <div class="update-item-header">
                  <span class="update-item-title">Chromium 内核</span>
                  <span class="update-item-version">v{{ kernelUpdate.current_version }} → v{{ kernelUpdate.version }}</span>
                </div>
                <p class="update-item-size" v-if="kernelUpdate.file_size">
                  文件大小: {{ formatFileSize(kernelUpdate.file_size) }}
                </p>
                <div class="update-item-actions">
                  <button 
                    v-for="source in kernelUpdate.downloads" 
                    :key="source.id"
                    class="source-btn"
                    :class="{ 'recommended': source.priority === 1 }"
                    @click="handleKernelUpdate(source)"
                    :disabled="isDownloadingUpdate"
                  >
                    {{ source.name }}
                  </button>
                </div>
              </div>
            </div>

            <!-- 下载进度 -->
            <div v-if="isDownloadingUpdate && updateDownloadProgress" class="download-progress-section">
              <div class="progress-header">
                <span class="progress-label">{{ updateDownloadProgress.message }}</span>
                <span class="progress-speed">{{ formatUpdateSpeed(updateDownloadProgress.speed) }}</span>
              </div>
              <div class="progress-bar-container">
                <div class="progress-bar" :style="{ width: updateDownloadProgress.percent + '%' }"></div>
              </div>
              <div class="progress-footer">
                <span>{{ formatFileSize(updateDownloadProgress.downloaded) }} / {{ formatFileSize(updateDownloadProgress.total) }}</span>
                <span>{{ updateDownloadProgress.percent }}%</span>
              </div>
            </div>
          </div>
        </div>

        <!-- 代理设置 -->
        <div v-show="currentSection === 'proxy'" class="space-y-6">
          <div class="settings-card">
            <h2 class="card-title">
              <span class="title-indicator"></span>
              全局代理设置
            </h2>

            <div class="space-y-4">
              <div class="form-grid">
                <div class="form-group">
                  <label class="form-label">代理协议</label>
                  <select class="form-select">
                    <option>HTTP/HTTPS</option>
                    <option>SOCKS5</option>
                    <option>SSH 隧道</option>
                  </select>
                </div>
                <div class="form-group">
                  <label class="form-label">服务器地址</label>
                  <input
                    v-model="settings.defaultProxy"
                    type="text"
                    class="form-input"
                    placeholder="例如 127.0.0.1:8080"
                  />
                </div>
              </div>
              <div class="info-box">
                <span class="material-symbols-outlined">info</span>
                <p>格式：协议://主机:端口。留空则使用系统直连方式。</p>
              </div>
            </div>
          </div>

          <div class="settings-card">
            <h2 class="card-title">
              <span class="title-indicator"></span>
              安全与隐私快速切换
            </h2>

            <div class="toggle-list">
              <div class="toggle-item">
                <div class="toggle-info">
                  <p class="toggle-title">Canvas 指纹保护</p>
                  <p class="toggle-desc">防止网站通过硬件特征识别您的身份。</p>
                </div>
                <label class="toggle-switch">
                  <input type="checkbox" checked />
                  <span class="toggle-slider"></span>
                </label>
              </div>

              <div class="toggle-item">
                <div class="toggle-info">
                  <p class="toggle-title">WebGL 元数据掩蔽</p>
                  <p class="toggle-desc">在 GPU 渲染数据中注入噪声。</p>
                </div>
                <label class="toggle-switch">
                  <input type="checkbox" checked />
                  <span class="toggle-slider"></span>
                </label>
              </div>

              <div class="toggle-item">
                <div class="toggle-info">
                  <p class="toggle-title">强制全站 HTTPS</p>
                  <p class="toggle-desc">自动将不安全的 HTTP 请求升级为加密的 HTTPS。</p>
                </div>
                <label class="toggle-switch">
                  <input type="checkbox" />
                  <span class="toggle-slider"></span>
                </label>
              </div>
            </div>
          </div>
        </div>

        <!-- 浏览器内核设置 -->
        <div v-show="currentSection === 'kernel'" class="space-y-6">
          <div class="settings-card">
            <h2 class="card-title">
              <span class="title-indicator"></span>
              浏览器内核
            </h2>
            <p class="card-desc">配置浏览器内核可执行文件路径</p>

            <!-- 内核状态警告 -->
            <div v-if="!kernelInstalled" class="warning-box">
              <span class="material-symbols-outlined">warning</span>
              <div class="warning-content">
                <p class="warning-title">内核未安装</p>
                <p class="warning-desc">请下载或选择内核文件</p>
              </div>
            </div>

            <!-- 下载内核区域 -->
            <div class="kernel-download">
              <div class="download-header">
                <span class="material-symbols-outlined">download</span>
                <span class="download-title">下载内核</span>
              </div>
              <div class="form-group">
                <label class="form-label">内核下载地址</label>
                <div class="input-with-btn">
                  <input
                    v-model="customKernelUrl"
                    type="text"
                    class="form-input"
                    placeholder="https://github.com/..."
                  />
                  <button class="icon-btn" @click="resetKernelUrl" title="重置">
                    <span class="material-symbols-outlined">refresh</span>
                  </button>
                </div>
              </div>
              <div class="info-box">
                <span class="material-symbols-outlined">info</span>
                <p>默认从 GitHub Releases 下载，如下载缓慢可使用镜像地址</p>
              </div>
              <button class="download-btn" @click="handleDownloadKernel" :disabled="isDownloading">
                <span class="material-symbols-outlined">download</span>
                {{ isDownloading ? '下载中...' : '开始下载' }}
              </button>
            </div>

            <!-- 内核路径 -->
            <div class="form-group mt-6">
              <label class="form-label required">内核路径</label>
              <div class="input-with-btn">
                <input
                  v-model="settings.kernelPath"
                  type="text"
                  class="form-input"
                  placeholder="D:\Program Files\browser-manager\resources\kernel\..."
                />
                <button class="icon-btn" @click="selectKernelPath" title="选择文件">
                  <span class="material-symbols-outlined">folder_open</span>
                </button>
              </div>
              <p v-if="embeddedKernelPath" class="form-hint success">
                <span class="material-symbols-outlined">check_circle</span>
                检测到内嵌内核：{{ embeddedKernelPath }}
              </p>
              <p class="form-hint">如未配置将自动使用内嵌内核</p>
            </div>

            <!-- 用户数据目录 -->
            <div class="form-group mt-4">
              <label class="form-label">用户数据目录</label>
              <div class="input-with-btn">
                <input
                  v-model="settings.userDataDir"
                  type="text"
                  class="form-input"
                  placeholder="D:\QutabBrowser-Cache"
                />
                <button class="icon-btn" @click="selectUserDataDir" title="选择目录">
                  <span class="material-symbols-outlined">folder_open</span>
                </button>
              </div>
              <p class="form-hint">用于存储浏览器配置文件和用户数据，留空则使用默认目录</p>
            </div>
          </div>
        </div>

        <!-- 快捷键设置 -->
        <div v-show="currentSection === 'shortcuts'" class="space-y-6">
          <div class="settings-card">
            <h2 class="card-title">
              <span class="title-indicator"></span>
              快捷键绑定
            </h2>
            <p class="card-desc">自定义全局快捷键以快速执行常用操作</p>

            <div class="shortcut-list">
              <div class="shortcut-item">
                <span class="shortcut-name">新建窗口</span>
                <kbd class="shortcut-key">Ctrl + N</kbd>
              </div>
              <div class="shortcut-item">
                <span class="shortcut-name">快速启动</span>
                <kbd class="shortcut-key">Ctrl + Shift + L</kbd>
              </div>
              <div class="shortcut-item">
                <span class="shortcut-name">老板键（隐藏所有窗口）</span>
                <kbd class="shortcut-key">Ctrl + Shift + H</kbd>
              </div>
            </div>
          </div>
        </div>

        <!-- 安全与隐私 -->
        <div v-show="currentSection === 'security'" class="space-y-6">
          <div class="settings-card">
            <h2 class="card-title">
              <span class="title-indicator"></span>
              安全与隐私
            </h2>
            <p class="card-desc">保护您的浏览数据和隐私安全</p>

            <div class="toggle-list">
              <div class="toggle-item">
                <div class="toggle-info">
                  <p class="toggle-title">自动清除浏览历史</p>
                  <p class="toggle-desc">关闭窗口时自动清除浏览记录</p>
                </div>
                <label class="toggle-switch">
                  <input type="checkbox" />
                  <span class="toggle-slider"></span>
                </label>
              </div>

              <div class="toggle-item">
                <div class="toggle-info">
                  <p class="toggle-title">禁用第三方 Cookie</p>
                  <p class="toggle-desc">阻止网站跨站追踪您的行为</p>
                </div>
                <label class="toggle-switch">
                  <input type="checkbox" checked />
                  <span class="toggle-slider"></span>
                </label>
              </div>
            </div>
          </div>
        </div>

        <!-- 云端同步 -->
        <div v-show="currentSection === 'sync'" class="space-y-6">
          <div class="settings-card">
            <h2 class="card-title">
              <span class="title-indicator"></span>
              云端同步
            </h2>

            <div class="empty-state">
              <div class="empty-icon">☁️</div>
              <p class="empty-text">云端同步功能开发中...</p>
              <button class="empty-btn">了解更多</button>
            </div>
          </div>
        </div>

        <!-- 恢复默认 -->
        <div v-show="currentSection === 'reset'" class="space-y-6">
          <div class="settings-card">
            <h2 class="card-title">
              <span class="title-indicator danger"></span>
              恢复默认设置
            </h2>

            <div class="danger-box">
              <div class="danger-header">
                <span class="material-symbols-outlined">warning</span>
                <div class="danger-content">
                  <p class="danger-title">此操作将重置所有设置</p>
                  <ul class="danger-list">
                    <li>所有全局设置将恢复为默认值</li>
                    <li>您的窗口配置和数据不会受到影响</li>
                    <li>此操作无法撤销</li>
                  </ul>
                </div>
              </div>
              <button class="danger-btn">确认恢复默认设置</button>
            </div>
          </div>
        </div>
      </div>
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
import { useAppUpdate } from './useAppUpdate'

// Navigation sections
const sections = [
  { id: 'basic', label: '基础设置', icon: 'tune' },
  { id: 'proxy', label: '全局代理设置', icon: 'router' },
  { id: 'kernel', label: '内核设置', icon: 'memory' },
  { id: 'shortcuts', label: '快捷键绑定', icon: 'keyboard' },
  { id: 'security', label: '安全与隐私', icon: 'security' },
  { id: 'sync', label: '云端同步', icon: 'cloud_sync' },
  { id: 'reset', label: '恢复默认设置', icon: 'restart_alt' }
]

const currentSection = ref('basic')

// Settings data
const settings = ref({
  kernelPath: '',
  userDataDir: '',
  defaultProxy: ''
})

// Basic settings
const themeMode = ref('system')
const language = ref('zh-CN')
const autoStart = ref(true)
const restoreSession = ref(false)
const autoUpdate = ref(true)

// App update
const {
  launcherUpdate,
  kernelUpdate,
  isChecking: isCheckingUpdate,
  isDownloading: isDownloadingUpdate,
  downloadProgress: updateDownloadProgress,
  checkAllUpdates,
  handleLauncherUpdate,
  handleKernelUpdate,
  formatFileSize,
  formatSpeed: formatUpdateSpeed,
} = useAppUpdate()

// Handle check update
const handleCheckUpdate = async () => {
  await checkAllUpdates()
}

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

// Validate settings
const isValid = computed(() => {
  return settings.value.kernelPath.trim() !== ''
})

// Embedded kernel path (alias for bundledKernelPath)
const embeddedKernelPath = computed(() => bundledKernelPath.value)

// Check kernel status
const checkKernelStatus = async () => {
  try {
    kernelInstalled.value = await kernelApi.isKernelInstalled()
    if (kernelInstalled.value) {
      kernelVersion.value = await kernelApi.getKernelVersion()
    }
    // Check bundled kernel
    bundledKernelPath.value = await kernelApi.getBundledKernelPath()
    
    // 如果用户没有配置内核路径，且检测到内嵌内核，自动使用内嵌内核
    if (!settings.value.kernelPath && bundledKernelPath.value) {
      settings.value.kernelPath = bundledKernelPath.value
    }
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
    
    // 获取默认用户数据目录
    const defaultUserDataDir = await settingsApi.getDefaultUserDataDir()
    
    settings.value = {
      kernelPath: allSettings.kernel_path || '',
      userDataDir: allSettings.user_data_dir || defaultUserDataDir || '',
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
  display: flex;
  flex-direction: column;
  height: 100%;
  max-width: 100%;
  margin: 0;
  padding: 0;
  overflow: hidden;
}

// Page header
.page-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 1.5rem 2rem;
  background: white;
  border-bottom: 1px solid #e2e8f0;
}

.header-left {
  display: flex;
  flex-direction: column;
  gap: 0.25rem;
}

.header-title-row {
  display: flex;
  align-items: center;
  gap: 0.75rem;
}

.header-title {
  font-size: 1.5rem;
  font-weight: 700;
  color: var(--text-primary);
  margin: 0;
}

.header-badge {
  padding: 0.125rem 0.5rem;
  font-size: 0.625rem;
  font-weight: 600;
  color: var(--color-primary);
  background: rgba(59, 130, 246, 0.1);
  border-radius: 0.25rem;
  text-transform: uppercase;
  letter-spacing: 0.05em;
}

.header-desc {
  font-size: 0.875rem;
  color: #64748b;
  margin: 0;
}

.header-right {
  display: flex;
  align-items: center;
  gap: 0.75rem;
}

.btn-discard {
  padding: 0.5rem 1rem;
  font-size: 0.875rem;
  font-weight: 500;
  color: var(--text-primary);
  background: white;
  border: 1px solid #e2e8f0;
  border-radius: 0.5rem;
  cursor: pointer;
  transition: all 0.2s;

  &:hover {
    background: #f8fafc;
  }
}

.btn-save {
  padding: 0.5rem 1rem;
  font-size: 0.875rem;
  font-weight: 500;
  color: white;
  background: var(--color-primary);
  border: none;
  border-radius: 0.5rem;
  cursor: pointer;
  transition: all 0.2s;
  box-shadow: 0 4px 6px -1px rgba(59, 130, 246, 0.2);

  &:hover {
    background: #2563eb;
  }

  &:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
}

// Settings body
.settings-body {
  padding: 2rem;
  background: #f9fafb;
  flex: 1;
  overflow-y: auto;
}

// Settings layout
.settings-layout {
  display: flex;
  gap: 2rem;
}

.settings-nav {
  width: 220px;
  flex-shrink: 0;
  display: flex;
  flex-direction: column;
  gap: 0.25rem;
}

.settings-content {
  flex: 1;
  max-width: 900px;
  overflow-y: auto;
  padding-bottom: 2rem;
}

// Settings card
.settings-card {
  background: white;
  border: 1px solid #e2e8f0;
  border-radius: 1rem;
  padding: 1.5rem;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.05);
}

.card-title {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  font-size: 1.125rem;
  font-weight: 700;
  color: #1e293b;
  margin: 0 0 1.5rem 0;
}

.title-indicator {
  width: 4px;
  height: 1.5rem;
  background: var(--color-primary);
  border-radius: 2px;

  &.danger {
    background: #ef4444;
  }
}

.card-desc {
  font-size: 0.875rem;
  color: #64748b;
  margin: -1rem 0 1.5rem 0;
}

// Form elements
.form-grid {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 1rem;
}

.form-group {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.form-label {
  font-size: 0.875rem;
  font-weight: 600;
  color: #475569;
}

.form-input,
.form-select {
  width: 100%;
  padding: 0.625rem 0.875rem;
  font-size: 0.875rem;
  color: #1e293b;
  background: white;
  border: 1px solid #e2e8f0;
  border-radius: 0.5rem;
  outline: none;
  transition: border-color 0.2s, box-shadow 0.2s;

  &:focus {
    border-color: var(--color-primary);
    box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
  }

  &::placeholder {
    color: #94a3b8;
  }
}

// Info box
.info-box {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  padding: 1rem;
  background: #eff6ff;
  border: 1px solid #dbeafe;
  border-radius: 0.5rem;

  .material-symbols-outlined {
    color: var(--color-primary);
    font-size: 1.25rem;
  }

  p {
    font-size: 0.75rem;
    color: #475569;
    margin: 0;
  }
}

// Warning box
.warning-box {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  padding: 1rem;
  background: #fef3c7;
  border: 1px solid #fcd34d;
  border-radius: 0.5rem;
  margin-bottom: 1.5rem;

  > .material-symbols-outlined {
    color: #d97706;
    font-size: 1.5rem;
  }
}

.warning-content {
  flex: 1;
}

.warning-title {
  font-size: 0.9375rem;
  font-weight: 600;
  color: #92400e;
  margin: 0 0 0.25rem 0;
}

.warning-desc {
  font-size: 0.75rem;
  color: #a16207;
  margin: 0;
}

// Kernel download section
.kernel-download {
  padding: 1.5rem;
  background: #f8fafc;
  border: 1px solid #e2e8f0;
  border-radius: 0.75rem;
  margin-bottom: 1.5rem;
}

.download-header {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  margin-bottom: 1rem;

  .material-symbols-outlined {
    color: var(--color-primary);
    font-size: 1.25rem;
  }
}

.download-title {
  font-size: 0.9375rem;
  font-weight: 600;
  color: #1e293b;
}

.download-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 0.5rem;
  width: 100%;
  padding: 0.75rem 1rem;
  margin-top: 1rem;
  font-size: 0.875rem;
  font-weight: 500;
  color: white;
  background: var(--color-primary);
  border: none;
  border-radius: 0.5rem;
  cursor: pointer;
  transition: background 0.2s;

  &:hover:not(:disabled) {
    background: #2563eb;
  }

  &:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }

  .material-symbols-outlined {
    font-size: 1.125rem;
  }
}

// Input with button
.input-with-btn {
  display: flex;
  gap: 0.5rem;

  .form-input {
    flex: 1;
  }
}

.icon-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 40px;
  height: 40px;
  background: white;
  border: 1px solid #e2e8f0;
  border-radius: 0.5rem;
  cursor: pointer;
  transition: all 0.2s;

  .material-symbols-outlined {
    font-size: 1.25rem;
    color: #64748b;
  }

  &:hover {
    background: #f8fafc;
    border-color: var(--color-primary);

    .material-symbols-outlined {
      color: var(--color-primary);
    }
  }
}

// Form hint
.form-hint {
  display: flex;
  align-items: center;
  gap: 0.375rem;
  font-size: 0.75rem;
  color: #64748b;
  margin: 0.5rem 0 0 0;

  .material-symbols-outlined {
    font-size: 1rem;
  }

  &.success {
    color: #16a34a;
  }
}

.form-label.required::after {
  content: ' *';
  color: #ef4444;
}

.mt-4 { margin-top: 1rem; }
.mt-6 { margin-top: 1.5rem; }
.w-64 { width: 16rem; }
.w-3\/4 { width: 75%; }
.w-1\/2 { width: 50%; }
.sr-only {
  position: absolute;
  width: 1px;
  height: 1px;
  padding: 0;
  margin: -1px;
  overflow: hidden;
  clip: rect(0, 0, 0, 0);
  border: 0;
}

// Theme grid
.theme-grid {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 1rem;
}

.theme-option {
  cursor: pointer;
}

.theme-card {
  padding: 1rem;
  border: 2px solid #e2e8f0;
  border-radius: 0.75rem;
  transition: all 0.2s;

  &.active {
    border-color: var(--color-primary);
    background: rgba(59, 130, 246, 0.05);
  }

  &:hover {
    border-color: #cbd5e1;
  }
}

.theme-preview {
  height: 6rem;
  border-radius: 0.375rem;
  margin-bottom: 0.75rem;
  border: 1px solid #e2e8f0;
  overflow: hidden;
  display: flex;
  flex-direction: column;

  &.light {
    background: white;

    .preview-header {
      height: 1rem;
      background: #f1f5f9;
      border-bottom: 1px solid #e2e8f0;
    }

    .preview-line {
      height: 0.5rem;
      background: #e2e8f0;
      border-radius: 0.25rem;
    }
  }

  &.dark {
    background: #1e293b;
    border-color: #334155;

    .preview-header {
      height: 1rem;
      background: #334155;
      border-bottom: 1px solid #334155;
    }

    .preview-line {
      height: 0.5rem;
      background: #334155;
      border-radius: 0.25rem;
    }
  }

  &.system {
    position: relative;
    flex-direction: row;

    .preview-half {
      width: 50%;

      &.light {
        background: white;
        border-right: 1px solid #e2e8f0;
      }

      &.dark {
        background: #1e293b;
      }
    }

    .preview-icon {
      position: absolute;
      top: 50%;
      left: 50%;
      transform: translate(-50%, -50%);
      color: #94a3b8;
      font-size: 1.5rem;
    }
  }
}

.preview-content {
  padding: 0.5rem;
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.theme-label {
  font-size: 0.875rem;
  font-weight: 500;
  color: #1e293b;
}

// Setting row
.setting-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 1rem;
}

.setting-info {
  flex: 1;
}

.setting-title {
  font-size: 1.125rem;
  font-weight: 600;
  color: #1e293b;
  margin: 0 0 0.25rem 0;
}

.setting-desc {
  font-size: 0.875rem;
  color: #64748b;
  margin: 0;
}

// Behavior list
.behavior-list {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.behavior-item {
  display: flex;
  align-items: center;
  padding: 1rem;
  background: white;
  border: 1px solid #e2e8f0;
  border-radius: 0.75rem;
}

.behavior-icon {
  width: 2.5rem;
  height: 2.5rem;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  margin-right: 1rem;
  flex-shrink: 0;

  .material-symbols-outlined {
    font-size: 1.25rem;
  }

  &.blue {
    background: rgba(59, 130, 246, 0.1);
    color: var(--color-primary);
  }

  &.green {
    background: rgba(34, 197, 94, 0.1);
    color: #16a34a;
  }

  &.orange {
    background: rgba(249, 115, 22, 0.1);
    color: #ea580c;
  }
}

.behavior-info {
  flex: 1;
}

.behavior-title {
  font-size: 0.9375rem;
  font-weight: 500;
  color: #1e293b;
  margin: 0 0 0.25rem 0;
}

.behavior-desc {
  font-size: 0.75rem;
  color: #64748b;
  margin: 0;
}

// Toggle list
.toggle-list {
  display: flex;
  flex-direction: column;
}

.toggle-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 1rem 0;
  border-bottom: 1px solid #f1f5f9;

  &:last-child {
    border-bottom: none;
  }
}

.toggle-info {
  flex: 1;
}

.toggle-title {
  font-size: 0.9375rem;
  font-weight: 500;
  color: #1e293b;
  margin: 0 0 0.25rem 0;
}

.toggle-desc {
  font-size: 0.75rem;
  color: #64748b;
  margin: 0;
}

// Toggle switch
.toggle-switch {
  position: relative;
  display: inline-flex;
  align-items: center;
  cursor: pointer;

  input {
    position: absolute;
    opacity: 0;
    width: 0;
    height: 0;
  }

  .toggle-slider {
    width: 44px;
    height: 24px;
    background: #cbd5e1;
    border-radius: 24px;
    position: relative;
    transition: background 0.2s;

    &::after {
      content: '';
      position: absolute;
      top: 2px;
      left: 2px;
      width: 20px;
      height: 20px;
      background: white;
      border-radius: 50%;
      box-shadow: 0 1px 3px rgba(0, 0, 0, 0.2);
      transition: transform 0.2s;
    }
  }

  input:checked + .toggle-slider {
    background: var(--color-primary);

    &::after {
      transform: translateX(20px);
    }
  }
}

// Shortcut list
.shortcut-list {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
}

.shortcut-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0.75rem 1rem;
  background: #f8fafc;
  border-radius: 0.5rem;
}

.shortcut-name {
  font-size: 0.875rem;
  font-weight: 500;
  color: #1e293b;
}

.shortcut-key {
  padding: 0.375rem 0.75rem;
  font-size: 0.75rem;
  font-family: ui-monospace, monospace;
  color: #475569;
  background: white;
  border: 1px solid #e2e8f0;
  border-radius: 0.375rem;
}

// Empty state
.empty-state {
  text-align: center;
  padding: 3rem 0;
}

.empty-icon {
  font-size: 4rem;
  margin-bottom: 1rem;
  opacity: 0.5;
}

.empty-text {
  font-size: 0.875rem;
  color: #64748b;
  margin-bottom: 1rem;
}

.empty-btn {
  padding: 0.5rem 1rem;
  font-size: 0.875rem;
  color: #475569;
  background: transparent;
  border: 1px solid #e2e8f0;
  border-radius: 0.5rem;
  cursor: pointer;
  transition: all 0.2s;

  &:hover {
    background: #f8fafc;
  }
}

// Danger box
.danger-box {
  padding: 1.5rem;
  background: #fef2f2;
  border: 1px solid #fecaca;
  border-radius: 0.5rem;
}

.danger-header {
  display: flex;
  align-items: flex-start;
  gap: 0.75rem;
  margin-bottom: 1rem;

  .material-symbols-outlined {
    font-size: 1.5rem;
    color: #ef4444;
  }
}

.danger-content {
  flex: 1;
}

.danger-title {
  font-size: 0.9375rem;
  font-weight: 600;
  color: #991b1b;
  margin: 0 0 0.5rem 0;
}

.danger-list {
  margin: 0;
  padding-left: 1.25rem;
  font-size: 0.875rem;
  color: #b91c1c;

  li {
    margin-bottom: 0.25rem;
  }
}

.danger-btn {
  width: 100%;
  padding: 0.75rem 1rem;
  font-size: 0.875rem;
  font-weight: 500;
  color: white;
  background: #dc2626;
  border: none;
  border-radius: 0.5rem;
  cursor: pointer;
  transition: background 0.2s;

  &:hover {
    background: #b91c1c;
  }
}

// Navigation items
.nav-item {
  width: 100%;
  display: flex;
  align-items: center;
  gap: 0.75rem;
  padding: 0.75rem 1rem;
  font-size: 0.875rem;
  font-weight: 500;
  color: #64748b;
  background: transparent;
  border: none;
  border-radius: 0.5rem;
  cursor: pointer;
  transition: all 0.2s;
  text-align: left;

  &:hover {
    color: #334155;
  }

  &.active {
    color: var(--color-primary);
    font-weight: 600;
    background: white;
    border: 1px solid #e2e8f0;
    border-right: 3px solid var(--color-primary);
    box-shadow: 0 1px 2px rgba(0, 0, 0, 0.05);
  }

  .material-symbols-outlined {
    font-size: 1.25rem;
  }
}

// Utility classes
.flex { display: flex; }
.items-center { align-items: center; }
.items-start { align-items: flex-start; }
.justify-between { justify-content: space-between; }
.gap-1 { gap: 0.25rem; }
.gap-2 { gap: 0.5rem; }
.gap-3 { gap: 0.75rem; }
.gap-4 { gap: 1rem; }
.mb-1 { margin-bottom: 0.25rem; }
.mb-2 { margin-bottom: 0.5rem; }
.mb-4 { margin-bottom: 1rem; }
.mb-6 { margin-bottom: 1.5rem; }
.mb-8 { margin-bottom: 2rem; }
.mt-2 { margin-top: 0.5rem; }
.text-xs { font-size: 0.75rem; line-height: 1rem; }
.text-sm { font-size: 0.875rem; line-height: 1.25rem; }
.text-lg { font-size: 1.125rem; line-height: 1.75rem; }
.text-2xl { font-size: 1.5rem; line-height: 2rem; }
.text-6xl { font-size: 3.75rem; line-height: 1; }
.font-medium { font-weight: 500; }
.font-semibold { font-weight: 600; }
.font-bold { font-weight: 700; }
.px-2 { padding-left: 0.5rem; padding-right: 0.5rem; }
.px-3 { padding-left: 0.75rem; padding-right: 0.75rem; }
.px-4 { padding-left: 1rem; padding-right: 1rem; }
.px-6 { padding-left: 1.5rem; padding-right: 1.5rem; }
.py-1 { padding-top: 0.25rem; padding-bottom: 0.25rem; }
.py-2 { padding-top: 0.5rem; padding-bottom: 0.5rem; }
.py-3 { padding-top: 0.75rem; padding-bottom: 0.75rem; }
.py-12 { padding-top: 3rem; padding-bottom: 3rem; }
.rounded { border-radius: 0.25rem; }
.rounded-lg { border-radius: 0.5rem; }
.rounded-2xl { border-radius: 1rem; }
.rounded-full { border-radius: 9999px; }
.uppercase { text-transform: uppercase; }
.tracking-wider { letter-spacing: 0.05em; }
.border { border-width: 1px; }
.border-r-4 { border-right-width: 4px; }
.border-r-primary { border-right-color: var(--color-primary); }
.w-full { width: 100%; }
.flex-1 { flex: 1 1 0%; }
.col-span-3 { grid-column: span 3 / span 3; }
.col-span-9 { grid-column: span 9 / span 9; }
.grid { display: grid; }
.grid-cols-2 { grid-template-columns: repeat(2, minmax(0, 1fr)); }
.grid-cols-12 { grid-template-columns: repeat(12, minmax(0, 1fr)); }
.gap-4 { gap: 1rem; }
.gap-8 { gap: 2rem; }
.space-y-1 > * + * { margin-top: 0.25rem; }
.space-y-2 > * + * { margin-top: 0.5rem; }
.space-y-3 > * + * { margin-top: 0.75rem; }
.space-y-4 > * + * { margin-top: 1rem; }
.space-y-6 > * + * { margin-top: 1.5rem; }
.text-left { text-align: left; }
.text-center { text-align: center; }
.w-1\.5 { width: 0.375rem; }
.w-11 { width: 2.75rem; }
.w-12 { width: 3rem; }
.h-2 { height: 0.5rem; }
.h-6 { height: 1.5rem; }
.h-12 { height: 3rem; }
.overflow-hidden { overflow: hidden; }
.font-mono { font-family: ui-monospace, monospace; }
.opacity-50 { opacity: 0.5; }

// Color utilities
.text-primary { color: var(--color-primary); }
.bg-primary { background-color: var(--color-primary); }
.bg-primary\/10 { background-color: rgba(59, 130, 246, 0.1); }
.text-slate-500 { color: #64748b; }
.text-slate-600 { color: #475569; }
.text-slate-700 { color: #334155; }
.text-slate-900 { color: #0f172a; }
.border-slate-100 { border-color: #f1f5f9; }
.border-slate-200 { border-color: #e2e8f0; }
.border-slate-700 { border-color: #334155; }
.bg-white { background-color: white; }
.bg-slate-50 { background-color: #f8fafc; }
.bg-blue-50 { background-color: #eff6ff; }
.border-blue-100 { border-color: #dbeafe; }
.text-green-600 { color: #16a34a; }
.bg-green-100 { background-color: #dcfce7; }
.bg-green-600 { background-color: #16a34a; }
.text-yellow-600 { color: #ca8a04; }
.bg-yellow-100 { background-color: #fef3c7; }
.bg-red-50 { background-color: #fef2f2; }
.border-red-100 { border-color: #fee2e2; }
.text-red-500 { color: #ef4444; }
.bg-red-500 { background-color: #ef4444; }
.bg-red-600 { background-color: #dc2626; }
.text-red-700 { color: #b91c1c; }
.text-red-900 { color: #7f1d1d; }

// Transitions
.transition-colors { transition-property: color, background-color, border-color; transition-duration: 0.2s; transition-timing-function: ease; }
.transition-all { transition-property: all; transition-duration: 0.2s; transition-timing-function: ease; }

// Hover effects
.hover\:bg-slate-50:hover { background-color: #f8fafc; }
.hover\:bg-slate-800:hover { background-color: #1e293b; }
.hover\:bg-blue-600:hover { background-color: #2563eb; }
.hover\:bg-green-700:hover { background-color: #15803d; }
.hover\:bg-red-700:hover { background-color: #b91c1c; }

// Shadow
.shadow-sm { box-shadow: 0 1px 2px 0 rgba(0, 0, 0, 0.05); }
.shadow-lg { box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.1); }
.shadow-blue-500\/20 { box-shadow: 0 10px 15px -3px rgba(59, 130, 246, 0.2); }

// Update section styles
.update-section {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-top: 1.5rem;
  padding-top: 1.5rem;
  border-top: 1px solid #e2e8f0;
}

.current-version {
  display: flex;
  flex-direction: column;
  gap: 0.25rem;
}

.version-label {
  font-size: 0.75rem;
  color: #64748b;
}

.version-value {
  font-size: 1.125rem;
  font-weight: 600;
  color: #1e293b;
}

.check-update-btn {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.625rem 1.25rem;
  font-size: 0.875rem;
  font-weight: 500;
  color: var(--color-primary);
  background: rgba(59, 130, 246, 0.1);
  border: 1px solid rgba(59, 130, 246, 0.2);
  border-radius: 0.5rem;
  cursor: pointer;
  transition: all 0.2s;

  &:hover:not(:disabled) {
    background: rgba(59, 130, 246, 0.15);
  }

  &:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }

  .material-symbols-outlined {
    font-size: 1.125rem;
  }

  .spinning {
    animation: spin 1s linear infinite;
  }
}

@keyframes spin {
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
}

.update-available {
  margin-top: 1.5rem;
  padding: 1.25rem;
  background: linear-gradient(135deg, rgba(34, 197, 94, 0.1) 0%, rgba(59, 130, 246, 0.1) 100%);
  border: 1px solid rgba(34, 197, 94, 0.3);
  border-radius: 0.75rem;
}

.update-badge {
  display: inline-flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.375rem 0.75rem;
  font-size: 0.75rem;
  font-weight: 600;
  color: #16a34a;
  background: rgba(34, 197, 94, 0.15);
  border-radius: 1rem;
  margin-bottom: 1rem;

  .material-symbols-outlined {
    font-size: 1rem;
  }
}

.update-item {
  padding: 1rem;
  background: white;
  border: 1px solid #e2e8f0;
  border-radius: 0.5rem;
  margin-bottom: 0.75rem;

  &:last-child {
    margin-bottom: 0;
  }
}

.update-item-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 0.5rem;
}

.update-item-title {
  font-size: 0.9375rem;
  font-weight: 600;
  color: #1e293b;
}

.update-item-version {
  font-size: 0.75rem;
  color: #16a34a;
  font-weight: 500;
}

.update-item-size {
  font-size: 0.75rem;
  color: #64748b;
  margin: 0 0 0.75rem 0;
}

.update-item-actions {
  display: flex;
  gap: 0.5rem;
  flex-wrap: wrap;
}

.source-btn {
  padding: 0.375rem 0.75rem;
  font-size: 0.75rem;
  font-weight: 500;
  color: #475569;
  background: #f1f5f9;
  border: 1px solid #e2e8f0;
  border-radius: 0.375rem;
  cursor: pointer;
  transition: all 0.2s;

  &:hover:not(:disabled) {
    background: #e2e8f0;
  }

  &.recommended {
    color: white;
    background: var(--color-primary);
    border-color: var(--color-primary);

    &:hover:not(:disabled) {
      background: #2563eb;
    }
  }

  &:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
}

.download-progress-section {
  margin-top: 1.5rem;
  padding: 1rem;
  background: #f8fafc;
  border: 1px solid #e2e8f0;
  border-radius: 0.5rem;
}

.progress-header {
  display: flex;
  justify-content: space-between;
  margin-bottom: 0.5rem;
}

.progress-label {
  font-size: 0.875rem;
  font-weight: 500;
  color: #1e293b;
}

.progress-speed {
  font-size: 0.75rem;
  color: #64748b;
}

.progress-bar-container {
  height: 8px;
  background: #e2e8f0;
  border-radius: 4px;
  overflow: hidden;
}

.progress-bar {
  height: 100%;
  background: linear-gradient(90deg, var(--color-primary) 0%, #60a5fa 100%);
  border-radius: 4px;
  transition: width 0.3s ease;
}

.progress-footer {
  display: flex;
  justify-content: space-between;
  margin-top: 0.5rem;
  font-size: 0.75rem;
  color: #64748b;
}

// Dark mode
html.dark {
  .page-header {
    background: #1f2937;
    border-color: #374151;
  }
  
  .header-desc {
    color: #9ca3af;
  }
  
  .btn-discard {
    background: #374151;
    border-color: #4b5563;
    color: #e5e7eb;
    
    &:hover {
      background: #4b5563;
    }
  }
  
  .settings-body {
    background: #111827;
  }
  
  .nav-item {
    color: #9ca3af;
    
    &:hover {
      color: #e5e7eb;
    }
    
    &.active {
      background: #374151;
      border-color: #4b5563;
      color: var(--color-primary);
    }
  }

  .settings-card {
    background: #1f2937;
    border-color: #374151;
  }

  .card-title {
    color: #f1f5f9;
  }

  .form-label {
    color: #94a3b8;
  }

  .form-input,
  .form-select {
    background: #111827;
    border-color: #374151;
    color: #f1f5f9;
  }

  .info-box {
    background: rgba(59, 130, 246, 0.1);
    border-color: rgba(59, 130, 246, 0.2);

    p {
      color: #94a3b8;
    }
  }

  .toggle-item {
    border-color: #374151;
  }

  .toggle-title {
    color: #f1f5f9;
  }

  .toggle-desc {
    color: #9ca3af;
  }

  .toggle-slider {
    background: #4b5563;
  }

  .card-desc {
    color: #9ca3af;
  }

  .shortcut-item {
    background: #1f2937;
  }

  .shortcut-name {
    color: #f1f5f9;
  }

  .shortcut-key {
    background: #111827;
    border-color: #374151;
    color: #94a3b8;
  }

  .empty-text {
    color: #9ca3af;
  }

  .empty-btn {
    border-color: #374151;
    color: #94a3b8;

    &:hover {
      background: #1f2937;
    }
  }

  .danger-box {
    background: rgba(239, 68, 68, 0.1);
    border-color: rgba(239, 68, 68, 0.3);
  }

  .danger-title {
    color: #fca5a5;
  }

  .danger-list {
    color: #f87171;
  }

  .theme-card {
    border-color: #374151;

    &.active {
      background: rgba(59, 130, 246, 0.1);
    }

    &:hover {
      border-color: #4b5563;
    }
  }

  .theme-label {
    color: #f1f5f9;
  }

  .setting-title {
    color: #f1f5f9;
  }

  .setting-desc {
    color: #9ca3af;
  }

  .behavior-item {
    background: #1f2937;
    border-color: #374151;
  }

  .behavior-title {
    color: #f1f5f9;
  }

  .behavior-desc {
    color: #9ca3af;
  }

  .warning-box {
    background: rgba(251, 191, 36, 0.1);
    border-color: rgba(251, 191, 36, 0.3);
  }

  .warning-title {
    color: #fbbf24;
  }

  .warning-desc {
    color: #fcd34d;
  }

  .kernel-download {
    background: #1f2937;
    border-color: #374151;
  }

  .download-title {
    color: #f1f5f9;
  }

  .icon-btn {
    background: #1f2937;
    border-color: #374151;

    .material-symbols-outlined {
      color: #94a3b8;
    }

    &:hover {
      background: #374151;
    }
  }

  .form-hint {
    color: #9ca3af;

    &.success {
      color: #4ade80;
    }
  }

  .bg-white { background-color: #374151; }
  .text-slate-500 { color: #9ca3af; }
  .text-slate-600 { color: #9ca3af; }
  .text-slate-700 { color: #d1d5db; }
  .border-slate-100 { border-color: #4b5563; }
  .border-slate-200 { border-color: #4b5563; }
  .border-slate-700 { border-color: #4b5563; }
  .bg-slate-50 { background-color: rgba(51, 65, 85, 0.5); }
  .bg-blue-50 { background-color: rgba(30, 58, 138, 0.1); }
  .border-blue-100 { border-color: rgba(30, 64, 175, 1); }
  .text-slate-400 { color: #9ca3af; }
  .bg-slate-900 { background-color: #0f172a; }
  .hover\:bg-slate-50:hover { background-color: #1e293b; }
  .bg-red-50 { background-color: rgba(127, 29, 29, 0.1); }
  .border-red-100 { border-color: rgba(153, 27, 27, 1); }
  .text-red-200 { color: #fecaca; }
  .text-red-300 { color: #fca5a5; }
  .text-red-700 { color: #fca5a5; }

  // Update section dark mode
  .update-section {
    border-color: #374151;
  }

  .version-label {
    color: #9ca3af;
  }

  .version-value {
    color: #f1f5f9;
  }

  .check-update-btn {
    background: rgba(59, 130, 246, 0.15);
    border-color: rgba(59, 130, 246, 0.3);

    &:hover:not(:disabled) {
      background: rgba(59, 130, 246, 0.25);
    }
  }

  .update-available {
    background: linear-gradient(135deg, rgba(34, 197, 94, 0.15) 0%, rgba(59, 130, 246, 0.15) 100%);
    border-color: rgba(34, 197, 94, 0.4);
  }

  .update-badge {
    background: rgba(34, 197, 94, 0.2);
    color: #4ade80;
  }

  .update-item {
    background: #1f2937;
    border-color: #374151;
  }

  .update-item-title {
    color: #f1f5f9;
  }

  .update-item-version {
    color: #4ade80;
  }

  .update-item-size {
    color: #9ca3af;
  }

  .source-btn {
    background: #374151;
    border-color: #4b5563;
    color: #d1d5db;

    &:hover:not(:disabled) {
      background: #4b5563;
    }
  }

  .download-progress-section {
    background: #1f2937;
    border-color: #374151;
  }

  .progress-label {
    color: #f1f5f9;
  }

  .progress-speed {
    color: #9ca3af;
  }

  .progress-bar-container {
    background: #374151;
  }

  .progress-footer {
    color: #9ca3af;
  }
}
</style>
