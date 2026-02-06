<script setup lang="ts">
import { computed } from 'vue'

export interface LaunchStep {
  key: string
  label: string
  status: 'pending' | 'running' | 'done' | 'error'
  message?: string
}

interface Props {
  visible: boolean
  profileName?: string
  progress: number
  steps: LaunchStep[]
  error?: string
}

const props = defineProps<Props>()
const emit = defineEmits<{
  close: []
  cancel: []
}>()

const title = computed(() => {
  if (props.error) return '启动失败'
  if (props.progress >= 100) return '启动完成'
  return '浏览器正在打开...'
})

const canClose = computed(() => {
  return props.progress >= 100 || !!props.error
})

const handleClose = () => {
  // 允许随时关闭对话框（启动完成或有错误时自动关闭，否则最小化）
  emit('close')
}

// @ts-ignore
// eslint-disable-next-line @typescript-eslint/no-unused-vars
const handleCancel = () => {
  emit('cancel')
}

const getStepIcon = (status: LaunchStep['status']) => {
  switch (status) {
    case 'done': return 'check_circle'
    case 'running': return 'sync'
    case 'error': return 'error'
    default: return 'radio_button_unchecked'
  }
}

const getStepClass = (status: LaunchStep['status']) => {
  return `step-${status}`
}
</script>

<template>
  <Teleport to="body">
    <div v-if="visible" class="launch-dialog-overlay" @click.self="handleClose">
      <div class="launch-dialog">
        <!-- 标题栏 -->
        <div class="dialog-header">
          <span class="dialog-title">{{ title }}</span>
          <div class="dialog-actions">
            <button 
              class="action-btn close"
              :title="canClose ? '关闭' : '最小化（后台继续启动）'"
              @click="handleClose"
            >
              <span class="material-symbols-outlined">{{ canClose ? 'close' : 'remove' }}</span>
            </button>
          </div>
        </div>

        <!-- 进度条 -->
        <div class="progress-section">
          <div class="progress-bar">
            <div 
              class="progress-fill" 
              :class="{ error: !!error }"
              :style="{ width: `${progress}%` }"
            ></div>
          </div>
          <span class="progress-text">{{ progress }}%</span>
        </div>

        <!-- 步骤列表 -->
        <div class="steps-section">
          <div 
            v-for="step in steps" 
            :key="step.key"
            class="step-item"
            :class="getStepClass(step.status)"
          >
            <span class="step-icon material-symbols-outlined" :class="{ spinning: step.status === 'running' }">
              {{ getStepIcon(step.status) }}
            </span>
            <span class="step-label">{{ step.label }}</span>
            <span v-if="step.message" class="step-message">{{ step.message }}</span>
          </div>
        </div>

        <!-- 错误信息 -->
        <div v-if="error" class="error-section">
          <span class="material-symbols-outlined">error</span>
          <span class="error-text">{{ error }}</span>
        </div>

        <!-- 底部操作 -->
        <div v-if="canClose" class="dialog-footer">
          <button class="btn-primary" @click="handleClose">确定</button>
        </div>
      </div>
    </div>
  </Teleport>
</template>

<style scoped lang="scss">
.launch-dialog-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 9999;
}

.launch-dialog {
  background: #ffffff;
  border-radius: 8px;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.2);
  width: 420px;
  max-width: 90vw;
  overflow: hidden;
}

.dialog-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 16px 20px;
  border-bottom: 1px solid #e5e7eb;
}

.dialog-title {
  font-size: 16px;
  font-weight: 600;
  color: #1f2937;
}

.dialog-actions {
  display: flex;
  gap: 4px;
}

.action-btn {
  width: 28px;
  height: 28px;
  border: none;
  background: transparent;
  border-radius: 4px;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  color: #6b7280;
  transition: all 0.2s;

  &:hover {
    background: #f3f4f6;
    color: #374151;
  }

  .material-symbols-outlined {
    font-size: 18px;
  }
}

.progress-section {
  padding: 20px;
  display: flex;
  align-items: center;
  gap: 12px;
}

.progress-bar {
  flex: 1;
  height: 8px;
  background: #e5e7eb;
  border-radius: 4px;
  overflow: hidden;
}

.progress-fill {
  height: 100%;
  background: linear-gradient(90deg, #3b82f6, #2563eb);
  border-radius: 4px;
  transition: width 0.3s ease;

  &.error {
    background: linear-gradient(90deg, #ef4444, #dc2626);
  }
}

.progress-text {
  font-size: 14px;
  font-weight: 600;
  color: #3b82f6;
  min-width: 45px;
  text-align: right;
}

.steps-section {
  padding: 0 20px 20px;
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.step-item {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 8px 12px;
  border-radius: 6px;
  font-size: 14px;
  transition: all 0.2s;

  &.step-pending {
    color: #9ca3af;
    
    .step-icon {
      color: #d1d5db;
    }
  }

  &.step-running {
    color: #3b82f6;
    background: #eff6ff;
    
    .step-icon {
      color: #3b82f6;
    }
  }

  &.step-done {
    color: #059669;
    
    .step-icon {
      color: #10b981;
    }
  }

  &.step-error {
    color: #dc2626;
    background: #fef2f2;
    
    .step-icon {
      color: #ef4444;
    }
  }
}

.step-icon {
  font-size: 18px;

  &.spinning {
    animation: spin 1s linear infinite;
  }
}

@keyframes spin {
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
}

.step-label {
  flex: 1;
}

.step-message {
  font-size: 12px;
  color: #6b7280;
}

.error-section {
  margin: 0 20px 20px;
  padding: 12px 16px;
  background: #fef2f2;
  border: 1px solid #fecaca;
  border-radius: 6px;
  display: flex;
  align-items: flex-start;
  gap: 10px;
  color: #dc2626;

  .material-symbols-outlined {
    font-size: 20px;
    flex-shrink: 0;
  }

  .error-text {
    font-size: 13px;
    line-height: 1.5;
  }
}

.dialog-footer {
  padding: 16px 20px;
  border-top: 1px solid #e5e7eb;
  display: flex;
  justify-content: flex-end;
}

.btn-primary {
  padding: 8px 24px;
  background: #3b82f6;
  color: #ffffff;
  border: none;
  border-radius: 6px;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;

  &:hover {
    background: #2563eb;
  }
}

// 深色模式
:root.dark {
  .launch-dialog {
    background: #1f2937;
  }

  .dialog-header {
    border-bottom-color: #374151;
  }

  .dialog-title {
    color: #f3f4f6;
  }

  .action-btn {
    color: #9ca3af;

    &:hover {
      background: #374151;
      color: #e5e7eb;
    }
  }

  .progress-bar {
    background: #374151;
  }

  .step-item {
    &.step-running {
      background: rgba(59, 130, 246, 0.1);
    }

    &.step-error {
      background: rgba(239, 68, 68, 0.1);
    }
  }

  .error-section {
    background: rgba(239, 68, 68, 0.1);
    border-color: rgba(239, 68, 68, 0.3);
  }

  .dialog-footer {
    border-top-color: #374151;
  }
}
</style>
