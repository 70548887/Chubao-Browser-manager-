<template>
  <div class="button-group" :class="{ 'full-width': fullWidth }">
    <button
      v-for="(option, index) in options"
      :key="option.value"
      type="button"
      class="btn-group-item"
      :class="{
        'btn-active': modelValue === option.value,
        'btn-first': index === 0,
        'btn-last': index === options.length - 1
      }"
      @click="handleSelect(option.value)"
    >
      {{ option.label }}
    </button>
  </div>
</template>

<script setup lang="ts">
export interface ButtonGroupOption {
  label: string
  value: string | number
}

interface Props {
  options: ButtonGroupOption[]
  modelValue: string | number
  fullWidth?: boolean
}

interface Emits {
  (e: 'update:modelValue', value: string | number): void
}

withDefaults(defineProps<Props>(), {
  fullWidth: false
})

const emit = defineEmits<Emits>()

const handleSelect = (value: string | number) => {
  emit('update:modelValue', value)
}
</script>

<style scoped lang="scss">
.button-group {
  display: inline-flex;
  border-radius: var(--radius-md);
  box-shadow: var(--shadow-sm);
  overflow: hidden;

  &.full-width {
    width: 100%;

    .btn-group-item {
      flex: 1;
    }
  }
}

.btn-group-item {
  padding: 0.5rem 1rem;
  font-size: 14px;
  font-weight: 500;
  border: 1px solid var(--color-border-default);
  background: var(--color-bg-secondary);
  color: var(--color-text-secondary);
  cursor: pointer;
  transition: all 0.2s ease;
  position: relative;
  margin-left: -1px;

  &:first-child {
    margin-left: 0;
  }

  &.btn-first {
    border-top-left-radius: var(--radius-md);
    border-bottom-left-radius: var(--radius-md);
  }

  &.btn-last {
    border-top-right-radius: var(--radius-md);
    border-bottom-right-radius: var(--radius-md);
  }

  &:hover:not(.btn-active) {
    background: var(--color-bg-hover);
    color: var(--color-text-primary);
    z-index: 1;
  }

  &:focus {
    outline: none;
    z-index: 2;
    ring: 2px solid var(--color-accent-blue-hover);
  }

  &.btn-active {
    background: var(--color-accent-blue);
    border-color: var(--color-accent-blue);
    color: white;
    z-index: 3;
    box-shadow: 0 2px 4px rgba(37, 99, 235, 0.2);

    &:hover {
      background: var(--color-accent-blue-hover);
      border-color: var(--color-accent-blue-hover);
    }
  }
}

// 暗色模式
.dark {
  .btn-group-item {
    border-color: var(--color-border-dark, #334155);
    background: var(--color-bg-secondary);
    color: var(--color-text-secondary);

    &:hover:not(.btn-active) {
      background: var(--color-bg-hover);
    }

    &.btn-active {
      background: var(--color-accent-blue);
      border-color: var(--color-accent-blue);
      color: white;
    }
  }
}
</style>
