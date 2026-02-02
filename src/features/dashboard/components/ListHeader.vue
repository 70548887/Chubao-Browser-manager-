<script setup lang="ts">
/**
 * @description 环境列表表头组件 - 完全对齐原型
 * @author DeepAgent
 */

interface Props {
  isAllSelected?: boolean
  sortField?: string
  sortOrder?: 'asc' | 'desc'
}

withDefaults(defineProps<Props>(), {
  isAllSelected: false,
  sortField: '',
  sortOrder: 'asc',
})

const emit = defineEmits<{
  (e: 'selectAll', value: boolean): void
  (e: 'sort', field: string): void
}>()
</script>

<template>
  <div class="list-header">
    <div class="col-checkbox">
      <input 
        type="checkbox" 
        :checked="isAllSelected"
        class="checkbox-input"
        @change="emit('selectAll', ($event.target as HTMLInputElement).checked)"
      />
    </div>
    
    <div class="col-header col-index">序号</div>
    <div class="col-header col-status">状态</div>
    <div class="col-header col-name">环境名称 / ID</div>
    <div class="col-header col-group">分组</div>
    <div class="col-header col-proxy">代理信息</div>
    <div class="col-header col-platform">平台</div>
    <div class="col-header col-time">最近更新</div>
    <div class="col-header col-actions">操作</div>
  </div>
</template>

<style scoped lang="scss">
.list-header {
  display: grid;
  grid-template-columns: 48px 60px 100px 3fr 160px 200px 100px 100px 120px;
  align-items: center;
  padding: 14px 24px;
  background: rgba(248, 250, 252, 0.8);
  border-bottom: 1px solid #e2e8f0;
  font-size: 12px;
  font-weight: 600;
  color: #64748b;
  text-transform: uppercase;
  letter-spacing: 0.5px;
  backdrop-filter: blur(8px);
  position: sticky;
  top: 0;
  z-index: 10;
}

.col-checkbox {
  display: flex;
  align-items: center;
  justify-content: center;
}

.col-header {
  &.col-actions {
    text-align: right;
  }
}

.checkbox-input {
  width: 16px;
  height: 16px;
  border-radius: 4px;
  border: 1px solid #d1d5db;
  cursor: pointer;
  
  &:checked {
    background-color: #2563eb;
    border-color: #2563eb;
  }
}
</style>
