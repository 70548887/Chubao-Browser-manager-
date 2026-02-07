<script setup lang="ts">
/**
 * @description 自定义标题栏组件 - 桌面原生化
 * @author DeepAgent
 */

import { ref, onMounted } from 'vue'
import { useTheme } from '@/composables/useTheme'
import ThemeToggle from '@/components/common/ThemeToggle.vue'
import { useUIStore } from '@/stores/ui.store'
import { isTauri } from '@tauri-apps/api/core'

useTheme()
const uiStore = useUIStore()

// 通知按钮引用，用于计算下拉框位置
const notificationBtnRef = ref<HTMLButtonElement | null>(null)
const dropdownPosition = ref({ top: 0, right: 0 })

// 计算下拉框位置
const updateDropdownPosition = () => {
  if (notificationBtnRef.value) {
    const rect = notificationBtnRef.value.getBoundingClientRect()
    dropdownPosition.value = {
      top: rect.bottom + 8, // 按钮底部 + 8px 间距
      right: window.innerWidth - rect.right // 右对齐
    }
  }
}

// 检测是否在 Tauri 环境中
const isTauriEnv = ref(false)

// 窗口状态
const isMaximized = ref(false)

// ==================== 消息通知 ====================
interface Notification {
  id: string
  type: 'warning' | 'info' | 'success' | 'error'
  title: string
  content: string
  time: string
  isRead: boolean
}

const notifications = ref<Notification[]>([
  {
    id: '1',
    type: 'warning',
    title: '连接异常警告',
    content: '环境 ID: 8820991 (PayPal-Audit-03) 连接超时，请检查代理设置。',
    time: '10分钟前',
    isRead: false
  },
  {
    id: '2',
    type: 'info',
    title: '系统更新通知',
    content: 'DeepBrowser 内核已更新至 v112.0.5615.138，建议重启应用以获得最佳体验。',
    time: '2小时前',
    isRead: true
  },
  {
    id: '3',
    type: 'success',
    title: 'RPA 任务完成',
    content: '“TikTok 每日养号” 任务已完成，成功执行 5/5 个账号。',
    time: '5小时前',
    isRead: true
  }
])

const hasUnreadNotifications = ref(true)
const notificationDropdownVisible = ref(false)
const notificationDrawerVisible = ref(false) // 消息中心抽屉

const getNotificationIcon = (type: string) => {
  const icons = {
    warning: 'warning',
    info: 'update',
    success: 'check_circle',
    error: 'error'
  }
  return icons[type as keyof typeof icons] || 'notifications'
}

const getNotificationIconClass = (type: string) => {
  const classes = {
    warning: 'icon-warning',
    info: 'icon-info',
    success: 'icon-success',
    error: 'icon-error'
  }
  return classes[type as keyof typeof classes] || ''
}

const handleMarkAllRead = () => {
  notifications.value.forEach(n => n.isRead = true)
  hasUnreadNotifications.value = false
}

const handleNotificationClick = (notification: Notification) => {
  notification.isRead = true
  hasUnreadNotifications.value = notifications.value.some(n => !n.isRead)
  console.log('Notification clicked:', notification)
}

const handleViewAll = () => {
  notificationDropdownVisible.value = false
  notificationDrawerVisible.value = true
}

// 切换通知下拉框
const toggleNotificationDropdown = () => {
  if (!notificationDropdownVisible.value) {
    // 显示前先计算位置
    updateDropdownPosition()
  }
  notificationDropdownVisible.value = !notificationDropdownVisible.value
}

// 窗口控制
const minimize = async () => {
  console.log('minimize clicked, isTauri:', isTauriEnv.value)
  if (!isTauriEnv.value) {
    console.warn('Not in Tauri environment')
    return
  }
  try {
    const { getCurrentWindow } = await import('@tauri-apps/api/window')
    const win = getCurrentWindow()
    console.log('Window object:', win)
    await win.minimize()
    console.log('minimize success')
  } catch (e) {
    console.error('minimize error:', e)
  }
}

const toggleMaximize = async () => {
  console.log('toggleMaximize clicked, isTauri:', isTauriEnv.value)
  if (!isTauriEnv.value) return
  try {
    const { getCurrentWindow } = await import('@tauri-apps/api/window')
    const appWindow = getCurrentWindow()
    await appWindow.toggleMaximize()
    isMaximized.value = await appWindow.isMaximized()
    console.log('toggleMaximize success, isMaximized:', isMaximized.value)
  } catch (e) {
    console.error('toggleMaximize error:', e)
  }
}

const close = async () => {
  console.log('close clicked, isTauri:', isTauriEnv.value)
  if (!isTauriEnv.value) {
    console.log('在浏览器环境中，无法关闭窗口')
    return
  }
  try {
    const { getCurrentWindow } = await import('@tauri-apps/api/window')
    await getCurrentWindow().close()
  } catch (e) {
    console.error('close error:', e)
  }
}

// 初始化窗口状态
onMounted(async () => {
  isTauriEnv.value = isTauri()
  console.log('CustomTitlebar mounted, isTauri:', isTauriEnv.value, 'isTauri():', isTauriEnv.value)
  if (isTauriEnv.value) {
    try {
      const { getCurrentWindow } = await import('@tauri-apps/api/window')
      const appWindow = getCurrentWindow()
      isMaximized.value = await appWindow.isMaximized()
      console.log('Initial isMaximized:', isMaximized.value)
    } catch (e) {
      console.error('onMounted error:', e)
    }
  }
})
</script>

<template>
  <div class="custom-titlebar" data-tauri-drag-region>
    <!-- 左侧：Logo 和标题 -->
    <div class="titlebar-left">
      <div class="app-logo">
        <el-icon :size="18">
          <Monitor />
        </el-icon>
      </div>
      <div class="app-info">
        <span class="app-title">触宝指纹浏览器</span>
        <span class="app-version">v0.1.0</span>
      </div>
    </div>
    
    <!-- 右侧：通知、主题切换和窗口控制 -->
    <div class="titlebar-right">
      <!-- 发现新版本按钮，只在有更新时显示 -->
      <button 
        v-if="uiStore.hasUpdate" 
        class="titlebar-icon-btn update-btn" 
        title="发现新版本" 
        @click="uiStore.setUpdateDialogVisible(true)"
      >
        <span class="material-symbols-outlined">rocket_launch</span>
        <span class="update-badge"></span>
      </button>

      <!-- 通知按钮 -->
      <div class="notification-wrapper">
        <button 
          ref="notificationBtnRef"
          class="titlebar-icon-btn"
          title="通知"
          @click="toggleNotificationDropdown"
        >
          <span class="material-symbols-outlined">notifications</span>
          <span v-if="hasUnreadNotifications" class="notification-dot"></span>
        </button>
      </div>
        
      <!-- 通知下拉框 - 使用 Teleport 传送到 body，避免被其他元素遮挡 -->
      <Teleport to="body">
        <transition name="dropdown">
          <div 
            v-if="notificationDropdownVisible" 
            class="notification-dropdown-teleport"
            :style="{ top: dropdownPosition.top + 'px', right: dropdownPosition.right + 'px' }"
            @click.stop
          >
            <!-- 头部 -->
            <div class="dropdown-header">
              <h3 class="dropdown-title">消息通知</h3>
              <button class="mark-all-btn" @click="handleMarkAllRead">全部已读</button>
            </div>
            
            <!-- 通知列表 -->
            <div class="notification-list">
              <div
                v-for="notification in notifications"
                :key="notification.id"
                class="notification-item"
                :class="{ 'is-unread': !notification.isRead }"
                @click="handleNotificationClick(notification)"
              >
                <!-- 未读标记 -->
                <div v-if="!notification.isRead" class="unread-dot"></div>
                
                <div class="notification-content">
                  <!-- 图标 -->
                  <div 
                    class="notification-icon"
                    :class="getNotificationIconClass(notification.type)"
                  >
                    <span class="material-symbols-outlined">
                      {{ getNotificationIcon(notification.type) }}
                    </span>
                  </div>
                  
                  <!-- 文本 -->
                  <div class="notification-text">
                    <p class="notification-title">{{ notification.title }}</p>
                    <p class="notification-desc">{{ notification.content }}</p>
                    <p class="notification-time">{{ notification.time }}</p>
                  </div>
                </div>
              </div>
            </div>
            
            <!-- 底部 -->
            <div class="dropdown-footer">
              <button class="view-all-btn" @click="handleViewAll">
                查看所有通知
              </button>
            </div>
          </div>
        </transition>
      </Teleport>
      
      <!-- 主题切换 -->
      <div class="theme-toggle-wrapper">
        <ThemeToggle />
      </div>
      
      <!-- 分隔线 -->
      <div class="divider"></div>
      
      <!-- 窗口控制按钮 - Windows 11 风格 -->
      <div class="window-controls" @mousedown.stop @click.stop>
        <button 
          class="control-btn minimize" 
          @click.stop="minimize" 
          title="最小化"
          aria-label="最小化"
        >
          <svg width="12" height="12" viewBox="0 0 12 12">
            <rect width="10" height="1" x="1" y="5.5" fill="currentColor"/>
          </svg>
        </button>
        
        <button 
          class="control-btn maximize" 
          @click.stop="toggleMaximize" 
          :title="isMaximized ? '还原' : '最大化'"
          :aria-label="isMaximized ? '还原' : '最大化'"
        >
          <svg v-if="isMaximized" width="12" height="12" viewBox="0 0 12 12">
            <rect width="7" height="7" x="1.5" y="3.5" fill="none" stroke="currentColor"/>
            <polyline points="3.5,3.5 3.5,1.5 10.5,1.5 10.5,8.5 8.5,8.5" fill="none" stroke="currentColor"/>
          </svg>
          <svg v-else width="12" height="12" viewBox="0 0 12 12">
            <rect width="9" height="9" x="1.5" y="1.5" fill="none" stroke="currentColor"/>
          </svg>
        </button>
        
        <button 
          class="control-btn close" 
          @click.stop="close" 
          title="关闭"
          aria-label="关闭"
        >
          <svg width="12" height="12" viewBox="0 0 12 12">
            <path d="M1.5,1.5 L10.5,10.5 M10.5,1.5 L1.5,10.5" stroke="currentColor" stroke-width="1.2"/>
          </svg>
        </button>
      </div>
    </div>
  </div>

  <!-- 消息中心抽屉 -->
  <transition name="drawer">
    <div v-if="notificationDrawerVisible" class="notification-drawer-overlay" @click="notificationDrawerVisible = false">
      <div class="notification-drawer" @click.stop>
        <!-- 头部 -->
        <div class="drawer-header">
          <div>
            <h2 class="drawer-title">消息中心</h2>
            <p class="drawer-subtitle">您有 {{ notifications.filter(n => !n.isRead).length }} 条未读消息</p>
          </div>
          <div class="header-actions">
            <button class="action-btn" title="设置" @click.stop>
              <span class="material-symbols-outlined">settings</span>
            </button>
            <button class="action-btn" title="关闭" @click="notificationDrawerVisible = false">
              <span class="material-symbols-outlined">close</span>
            </button>
          </div>
        </div>

        <!-- 标签栏 -->
        <div class="drawer-tabs">
          <button class="tab-btn active">
            全部
          </button>
          <button class="tab-btn">
            未读
            <span class="tab-badge">{{ notifications.filter(n => !n.isRead).length }}</span>
          </button>
          <button class="tab-btn">
            告警
          </button>
          <button class="tab-btn">
            系统
          </button>
        </div>

        <!-- 消息列表 -->
        <div class="drawer-content">
          <!-- 今天 -->
          <div class="message-section">
            <div class="section-header">今天</div>
            
            <!-- 连接异常警告 -->
            <div class="message-item unread">
              <div class="unread-indicator"></div>
              <div class="message-content">
                <div class="message-icon icon-warning">
                  <span class="material-symbols-outlined">warning</span>
                </div>
                <div class="message-body">
                  <h4 class="message-title">连接异常警告</h4>
                  <p class="message-desc">环境 <span class="code">ID: 8820991</span> (PayPal-Audit-03) 连接超时，请检查代理设置。</p>
                  <span class="message-time">10分钟前</span>
                </div>
              </div>
              <div class="message-actions">
                <button class="msg-btn">查看详情</button>
                <button class="msg-btn primary">检查代理</button>
              </div>
            </div>

            <!-- DeepBrowser 内核更新 -->
            <div class="message-item unread">
              <div class="unread-indicator"></div>
              <div class="message-content">
                <div class="message-icon icon-info">
                  <span class="material-symbols-outlined">system_update</span>
                </div>
                <div class="message-body">
                  <h4 class="message-title">DeepBrowser 内核更新</h4>
                  <p class="message-desc">内核已更新至 v112.0.5615.138。包含重要的安全补丁和指纹隐匿性提升。</p>
                  <span class="message-time">2小时前</span>
                </div>
              </div>
            </div>

            <!-- RPA 任务完成 -->
            <div class="message-item">
              <div class="message-content">
                <div class="message-icon icon-success">
                  <span class="material-symbols-outlined">check_circle</span>
                </div>
                <div class="message-body">
                  <h4 class="message-title">RPA 任务完成</h4>
                  <p class="message-desc">“TikTok 每日养号” 任务已完成，成功执行 5/5 个账号。</p>
                  <span class="message-time">5小时前</span>
                </div>
              </div>
            </div>
          </div>

          <!-- 昨天 -->
          <div class="message-section">
            <div class="section-header">昨天</div>
            
            <!-- 新设备登录提醒 -->
            <div class="message-item">
              <div class="message-content">
                <div class="message-icon icon-account">
                  <span class="material-symbols-outlined">account_circle</span>
                </div>
                <div class="message-body">
                  <h4 class="message-title">新设备登录提醒</h4>
                  <p class="message-desc">您的账号在新的设备 (MacBook Pro) 上登录。IP: 192.168.1.5</p>
                  <span class="message-time">昨天 18:23</span>
                </div>
              </div>
            </div>

            <!-- 本周运行周报 -->
            <div class="message-item">
              <div class="message-content">
                <div class="message-icon icon-report">
                  <span class="material-symbols-outlined">receipt_long</span>
                </div>
                <div class="message-body">
                  <h4 class="message-title">本周运行周报</h4>
                  <p class="message-desc">您的环境运行效率提升了 12%。点击查看详细数据分析。</p>
                  <span class="message-time">昨天 09:00</span>
                </div>
              </div>
            </div>
          </div>
        </div>

        <!-- 底部 -->
        <div class="drawer-footer">
          <button class="footer-btn" @click="handleMarkAllRead">
            <span class="material-symbols-outlined">done_all</span>
            全部已读
          </button>
          <button class="footer-btn primary">
            查看历史消息
          </button>
        </div>
      </div>
    </div>
  </transition>
</template>

<style scoped lang="scss">
.custom-titlebar {
  height: 40px; // 增加高度到 40px
  background: var(--color-bg-secondary);
  border-bottom: 1px solid var(--color-border-default);
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0 0 0 12px;
  user-select: none;
  -webkit-user-select: none;
  flex-shrink: 0;
  position: relative;
  z-index: 9900; // 提高到9900，高于消息中心抽屉，确保子元素（通知下拉框）也在高层级
  transition: background-color var(--duration-normal), border-color var(--duration-normal);
  
  // 顶部微妙高光
  &::before {
    content: '';
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    height: 1px;
    background: linear-gradient(
      90deg,
      transparent 0%,
      rgba(255, 255, 255, 0.06) 50%,
      transparent 100%
    );
  }
}

.titlebar-left {
  display: flex;
  align-items: center;
  gap: 8px;
  pointer-events: none;
  
  .app-logo {
    width: 24px;
    height: 24px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: linear-gradient(135deg, var(--color-accent-blue) 0%, #1d4ed8 100%);
    border-radius: 4px;
    color: white;
    
    .el-icon {
      font-size: 14px;
    }
  }
  
  .app-info {
    display: flex;
    align-items: baseline;
    gap: 8px;
  }
  
  .app-title {
    color: var(--color-text-primary);
    font-size: 14px;
    font-weight: 600;
    letter-spacing: 0.3px;
    transition: color var(--duration-normal);
  }
  
  .app-version {
    color: var(--color-text-tertiary);
    font-size: 12px;
    font-weight: 400;
    transition: color var(--duration-normal);
  }
}

.titlebar-right {
  display: flex;
  align-items: center;
  height: 100%;
}

// 标题栏图标按钮（通知等）
.titlebar-icon-btn {
  position: relative;
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
  pointer-events: auto;
  transition: all 0.15s;
  margin-right: 4px;
  
  .material-symbols-outlined {
    font-size: 20px;
  }
  
  &:hover {
    background: var(--color-hover-bg);
    color: var(--color-text-primary);
  }
  
  &:active {
    transform: scale(0.95);
  }
}

// 消息通知样式
.notification-wrapper {
  position: relative;
}

// 原来的下拉框样式（保留以防万一）
.notification-dropdown {
  position: absolute;
  right: 0;
  top: calc(100% + 8px);
  width: 320px;
  background: var(--color-bg-secondary);
  border: 1px solid var(--color-border-default);
  border-radius: 12px;
  box-shadow: var(--shadow-lg);
  z-index: 10000;
  overflow: hidden;
  transition: background-color var(--duration-normal), border-color var(--duration-normal);
}

.dropdown-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 16px;
  border-bottom: 1px solid var(--color-border-subtle);
  background: var(--color-bg-secondary);
  transition: background-color var(--duration-normal), border-color var(--duration-normal);
}

.dropdown-title {
  font-size: 15px;
  font-weight: 600;
  color: var(--color-text-primary);
  margin: 0;
  transition: color var(--duration-normal);
}

.mark-all-btn {
  padding: 4px 8px;
  background: transparent;
  border: none;
  color: var(--color-accent-blue);
  font-size: 12px;
  font-weight: 500;
  cursor: pointer;
  border-radius: 4px;
  transition: all 0.15s;
  
  &:hover {
    background: var(--color-selected-bg);
  }
}

.notification-list {
  max-height: 320px;
  overflow-y: auto;
  
  &::-webkit-scrollbar {
    width: 6px;
  }
  
  &::-webkit-scrollbar-track {
    background: transparent;
  }
  
  &::-webkit-scrollbar-thumb {
    background: var(--color-border-strong);
    border-radius: 3px;
  }
  
  &::-webkit-scrollbar-thumb:hover {
    background: var(--color-text-tertiary);
  }
}

.notification-item {
  position: relative;
  padding: 12px;
  border-bottom: 1px solid var(--color-border-subtle);
  cursor: pointer;
  transition: background 0.15s;
  
  &:last-child {
    border-bottom: none;
  }
  
  &:hover {
    background: var(--color-hover-bg);
  }
}

.unread-dot {
  position: absolute;
  top: 16px;
  right: 12px;
  width: 8px;
  height: 8px;
  background: var(--color-accent-blue);
  border-radius: 50%;
}

.notification-content {
  display: flex;
  gap: 12px;
}

.notification-icon {
  width: 32px;
  height: 32px;
  flex-shrink: 0;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  
  .material-symbols-outlined {
    font-size: 18px;
  }
  
  &.icon-warning {
    background: #fef2f2;
    color: var(--color-accent-danger);
  }
  
  &.icon-info {
    background: var(--color-selected-bg);
    color: var(--color-accent-blue);
  }
  
  &.icon-success {
    background: #d1fae5;
    color: var(--color-accent-success);
  }
  
  &.icon-error {
    background: #fef2f2;
    color: var(--color-accent-danger);
  }
}

.notification-text {
  flex: 1;
  min-width: 0;
}

.notification-title {
  font-size: 13px;
  font-weight: 500;
  color: var(--color-text-primary);
  margin: 0 0 4px 0;
  transition: color var(--duration-normal);
}

.notification-desc {
  font-size: 12px;
  color: var(--color-text-tertiary);
  margin: 0 0 4px 0;
  line-height: 1.5;
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
  transition: color var(--duration-normal);
}

.notification-time {
  font-size: 11px;
  color: var(--color-text-tertiary);
  margin: 0;
  transition: color var(--duration-normal);
}

.dropdown-footer {
  padding: 8px;
  background: var(--color-bg-overlay);
  border-top: 1px solid var(--color-border-subtle);
  text-align: center;
  transition: background-color var(--duration-normal), border-color var(--duration-normal);
}

.view-all-btn {
  width: 100%;
  padding: 6px;
  background: transparent;
  border: none;
  color: var(--color-text-tertiary);
  font-size: 12px;
  font-weight: 500;
  cursor: pointer;
  border-radius: 6px;
  transition: all 0.15s;
  
  &:hover {
    background: var(--color-bg-secondary);
    color: var(--color-accent-blue);
  }
}

// 下拉框动画
.dropdown-enter-active,
.dropdown-leave-active {
  transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
}

.dropdown-enter-from,
.dropdown-leave-to {
  opacity: 0;
  transform: translateY(-8px) scale(0.95);
}

// 通知红点
.notification-dot {
  position: absolute;
  top: 4px;
  right: 4px;
  width: 8px;
  height: 8px;
  background: var(--color-status-error);
  border-radius: 50%;
  border: 2px solid var(--color-bg-secondary);
}

// 升级提示红点
.update-badge {
  position: absolute;
  top: 4px;
  right: 4px;
  width: 8px;
  height: 8px;
  background: var(--color-accent-blue);
  border-radius: 50%;
  border: 2px solid var(--color-bg-secondary);
  animation: pulse-blue 2s infinite;
}

@keyframes pulse-blue {
  0% { transform: scale(0.95); box-shadow: 0 0 0 0 rgba(37, 99, 235, 0.4); }
  70% { transform: scale(1); box-shadow: 0 0 0 6px rgba(37, 99, 235, 0); }
  100% { transform: scale(0.95); box-shadow: 0 0 0 0 rgba(37, 99, 235, 0); }
}

.update-btn {
  color: var(--color-accent-blue);
  
  &:hover {
    color: #1d4ed8;
    background: var(--color-selected-bg);
  }
}

.theme-toggle-wrapper {
  pointer-events: auto;
  padding: 0 8px;
  
  :deep(.el-button) {
    background-color: transparent;
    border: 1px solid var(--color-border-default);
    color: var(--color-text-tertiary);
    width: 26px;
    height: 26px;
    padding: 0;
    
    &:hover {
      background-color: var(--color-hover-bg);
      border-color: var(--color-border-strong);
      color: var(--color-text-primary);
    }
  }
}

.divider {
  width: 1px;
  height: 18px;
  background-color: var(--color-border-default);
  margin: 0 4px;
  transition: background-color var(--duration-normal);
}

.window-controls {
  display: flex;
  align-items: center;
  height: 100%;
  pointer-events: auto;
}

.control-btn {
  width: 46px;
  height: 100%;
  border: none;
  background: transparent;
  color: var(--color-text-tertiary);
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.1s ease;
  
  svg {
    width: 12px;
    height: 12px;
    transition: transform 0.1s;
  }
  
  &:hover {
    background-color: var(--color-hover-bg);
    color: var(--color-text-primary);
    
    svg {
      transform: scale(1.1);
    }
  }
  
  &:active {
    background-color: var(--color-active-bg);
    
    svg {
      transform: scale(0.95);
    }
  }
  
  // 关闭按钮特殊样式 - Windows 11 风格
  &.close {
    &:hover {
      background-color: #c42b1c;
      color: white;
    }
    
    &:active {
      background-color: #b22a1c;
    }
  }
}

// ==================== 消息中心抽屉样式 ====================
.notification-drawer-overlay {
  position: fixed;
  top: 0;
  right: 0;
  bottom: 0;
  left: 0;
  background: rgba(15, 23, 42, 0.15);
  backdrop-filter: blur(1px);
  z-index: 9997; // 略低于通知下拉框，但仍高于其他内容
  display: flex;
  justify-content: flex-end;
}

.notification-drawer {
  width: 400px;
  height: 100%;
  background: var(--color-bg-secondary);
  box-shadow: var(--shadow-xl);
  border-left: 1px solid var(--color-border-default);
  display: flex;
  flex-direction: column;
  animation: slideInRight 0.3s ease-out;
  transition: background-color var(--duration-normal), border-color var(--duration-normal);
}

@keyframes slideInRight {
  from {
    transform: translateX(100%);
  }
  to {
    transform: translateX(0);
  }
}

// 抽屉头部
.drawer-header {
  padding: 20px;
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  border-bottom: 1px solid var(--color-border-default);
  background: var(--color-bg-overlay);
  transition: background-color var(--duration-normal), border-color var(--duration-normal);
}

.drawer-title {
  font-size: 16px;
  font-weight: 700;
  color: var(--color-text-primary);
  margin: 0 0 4px 0;
  transition: color var(--duration-normal);
}

.drawer-subtitle {
  font-size: 12px;
  color: var(--color-text-tertiary);
  margin: 0;
  transition: color var(--duration-normal);
}

.header-actions {
  display: flex;
  gap: 4px;
}

.action-btn {
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

  .material-symbols-outlined {
    font-size: 20px;
  }

  &:hover {
    background: var(--color-bg-secondary);
    color: var(--color-text-primary);
  }
}

// 标签栏
.drawer-tabs {
  padding: 0 20px;
  display: flex;
  gap: 24px;
  border-bottom: 1px solid var(--color-border-default);
  background: var(--color-bg-secondary);
  transition: background-color var(--duration-normal), border-color var(--duration-normal);
}

.tab-btn {
  position: relative;
  padding: 12px 0;
  background: transparent;
  border: none;
  border-bottom: 2px solid transparent;
  color: var(--color-text-tertiary);
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.15s;
  display: flex;
  align-items: center;
  gap: 6px;

  &:hover {
    color: var(--color-text-primary);
  }

  &.active {
    color: var(--color-accent-blue);
    border-bottom-color: var(--color-accent-blue);
  }
}

.tab-badge {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  min-width: 18px;
  height: 18px;
  padding: 0 6px;
  background: #fef2f2;
  color: var(--color-status-error);
  border-radius: 9px;
  font-size: 11px;
  font-weight: 600;
}

// 消息列表内容
.drawer-content {
  flex: 1;
  overflow-y: auto;
  background: var(--color-bg-secondary);
  transition: background-color var(--duration-normal);

  &::-webkit-scrollbar {
    width: 6px;
  }

  &::-webkit-scrollbar-track {
    background: transparent;
  }

  &::-webkit-scrollbar-thumb {
    background: var(--color-border-strong);
    border-radius: 3px;

    &:hover {
      background: var(--color-text-tertiary);
    }
  }
}

.message-section {
  border-bottom: 1px solid var(--color-border-subtle);

  &:last-child {
    border-bottom: none;
  }
}

.section-header {
  padding: 12px 20px;
  background: var(--color-bg-overlay);
  border-bottom: 1px solid var(--color-border-default);
  font-size: 12px;
  font-weight: 600;
  color: var(--color-text-tertiary);
  text-transform: uppercase;
  letter-spacing: 0.5px;
  position: sticky;
  top: 0;
  z-index: 1;
  backdrop-filter: blur(8px);
  transition: background-color var(--duration-normal), border-color var(--duration-normal), color var(--duration-normal);
}

.message-item {
  padding: 16px 20px;
  border-bottom: 1px solid var(--color-border-subtle);
  cursor: pointer;
  transition: all 0.15s;
  position: relative;

  &:hover {
    background: var(--color-hover-bg);
  }

  &.unread {
    background: var(--color-selected-bg);

    &:hover {
      background: var(--color-active-bg);
    }
  }

  &:last-child {
    border-bottom: none;
  }
}

.unread-indicator {
  position: absolute;
  right: 16px;
  top: 20px;
  width: 8px;
  height: 8px;
  background: var(--color-status-error);
  border-radius: 50%;
  border: 2px solid var(--color-bg-secondary);
}

.message-content {
  display: flex;
  gap: 12px;
}

.message-icon {
  width: 40px;
  height: 40px;
  flex-shrink: 0;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;

  .material-symbols-outlined {
    font-size: 20px;
  }

  &.icon-warning {
    background: #fef2f2;
    color: var(--color-status-error);
  }

  &.icon-info {
    background: var(--color-selected-bg);
    color: var(--color-accent-blue);
  }

  &.icon-success {
    background: #f0fdf4;
    color: var(--color-accent-success);
  }

  &.icon-account {
    background: #faf5ff;
    color: #a855f7;
  }

  &.icon-report {
    background: var(--color-bg-primary);
    color: var(--color-text-tertiary);
  }
}

.message-body {
  flex: 1;
  min-width: 0;
}

.message-title {
  font-size: 14px;
  font-weight: 600;
  color: var(--color-text-primary);
  margin: 0 0 4px 0;
  line-height: 1.4;
  transition: color var(--duration-normal);
}

.message-desc {
  font-size: 12px;
  color: var(--color-text-tertiary);
  line-height: 1.5;
  margin: 0 0 6px 0;
  transition: color var(--duration-normal);

  .code {
    font-family: 'Courier New', monospace;
    color: var(--color-text-primary);
    font-weight: 600;
  }
}

.message-time {
  font-size: 10px;
  color: var(--color-text-tertiary);
  transition: color var(--duration-normal);
}

.message-actions {
  display: flex;
  gap: 8px;
  margin-top: 12px;
  padding-left: 52px;
}

.msg-btn {
  padding: 6px 12px;
  font-size: 12px;
  font-weight: 500;
  border: 1px solid var(--color-border-default);
  border-radius: 6px;
  background: var(--color-bg-secondary);
  color: var(--color-text-secondary);
  cursor: pointer;
  transition: all 0.15s;

  &:hover {
    background: var(--color-hover-bg);
    border-color: var(--color-border-strong);
  }

  &.primary {
    background: var(--color-selected-bg);
    border-color: var(--color-border-interactive);
    color: var(--color-accent-blue);

    &:hover {
      background: var(--color-active-bg);
      border-color: var(--color-accent-blue);
    }
  }
}

// 抽屉底部
.drawer-footer {
  padding: 16px 20px;
  border-top: 1px solid var(--color-border-default);
  background: var(--color-bg-overlay);
  display: flex;
  justify-content: space-between;
  gap: 12px;
  transition: background-color var(--duration-normal), border-color var(--duration-normal);
}

.footer-btn {
  flex: 1;
  padding: 8px 12px;
  font-size: 12px;
  font-weight: 500;
  border: 1px solid var(--color-border-default);
  border-radius: 6px;
  background: var(--color-bg-secondary);
  color: var(--color-text-tertiary);
  cursor: pointer;
  transition: all 0.15s;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 6px;

  .material-symbols-outlined {
    font-size: 16px;
  }

  &:hover {
    background: var(--color-hover-bg);
    border-color: var(--color-border-strong);
    color: var(--color-text-primary);
  }

  &.primary {
    background: transparent;
    border: none;
    color: var(--color-accent-blue);
    font-weight: 600;

    &:hover {
      color: #1d4ed8;
    }
  }
}

// 抽屉动画
.drawer-enter-active,
.drawer-leave-active {
  transition: all 0.3s ease;
}

.drawer-enter-from .notification-drawer,
.drawer-leave-to .notification-drawer {
  transform: translateX(100%);
}

.drawer-enter-from,
.drawer-leave-to {
  opacity: 0;
}
</style>

<!-- 全局样式 - 用于 Teleport 传送到 body 的通知下拉框 -->
<style lang="scss">
.notification-dropdown-teleport {
  position: fixed;
  width: 320px;
  background: var(--color-bg-secondary);
  border: 1px solid var(--color-border-default);
  border-radius: 12px;
  box-shadow: var(--shadow-lg);
  z-index: 99999; // 最高层级，确保不被任何元素遮挡
  overflow: hidden;
  
  .dropdown-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 16px;
    border-bottom: 1px solid var(--color-border-subtle);
    background: var(--color-bg-secondary);
  }
  
  .dropdown-title {
    font-size: 15px;
    font-weight: 600;
    color: var(--color-text-primary);
    margin: 0;
  }
  
  .mark-all-btn {
    font-size: 13px;
    color: var(--color-primary);
    background: none;
    border: none;
    cursor: pointer;
    padding: 4px 8px;
    border-radius: 4px;
    
    &:hover {
      background: var(--color-hover-bg);
    }
  }
  
  .notification-list {
    max-height: 400px;
    overflow-y: auto;
  }
  
  .notification-item {
    display: flex;
    align-items: flex-start;
    padding: 12px 16px;
    cursor: pointer;
    position: relative;
    
    &:hover {
      background: var(--color-hover-bg);
    }
    
    &.is-unread {
      background: var(--color-bg-tertiary);
    }
  }
  
  .unread-dot {
    position: absolute;
    left: 8px;
    top: 50%;
    transform: translateY(-50%);
    width: 6px;
    height: 6px;
    border-radius: 50%;
    background: var(--color-primary);
  }
  
  .notification-content {
    display: flex;
    gap: 12px;
    flex: 1;
  }
  
  .notification-icon {
    width: 36px;
    height: 36px;
    border-radius: 8px;
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
    
    .material-symbols-outlined {
      font-size: 20px;
    }
    
    &.icon-warning {
      background: rgba(245, 158, 11, 0.1);
      color: #f59e0b;
    }
    
    &.icon-info {
      background: rgba(59, 130, 246, 0.1);
      color: #3b82f6;
    }
    
    &.icon-success {
      background: rgba(34, 197, 94, 0.1);
      color: #22c55e;
    }
    
    &.icon-error {
      background: rgba(239, 68, 68, 0.1);
      color: #ef4444;
    }
  }
  
  .notification-text {
    flex: 1;
    min-width: 0;
  }
  
  .notification-title {
    font-size: 14px;
    font-weight: 500;
    color: var(--color-text-primary);
    margin: 0 0 4px 0;
  }
  
  .notification-desc {
    font-size: 13px;
    color: var(--color-text-secondary);
    margin: 0 0 4px 0;
    line-height: 1.4;
    overflow: hidden;
    text-overflow: ellipsis;
    display: -webkit-box;
    -webkit-line-clamp: 2;
    -webkit-box-orient: vertical;
  }
  
  .notification-time {
    font-size: 12px;
    color: var(--color-text-tertiary);
    margin: 0;
  }
  
  .dropdown-footer {
    padding: 12px 16px;
    border-top: 1px solid var(--color-border-subtle);
    text-align: center;
    background: var(--color-bg-secondary);
  }
  
  .view-all-btn {
    font-size: 13px;
    color: var(--color-text-secondary);
    background: none;
    border: none;
    cursor: pointer;
    padding: 8px 16px;
    border-radius: 6px;
    width: 100%;
    
    &:hover {
      background: var(--color-hover-bg);
      color: var(--color-text-primary);
    }
  }
}

// 下拉框动画
.dropdown-enter-active,
.dropdown-leave-active {
  transition: all 0.2s ease;
}

.dropdown-enter-from,
.dropdown-leave-to {
  opacity: 0;
  transform: translateY(-8px);
}
</style>
