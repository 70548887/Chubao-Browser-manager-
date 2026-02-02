<script setup lang="ts">
/**
 * @description 主题切换按钮组件 - 桌面原生化
 * @author DeepAgent
 */

import { useTheme } from '@/composables/useTheme'

const { theme, toggleTheme } = useTheme()
</script>

<template>
  <el-tooltip :content="theme === 'dark' ? '切换到白天模式' : '切换到暗黑模式'" placement="bottom">
    <button 
      class="theme-toggle-btn"
      @click="toggleTheme"
      :aria-label="theme === 'dark' ? '切换到白天模式' : '切换到暗黑模式'"
    >
      <span class="icon-wrapper" :class="theme">
        <el-icon v-if="theme === 'dark'" class="theme-icon sun">
          <Sunny />
        </el-icon>
        <el-icon v-else class="theme-icon moon">
          <Moon />
        </el-icon>
      </span>
    </button>
  </el-tooltip>
</template>

<style scoped lang="scss">
.theme-toggle-btn {
  width: 28px;
  height: 28px;
  display: flex;
  align-items: center;
  justify-content: center;
  border: 1px solid var(--color-border-default);
  border-radius: var(--radius-md);
  background: transparent;
  cursor: pointer;
  transition: all var(--duration-fast) var(--ease-out-quart);
  
  &:hover {
    background: var(--color-hover-bg);
    border-color: var(--color-border-strong);
    
    .icon-wrapper {
      transform: rotate(15deg) scale(1.1);
    }
  }
  
  &:active {
    transform: scale(0.95);
  }
}

.icon-wrapper {
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all var(--duration-fast) var(--ease-out-quart);
  
  &.dark .theme-icon {
    color: #fbbf24;
  }
  
  &.light .theme-icon {
    color: #6366f1;
  }
}

.theme-icon {
  font-size: 16px;
  transition: all var(--duration-fast) var(--ease-out-quart);
  
  &.sun {
    animation: sun-rise var(--duration-normal) var(--ease-out-expo);
  }
  
  &.moon {
    animation: moon-rise var(--duration-normal) var(--ease-out-expo);
  }
}

@keyframes sun-rise {
  0% {
    opacity: 0;
    transform: rotate(-90deg) scale(0.5);
  }
  100% {
    opacity: 1;
    transform: rotate(0) scale(1);
  }
}

@keyframes moon-rise {
  0% {
    opacity: 0;
    transform: rotate(90deg) scale(0.5);
  }
  100% {
    opacity: 1;
    transform: rotate(0) scale(1);
  }
}
</style>
