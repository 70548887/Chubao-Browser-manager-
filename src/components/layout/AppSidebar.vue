<script setup lang="ts">
/**
 * @description 应用侧边栏组件 - 桌面原生化
 * @author DeepAgent
 */

import { ref, computed, inject } from 'vue'
import { mockProfiles } from '@/features/dashboard/mock.data'

// 注入页面导航
const navigateTo = inject<(page: 'dashboard' | 'groups' | 'recycle' | 'proxy' | 'tags' | 'settings' | 'rpa' | 'extensions') => void>('navigateTo')

interface Props {
  collapsed?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  collapsed: false
})

// 计算运行中的环境数量
const runningCount = computed(() => {
  return mockProfiles.filter(p => p.status === 'running').length
})

const menuItems = [
  // 核心功能组
  { 
    icon: 'web', 
    title: '窗口列表', 
    path: '/',
    group: null as string | null,
    separator: false,
    count: runningCount,
    showCount: false,
    highlight: true
  },
  { icon: 'folder_open', title: '分组管理', path: '/groups', group: null as string | null, separator: false },
  { icon: 'label', title: '标签管理', path: '/tags', group: null as string | null, separator: false },
  { icon: 'delete', title: '回收站', path: '/recycle', group: null as string | null, separator: false },
  // 分隔符
  { separator: true, group: null as string | null, path: '', icon: '', title: '' },
  // 资源配置组
  { icon: 'dns', title: '代理管理', path: '/proxy', group: '资源配置', separator: false },
  { icon: 'extension', title: '扩展中心', path: '/extensions', group: '资源配置', separator: false },
  // 分隔符
  { separator: true, group: null as string | null, path: '', icon: '', title: '' },
  // 自动化组
  { icon: 'smart_toy', title: 'RPA 机器人', path: '/automation', group: '自动化', separator: false },
  { icon: 'sync', title: '同步中心', path: '/sync', group: '自动化', separator: false },
]

const activeIndex = ref('/')

// 新建浏览器和折叠功能
const emit = defineEmits(['create-new', 'toggle-collapse'])
const handleCreateNew = () => {
  console.log('🟡 [Sidebar] handleCreateNew 被调用，即将触发 create-new 事件')
  emit('create-new')
  console.log('✅ [Sidebar] create-new 事件已触发')
}
const handleToggleCollapse = () => {
  emit('toggle-collapse')
}

// 处理菜单点击
const handleMenuClick = (item: typeof menuItems[0]) => {
  activeIndex.value = item.path
  
  // 导航到对应页面
  if (item.path === '/') {
    navigateTo?.('dashboard')
  } else if (item.path === '/groups') {
    navigateTo?.('groups')
  } else if (item.path === '/recycle') {
    navigateTo?.('recycle')
  } else if (item.path === '/proxy') {
    navigateTo?.('proxy')
  } else if (item.path === '/tags') {
    navigateTo?.('tags')
  } else if (item.path === '/settings') {
    navigateTo?.('settings')
  } else if (item.path === '/automation') {
    navigateTo?.('rpa')
  } else if (item.path === '/extensions') {
    navigateTo?.('extensions')
  }
}
</script>

<template>
  <aside class="sidebar" :class="{ collapsed: props.collapsed }">
    <!-- 折叠按钮 - AdsPower 风格 -->
    <button
      class="collapse-toggle-btn"
      @click="handleToggleCollapse"
      :aria-label="props.collapsed ? '展开侧边栏' : '收起侧边栏'"
    >
      <el-icon :size="12">
        <ArrowLeft v-if="!props.collapsed" />
        <ArrowRight v-else />
      </el-icon>
    </button>
    
    <!-- Logo Area - 原型风格 -->
    <div class="logo-area">
      <div class="logo-content">
        <span class="material-symbols-outlined logo-icon">fingerprint</span>
        <transition name="fade-slide">
          <span v-if="!props.collapsed" class="logo-text">DeepBrowser</span>
        </transition>
      </div>
    </div>
    
    <!-- New Browser Button - 原型风格渐变 -->
    <div v-if="!props.collapsed" class="new-browser-section">
      <button class="new-browser-btn" @click="handleCreateNew">
        <span class="material-symbols-outlined btn-icon">add_circle</span>
        <span>创建窗口</span>
      </button>
    </div>
    <div v-else class="new-browser-section collapsed">
      <el-tooltip content="创建窗口" placement="right" :show-after="200">
        <button class="new-browser-btn-mini" @click="handleCreateNew">
          <span class="material-symbols-outlined">add_circle</span>
        </button>
      </el-tooltip>
    </div>
    
    <!-- Navigation Menu - 原型风格分组 -->
    <nav class="nav-menu">
      <template v-for="(item, index) in menuItems" :key="index">
        <!-- 分隔符 -->
        <div v-if="item.separator" class="menu-separator"></div>
        
        <!-- 分组标题 -->
        <div v-else-if="item.group && (!menuItems[index - 1] || menuItems[index - 1].group !== item.group)" 
             class="menu-group-title">
          <span v-if="!props.collapsed">{{ item.group }}</span>
        </div>
        
        <!-- 菜单项 - 收缩时显示 tooltip -->
        <el-tooltip
          v-if="!item.separator"
          :content="item.title"
          placement="right"
          :disabled="!props.collapsed"
          :show-after="200"
        >
          <div
            class="menu-item"
            :class="{ 
              active: activeIndex === item.path,
              highlight: item.highlight 
            }"
            @click="handleMenuClick(item)"
          >
            <span class="material-symbols-outlined menu-icon">{{ item.icon }}</span>
            <transition name="fade-slide">
              <span v-if="!props.collapsed" class="menu-title">{{ item.title }}</span>
            </transition>
            <el-badge 
              v-if="!props.collapsed && item.showCount && item.count" 
              :value="item.count.value" 
              :max="99"
              class="menu-badge"
            />
          </div>
        </el-tooltip>
      </template>
    </nav>
    
    <!-- User Profile - 原型风格 -->
    <div class="user-profile">
      <div class="avatar">DA</div>
      <transition name="fade-slide">
        <div v-if="!props.collapsed" class="user-info">
          <div class="username">DeepAgent 专业版</div>
          <div class="user-id">ID: 8847291</div>
        </div>
      </transition>
      <button v-if="!props.collapsed" class="settings-btn" title="设置" @click="navigateTo?.('settings')">
        <span class="material-symbols-outlined">settings</span>
      </button>
    </div>
  </aside>
</template>

<style scoped lang="scss">
.sidebar {
  position: relative;
  z-index: 50; // 设置较低的 z-index，确保不会遮挡 Dialog (z-index: 2000+)
  width: 256px; // w-64 = 256px
  background-color: var(--color-bg-secondary);
  border-right: 1px solid var(--color-border-default);
  display: flex;
  flex-direction: column;
  flex-shrink: 0;
  overflow: visible;
  transition: width 0.3s cubic-bezier(0.4, 0, 0.2, 1), background-color var(--duration-normal);
  box-shadow: var(--shadow-xs);
  
  &.collapsed {
    width: 64px;
  }
}

// 过渡动画
.fade-slide-enter-active,
.fade-slide-leave-active {
  transition: all 0.15s ease;
}

.fade-slide-enter-from,
.fade-slide-leave-to {
  opacity: 0;
  transform: translateX(-8px);
}

// Logo 区域 - 支持主题切换
.logo-area {
  height: 64px;
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 24px;
  border-bottom: 1px solid var(--color-border-default);
  background: var(--color-bg-overlay);
  backdrop-filter: blur(8px);
  transition: background-color var(--duration-normal), border-color var(--duration-normal);
  
  .logo-content {
    display: flex;
    align-items: center;
    gap: 8px;
    flex: 1;
    min-width: 0;
  }
  
  .logo-icon {
    font-size: 30px;
    color: var(--color-accent-blue);
    flex-shrink: 0;
  }
  
  .logo-text {
    font-size: 20px;
    font-weight: 700;
    letter-spacing: -0.5px;
    color: var(--color-text-primary);
    white-space: nowrap;
    overflow: hidden;
    transition: color var(--duration-normal);
  }
}

// AdsPower 风格折叠按钮 - 圆形
.collapse-toggle-btn {
  position: absolute;
  right: -12px;
  top: 132px; // 展开状态
  z-index: 10; // 降低 z-index，避免遮挡 Dialog
  
  width: 24px;
  height: 24px;
  display: flex;
  align-items: center;
  justify-content: center;
  
  background: var(--color-selected-bg);
  border: 1px solid var(--color-border-interactive);
  border-radius: 50%;
  color: var(--color-accent-blue);
  cursor: pointer;
  
  box-shadow: var(--shadow-sm);
  transition: all 0.15s ease;
  
  &:hover {
    background: var(--color-active-bg);
    box-shadow: var(--shadow-md);
  }
  
  &:active {
    transform: scale(0.95);
  }
  
  .el-icon {
    font-size: 12px;
  }
}

// 折叠状态下的折叠按钮位置
.sidebar.collapsed .collapse-toggle-btn {
  top: 108px; // 收缩状态
}

// 新建浏览器按钮 - 保持品牌渐变色
.new-browser-section {
  padding: 16px; // 匹配 p-4
  
  &.collapsed {
    padding: 8px;
    display: flex;
    justify-content: center;
  }
}

.new-browser-btn {
  width: 100%;
  padding: 12px 16px;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  background: linear-gradient(to right, var(--color-accent-blue), var(--color-accent-blue-hover));
  border: none;
  border-radius: 10px;
  color: white;
  font-size: 15px;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  box-shadow: 0 4px 12px rgba(37, 99, 235, 0.35);
  
  &:hover {
    filter: brightness(1.1);
    transform: translateY(-2px);
    box-shadow: 0 8px 20px rgba(37, 99, 235, 0.45);
  }
  
  &:active {
    transform: translateY(0);
  }
  
  // 图标样式 - 确保白色
  .btn-icon {
    font-size: 20px;
    color: white !important;
    line-height: 1;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    transition: transform 0.2s ease;
  }
  
  // 文字样式
  > span:not(.btn-icon) {
    color: white;
    line-height: 1;
  }
  
  &:hover .btn-icon {
    transform: scale(1.1);
  }
}

.new-browser-btn-mini {
  width: 40px;
  height: 40px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: linear-gradient(to right, var(--color-accent-blue), var(--color-accent-blue-hover));
  border: none;
  border-radius: 8px;
  color: white;
  cursor: pointer;
  transition: all 0.15s ease;
  box-shadow: 0 2px 8px rgba(37, 99, 235, 0.3);
  
  &:hover {
    filter: brightness(1.1);
    transform: scale(1.05);
    box-shadow: 0 4px 12px rgba(37, 99, 235, 0.4);
  }
  
  &:active {
    transform: scale(0.95);
  }
  
  .material-symbols-outlined {
    font-size: 20px;
  }
}

// 导航菜单 - 原型风格分组
.nav-menu {
  flex: 1;
  padding: 8px 12px;
  overflow-y: auto;
  overflow-x: hidden;
}

// 分组标题
.menu-group-title {
  padding: 12px 12px 8px;
  font-size: 11px;
  font-weight: 700;
  color: var(--color-text-tertiary);
  text-transform: uppercase;
  letter-spacing: 0.5px;
  margin-top: 8px;
  transition: color var(--duration-normal);
}

// 分隔符
.menu-separator {
  height: 1px;
  background: var(--color-border-default);
  margin: 8px 0;
  opacity: 0.6;
  transition: background-color var(--duration-normal);
}

.menu-item {
  display: flex;
  align-items: center;
  height: 44px;
  padding: 0 12px;
  margin-bottom: 4px;
  border-radius: 8px;
  cursor: pointer;
  position: relative;
  transition: all 0.15s ease;
  border: 1px solid transparent;
  
  .menu-icon {
    font-size: 22px;
    color: var(--color-text-tertiary);
    flex-shrink: 0;
    transition: color 0.15s;
  }
  
  .menu-title {
    margin-left: 12px;
    font-size: 14px;
    font-weight: 500;
    color: var(--color-text-secondary);
    white-space: nowrap;
    transition: color 0.15s;
  }
  
  &:hover {
    background-color: var(--color-hover-bg);
    
    .menu-icon,
    .menu-title {
      color: var(--color-text-primary);
    }
  }
  
  &.active {
    background-color: var(--color-selected-bg);
    border-color: var(--color-border-interactive);
    box-shadow: var(--shadow-xs);
    
    .menu-icon {
      color: var(--color-accent-blue);
    }
    
    .menu-title {
      color: var(--color-accent-blue);
      font-weight: 600;
    }
  }
  
  .menu-badge {
    margin-left: auto;
    
    :deep(.el-badge__content) {
      background-color: var(--color-accent-success);
      border: none;
      font-size: 10px;
      height: 16px;
      line-height: 16px;
      padding: 0 5px;
    }
  }
}

// 折叠状态下的菜单项
.sidebar.collapsed .menu-item {
  justify-content: center;
  padding: 0;
  
  .menu-icon {
    margin: 0;
  }
}

// 用户信息 - 支持主题切换
.user-profile {
  padding: 16px;
  border-top: 1px solid var(--color-border-default);
  background: var(--color-bg-overlay);
  display: flex;
  align-items: center;
  gap: 12px;
  cursor: pointer;
  transition: background-color var(--duration-normal), border-color var(--duration-normal);
  
  &:hover {
    background-color: var(--color-hover-bg);
  }
  
  .avatar {
    width: 40px;
    height: 40px;
    border-radius: 9999px;
    background: linear-gradient(135deg, var(--color-selected-bg) 0%, var(--color-bg-overlay) 100%);
    border: 1px solid var(--color-border-interactive);
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--color-accent-blue);
    font-weight: 700;
    font-size: 14px;
    flex-shrink: 0;
    box-shadow: var(--shadow-sm);
  }
  
  .user-info {
    flex: 1;
    min-width: 0;
    
    .username {
      color: var(--color-text-primary);
      font-weight: 700;
      font-size: 14px;
      white-space: nowrap;
      overflow: hidden;
      text-overflow: ellipsis;
      line-height: 1.4;
      transition: color var(--duration-normal);
    }
    
    .user-id {
      color: var(--color-text-tertiary);
      font-size: 12px;
      line-height: 1.4;
      transition: color var(--duration-normal);
    }
  }
  
  .settings-btn {
    width: 32px;
    height: 32px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: transparent;
    border: none;
    border-radius: 6px;
    color: var(--color-text-tertiary);
    cursor: pointer;
    transition: all 0.15s;
    
    &:hover {
      background: var(--color-hover-bg);
      color: var(--color-text-secondary);
    }
    
    .material-symbols-outlined {
      font-size: 20px;
    }
  }
}

// 折叠状态下的用户信息
.sidebar.collapsed .user-profile {
  justify-content: center;
  padding: 16px 8px;
}
</style>
