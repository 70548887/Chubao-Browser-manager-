<script setup lang="ts">
/**
 * @description 批量操作结果展示对话框
 * @author DeepAgent
 */

import { computed } from 'vue'
import { Message } from '@/utils/message'
import type { BatchResult } from '@/types'

interface Props {
  visible: boolean
  title?: string
  result: BatchResult | null
}

interface Emits {
  (e: 'update:visible', value: boolean): void
  (e: 'close'): void
}

const props = withDefaults(defineProps<Props>(), {
  title: '批量操作结果'
})
const emit = defineEmits<Emits>()

// 失败项列表
const failedItems = computed(() => {
  if (!props.result) return []
  return props.result.results.filter(r => !r.ok)
})

// 成功项列表
const successItems = computed(() => {
  if (!props.result) return []
  return props.result.results.filter(r => r.ok)
})

// 是否全部成功
const isAllSuccess = computed(() => {
  return props.result ? props.result.failureCount === 0 : true
})

// 关闭对话框
const handleClose = () => {
  emit('update:visible', false)
  emit('close')
}

// 复制失败信息
const copyFailedInfo = async () => {
  if (failedItems.value.length === 0) return

  const text = failedItems.value
    .map(item => `${item.profileId}: ${item.error || '未知错误'}`)
    .join('\n')

  try {
    await navigator.clipboard.writeText(text)
    Message.success('失败信息已复制到剪贴板')
  } catch {
    Message.error('复制失败，请手动复制')
  }
}
</script>

<template>
  <el-dialog :model-value="visible" :title="title" width="500px" :close-on-click-modal="false"
    @update:model-value="emit('update:visible', $event)" @close="handleClose">
    <div v-if="result" class="batch-result">
      <!-- 摘要 -->
      <div class="result-summary" :class="{ success: isAllSuccess, partial: !isAllSuccess }">
        <div class="summary-icon">
          <el-icon v-if="isAllSuccess" class="icon-success">
            <CircleCheck />
          </el-icon>
          <el-icon v-else class="icon-warning">
            <Warning />
          </el-icon>
        </div>
        <div class="summary-text">
          <div class="summary-title">
            {{ isAllSuccess ? '操作完成' : '部分操作失败' }}
          </div>
          <div class="summary-detail">
            共 {{ result.total }} 项，成功 {{ result.successCount }} 项，失败 {{ result.failureCount }} 项
          </div>
        </div>
      </div>

      <!-- 失败列表 -->
      <div v-if="failedItems.length > 0" class="failed-list">
        <div class="list-header">
          <span class="list-title">失败详情 ({{ failedItems.length }})</span>
          <el-button type="primary" link size="small" @click="copyFailedInfo">
            <el-icon>
              <CopyDocument />
            </el-icon>
            复制
          </el-button>
        </div>
        <div class="list-content">
          <div v-for="item in failedItems" :key="item.profileId" class="failed-item">
            <span class="item-id">{{ item.profileId }}</span>
            <span class="item-error">{{ item.error || '未知错误' }}</span>
          </div>
        </div>
      </div>

      <!-- 成功列表（可折叠） -->
      <el-collapse v-if="successItems.length > 0" class="success-collapse">
        <el-collapse-item :title="`成功项 (${successItems.length})`" name="success">
          <div class="success-list">
            <el-tag v-for="item in successItems" :key="item.profileId" type="success" size="small" class="success-tag">
              {{ item.profileId }}
            </el-tag>
          </div>
        </el-collapse-item>
      </el-collapse>
    </div>

    <template #footer>
      <div class="dialog-footer">
        <el-button type="primary" @click="handleClose">确定</el-button>
      </div>
    </template>
  </el-dialog>
</template>

<style scoped lang="scss">
.batch-result {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-lg);
}

.result-summary {
  display: flex;
  align-items: center;
  gap: var(--spacing-md);
  padding: var(--spacing-lg);
  border-radius: var(--radius-lg);

  &.success {
    background: #f0f9eb;
    border: 1px solid #e1f3d8;
  }

  &.partial {
    background: #fdf6ec;
    border: 1px solid #faecd8;
  }
}

.summary-icon {
  font-size: 32px;

  .icon-success {
    color: #67c23a;
  }

  .icon-warning {
    color: #e6a23c;
  }
}

.summary-text {
  flex: 1;
}

.summary-title {
  font-size: var(--text-lg);
  font-weight: var(--font-semibold);
  color: var(--color-text-primary);
  margin-bottom: 4px;
}

.summary-detail {
  font-size: var(--text-sm);
  color: var(--color-text-secondary);
}

.failed-list {
  border: 1px solid #fde2e2;
  border-radius: var(--radius-md);
  background: #fef0f0;
}

.list-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: var(--spacing-sm) var(--spacing-md);
  border-bottom: 1px solid #fde2e2;
}

.list-title {
  font-weight: var(--font-medium);
  color: #f56c6c;
}

.list-content {
  max-height: 200px;
  overflow-y: auto;
  padding: var(--spacing-sm);
}

.failed-item {
  display: flex;
  align-items: flex-start;
  gap: var(--spacing-sm);
  padding: var(--spacing-xs) var(--spacing-sm);
  border-radius: var(--radius-sm);

  &:hover {
    background: rgba(245, 108, 108, 0.1);
  }
}

.item-id {
  flex-shrink: 0;
  font-family: monospace;
  font-size: var(--text-xs);
  color: var(--color-text-secondary);
  max-width: 120px;
  overflow: hidden;
  text-overflow: ellipsis;
}

.item-error {
  flex: 1;
  font-size: var(--text-sm);
  color: #f56c6c;
  word-break: break-all;
}

.success-collapse {
  :deep(.el-collapse-item__header) {
    padding-left: var(--spacing-sm);
    background: #f0f9eb;
    border-radius: var(--radius-md);
  }
}

.success-list {
  display: flex;
  flex-wrap: wrap;
  gap: var(--spacing-xs);
  padding: var(--spacing-sm);
}

.success-tag {
  font-family: monospace;
}

.dialog-footer {
  display: flex;
  justify-content: flex-end;
}
</style>
