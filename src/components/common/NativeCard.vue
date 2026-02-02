<script setup lang="ts">
/**
 * @description 通用卡片组件 - 桌面原生化
 * @author DeepAgent
 */

interface Props {
  /** 卡片标题 */
  title?: string
  /** 卡片副标题 */
  subtitle?: string
  /** 是否显示头部 */
  showHeader?: boolean
  /** 是否可折叠 */
  collapsible?: boolean
  /** 是否默认折叠 */
  defaultCollapsed?: boolean
  /** 是否有边框 */
  bordered?: boolean
  /** 是否有阴影 */
  shadow?: boolean | 'hover'
  /** 是否有内边距 */
  padded?: boolean
  /** 卡片尺寸 */
  size?: 'sm' | 'md' | 'lg'
}

const props = withDefaults(defineProps<Props>(), {
  showHeader: true,
  collapsible: false,
  defaultCollapsed: false,
  bordered: true,
  shadow: false,
  padded: true,
  size: 'md',
})

const isCollapsed = ref(props.defaultCollapsed)

const toggleCollapse = () => {
  if (props.collapsible) {
    isCollapsed.value = !isCollapsed.value
  }
}

import { ref } from 'vue'
</script>

<template>
  <div 
    class="native-card"
    :class="[
      `native-card--${size}`,
      {
        'native-card--bordered': bordered,
        'native-card--shadow': shadow === true,
        'native-card--shadow-hover': shadow === 'hover',
        'native-card--collapsed': isCollapsed,
      }
    ]"
  >
    <div 
      v-if="showHeader && (title || $slots.header)"
      class="native-card__header"
      :class="{ 'is-clickable': collapsible }"
      @click="toggleCollapse"
    >
      <div class="native-card__header-content">
        <slot name="header">
          <div class="native-card__title-group">
            <h3 v-if="title" class="native-card__title">{{ title }}</h3>
            <p v-if="subtitle" class="native-card__subtitle">{{ subtitle }}</p>
          </div>
        </slot>
      </div>
      
      <div class="native-card__header-actions">
        <slot name="header-actions"></slot>
        <button v-if="collapsible" class="collapse-btn" type="button">
          <el-icon :class="{ 'is-rotated': !isCollapsed }">
            <ArrowDown />
          </el-icon>
        </button>
      </div>
    </div>
    
    <transition name="collapse">
      <div v-show="!isCollapsed" class="native-card__body" :class="{ 'native-card__body--padded': padded }">
        <slot></slot>
      </div>
    </transition>
    
    <div v-if="$slots.footer" class="native-card__footer">
      <slot name="footer"></slot>
    </div>
  </div>
</template>

<style scoped lang="scss">
.native-card {
  background-color: var(--color-bg-elevated);
  border-radius: var(--radius-lg);
  transition: all var(--duration-fast) var(--ease-out-quart);
  
  &--bordered {
    border: 1px solid var(--color-border-default);
  }
  
  &--shadow {
    box-shadow: var(--shadow-md);
  }
  
  &--shadow-hover {
    &:hover {
      box-shadow: var(--shadow-md);
      border-color: var(--color-border-strong);
    }
  }
  
  // 尺寸
  &--sm {
    .native-card__header {
      padding: var(--spacing-sm) var(--spacing-md);
    }
    .native-card__body--padded {
      padding: var(--spacing-md);
    }
    .native-card__title {
      font-size: var(--text-base);
    }
  }
  
  &--md {
    .native-card__header {
      padding: var(--spacing-md) var(--spacing-lg);
    }
    .native-card__body--padded {
      padding: var(--spacing-lg);
    }
  }
  
  &--lg {
    .native-card__header {
      padding: var(--spacing-lg) var(--spacing-xl);
    }
    .native-card__body--padded {
      padding: var(--spacing-xl);
    }
    .native-card__title {
      font-size: var(--text-xl);
    }
  }
}

.native-card__header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  border-bottom: 1px solid var(--color-border-subtle);
  
  &.is-clickable {
    cursor: pointer;
    transition: background-color var(--duration-fast);
    
    &:hover {
      background-color: var(--color-hover-bg);
    }
  }
}

.native-card__header-content {
  flex: 1;
  min-width: 0;
}

.native-card__title-group {
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.native-card__title {
  font-size: var(--text-lg);
  font-weight: var(--font-semibold);
  color: var(--color-text-primary);
  margin: 0;
  line-height: 1.4;
}

.native-card__subtitle {
  font-size: var(--text-sm);
  color: var(--color-text-tertiary);
  margin: 0;
}

.native-card__header-actions {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
  margin-left: var(--spacing-md);
}

.collapse-btn {
  width: 24px;
  height: 24px;
  display: flex;
  align-items: center;
  justify-content: center;
  border: none;
  background: transparent;
  color: var(--color-text-tertiary);
  cursor: pointer;
  border-radius: var(--radius-sm);
  transition: all var(--duration-fast);
  
  &:hover {
    color: var(--color-text-primary);
    background-color: var(--color-hover-bg);
  }
  
  .el-icon {
    font-size: 14px;
    transition: transform var(--duration-fast) var(--ease-out-quart);
    
    &.is-rotated {
      transform: rotate(180deg);
    }
  }
}

.native-card__body {
  &--padded {
    padding: var(--spacing-lg);
  }
}

.native-card__footer {
  padding: var(--spacing-md) var(--spacing-lg);
  border-top: 1px solid var(--color-border-subtle);
  background-color: var(--color-bg-secondary);
  border-radius: 0 0 var(--radius-lg) var(--radius-lg);
}

// 折叠动画
.collapse-enter-active,
.collapse-leave-active {
  transition: all var(--duration-normal) var(--ease-out-quart);
  overflow: hidden;
}

.collapse-enter-from,
.collapse-leave-to {
  opacity: 0;
  max-height: 0;
  padding-top: 0 !important;
  padding-bottom: 0 !important;
}

.native-card--collapsed {
  .native-card__header {
    border-bottom: none;
  }
}
</style>
