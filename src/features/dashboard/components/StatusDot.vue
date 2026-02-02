<script setup lang="ts">
/**
 * @description 状态指示灯组件 - 桌面原生化
 * @author DeepAgent
 */

interface Props {
  /** 状态 */
  status: 'running' | 'stopped' | 'launching' | 'error'
  /** 尺寸 */
  size?: 'sm' | 'md' | 'lg'
  /** 显示文字 */
  showText?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  size: 'md',
  showText: false,
})

// 使用设计令牌中定义的状态色
const statusConfig = {
  running: { colorVar: '--color-status-running', text: '运行中', pulse: true },
  stopped: { colorVar: '--color-status-stopped', text: '已停止', pulse: false },
  launching: { colorVar: '--color-status-warning', text: '启动中', pulse: true },
  error: { colorVar: '--color-status-error', text: '错误', pulse: false },
}

const config = statusConfig[props.status]

const sizeMap = {
  sm: 8,
  md: 10,
  lg: 12,
}
</script>

<template>
  <div class="status-dot-wrapper">
    <span 
      class="status-dot"
      :class="[props.status, { pulse: config.pulse }]"
      :style="{ '--dot-size': `${sizeMap[size]}px` }"
    >
      <span class="dot-inner"></span>
      <span v-if="config.pulse" class="dot-ring"></span>
    </span>
    <span v-if="showText" class="status-text" :class="props.status">
      {{ config.text }}
    </span>
  </div>
</template>

<style scoped lang="scss">
.status-dot-wrapper {
  display: inline-flex;
  align-items: center;
  gap: var(--spacing-sm);
}

.status-dot {
  width: var(--dot-size);
  height: var(--dot-size);
  position: relative;
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
  
  .dot-inner {
    width: 100%;
    height: 100%;
    border-radius: var(--radius-full);
    transition: all var(--duration-fast) var(--ease-out-quart);
  }
  
  .dot-ring {
    position: absolute;
    inset: -3px;
    border-radius: var(--radius-full);
    opacity: 0;
  }
  
  // 状态色
  &.running {
    .dot-inner {
      background-color: var(--color-status-running);
      box-shadow: 0 0 8px var(--color-status-running);
    }
    .dot-ring {
      border: 1.5px solid var(--color-status-running);
    }
  }
  
  &.stopped {
    .dot-inner {
      background-color: var(--color-status-stopped);
    }
  }
  
  &.launching {
    .dot-inner {
      background-color: var(--color-status-warning);
      box-shadow: 0 0 8px var(--color-status-warning);
    }
    .dot-ring {
      border: 1.5px solid var(--color-status-warning);
    }
  }
  
  &.error {
    .dot-inner {
      background-color: var(--color-status-error);
      box-shadow: 0 0 6px var(--color-status-error);
    }
  }
  
  // 脉动动画 - 更流畅的桌面原生感
  &.pulse {
    .dot-inner {
      animation: dot-glow 2s var(--ease-in-out-cubic) infinite;
    }
    .dot-ring {
      animation: ring-pulse 2s var(--ease-out-expo) infinite;
    }
  }
}

.status-text {
  font-size: var(--text-sm);
  font-weight: var(--font-medium);
  color: var(--color-text-secondary);
  transition: color var(--duration-fast) var(--ease-out-quart);
  
  &.running {
    color: var(--color-status-running);
  }
  
  &.error {
    color: var(--color-status-error);
  }
  
  &.launching {
    color: var(--color-status-warning);
  }
}

@keyframes dot-glow {
  0%, 100% {
    opacity: 1;
    transform: scale(1);
  }
  50% {
    opacity: 0.85;
    transform: scale(1.1);
  }
}

@keyframes ring-pulse {
  0% {
    opacity: 0.6;
    transform: scale(1);
  }
  100% {
    opacity: 0;
    transform: scale(1.8);
  }
}
</style>
