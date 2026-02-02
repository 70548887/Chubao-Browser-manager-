<script setup lang="ts">
/**
 * @description 页面头部组件（标题 + 通知横幅）
 */

interface Props {
  title: string
  showNotification?: boolean
  notificationText?: string
  notificationButtonText?: string
}

withDefaults(defineProps<Props>(), {
  showNotification: true,
  notificationText: '为了您的账号安全，建议您尽快前往个人设置绑定身份验证器。异常登录时，可通过身份验证器验证身份，保障您的账号安全。',
  notificationButtonText: '绑定验证器'
})

const emit = defineEmits<{
  (e: 'notificationAction'): void
}>()
</script>

<template>
  <div class="page-header-wrapper">
    <!-- 页面标题 -->
    <div class="page-header">
      <h1 class="page-title">{{ title }}</h1>
    </div>
    
    <!-- 通知横幅 -->
    <div v-if="showNotification" class="page-notification banner-warning">
      <div class="notification-icon-wrapper banner-icon">
        <el-icon><InfoFilled /></el-icon>
      </div>
      <div class="notification-content banner-content">
        <div class="banner-text">
          {{ notificationText }}
        </div>
      </div>
      <button class="upgrade-btn banner-action" @click="emit('notificationAction')">
        <span>{{ notificationButtonText }}</span>
      </button>
    </div>
  </div>
</template>

<style scoped lang="scss">
.page-header-wrapper {
  background: white;
}

// 页面标题
.page-header {
  padding: var(--spacing-lg) var(--spacing-xl);
  border-bottom: 1px solid #E5E5E5;
  
  .page-title {
    font-size: 20px;
    font-weight: var(--font-semibold);
    color: var(--color-text-primary);
    margin: 0;
  }
}

// 通知横幅
.page-notification {
  display: flex;
  align-items: center;
  padding: var(--spacing-md) var(--spacing-xl);
  background: linear-gradient(90deg, #FFF7E6 0%, #FFE7BA 100%);
  border-bottom: 1px solid #E5E5E5;
  gap: var(--spacing-md);
  
  .notification-icon-wrapper {
    width: 28px;
    height: 28px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: #FF9800;
    border-radius: var(--radius-full);
    color: white;
    flex-shrink: 0;
    
    .el-icon {
      font-size: 14px;
    }
  }
  
  .notification-content {
    flex: 1;
    
    .banner-text {
      color: var(--color-text-secondary);
      font-size: var(--text-sm);
      line-height: 1.5;
    }
  }
  
  .upgrade-btn {
    display: flex;
    align-items: center;
    gap: var(--spacing-xs);
    height: 28px;
    padding: 0 var(--spacing-md);
    background: #FF9800;
    border: none;
    border-radius: var(--radius-md);
    color: white;
    font-size: var(--text-sm);
    font-weight: var(--font-medium);
    cursor: pointer;
    transition: all var(--duration-fast);
    white-space: nowrap;
    
    &:hover {
      background: #F57C00;
      transform: translateY(-1px);
      box-shadow: 0 2px 8px rgba(255, 152, 0, 0.3);
    }
    
    &:active {
      transform: translateY(0);
    }
  }
}
</style>
