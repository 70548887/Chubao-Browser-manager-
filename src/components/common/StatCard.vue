<script setup lang="ts">
/**
 * @description 统计卡片组件 - 桌面原生化
 * @author DeepAgent
 */

interface Props {
  /** 标题 */
  title: string
  /** 值 */
  value: string | number
  /** 图标 */
  icon?: string
  /** 卡片类型/颜色 */
  type?: 'default' | 'primary' | 'success' | 'warning' | 'danger'
  /** 趋势 */
  trend?: {
    value: number
    isUp: boolean
  }
  /** 副标题 */
  subtitle?: string
}

withDefaults(defineProps<Props>(), {
  type: 'default',
})
</script>

<template>
  <div class="stat-card" :class="[`stat-card--${type}`]">
    <div class="stat-card__header">
      <span class="stat-card__title">{{ title }}</span>
      <div v-if="icon" class="stat-card__icon">
        <el-icon><component :is="icon" /></el-icon>
      </div>
    </div>
    
    <div class="stat-card__body">
      <span class="stat-card__value">{{ value }}</span>
      <span v-if="subtitle" class="stat-card__subtitle">{{ subtitle }}</span>
    </div>
    
    <div v-if="trend" class="stat-card__trend" :class="{ 'is-up': trend.isUp, 'is-down': !trend.isUp }">
      <el-icon v-if="trend.isUp"><ArrowUp /></el-icon>
      <el-icon v-else><ArrowDown /></el-icon>
      <span>{{ Math.abs(trend.value) }}%</span>
    </div>
  </div>
</template>

<style scoped lang="scss">
.stat-card {
  background-color: var(--color-bg-elevated);
  border: 1px solid var(--color-border-default);
  border-radius: var(--radius-lg);
  padding: var(--spacing-lg);
  transition: all var(--duration-fast) var(--ease-out-quart);
  position: relative;
  overflow: hidden;
  
  // 左侧装饰条
  &::before {
    content: '';
    position: absolute;
    left: 0;
    top: 0;
    bottom: 0;
    width: 3px;
    background: var(--card-accent-color, var(--color-accent-blue));
    opacity: 0.8;
  }
  
  &:hover {
    border-color: var(--color-border-strong);
    box-shadow: var(--shadow-md);
    transform: translateY(-2px);
  }
  
  &--primary {
    --card-accent-color: var(--color-accent-blue);
  }
  
  &--success {
    --card-accent-color: var(--color-status-running);
  }
  
  &--warning {
    --card-accent-color: var(--color-status-warning);
  }
  
  &--danger {
    --card-accent-color: var(--color-accent-danger);
  }
}

.stat-card__header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: var(--spacing-md);
}

.stat-card__title {
  font-size: var(--text-sm);
  font-weight: var(--font-medium);
  color: var(--color-text-secondary);
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.stat-card__icon {
  width: 32px;
  height: 32px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: linear-gradient(135deg, rgba(0, 120, 212, 0.1) 0%, rgba(0, 120, 212, 0.05) 100%);
  border-radius: var(--radius-md);
  color: var(--card-accent-color, var(--color-accent-blue));
  
  .el-icon {
    font-size: 18px;
  }
}

.stat-card__body {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-xs);
}

.stat-card__value {
  font-size: var(--text-2xl);
  font-weight: var(--font-bold);
  color: var(--color-text-primary);
  line-height: 1.2;
}

.stat-card__subtitle {
  font-size: var(--text-sm);
  color: var(--color-text-tertiary);
}

.stat-card__trend {
  display: inline-flex;
  align-items: center;
  gap: var(--spacing-xs);
  margin-top: var(--spacing-md);
  padding: var(--spacing-xs) var(--spacing-sm);
  border-radius: var(--radius-sm);
  font-size: var(--text-sm);
  font-weight: var(--font-medium);
  
  .el-icon {
    font-size: 12px;
  }
  
  &.is-up {
    background-color: rgba(40, 167, 69, 0.1);
    color: var(--color-status-running);
  }
  
  &.is-down {
    background-color: rgba(220, 53, 69, 0.1);
    color: var(--color-accent-danger);
  }
}
</style>
