<script setup lang="ts">
/**
 * @description 环境配置卡片组件 - 桌面原生化
 * @author DeepAgent
 */

import type { Profile } from '@/types'

interface Props {
  profile: Profile
  selected?: boolean
}

withDefaults(defineProps<Props>(), {
  selected: false,
})

const emit = defineEmits<{
  (e: 'select'): void
  (e: 'launch'): void
  (e: 'stop'): void
  (e: 'edit'): void
  (e: 'delete'): void
}>()

// 格式化时间
const formatDate = (timestamp: number) => {
  const date = new Date(timestamp)
  return date.toLocaleDateString('zh-CN', {
    month: '2-digit',
    day: '2-digit',
  })
}

const statusMap = {
  running: { label: '运行中', color: 'success' },
  stopped: { label: '已停止', color: 'info' },
  launching: { label: '启动中', color: 'warning' },
  error: { label: '错误', color: 'danger' },
}
</script>

<template>
  <div 
    class="profile-card"
    :class="{ 
      'is-selected': selected,
      'is-running': profile.status === 'running' 
    }"
    @click="emit('select')"
  >
    <!-- 状态指示条 -->
    <div class="profile-card__indicator"></div>
    
    <!-- 头部 -->
    <div class="profile-card__header">
      <div class="profile-card__avatar">
        <el-icon><Monitor /></el-icon>
      </div>
      <div class="profile-card__info">
        <h4 class="profile-card__name">{{ profile.name }}</h4>
        <span class="profile-card__id">#{{ profile.id.slice(0, 8) }}</span>
      </div>
      <div class="profile-card__status">
        <span class="status-dot" :class="profile.status"></span>
        <span class="status-text">{{ statusMap[profile.status].label }}</span>
      </div>
    </div>
    
    <!-- 详情 -->
    <div class="profile-card__details">
      <div class="detail-item">
        <span class="detail-label">平台</span>
        <span class="detail-value">{{ profile.fingerprint.platform }}</span>
      </div>
      <div class="detail-item">
        <span class="detail-label">代理</span>
        <span class="detail-value">
          {{ profile.proxy ? `${profile.proxy.host}:${profile.proxy.port}` : '无' }}
        </span>
      </div>
      <div class="detail-item">
        <span class="detail-label">分组</span>
        <span class="detail-value">{{ profile.group || '默认' }}</span>
      </div>
      <div class="detail-item">
        <span class="detail-label">更新</span>
        <span class="detail-value">{{ formatDate(profile.updatedAt) }}</span>
      </div>
    </div>
    
    <!-- 操作按钮 -->
    <div class="profile-card__actions" @click.stop>
      <template v-if="profile.status === 'running'">
        <button class="action-btn stop" @click="emit('stop')">
          <el-icon><VideoPause /></el-icon>
          <span>停止</span>
        </button>
      </template>
      <template v-else>
        <button class="action-btn launch" @click="emit('launch')">
          <el-icon><VideoPlay /></el-icon>
          <span>启动</span>
        </button>
      </template>
      
      <button class="action-btn secondary" @click="emit('edit')">
        <el-icon><Edit /></el-icon>
      </button>
      
      <button class="action-btn secondary danger" @click="emit('delete')">
        <el-icon><Delete /></el-icon>
      </button>
    </div>
  </div>
</template>

<style scoped lang="scss">
.profile-card {
  background-color: var(--color-bg-elevated);
  border: 1px solid var(--color-border-default);
  border-radius: var(--radius-lg);
  padding: var(--spacing-lg);
  transition: all var(--duration-fast) var(--ease-out-quart);
  position: relative;
  overflow: hidden;
  cursor: pointer;
  
  &:hover {
    border-color: var(--color-border-strong);
    box-shadow: var(--shadow-md);
    transform: translateY(-2px);
    
    .profile-card__actions {
      opacity: 1;
    }
  }
  
  &.is-selected {
    border-color: var(--color-accent-blue);
    box-shadow: 0 0 0 2px rgba(0, 120, 212, 0.1);
    
    .profile-card__indicator {
      opacity: 1;
    }
  }
  
  &.is-running {
    .profile-card__indicator {
      background: var(--color-status-running);
      opacity: 1;
      box-shadow: 0 0 8px var(--color-status-running);
    }
  }
}

.profile-card__indicator {
  position: absolute;
  left: 0;
  top: 0;
  bottom: 0;
  width: 3px;
  background: var(--color-accent-blue);
  opacity: 0;
  transition: all var(--duration-fast);
}

.profile-card__header {
  display: flex;
  align-items: center;
  gap: var(--spacing-md);
  margin-bottom: var(--spacing-md);
}

.profile-card__avatar {
  width: 40px;
  height: 40px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: linear-gradient(135deg, var(--color-accent-blue) 0%, #005a9e 100%);
  border-radius: var(--radius-md);
  color: white;
  flex-shrink: 0;
  
  .el-icon {
    font-size: 20px;
  }
}

.profile-card__info {
  flex: 1;
  min-width: 0;
}

.profile-card__name {
  font-size: var(--text-md);
  font-weight: var(--font-semibold);
  color: var(--color-text-primary);
  margin: 0;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.profile-card__id {
  font-size: var(--text-xs);
  color: var(--color-text-tertiary);
  font-family: monospace;
}

.profile-card__status {
  display: flex;
  align-items: center;
  gap: var(--spacing-xs);
  
  .status-dot {
    width: 8px;
    height: 8px;
    border-radius: var(--radius-full);
    
    &.running {
      background: var(--color-status-running);
      box-shadow: 0 0 6px var(--color-status-running);
      animation: pulse 2s ease-in-out infinite;
    }
    
    &.stopped {
      background: var(--color-status-stopped);
    }
    
    &.launching {
      background: var(--color-status-warning);
      animation: pulse 1s ease-in-out infinite;
    }
    
    &.error {
      background: var(--color-accent-danger);
    }
  }
  
  .status-text {
    font-size: var(--text-xs);
    color: var(--color-text-secondary);
  }
}

@keyframes pulse {
  0%, 100% {
    opacity: 1;
  }
  50% {
    opacity: 0.6;
  }
}

.profile-card__details {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: var(--spacing-sm) var(--spacing-lg);
  padding: var(--spacing-md) 0;
  border-top: 1px solid var(--color-border-subtle);
  border-bottom: 1px solid var(--color-border-subtle);
  margin-bottom: var(--spacing-md);
}

.detail-item {
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.detail-label {
  font-size: var(--text-xs);
  color: var(--color-text-tertiary);
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.detail-value {
  font-size: var(--text-sm);
  color: var(--color-text-primary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.profile-card__actions {
  display: flex;
  gap: var(--spacing-sm);
  opacity: 0.8;
  transition: opacity var(--duration-fast);
}

.action-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: var(--spacing-xs);
  height: 28px;
  padding: 0 var(--spacing-md);
  border: none;
  border-radius: var(--radius-md);
  font-size: var(--text-sm);
  font-weight: var(--font-medium);
  cursor: pointer;
  transition: all var(--duration-fast) var(--ease-out-quart);
  
  .el-icon {
    font-size: 14px;
  }
  
  &.launch {
    flex: 1;
    background: var(--color-accent-blue);
    color: white;
    
    &:hover {
      background: var(--color-accent-blue-hover);
      transform: translateY(-1px);
    }
  }
  
  &.stop {
    flex: 1;
    background: var(--color-accent-danger);
    color: white;
    
    &:hover {
      background: #c82333;
      transform: translateY(-1px);
    }
  }
  
  &.secondary {
    width: 28px;
    padding: 0;
    background: var(--color-bg-secondary);
    border: 1px solid var(--color-border-default);
    color: var(--color-text-secondary);
    
    &:hover {
      background: var(--color-hover-bg);
      border-color: var(--color-border-strong);
      color: var(--color-text-primary);
    }
    
    &.danger:hover {
      background: rgba(220, 53, 69, 0.1);
      border-color: var(--color-accent-danger);
      color: var(--color-accent-danger);
    }
  }
}
</style>
