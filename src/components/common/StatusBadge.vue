<template>
  <span class="status-badge" :class="`status-${type}`">
    <span v-if="showDot" class="status-dot" :class="{ 'dot-pulse': pulse }"></span>
    <slot>{{ label }}</slot>
  </span>
</template>

<script setup lang="ts">
interface Props {
  type?: 'success' | 'warning' | 'danger' | 'info' | 'neutral' | 'primary'
  label?: string
  showDot?: boolean
  pulse?: boolean
}

withDefaults(defineProps<Props>(), {
  type: 'neutral',
  label: '',
  showDot: false,
  pulse: false
})
</script>

<style scoped lang="scss">
.status-badge {
  display: inline-flex;
  align-items: center;
  gap: 0.375rem;
  padding: 0.25rem 0.625rem;
  font-size: 12px;
  font-weight: 500;
  border-radius: var(--radius-sm, 4px);
  border: 1px solid;
  transition: all 0.2s ease;
}

.status-dot {
  width: 6px;
  height: 6px;
  border-radius: 50%;
  flex-shrink: 0;

  &.dot-pulse {
    animation: pulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite;
  }
}

@keyframes pulse {
  0%, 100% {
    opacity: 1;
  }
  50% {
    opacity: 0.5;
  }
}

// 成功状态（绿色）
.status-success {
  background: #f0fdf4;
  border-color: #86efac;
  color: #16a34a;

  .dark & {
    background: rgba(22, 163, 74, 0.15);
    border-color: rgba(134, 239, 172, 0.3);
    color: #86efac;
  }

  .status-dot {
    background: #16a34a;
  }
}

// 警告状态（黄色）
.status-warning {
  background: #fefce8;
  border-color: #fde047;
  color: #ca8a04;

  .dark & {
    background: rgba(202, 138, 4, 0.15);
    border-color: rgba(253, 224, 71, 0.3);
    color: #fde047;
  }

  .status-dot {
    background: #ca8a04;
  }
}

// 危险状态（红色）
.status-danger {
  background: #fef2f2;
  border-color: #fca5a5;
  color: #dc2626;

  .dark & {
    background: rgba(220, 38, 38, 0.15);
    border-color: rgba(252, 165, 165, 0.3);
    color: #fca5a5;
  }

  .status-dot {
    background: #dc2626;
  }
}

// 信息状态（蓝色）
.status-info {
  background: #eff6ff;
  border-color: #93c5fd;
  color: #2563eb;

  .dark & {
    background: rgba(37, 99, 235, 0.15);
    border-color: rgba(147, 197, 253, 0.3);
    color: #93c5fd;
  }

  .status-dot {
    background: #2563eb;
  }
}

// 主题色状态
.status-primary {
  background: #eff6ff;
  border-color: #60a5fa;
  color: #2563eb;

  .dark & {
    background: rgba(37, 99, 235, 0.15);
    border-color: rgba(96, 165, 250, 0.3);
    color: #60a5fa;
  }

  .status-dot {
    background: #2563eb;
  }
}

// 中性状态（灰色）
.status-neutral {
  background: #f8fafc;
  border-color: #cbd5e1;
  color: #64748b;

  .dark & {
    background: rgba(100, 116, 139, 0.15);
    border-color: rgba(203, 213, 225, 0.3);
    color: #cbd5e1;
  }

  .status-dot {
    background: #64748b;
  }
}
</style>
