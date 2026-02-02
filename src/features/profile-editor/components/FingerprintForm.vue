<script setup lang="ts">
/**
 * @description 指纹配置表单
 */

import type { ProfileCreateDTO, Fingerprint } from '@/types'

interface Props {
  modelValue: ProfileCreateDTO
}

interface Emits {
  (e: 'update:modelValue', value: ProfileCreateDTO): void
  (e: 'regenerate'): void
}

const props = defineProps<Props>()
const emit = defineEmits<Emits>()

// 更新指纹字段
const updateFingerprint = (field: keyof Fingerprint, value: any) => {
  emit('update:modelValue', {
    ...props.modelValue,
    fingerprint: {
      ...props.modelValue.fingerprint,
      [field]: value,
    },
  })
}

// 平台选项
const platformOptions = [
  { label: 'Windows', value: 'windows' },
  { label: 'macOS', value: 'macos' },
  { label: 'Linux', value: 'linux' },
]

// 浏览器选项
const browserOptions = [
  { label: 'Chrome', value: 'chrome' },
  { label: 'Edge', value: 'edge' },
  { label: 'Brave', value: 'brave' },
]

// CPU 核心数选项
const cpuOptions = [4, 8, 12, 16]

// 内存选项
const memoryOptions = [8, 16, 32]

// 时区选项
const timezoneOptions = [
  { label: '中国 (Asia/Shanghai)', value: 'Asia/Shanghai' },
  { label: '美国东部 (America/New_York)', value: 'America/New_York' },
  { label: '英国 (Europe/London)', value: 'Europe/London' },
  { label: '日本 (Asia/Tokyo)', value: 'Asia/Tokyo' },
]

// 语言选项
const languageOptions = [
  { label: '简体中文', value: 'zh-CN' },
  { label: 'English', value: 'en-US' },
  { label: '日本語', value: 'ja-JP' },
]
</script>

<template>
  <div class="fingerprint-form">
    <!-- 快捷操作 -->
    <div class="quick-actions">
      <el-button type="primary" @click="emit('regenerate')">
        <el-icon><Refresh /></el-icon>
        随机生成指纹
      </el-button>
    </div>

    <el-form label-width="120px" label-position="left" v-if="modelValue.fingerprint">
      <!-- 平台信息 -->
      <el-divider content-position="left">平台信息</el-divider>
      
      <el-form-item label="操作系统">
        <el-select
          :model-value="modelValue.fingerprint.platform"
          @update:model-value="updateFingerprint('platform', $event)"
          placeholder="选择操作系统"
        >
          <el-option
            v-for="item in platformOptions"
            :key="item.value"
            :label="item.label"
            :value="item.value"
          />
        </el-select>
      </el-form-item>

      <el-form-item label="浏览器品牌">
        <el-select
          :model-value="modelValue.fingerprint.browser"
          @update:model-value="updateFingerprint('browser', $event)"
          placeholder="选择浏览器"
        >
          <el-option
            v-for="item in browserOptions"
            :key="item.value"
            :label="item.label"
            :value="item.value"
          />
        </el-select>
      </el-form-item>

      <!-- 硬件信息 -->
      <el-divider content-position="left">硬件信息</el-divider>

      <el-form-item label="CPU 核心数">
        <el-select
          :model-value="modelValue.fingerprint.hardwareConcurrency"
          @update:model-value="updateFingerprint('hardwareConcurrency', $event)"
        >
          <el-option
            v-for="cpu in cpuOptions"
            :key="cpu"
            :label="`${cpu} 核`"
            :value="cpu"
          />
        </el-select>
      </el-form-item>

      <el-form-item label="内存大小">
        <el-select
          :model-value="modelValue.fingerprint.deviceMemory"
          @update:model-value="updateFingerprint('deviceMemory', $event)"
        >
          <el-option
            v-for="mem in memoryOptions"
            :key="mem"
            :label="`${mem} GB`"
            :value="mem"
          />
        </el-select>
      </el-form-item>

      <el-form-item label="屏幕分辨率">
        <el-input
          :model-value="modelValue.fingerprint.screenResolution"
          @update:model-value="updateFingerprint('screenResolution', $event)"
          placeholder="例如：1920x1080"
        />
      </el-form-item>

      <!-- 地区信息 -->
      <el-divider content-position="left">地区信息</el-divider>

      <el-form-item label="时区">
        <el-select
          :model-value="modelValue.fingerprint.timezone"
          @update:model-value="updateFingerprint('timezone', $event)"
        >
          <el-option
            v-for="item in timezoneOptions"
            :key="item.value"
            :label="item.label"
            :value="item.value"
          />
        </el-select>
      </el-form-item>

      <el-form-item label="语言">
        <el-select
          :model-value="modelValue.fingerprint.language"
          @update:model-value="updateFingerprint('language', $event)"
        >
          <el-option
            v-for="item in languageOptions"
            :key="item.value"
            :label="item.label"
            :value="item.value"
          />
        </el-select>
      </el-form-item>

      <!-- 噪声配置 -->
      <el-divider content-position="left">防检测噪声</el-divider>

      <el-form-item label="Canvas 噪声">
        <el-switch
          :model-value="modelValue.fingerprint.canvasNoise"
          @update:model-value="updateFingerprint('canvasNoise', $event)"
        />
      </el-form-item>

      <el-form-item label="WebGL 噪声">
        <el-switch
          :model-value="modelValue.fingerprint.webglNoise"
          @update:model-value="updateFingerprint('webglNoise', $event)"
        />
      </el-form-item>

      <el-form-item label="Audio 噪声">
        <el-switch
          :model-value="modelValue.fingerprint.audioNoise"
          @update:model-value="updateFingerprint('audioNoise', $event)"
        />
      </el-form-item>
    </el-form>
  </div>
</template>

<style scoped lang="scss">
.fingerprint-form {
  padding: 20px 0;
}

.quick-actions {
  margin-bottom: 24px;
  text-align: center;
}

.el-divider {
  margin: 24px 0 16px;
}
</style>
