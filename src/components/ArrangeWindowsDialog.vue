<script setup lang="ts">
/**
 * @description 排列窗口对话框 - 专业版
 * @author DeepAgent
 */

import { ref, computed, onMounted } from 'vue'
import { Message } from '@/utils/message'
import { invoke } from '@tauri-apps/api/core'

interface Monitor {
  id: number
  name: string
  width: number
  height: number
  x: number
  y: number
}

interface ArrangeConfig {
  monitorId: number
  startX: number
  startY: number
  windowWidth: number
  windowHeight: number
  columns: number
  gapX: number
  gapY: number
  order: 'asc' | 'desc'
  mode: 'grid' | 'diagonal'
}

const props = defineProps<{
  visible: boolean
  runningCount: number
}>()

const emit = defineEmits<{
  (e: 'update:visible', value: boolean): void
  (e: 'arranged'): void
}>()

// 显示器列表
const monitors = ref<Monitor[]>([])
const selectedMonitor = ref<number>(0)

// 排列模式
const arrangeMode = ref<'grid' | 'diagonal'>('grid')

// 配置
const config = ref<ArrangeConfig>({
  monitorId: 0,
  startX: 0,
  startY: 0,
  windowWidth: 500,
  windowHeight: 300,
  columns: 4,
  gapX: 0,
  gapY: 0,
  order: 'asc',
  mode: 'grid'
})

const isLoading = ref(false)

// 当前选中的显示器
const currentMonitor = computed(() => {
  return monitors.value.find(m => m.id === selectedMonitor.value) || monitors.value[0]
})

// 显示器选项
const monitorOptions = computed(() => {
  return monitors.value.map(m => ({
    value: m.id,
    label: `${m.name} (${m.width} x ${m.height}) ID: ${m.id}`
  }))
})

// 获取显示器信息
const fetchMonitors = async () => {
  try {
    const result = await invoke<Monitor[]>('get_monitors')
    monitors.value = result
    if (result.length > 0) {
      selectedMonitor.value = result[0].id
      // 根据显示器尺寸设置默认窗口大小
      const m = result[0]
      config.value.windowWidth = Math.floor(m.width / 4)
      config.value.windowHeight = Math.floor(m.height / 3)
    }
  } catch (e) {
    // 使用默认显示器
    monitors.value = [{
      id: 0,
      name: '主显示器',
      width: 1920,
      height: 1080,
      x: 0,
      y: 0
    }]
  }
}

// 一键自适应排列
const autoArrange = () => {
  const m = currentMonitor.value
  if (!m) return

  const count = props.runningCount
  if (count === 0) return

  // 计算最佳列数
  const cols = Math.ceil(Math.sqrt(count))
  const rows = Math.ceil(count / cols)

  // 计算窗口大小（留一点边距）
  const margin = 10
  const availableWidth = m.width - margin * 2
  const availableHeight = m.height - margin * 2

  config.value.columns = cols
  config.value.windowWidth = Math.floor(availableWidth / cols)
  config.value.windowHeight = Math.floor(availableHeight / rows)
  config.value.startX = m.x + margin
  config.value.startY = m.y + margin
  config.value.gapX = 0
  config.value.gapY = 0
}

// 开始排列
const handleArrange = async () => {
  if (props.runningCount === 0) {
    Message.warning('没有运行中的窗口')
    return
  }

  // 验证窗口大小
  if (config.value.windowWidth < 120) {
    Message.warning('窗口宽度不能小于 120px')
    return
  }
  if (config.value.windowHeight < 40) {
    Message.warning('窗口高度不能小于 40px')
    return
  }

  isLoading.value = true
  try {
    await invoke('arrange_windows_advanced', {
      config: {
        monitor_id: selectedMonitor.value,
        start_x: config.value.startX,
        start_y: config.value.startY,
        window_width: config.value.windowWidth,
        window_height: config.value.windowHeight,
        columns: config.value.columns,
        gap_x: config.value.gapX,
        gap_y: config.value.gapY,
        order: config.value.order,
        mode: arrangeMode.value
      }
    })
    Message.success(`已排列 ${props.runningCount} 个窗口`)
    emit('arranged')
    handleClose()
  } catch (e: any) {
    Message.error(e.message || '排列窗口失败')
  } finally {
    isLoading.value = false
  }
}

const handleClose = () => {
  emit('update:visible', false)
}

onMounted(() => {
  fetchMonitors()
})
</script>

<template>
  <el-dialog :model-value="visible" title="重新排列窗口及调整大小" width="560px" :close-on-click-modal="false"
    @update:model-value="$emit('update:visible', $event)">
    <!-- 排列模式切换 -->
    <div class="mode-tabs">
      <button :class="['mode-tab', { active: arrangeMode === 'grid' }]" @click="arrangeMode = 'grid'">
        宫格排列
      </button>
      <button :class="['mode-tab', { active: arrangeMode === 'diagonal' }]" @click="arrangeMode = 'diagonal'">
        对角线排列
      </button>
    </div>

    <div class="arrange-form">
      <!-- 显示器选择 -->
      <div class="form-row">
        <label class="form-label">显示器</label>
        <div class="form-content">
          <el-select v-model="selectedMonitor" style="width: 100%">
            <el-option v-for="opt in monitorOptions" :key="opt.value" :label="opt.label" :value="opt.value" />
          </el-select>
        </div>
      </div>

      <!-- 起始位置 -->
      <div class="form-row">
        <label class="form-label">起始位置：</label>
        <div class="form-content form-inline">
          <div class="inline-item">
            <span class="inline-label">X:</span>
            <el-input-number v-model="config.startX" :min="0" :max="9999" :controls="false" style="width: 100px" />
          </div>
          <div class="inline-item">
            <span class="inline-label">Y:</span>
            <el-input-number v-model="config.startY" :min="0" :max="9999" :controls="false" style="width: 100px" />
          </div>
        </div>
      </div>

      <!-- 窗口大小 -->
      <div class="form-row">
        <label class="form-label">窗口大小：</label>
        <div class="form-content form-inline">
          <div class="inline-item">
            <span class="inline-label">宽:</span>
            <el-input-number v-model="config.windowWidth" :min="120" :max="9999" :controls="false"
              style="width: 80px" />
          </div>
          <div class="inline-item">
            <span class="inline-label">高:</span>
            <el-input-number v-model="config.windowHeight" :min="40" :max="9999" :controls="false"
              style="width: 80px" />
          </div>
          <div class="inline-item">
            <span class="inline-label">每行窗口数:</span>
            <el-input-number v-model="config.columns" :min="1" :max="20" :controls="false" style="width: 60px" />
          </div>
        </div>
      </div>

      <!-- 窗口间距 -->
      <div class="form-row">
        <label class="form-label">窗口间距：</label>
        <div class="form-content form-inline">
          <div class="inline-item">
            <span class="inline-label">横向间距:</span>
            <el-input-number v-model="config.gapX" :min="0" :max="100" style="width: 100px" />
          </div>
          <div class="inline-item">
            <span class="inline-label">纵向间距:</span>
            <el-input-number v-model="config.gapY" :min="0" :max="100" style="width: 100px" />
          </div>
        </div>
      </div>

      <!-- 按序号排列 -->
      <div class="form-row">
        <label class="form-label">按序号排列</label>
        <div class="form-content">
          <el-radio-group v-model="config.order">
            <el-radio-button value="asc">正序</el-radio-button>
            <el-radio-button value="desc">倒序</el-radio-button>
          </el-radio-group>
        </div>
      </div>

      <!-- 提示信息 -->
      <div class="form-tip">
        注意：浏览器窗口可缩放的最小宽度为120px，最小高度为40px。
      </div>
    </div>

    <template #footer>
      <div class="dialog-footer">
        <button class="btn-auto" @click="autoArrange">
          一键自适应排列
        </button>
        <div class="footer-right">
          <el-button @click="handleClose">取消</el-button>
          <el-button type="primary" :loading="isLoading" @click="handleArrange">
            开始排列
          </el-button>
        </div>
      </div>
    </template>
  </el-dialog>
</template>

<style scoped lang="scss">
.mode-tabs {
  display: flex;
  margin-bottom: 20px;
  border-radius: 6px;
  overflow: hidden;
  border: 1px solid var(--color-border-default);
}

.mode-tab {
  flex: 1;
  padding: 10px 20px;
  border: none;
  background: var(--color-bg-secondary);
  color: var(--color-text-secondary);
  cursor: pointer;
  font-size: 14px;
  transition: all 0.2s;

  &:first-child {
    border-right: 1px solid var(--color-border-default);
  }

  &.active {
    background: var(--color-accent-blue);
    color: white;
  }

  &:hover:not(.active) {
    background: var(--color-bg-tertiary);
  }
}

.arrange-form {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.form-row {
  display: flex;
  align-items: center;
  gap: 12px;
}

.form-label {
  width: 80px;
  flex-shrink: 0;
  color: var(--color-text-secondary);
  font-size: 14px;
}

.form-content {
  flex: 1;
}

.form-inline {
  display: flex;
  align-items: center;
  gap: 16px;
  flex-wrap: wrap;
}

.inline-item {
  display: flex;
  align-items: center;
  gap: 6px;
}

.inline-label {
  color: var(--color-text-tertiary);
  font-size: 13px;
  white-space: nowrap;
}

.form-tip {
  margin-top: 8px;
  padding: 8px 12px;
  background: rgba(37, 99, 235, 0.1);
  border-radius: 4px;
  color: var(--color-accent-blue);
  font-size: 12px;
}

.dialog-footer {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.btn-auto {
  padding: 8px 16px;
  border: 1px solid var(--color-accent-blue);
  border-radius: 6px;
  background: transparent;
  color: var(--color-accent-blue);
  font-size: 14px;
  cursor: pointer;
  transition: all 0.2s;

  &:hover {
    background: var(--color-accent-blue);
    color: white;
  }
}

.footer-right {
  display: flex;
  gap: 8px;

  :deep(.el-button--primary) {
    background-color: var(--color-accent-blue);
    border-color: var(--color-accent-blue);
    color: white;

    &:hover {
      background-color: #1d4ed8;
      border-color: #1d4ed8;
    }
  }
}
</style>
