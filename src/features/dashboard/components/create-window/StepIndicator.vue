<script setup lang="ts">
/**
 * @description 步骤条组件 - 新建窗口向导专用
 */

interface Step {
  id: number
  label: string
}

interface Props {
  steps: Step[]
  current: number
}

defineProps<Props>()
const emit = defineEmits(['select'])

const handleStepClick = (stepId: number) => {
  emit('select', stepId)
}
</script>

<template>
  <div class="step-indicator">
    <div class="steps-container">
      <template v-for="(step, index) in steps" :key="step.id">
        <!-- 步骤项 -->
        <div 
          class="step-item" 
          :class="{ 
            completed: current > step.id,
            active: current === step.id,
            pending: current < step.id
          }"
          @click="handleStepClick(step.id)"
        >
          <div class="step-circle">
            <span v-if="current > step.id" class="material-symbols-outlined check-icon">check</span>
            <span v-else class="step-number">{{ step.id }}</span>
          </div>
          <span class="step-label">{{ step.label }}</span>
        </div>
        
        <!-- 连接线 -->
        <div 
          v-if="index < steps.length - 1" 
          class="step-line"
          :class="{ completed: current > step.id }"
        ></div>
      </template>
    </div>
  </div>
</template>

<style scoped lang="scss">
.step-indicator {
  width: 100%;
}

.steps-container {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  position: relative;
}

.step-item {
  display: flex;
  flex-direction: column;
  align-items: center;
  position: relative;
  z-index: 10;
  min-width: 80px;
  cursor: pointer;
  
  &.completed {
    .step-circle {
      background: #2563eb;
      border-color: #2563eb;
      color: white;
      box-shadow: 0 2px 8px rgba(37, 99, 235, 0.3);
    }
    
    .step-label {
      color: var(--color-text-secondary);
      font-weight: 500;
    }
  }
  
  &.active {
    .step-circle {
      background: #2563eb;
      border-color: #2563eb;
      color: white;
      box-shadow: 0 0 0 4px rgba(37, 99, 235, 0.15), 0 2px 8px rgba(37, 99, 235, 0.3);
      animation: pulse-ring 2s infinite;
    }
    
    .step-label {
      color: #2563eb;
      font-weight: 700;
    }
  }
  
  &.pending {
    .step-circle {
      background: white;
      border: 2px solid #cbd5e1;
      color: #94a3b8;
      
      .dark & {
        background: var(--color-bg-elevated);
        border-color: var(--color-border-dark);
        color: var(--color-text-tertiary);
      }
    }
    
    .step-label {
      color: var(--color-text-tertiary);
      font-weight: 500;
    }
  }
}

.step-circle {
  width: 32px;
  height: 32px;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 14px;
  font-weight: 700;
  transition: all 0.3s ease;
  
  .check-icon {
    font-size: 18px;
    color: inherit;
  }
  
  .step-number {
    line-height: 1;
  }
}

.step-label {
  margin-top: 8px;
  font-size: 12px;
  white-space: nowrap;
  transition: all 0.3s ease;
}

.step-line {
  flex: 1;
  height: 2px;
  background: #e2e8f0;
  margin: 16px 8px 0;
  position: relative;
  transition: background 0.3s ease;
  
  .dark & {
    background: var(--color-border-dark);
  }
  
  &.completed {
    background: #2563eb;
  }
}

@keyframes pulse-ring {
  0% {
    box-shadow: 0 0 0 4px rgba(37, 99, 235, 0.15), 0 2px 8px rgba(37, 99, 235, 0.3);
  }
  50% {
    box-shadow: 0 0 0 6px rgba(37, 99, 235, 0.1), 0 2px 8px rgba(37, 99, 235, 0.3);
  }
  100% {
    box-shadow: 0 0 0 4px rgba(37, 99, 235, 0.15), 0 2px 8px rgba(37, 99, 235, 0.3);
  }
}
</style>
