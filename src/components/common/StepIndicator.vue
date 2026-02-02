<template>
  <div class="step-indicator">
    <!-- 背景进度线 -->
    <div class="step-line-bg"></div>
    <!-- 激活进度线 -->
    <div class="step-line-active" :style="{ width: progressWidth }"></div>

    <!-- 步骤节点 -->
    <div
      v-for="(step, index) in steps"
      :key="index"
      class="step-item"
      :class="{
        'step-completed': index < currentStep,
        'step-active': index === currentStep,
        'step-pending': index > currentStep
      }"
    >
      <div class="step-circle">
        <!-- 已完成：显示勾选图标 -->
        <span v-if="index < currentStep" class="material-symbols-outlined">check</span>
        <!-- 当前/未完成：显示数字 -->
        <span v-else>{{ index + 1 }}</span>
      </div>
      <span class="step-label">{{ step }}</span>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'

interface Props {
  steps: string[]
  currentStep: number // 0-based index
}

const props = withDefaults(defineProps<Props>(), {
  steps: () => ['步骤1', '步骤2', '步骤3'],
  currentStep: 0
})

// 计算进度条宽度
const progressWidth = computed(() => {
  if (props.steps.length <= 1) return '0%'
  const percentage = (props.currentStep / (props.steps.length - 1)) * 100
  return `${Math.min(percentage, 100)}%`
})
</script>

<style scoped lang="scss">
.step-indicator {
  position: relative;
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  padding: 0 2rem;
}

// 背景线
.step-line-bg,
.step-line-active {
  position: absolute;
  top: 1rem;
  left: 0;
  height: 2px;
  z-index: 0;
}

.step-line-bg {
  width: 100%;
  background: var(--color-border-default);
}

.step-line-active {
  background: var(--color-accent-blue);
  transition: width 0.3s ease-out;
}

// 步骤项
.step-item {
  position: relative;
  z-index: 10;
  display: flex;
  flex-direction: column;
  align-items: center;
  flex: 0 0 auto;
  min-width: 80px;
}

.step-circle {
  width: 32px;
  height: 32px;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  font-weight: 700;
  font-size: 14px;
  transition: all 0.3s ease;
  margin-bottom: 0.5rem;

  .material-symbols-outlined {
    font-size: 18px;
  }
}

.step-label {
  font-size: 12px;
  font-weight: 500;
  text-align: center;
  white-space: nowrap;
  transition: all 0.3s ease;
}

// 已完成状态
.step-completed {
  .step-circle {
    background: var(--color-accent-blue);
    color: white;
    box-shadow: 0 2px 4px rgba(37, 99, 235, 0.3);
  }

  .step-label {
    color: var(--color-text-primary);
  }
}

// 当前激活状态
.step-active {
  .step-circle {
    background: var(--color-accent-blue);
    color: white;
    box-shadow: 0 4px 12px rgba(37, 99, 235, 0.3);
    ring: 4px solid rgba(37, 99, 235, 0.15);
  }

  .step-label {
    color: var(--color-accent-blue);
    font-weight: 700;
  }
}

// 待完成状态
.step-pending {
  .step-circle {
    background: var(--color-bg-secondary);
    border: 2px solid var(--color-border-default);
    color: var(--color-text-tertiary);
  }

  .step-label {
    color: var(--color-text-tertiary);
  }
}

// 暗色模式优化
.dark {
  .step-line-bg {
    background: var(--color-border-dark, #334155);
  }

  .step-pending {
    .step-circle {
      background: var(--color-bg-secondary);
      border-color: var(--color-border-dark, #334155);
    }
  }
}
</style>
