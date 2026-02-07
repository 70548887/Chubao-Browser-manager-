<template>
  <div v-if="visible" class="kernel-extraction-overlay">
    <div class="extraction-dialog">
      <div class="extraction-header">
        <div class="icon-wrapper">
          <svg v-if="status === 'extracting'" class="animate-spin" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor">
            <circle cx="12" cy="12" r="10" stroke-width="3" stroke-dasharray="60" stroke-dashoffset="15"/>
          </svg>
          <svg v-else-if="status === 'success'" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor">
            <path d="M20 6L9 17l-5-5" stroke-width="3" stroke-linecap="round" stroke-linejoin="round"/>
          </svg>
          <svg v-else-if="status === 'error'" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor">
            <circle cx="12" cy="12" r="10" stroke-width="3"/>
            <path d="M15 9l-6 6m0-6l6 6" stroke-width="3" stroke-linecap="round"/>
          </svg>
        </div>
        <h3>{{ title }}</h3>
      </div>

      <div class="extraction-body">
        <p class="status-message">{{ message }}</p>
        
        <!-- 进度条 -->
        <div v-if="showProgress" class="progress-container">
          <div class="progress-bar">
            <div 
              class="progress-fill" 
              :style="{ width: `${progressPercent}%` }"
              :class="{ 'progress-indeterminate': !hasTotal }"
            ></div>
          </div>
          <div class="progress-info">
            <span v-if="hasTotal">{{ progressPercent }}%</span>
            <span v-if="hasTotal && downloaded > 0">{{ formatSize(downloaded) }} / {{ formatSize(total!) }}</span>
          </div>
        </div>

        <!-- 速度和预计时间 -->
        <div v-if="speed > 0" class="stats">
          <span>速度: {{ formatSpeed(speed) }}</span>
        </div>
      </div>

      <div v-if="status === 'success' || status === 'error'" class="extraction-footer">
        <button @click="close" class="btn-close">确定</button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue'
import { listen, UnlistenFn } from '@tauri-apps/api/event'

interface DownloadProgress {
  downloaded: number
  total?: number
  speed: number
  status: 'Idle' | 'Downloading' | 'Extracting' | 'Completed' | 'Failed'
  message: string
}

const visible = ref(false)
const status = ref<'extracting' | 'success' | 'error'>('extracting')
const message = ref('')
const downloaded = ref(0)
const total = ref<number | null>(null)
const speed = ref(0)

let unlistenProgress: UnlistenFn | null = null
let unlistenComplete: UnlistenFn | null = null
let unlistenError: UnlistenFn | null = null

const title = computed(() => {
  switch (status.value) {
    case 'extracting':
      return '正在初始化内核'
    case 'success':
      return '初始化完成'
    case 'error':
      return '初始化失败'
    default:
      return '内核初始化'
  }
})

const showProgress = computed(() => status.value === 'extracting')

const hasTotal = computed(() => total.value !== null && total.value > 0)

const progressPercent = computed(() => {
  if (!hasTotal.value) return 0
  return Math.min(100, Math.round((downloaded.value / total.value!) * 100))
})

const formatSize = (bytes: number): string => {
  if (bytes === 0) return '0 B'
  const k = 1024
  const sizes = ['B', 'KB', 'MB', 'GB']
  const i = Math.floor(Math.log(bytes) / Math.log(k))
  return Math.round(bytes / Math.pow(k, i) * 100) / 100 + ' ' + sizes[i]
}

const formatSpeed = (bytesPerSec: number): string => {
  return formatSize(bytesPerSec) + '/s'
}

const close = () => {
  visible.value = false
  status.value = 'extracting'
  downloaded.value = 0
  total.value = null
  speed.value = 0
  message.value = ''
}

onMounted(async () => {
  // 监听进度事件
  unlistenProgress = await listen<DownloadProgress>('kernel-extraction-progress', (event) => {
    visible.value = true
    status.value = 'extracting'
    
    const progress = event.payload
    downloaded.value = progress.downloaded
    total.value = progress.total ?? null
    speed.value = progress.speed
    message.value = progress.message || '正在处理...'
  })

  // 监听完成事件
  unlistenComplete = await listen<boolean>('kernel-extraction-complete', (event) => {
    if (event.payload) {
      status.value = 'success'
      message.value = '内核初始化完成,应用已就绪!'
    } else {
      // 内核已存在,不显示对话框
      visible.value = false
    }
  })

  // 监听错误事件
  unlistenError = await listen<string>('kernel-extraction-error', (event) => {
    status.value = 'error'
    message.value = `初始化失败: ${event.payload}`
    visible.value = true
  })
})

onUnmounted(() => {
  unlistenProgress?.()
  unlistenComplete?.()
  unlistenError?.()
})
</script>

<style scoped>
.kernel-extraction-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  backdrop-filter: blur(4px);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 9999;
  animation: fadeIn 0.2s ease;
}

.extraction-dialog {
  background: var(--color-bg-elevated);
  border-radius: 12px;
  box-shadow: 0 20px 60px rgba(0, 0, 0, 0.3);
  min-width: 400px;
  max-width: 500px;
  overflow: hidden;
  animation: slideUp 0.3s ease;
}

.extraction-header {
  padding: 24px 24px 16px;
  display: flex;
  align-items: center;
  gap: 16px;
  border-bottom: 1px solid var(--color-border);
}

.icon-wrapper {
  width: 48px;
  height: 48px;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  background: var(--color-primary-bg);
  color: var(--color-primary);
}

.icon-wrapper svg {
  stroke: currentColor;
}

.extraction-header h3 {
  font-size: 18px;
  font-weight: 600;
  color: var(--color-text-primary);
  margin: 0;
}

.extraction-body {
  padding: 24px;
}

.status-message {
  color: var(--color-text-secondary);
  font-size: 14px;
  margin: 0 0 20px;
  line-height: 1.6;
}

.progress-container {
  margin: 20px 0;
}

.progress-bar {
  height: 6px;
  background: var(--color-bg-tertiary);
  border-radius: 3px;
  overflow: hidden;
  position: relative;
}

.progress-fill {
  height: 100%;
  background: linear-gradient(90deg, var(--color-primary), var(--color-primary-light));
  border-radius: 3px;
  transition: width 0.3s ease;
}

.progress-indeterminate {
  animation: indeterminate 1.5s infinite;
  width: 40% !important;
}

.progress-info {
  display: flex;
  justify-content: space-between;
  margin-top: 8px;
  font-size: 13px;
  color: var(--color-text-tertiary);
}

.stats {
  margin-top: 12px;
  font-size: 13px;
  color: var(--color-text-tertiary);
  display: flex;
  gap: 16px;
}

.extraction-footer {
  padding: 16px 24px;
  border-top: 1px solid var(--color-border);
  display: flex;
  justify-content: flex-end;
}

.btn-close {
  padding: 8px 24px;
  background: var(--color-primary);
  color: white;
  border: none;
  border-radius: 6px;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
}

.btn-close:hover {
  background: var(--color-primary-hover);
  transform: translateY(-1px);
}

.animate-spin {
  animation: spin 1s linear infinite;
}

@keyframes fadeIn {
  from {
    opacity: 0;
  }
  to {
    opacity: 1;
  }
}

@keyframes slideUp {
  from {
    opacity: 0;
    transform: translateY(20px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
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

@keyframes indeterminate {
  0% {
    transform: translateX(-100%);
  }
  100% {
    transform: translateX(350%);
  }
}

/* 深色模式适配 */
@media (prefers-color-scheme: dark) {
  .extraction-dialog {
    background: var(--color-bg-elevated, #1e1e1e);
  }
}
</style>
