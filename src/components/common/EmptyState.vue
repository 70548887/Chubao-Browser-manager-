<script setup lang="ts">
/**
 * @description 空状态组件 - 桌面原生化
 * @author DeepAgent
 */

interface Props {
  /** 图标 */
  icon?: string
  /** 标题 */
  title?: string
  /** 描述 */
  description?: string
  /** 操作按钮文字 */
  actionText?: string
  /** 图片路径 */
  image?: string
  /** 尺寸 */
  size?: 'sm' | 'md' | 'lg'
}

withDefaults(defineProps<Props>(), {
  icon: 'Files',
  title: '暂无数据',
  description: '',
  size: 'md',
})

const emit = defineEmits<{
  (e: 'action'): void
}>()

const sizeConfig = {
  sm: { icon: 40, title: 'var(--text-base)', padding: 'var(--spacing-lg)' },
  md: { icon: 56, title: 'var(--text-lg)', padding: 'var(--spacing-xl)' },
  lg: { icon: 72, title: 'var(--text-xl)', padding: 'var(--spacing-2xl)' },
}
</script>

<template>
  <div class="empty-state" :class="[`empty-state--${size}`]">
    <!-- 图标或图片 -->
    <div class="empty-state__visual">
      <img v-if="image" :src="image" class="empty-state__image" alt="No data" />
      <div v-else class="empty-state__icon-wrapper">
        <el-icon 
          class="empty-state__icon" 
          :size="sizeConfig[size].icon"
        >
          <component :is="icon" />
        </el-icon>
      </div>
    </div>
    
    <!-- 文字内容 -->
    <div class="empty-state__content">
      <h3 v-if="title" class="empty-state__title">{{ title }}</h3>
      <p v-if="description" class="empty-state__description">{{ description }}</p>
      <slot name="description"></slot>
    </div>
    
    <!-- 操作按钮 -->
    <div v-if="actionText || $slots.action" class="empty-state__action">
      <slot name="action">
        <button class="action-btn" @click="emit('action')">
          <el-icon><Plus /></el-icon>
          <span>{{ actionText }}</span>
        </button>
      </slot>
    </div>
    
    <!-- 额外内容 -->
    <slot></slot>
  </div>
</template>

<style scoped lang="scss">
.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  text-align: center;
  animation: fade-in var(--duration-normal) var(--ease-out-quart);
  
  &--sm {
    padding: var(--spacing-lg) 0;
    gap: var(--spacing-md);
  }
  
  &--md {
    padding: var(--spacing-xl) 0;
    gap: var(--spacing-lg);
  }
  
  &--lg {
    padding: var(--spacing-2xl) 0;
    gap: var(--spacing-xl);
  }
}

@keyframes fade-in {
  from {
    opacity: 0;
    transform: translateY(10px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

.empty-state__visual {
  position: relative;
}

.empty-state__icon-wrapper {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 100px;
  height: 100px;
  background: linear-gradient(
    135deg,
    rgba(0, 120, 212, 0.08) 0%,
    rgba(0, 120, 212, 0.02) 100%
  );
  border-radius: var(--radius-full);
  position: relative;
  
  // 外层装饰环
  &::before {
    content: '';
    position: absolute;
    inset: -8px;
    border: 2px dashed var(--color-border-default);
    border-radius: var(--radius-full);
    opacity: 0.5;
  }
}

.empty-state__icon {
  color: var(--color-text-tertiary);
  opacity: 0.6;
}

.empty-state__image {
  max-width: 200px;
  max-height: 150px;
  object-fit: contain;
  opacity: 0.8;
}

.empty-state__content {
  max-width: 320px;
}

.empty-state__title {
  font-size: var(--text-lg);
  font-weight: var(--font-semibold);
  color: var(--color-text-primary);
  margin: 0 0 var(--spacing-sm) 0;
}

.empty-state__description {
  font-size: var(--text-base);
  color: var(--color-text-tertiary);
  margin: 0;
  line-height: 1.6;
}

.empty-state__action {
  margin-top: var(--spacing-sm);
}

.action-btn {
  display: inline-flex;
  align-items: center;
  gap: var(--spacing-sm);
  height: 36px;
  padding: 0 var(--spacing-xl);
  background: var(--color-accent-blue);
  border: none;
  border-radius: var(--radius-md);
  color: white;
  font-size: var(--text-md);
  font-weight: var(--font-medium);
  cursor: pointer;
  transition: all var(--duration-fast) var(--ease-out-quart);
  
  .el-icon {
    font-size: 16px;
  }
  
  &:hover {
    background: var(--color-accent-blue-hover);
    transform: translateY(-2px);
    box-shadow: var(--shadow-md);
  }
  
  &:active {
    transform: translateY(0);
  }
  
  // 焦点环
  &:focus-visible {
    outline: 2px solid var(--color-accent-blue);
    outline-offset: 2px;
  }
}

// 尺寸调整
.empty-state--sm {
  .empty-state__icon-wrapper {
    width: 64px;
    height: 64px;
    
    &::before {
      inset: -6px;
    }
  }
  
  .empty-state__title {
    font-size: var(--text-base);
  }
  
  .empty-state__description {
    font-size: var(--text-sm);
  }
  
  .action-btn {
    height: 32px;
    font-size: var(--text-sm);
    padding: 0 var(--spacing-lg);
  }
}

.empty-state--lg {
  .empty-state__icon-wrapper {
    width: 120px;
    height: 120px;
    
    &::before {
      inset: -12px;
    }
  }
  
  .empty-state__title {
    font-size: var(--text-xl);
  }
  
  .action-btn {
    height: 40px;
    font-size: var(--text-lg);
    padding: 0 var(--spacing-2xl);
  }
}
</style>
