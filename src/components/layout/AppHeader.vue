<script setup lang="ts">
/**
 * @description 应用顶部工具栏 - 原型风格
 * @author DeepAgent
 */

import { inject, type ComputedRef } from 'vue'
import { useUIStore } from '@/stores/ui.store'

// 注入页面标题
const currentPage = inject<ComputedRef<string>>('currentPage')
const uiStore = useUIStore()

interface Emits {
  (e: 'create-new'): void
}

defineEmits<Emits>()

// 页面标题映射
const pageTitles: Record<string, string> = {
  dashboard: '窗口列表',
  groups: '分组管理',
  tags: '标签管理',
  proxy: '代理管理',
  recycle: '回收站',
  settings: '系统设置',
  rpa: 'RPA 机器人',
  extensions: '扩展中心',
}

// 搜索占位符映射
const searchPlaceholders: Record<string, string> = {
  dashboard: '搜索 ID、名称、备注、代理IP...',
  groups: '搜索分组名称、备注...',
  tags: '搜索标签名称...',
  proxy: '搜索代理名称、IP...',
  recycle: '搜索已删除环境...',
  rpa: '搜索自动化流程...',
  extensions: '搜索扩展程序 (例如: MetaMask, AdBlock)...',
}

const getPageTitle = () => {
  return pageTitles[currentPage?.value || 'dashboard'] || '窗口列表'
}

const getSearchPlaceholder = () => {
  return searchPlaceholders[currentPage?.value || 'dashboard'] || '搜索...'
}
</script>

<template>
  <header class="app-header" v-if="currentPage !== 'settings'">
    <!-- 左侧：标题区域 -->
    <div class="header-left">
      <h1 class="page-title">
        {{ getPageTitle() }}
        <span v-if="currentPage === 'rpa'" class="beta-badge">Beta</span>
        <span v-else class="pro-badge">Pro</span>
      </h1>
    </div>
    
    <!-- 右侧：搜索 -->
    <div class="header-right">
      <!-- 搜索框 - 原型风格 -->
      <div class="search-wrapper">
        <span class="material-symbols-outlined search-icon">search</span>
        <input
          v-model="uiStore.searchKeyword"
          type="text"
          class="search-input"
          :placeholder="getSearchPlaceholder()"
        />
      </div>
    </div>
  </header>
</template>

<style scoped lang="scss">
.app-header {
  height: 64px;
  flex-shrink: 0; // 防止在 flex 布局中被压缩
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 24px;
  border-bottom: 1px solid var(--color-border-default);
  background-color: var(--color-bg-secondary);
  position: sticky;
  top: 0;
  z-index: 10;
  box-shadow: var(--shadow-xs);
  transition: background-color var(--duration-normal), border-color var(--duration-normal);
}

// 左侧标题区域
.header-left {
  display: flex;
  align-items: center;
  gap: 16px;
}

.page-title {
  font-size: 20px;
  font-weight: 700;
  color: var(--color-text-primary);
  letter-spacing: -0.5px;
  display: flex;
  align-items: center;
  gap: 8px;
  margin: 0;
  transition: color var(--duration-normal);
}

.pro-badge {
  display: inline-flex;
  align-items: center;
  height: 20px;
  padding: 0 8px;
  background: rgba(37, 99, 235, 0.15);
  border: 1px solid rgba(37, 99, 235, 0.3);
  border-radius: 9999px;
  color: var(--color-accent-blue);
  font-size: 11px;
  font-weight: 600;
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.beta-badge {
  display: inline-flex;
  align-items: center;
  height: 20px;
  padding: 0 8px;
  background: var(--color-selected-bg);
  border: 1px solid var(--color-border-interactive);
  border-radius: 9999px;
  color: var(--color-accent-blue);
  font-size: 11px;
  font-weight: 600;
}

:global(.dark) {
  .beta-badge {
    background: rgba(49, 46, 129, 0.4);
    border-color: rgba(49, 46, 129, 0.6);
    color: #a5b4fc;
  }
}

// 右侧操作区域
.header-right {
  display: flex;
  align-items: center;
  gap: 16px;
}

// 搜索框 - 支持主题切换
.search-wrapper {
  position: relative;
  width: 384px;
  
  @media (min-width: 1024px) {
    width: 512px;
  }
  
  .search-icon {
    position: absolute;
    left: 12px;
    top: 50%;
    transform: translateY(-50%);
    font-size: 20px;
    color: var(--color-text-tertiary);
    pointer-events: none;
    transition: color 0.15s;
  }
  
  .search-input {
    width: 100%;
    height: 40px;
    padding: 0 12px 0 42px;
    border: 1px solid var(--color-border-default);
    border-radius: 8px;
    background: var(--color-bg-primary);
    color: var(--color-text-primary);
    font-size: 14px;
    outline: none;
    transition: all 0.15s;
    
    &::placeholder {
      color: var(--color-text-tertiary);
    }
    
    &:hover {
      border-color: var(--color-border-strong);
    }
    
    &:focus {
      border-color: var(--color-accent-blue);
      background: var(--color-bg-secondary);
      box-shadow: 0 0 0 3px rgba(37, 99, 235, 0.15);
    }
  }
  
  &:focus-within {
    .search-icon {
      color: var(--color-accent-blue);
    }
  }
}
</style>
