<script setup lang="ts">
import { computed } from 'vue'
import { useUIStore } from '@/stores/ui.store'
import { ElMessage, ElMessageBox } from 'element-plus'
import * as updateApi from '@/api/updateApi'

const uiStore = useUIStore()

// 计算属性
const hasLauncherUpdate = computed(() => uiStore.launcherUpdate?.has_update)
const hasKernelUpdate = computed(() => uiStore.kernelUpdate?.has_update)
const isDownloading = computed(() => uiStore.isDownloadingUpdate)
const progress = computed(() => uiStore.downloadProgress)

// 格式化文件大小
const formatFileSize = (bytes?: number) => {
  if (!bytes) return '-'
  if (bytes < 1024) return `${bytes} B`
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`
  return `${(bytes / 1024 / 1024).toFixed(1)} MB`
}

// 格式化速度
const formatSpeed = (bytesPerSec?: number) => {
  if (!bytesPerSec) return '-'
  if (bytesPerSec < 1024) return `${bytesPerSec} B/s`
  if (bytesPerSec < 1024 * 1024) return `${(bytesPerSec / 1024).toFixed(1)} KB/s`
  return `${(bytesPerSec / 1024 / 1024).toFixed(1)} MB/s`
}

// 解析更新日志
const parseChangelog = (changelog?: string): string[] => {
  if (!changelog) return []
  return changelog.split('\n').filter(line => line.trim())
}

const handleClose = () => {
  uiStore.setUpdateDialogVisible(false)
}

// 下载启动器更新
const handleLauncherUpdate = async (source: updateApi.DownloadSource) => {
  if (!uiStore.launcherUpdate?.file_hash) {
    ElMessage.error('缺少文件校验信息，无法安全更新')
    return
  }

  uiStore.setSelectedDownloadSource(source)
  uiStore.setDownloadingUpdate(true)
  
  try {
    const filePath = await updateApi.downloadAppUpdate(
      source.url,
      uiStore.launcherUpdate.file_hash
    )

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
    uiStore.setDownloadingUpdate(false)
    uiStore.setSelectedDownloadSource(null)
    uiStore.setDownloadProgress(null)
  }
}

// 下载内核更新
const handleKernelUpdate = async (source: updateApi.DownloadSource) => {
  if (!uiStore.kernelUpdate?.file_hash || !uiStore.kernelUpdate?.version) {
    ElMessage.error('缺少文件校验信息，无法安全更新')
    return
  }

  // 检查运行中的浏览器
  const runningCount = await updateApi.getRunningBrowserCount()
  if (runningCount > 0) {
    ElMessage.warning(`请先关闭所有运行中的浏览器（当前 ${runningCount} 个）`)
    return
  }

  uiStore.setSelectedDownloadSource(source)
  uiStore.setDownloadingUpdate(true)
  
  try {
    const filePath = await updateApi.downloadKernelUpdate(
      source.url,
      uiStore.kernelUpdate.file_hash
    )

    await updateApi.installKernelUpdate(filePath, uiStore.kernelUpdate.version)
    ElMessage.success(`内核已更新到 ${uiStore.kernelUpdate.version}`)
    uiStore.setKernelUpdate(null)
  } catch (error) {
    ElMessage.error(`内核更新失败: ${error}`)
  } finally {
    uiStore.setDownloadingUpdate(false)
    uiStore.setSelectedDownloadSource(null)
    uiStore.setDownloadProgress(null)
  }
}

// 获取下载源图标
const getSourceIcon = (name: string) => {
  if (name.includes('CDN') || name.includes('加速')) return 'cloud_download'
  if (name.includes('GitHub')) return 'code'
  if (name.includes('镜像')) return 'content_copy'
  return 'download'
}
</script>

<template>
  <div v-if="uiStore.showUpdateDialog" class="update-dialog-overlay">
    <div class="update-dialog-content" role="dialog" aria-modal="true">
      <!-- 装饰背景 -->
      <div class="decoration decoration-top"></div>
      <div class="decoration decoration-bottom"></div>

      <div class="relative">
        <!-- 头部 -->
        <div class="header">
          <div class="header-left">
            <div class="icon-box">
              <span class="material-symbols-outlined">rocket_launch</span>
            </div>
            <div class="title-area">
              <h3>发现新版本</h3>
              <p class="version-summary">
                <span v-if="hasLauncherUpdate">启动器 v{{ uiStore.launcherUpdate?.version }}</span>
                <span v-if="hasLauncherUpdate && hasKernelUpdate"> · </span>
                <span v-if="hasKernelUpdate">内核 v{{ uiStore.kernelUpdate?.version }}</span>
              </p>
            </div>
          </div>
          <button class="close-btn" @click="handleClose" :disabled="isDownloading">
            <span class="material-symbols-outlined">close</span>
          </button>
        </div>

        <!-- 内容区域 -->
        <div class="body">
          <!-- 下载进度 -->
          <div v-if="isDownloading && progress" class="progress-section">
            <div class="progress-header">
              <span class="progress-label">正在下载...</span>
              <span class="progress-stats">
                {{ formatFileSize(progress.downloaded) }} / {{ formatFileSize(progress.total) }}
                <span class="speed">{{ formatSpeed(progress.speed) }}</span>
              </span>
            </div>
            <div class="progress-bar">
              <div class="progress-fill" :style="{ width: `${progress.percent}%` }"></div>
            </div>
          </div>

          <!-- 启动器更新 -->
          <div v-if="hasLauncherUpdate && !isDownloading" class="update-section">
            <div class="section-header">
              <span class="material-symbols-outlined section-icon">desktop_windows</span>
              <span class="section-title">启动器更新</span>
              <span class="version-badge">v{{ uiStore.launcherUpdate?.version }}</span>
              <span class="file-size">{{ formatFileSize(uiStore.launcherUpdate?.file_size) }}</span>
            </div>
            
            <div v-if="uiStore.launcherUpdate?.release_notes" class="changelog-box">
              <ul class="changelog-list">
                <li v-for="(note, index) in parseChangelog(uiStore.launcherUpdate?.release_notes)" :key="index">
                  <span class="material-symbols-outlined check-icon">check_circle</span>
                  <span>{{ note }}</span>
                </li>
              </ul>
            </div>

            <div class="download-sources">
              <button 
                v-for="source in uiStore.launcherUpdate?.downloads" 
                :key="source.name"
                class="source-btn"
                @click="handleLauncherUpdate(source)"
              >
                <span class="material-symbols-outlined">{{ getSourceIcon(source.name) }}</span>
                <span>{{ source.name }}</span>
              </button>
            </div>
          </div>

          <!-- 内核更新 -->
          <div v-if="hasKernelUpdate && !isDownloading" class="update-section">
            <div class="section-header">
              <span class="material-symbols-outlined section-icon">memory</span>
              <span class="section-title">内核更新</span>
              <span class="version-badge">v{{ uiStore.kernelUpdate?.version }}</span>
              <span class="file-size">{{ formatFileSize(uiStore.kernelUpdate?.file_size) }}</span>
            </div>
            
            <div v-if="uiStore.kernelUpdate?.release_notes" class="changelog-box">
              <ul class="changelog-list">
                <li v-for="(note, index) in parseChangelog(uiStore.kernelUpdate?.release_notes)" :key="index">
                  <span class="material-symbols-outlined check-icon">check_circle</span>
                  <span>{{ note }}</span>
                </li>
              </ul>
            </div>

            <div class="download-sources">
              <button 
                v-for="source in uiStore.kernelUpdate?.downloads" 
                :key="source.name"
                class="source-btn"
                @click="handleKernelUpdate(source)"
              >
                <span class="material-symbols-outlined">{{ getSourceIcon(source.name) }}</span>
                <span>{{ source.name }}</span>
              </button>
            </div>
          </div>

          <p v-if="!isDownloading" class="footer-tip">
            建议您尽快更新以获得最佳体验。更新过程大约需要 1-2 分钟。
          </p>
        </div>

        <!-- 底部按钮 -->
        <div class="footer">
          <button class="btn-later" @click="handleClose" :disabled="isDownloading">
            {{ isDownloading ? '下载中...' : '稍后提醒' }}
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped lang="scss">
.update-dialog-overlay {
  position: fixed;
  inset: 0;
  z-index: 9999;
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 1.5rem;
  background-color: rgba(15, 23, 42, 0.4);
  backdrop-filter: blur(4px);
}

.update-dialog-content {
  position: relative;
  width: 100%;
  max-width: 480px;
  max-height: 80vh;
  background-color: white;
  border-radius: 1.25rem;
  padding: 1.5rem;
  box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.25);
  border: 1px solid #f1f5f9;
  overflow: hidden;
  animation: dialog-appear 0.3s cubic-bezier(0.16, 1, 0.3, 1);
}

@keyframes dialog-appear {
  from { opacity: 0; transform: scale(0.95) translateY(10px); }
  to { opacity: 1; transform: scale(1) translateY(0); }
}

.decoration {
  position: absolute;
  width: 8rem;
  height: 8rem;
  border-radius: 9999px;
  filter: blur(40px);
  opacity: 0.6;
  z-index: 0;

  &-top { top: -2.5rem; right: -2.5rem; background-color: #dbeafe; }
  &-bottom { bottom: -2.5rem; left: -2.5rem; background-color: #f3e8ff; }
}

.relative { position: relative; z-index: 1; }

.header {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  margin-bottom: 1.25rem;

  .header-left {
    display: flex;
    align-items: center;
    gap: 0.75rem;

    .icon-box {
      display: flex;
      height: 3rem;
      width: 3rem;
      align-items: center;
      justify-content: center;
      border-radius: 9999px;
      background-color: #dbeafe;
      color: #2563eb;
      box-shadow: 0 0 0 4px #eff6ff;

      .material-symbols-outlined { font-size: 1.5rem; }
    }

    .title-area {
      h3 {
        font-size: 1.125rem;
        font-weight: 700;
        color: #0f172a;
        margin: 0;
      }

      .version-summary {
        font-size: 0.75rem;
        font-weight: 600;
        color: #2563eb;
        margin: 0.125rem 0 0 0;
      }
    }
  }

  .close-btn {
    padding: 0.25rem;
    border-radius: 9999px;
    color: #94a3b8;
    background: transparent;
    border: none;
    cursor: pointer;
    transition: all 0.2s;

    &:hover { background-color: #f1f5f9; color: #64748b; }
    &:disabled { opacity: 0.5; cursor: not-allowed; }
  }
}

.body {
  margin-top: 1rem;
  max-height: 50vh;
  overflow-y: auto;

  .progress-section {
    background-color: #f0f9ff;
    border-radius: 0.75rem;
    padding: 1rem;
    margin-bottom: 1rem;
    border: 1px solid #bae6fd;

    .progress-header {
      display: flex;
      justify-content: space-between;
      align-items: center;
      margin-bottom: 0.5rem;

      .progress-label {
        font-size: 0.875rem;
        font-weight: 600;
        color: #0369a1;
      }

      .progress-stats {
        font-size: 0.75rem;
        color: #0284c7;
        
        .speed {
          margin-left: 0.5rem;
          color: #0ea5e9;
        }
      }
    }

    .progress-bar {
      height: 6px;
      background-color: #e0f2fe;
      border-radius: 3px;
      overflow: hidden;

      .progress-fill {
        height: 100%;
        background: linear-gradient(90deg, #0ea5e9, #2563eb);
        border-radius: 3px;
        transition: width 0.3s ease;
      }
    }
  }

  .update-section {
    background-color: #f8fafc;
    border-radius: 0.75rem;
    padding: 1rem;
    border: 1px solid #f1f5f9;
    margin-bottom: 1rem;

    .section-header {
      display: flex;
      align-items: center;
      gap: 0.5rem;
      margin-bottom: 0.75rem;

      .section-icon {
        font-size: 1.125rem;
        color: #64748b;
      }

      .section-title {
        font-size: 0.875rem;
        font-weight: 600;
        color: #334155;
      }

      .version-badge {
        font-size: 0.75rem;
        font-weight: 600;
        color: #2563eb;
        background-color: #dbeafe;
        padding: 0.125rem 0.5rem;
        border-radius: 9999px;
      }

      .file-size {
        font-size: 0.75rem;
        color: #94a3b8;
        margin-left: auto;
      }
    }

    .changelog-box {
      margin-bottom: 0.75rem;

      .changelog-list {
        list-style: none;
        padding: 0;
        margin: 0;
        display: flex;
        flex-direction: column;
        gap: 0.375rem;

        li {
          display: flex;
          align-items: flex-start;
          gap: 0.375rem;
          font-size: 0.8125rem;
          color: #475569;

          .check-icon {
            font-size: 0.875rem;
            color: #10b981;
            margin-top: 0.125rem;
          }
        }
      }
    }

    .download-sources {
      display: flex;
      flex-wrap: wrap;
      gap: 0.5rem;

      .source-btn {
        display: inline-flex;
        align-items: center;
        gap: 0.375rem;
        padding: 0.375rem 0.75rem;
        border-radius: 0.5rem;
        font-size: 0.8125rem;
        font-weight: 500;
        color: #475569;
        background-color: white;
        border: 1px solid #e2e8f0;
        cursor: pointer;
        transition: all 0.2s;

        .material-symbols-outlined { font-size: 1rem; }

        &:hover {
          background-color: #2563eb;
          color: white;
          border-color: #2563eb;
        }

        &:first-child {
          background-color: #2563eb;
          color: white;
          border-color: #2563eb;

          &:hover {
            background-color: #1d4ed8;
            border-color: #1d4ed8;
          }
        }
      }
    }
  }

  .footer-tip {
    font-size: 0.8125rem;
    color: #64748b;
    line-height: 1.5;
    margin: 0;
  }
}

.footer {
  margin-top: 1.25rem;
  display: flex;
  align-items: center;
  justify-content: flex-end;

  .btn-later {
    padding: 0.5rem 1rem;
    border-radius: 0.5rem;
    font-size: 0.875rem;
    font-weight: 500;
    color: #475569;
    background: transparent;
    border: none;
    cursor: pointer;
    transition: all 0.2s;

    &:hover { background-color: #f1f5f9; color: #0f172a; }
    &:disabled { opacity: 0.5; cursor: not-allowed; }
  }
}

// 暗色模式
:global(.dark) {
  .update-dialog-content {
    background-color: #1e293b;
    border-color: #334155;
    
    .decoration-top { background-color: rgba(30, 58, 138, 0.3); }
    .decoration-bottom { background-color: rgba(88, 28, 135, 0.3); }
    
    .header .header-left {
      .icon-box { background-color: rgba(30, 58, 138, 0.3); color: #60a5fa; box-shadow: 0 0 0 4px rgba(30, 58, 138, 0.2); }
      .title-area h3 { color: white; }
    }
    
    .body {
      .progress-section {
        background-color: rgba(14, 165, 233, 0.1);
        border-color: rgba(14, 165, 233, 0.3);
        .progress-header .progress-label { color: #38bdf8; }
        .progress-bar { background-color: rgba(14, 165, 233, 0.2); }
      }

      .update-section {
        background-color: rgba(15, 23, 42, 0.5);
        border-color: #334155;
        
        .section-header {
          .section-title { color: #e2e8f0; }
          .version-badge { background-color: rgba(37, 99, 235, 0.2); }
        }
        
        .changelog-box .changelog-list li { color: #cbd5e1; }
        
        .download-sources .source-btn {
          background-color: #334155;
          border-color: #475569;
          color: #e2e8f0;
          
          &:hover { background-color: #2563eb; border-color: #2563eb; }
          &:first-child { background-color: #2563eb; border-color: #2563eb; }
        }
      }
      
      .footer-tip { color: #94a3b8; }
    }
    
    .footer .btn-later {
      color: #94a3b8;
      &:hover { background-color: #334155; color: white; }
    }
  }
}
</style>
