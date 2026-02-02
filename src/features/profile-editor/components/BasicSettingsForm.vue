<script setup lang="ts">
/**
 * @description 基础设置表单
 */

import { computed } from 'vue'
import type { BasicSettingsConfig } from '@/types'

interface Props {
  modelValue: Partial<BasicSettingsConfig>
}

interface Emits {
  (e: 'update:modelValue', value: Partial<BasicSettingsConfig>): void
}

const props = defineProps<Props>()
const emit = defineEmits<Emits>()

// 默认值
const defaultSettings: Partial<BasicSettingsConfig> = {
  language: 'auto',
  displayLanguage: 'auto',
  timezone: 'auto',
  geolocationPrompt: 'ask',
  geolocation: 'auto',
  audio: true,
  image: true,
  video: true,
  windowSize: 'custom',
  windowWidth: 1000,
  windowHeight: 1000,
}

// 合并默认值
const settings = computed(() => ({
  ...defaultSettings,
  ...props.modelValue,
}))

// 更新字段
const updateField = (field: keyof BasicSettingsConfig, value: any) => {
  emit('update:modelValue', {
    ...props.modelValue,
    [field]: value,
  })
}
</script>

<template>
  <div class="step-content">
    <div class="form-section">
      <!-- 语言 -->
      <div class="form-row">
        <label class="form-label">语言</label>
        <div class="form-control">
          <div class="radio-group">
            <label class="radio-item" :class="{ active: settings.language === 'auto' }">
              <input type="radio" :checked="settings.language === 'auto'" @change="updateField('language', 'auto')" />
              <span class="radio-indicator"></span>
              <span>基于 IP 匹配</span>
            </label>
            <label class="radio-item" :class="{ active: settings.language === 'custom' }">
              <input type="radio" :checked="settings.language === 'custom'" @change="updateField('language', 'custom')" />
              <span class="radio-indicator"></span>
              <span>自定义</span>
            </label>
          </div>
          <input
            v-if="settings.language === 'custom'"
            :value="settings.languageValue"
            @input="updateField('languageValue', ($event.target as HTMLInputElement).value)"
            type="text"
            class="input"
            placeholder="例如：zh-CN,en-US"
          />
        </div>
      </div>

      <!-- 界面语言 -->
      <div class="form-row">
        <label class="form-label">界面语言</label>
        <div class="form-control">
          <div class="radio-group">
            <label class="radio-item" :class="{ active: settings.displayLanguage === 'auto' }">
              <input type="radio" :checked="settings.displayLanguage === 'auto'" @change="updateField('displayLanguage', 'auto')" />
              <span class="radio-indicator"></span>
              <span>基于 IP 匹配</span>
            </label>
            <label class="radio-item" :class="{ active: settings.displayLanguage === 'custom' }">
              <input type="radio" :checked="settings.displayLanguage === 'custom'" @change="updateField('displayLanguage', 'custom')" />
              <span class="radio-indicator"></span>
              <span>自定义</span>
            </label>
          </div>
          <div v-if="settings.displayLanguage === 'custom'" class="select-wrapper">
            <select
              :value="settings.displayLanguageValue"
              @change="updateField('displayLanguageValue', ($event.target as HTMLSelectElement).value)"
              class="select"
            >
              <option value="zh-CN">简体中文</option>
              <option value="en-US">English</option>
              <option value="ja-JP">日本語</option>
              <option value="ko-KR">한국어</option>
            </select>
            <span class="material-symbols-outlined select-icon">expand_more</span>
          </div>
        </div>
      </div>

      <!-- 时区 -->
      <div class="form-row">
        <label class="form-label">时区</label>
        <div class="form-control">
          <div class="radio-group">
            <label class="radio-item" :class="{ active: settings.timezone === 'auto' }">
              <input type="radio" :checked="settings.timezone === 'auto'" @change="updateField('timezone', 'auto')" />
              <span class="radio-indicator"></span>
              <span>基于 IP 匹配</span>
            </label>
            <label class="radio-item" :class="{ active: settings.timezone === 'custom' }">
              <input type="radio" :checked="settings.timezone === 'custom'" @change="updateField('timezone', 'custom')" />
              <span class="radio-indicator"></span>
              <span>自定义</span>
            </label>
          </div>
          <div v-if="settings.timezone === 'custom'" class="select-wrapper">
            <select
              :value="settings.timezoneValue"
              @change="updateField('timezoneValue', ($event.target as HTMLSelectElement).value)"
              class="select"
            >
              <option value="Asia/Shanghai">Asia/Shanghai (UTC+8)</option>
              <option value="America/New_York">America/New_York (UTC-5)</option>
              <option value="Europe/London">Europe/London (UTC+0)</option>
              <option value="Asia/Tokyo">Asia/Tokyo (UTC+9)</option>
            </select>
            <span class="material-symbols-outlined select-icon">expand_more</span>
          </div>
        </div>
      </div>

      <!-- 地理位置提示 -->
      <div class="form-row">
        <label class="form-label">地理位置提示</label>
        <div class="form-control">
          <div class="radio-group">
            <label class="radio-item" :class="{ active: settings.geolocationPrompt === 'ask' }">
              <input type="radio" :checked="settings.geolocationPrompt === 'ask'" @change="updateField('geolocationPrompt', 'ask')" />
              <span class="radio-indicator"></span>
              <span>询问</span>
            </label>
            <label class="radio-item" :class="{ active: settings.geolocationPrompt === 'allow' }">
              <input type="radio" :checked="settings.geolocationPrompt === 'allow'" @change="updateField('geolocationPrompt', 'allow')" />
              <span class="radio-indicator"></span>
              <span>允许</span>
            </label>
            <label class="radio-item" :class="{ active: settings.geolocationPrompt === 'block' }">
              <input type="radio" :checked="settings.geolocationPrompt === 'block'" @change="updateField('geolocationPrompt', 'block')" />
              <span class="radio-indicator"></span>
              <span>禁止</span>
            </label>
          </div>
        </div>
      </div>

      <!-- 地理位置 -->
      <div class="form-row">
        <label class="form-label">地理位置</label>
        <div class="form-control">
          <div class="radio-group">
            <label class="radio-item" :class="{ active: settings.geolocation === 'auto' }">
              <input type="radio" :checked="settings.geolocation === 'auto'" @change="updateField('geolocation', 'auto')" />
              <span class="radio-indicator"></span>
              <span>基于 IP 匹配</span>
            </label>
            <label class="radio-item" :class="{ active: settings.geolocation === 'custom' }">
              <input type="radio" :checked="settings.geolocation === 'custom'" @change="updateField('geolocation', 'custom')" />
              <span class="radio-indicator"></span>
              <span>自定义</span>
            </label>
          </div>
          <div v-if="settings.geolocation === 'custom'" class="geo-inputs">
            <input
              :value="settings.geolocationLatitude"
              @input="updateField('geolocationLatitude', Number(($event.target as HTMLInputElement).value))"
              type="number"
              class="input"
              placeholder="纬度"
            />
            <input
              :value="settings.geolocationLongitude"
              @input="updateField('geolocationLongitude', Number(($event.target as HTMLInputElement).value))"
              type="number"
              class="input"
              placeholder="经度"
            />
          </div>
        </div>
      </div>

      <!-- 声音 -->
      <div class="form-row">
        <label class="form-label">声音</label>
        <div class="form-control">
          <div class="radio-group">
            <label class="radio-item" :class="{ active: settings.audio === true }">
              <input type="radio" :checked="settings.audio === true" @change="updateField('audio', true)" />
              <span class="radio-indicator"></span>
              <span>开启</span>
            </label>
            <label class="radio-item" :class="{ active: settings.audio === false }">
              <input type="radio" :checked="settings.audio === false" @change="updateField('audio', false)" />
              <span class="radio-indicator"></span>
              <span>关闭</span>
            </label>
          </div>
        </div>
      </div>

      <!-- 图片 -->
      <div class="form-row">
        <label class="form-label">图片</label>
        <div class="form-control">
          <div class="radio-group">
            <label class="radio-item" :class="{ active: settings.image === true }">
              <input type="radio" :checked="settings.image === true" @change="updateField('image', true)" />
              <span class="radio-indicator"></span>
              <span>开启</span>
            </label>
            <label class="radio-item" :class="{ active: settings.image === false }">
              <input type="radio" :checked="settings.image === false" @change="updateField('image', false)" />
              <span class="radio-indicator"></span>
              <span>关闭</span>
            </label>
          </div>
        </div>
      </div>

      <!-- 视频 -->
      <div class="form-row">
        <label class="form-label">视频</label>
        <div class="form-control">
          <div class="radio-group">
            <label class="radio-item" :class="{ active: settings.video === true }">
              <input type="radio" :checked="settings.video === true" @change="updateField('video', true)" />
              <span class="radio-indicator"></span>
              <span>开启</span>
            </label>
            <label class="radio-item" :class="{ active: settings.video === false }">
              <input type="radio" :checked="settings.video === false" @change="updateField('video', false)" />
              <span class="radio-indicator"></span>
              <span>关闭</span>
            </label>
          </div>
        </div>
      </div>

      <!-- 窗口大小 -->
      <div class="form-row">
        <label class="form-label">窗口大小</label>
        <div class="form-control">
          <div class="radio-group">
            <label class="radio-item" :class="{ active: settings.windowSize === 'custom' }">
              <input type="radio" :checked="settings.windowSize === 'custom'" @change="updateField('windowSize', 'custom')" />
              <span class="radio-indicator"></span>
              <span>自定义</span>
            </label>
            <label class="radio-item" :class="{ active: settings.windowSize === 'fullscreen' }">
              <input type="radio" :checked="settings.windowSize === 'fullscreen'" @change="updateField('windowSize', 'fullscreen')" />
              <span class="radio-indicator"></span>
              <span>全屏</span>
            </label>
          </div>
          <div v-if="settings.windowSize === 'custom'" class="size-inputs">
            <div class="size-input-group">
              <span class="size-label">宽</span>
              <input
                :value="settings.windowWidth"
                @input="updateField('windowWidth', Number(($event.target as HTMLInputElement).value))"
                type="number"
                class="input size-input"
                min="800"
                max="3840"
              />
            </div>
            <div class="size-input-group">
              <span class="size-label">高</span>
              <input
                :value="settings.windowHeight"
                @input="updateField('windowHeight', Number(($event.target as HTMLInputElement).value))"
                type="number"
                class="input size-input"
                min="600"
                max="2160"
              />
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped lang="scss">
.step-content {
  max-width: 600px;
  margin: 0 auto;
  padding: 24px;
}

.form-section {
  display: flex;
  flex-direction: column;
  gap: 24px;
}

.form-row {
  display: grid;
  grid-template-columns: 100px 1fr;
  gap: 16px;
  align-items: start;
}

.form-label {
  font-size: 14px;
  font-weight: 500;
  color: #1e293b;
  padding-top: 10px;
}

.form-control {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

// 单选按钮组
.radio-group {
  display: flex;
  gap: 12px;
  flex-wrap: wrap;
}

.radio-item {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px 16px;
  background: white;
  border: 2px solid #e2e8f0;
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.2s;
  
  input {
    display: none;
  }
  
  &:hover {
    border-color: #cbd5e1;
  }
  
  &.active {
    border-color: #2563eb;
    background: #eff6ff;
    
    .radio-indicator {
      border-color: #2563eb;
      background: #2563eb;
      
      &::after {
        opacity: 1;
      }
    }
  }
  
  span {
    font-size: 14px;
    color: #374151;
  }
}

// 圆形指示器
.radio-indicator {
  width: 18px;
  height: 18px;
  border: 2px solid #cbd5e1;
  border-radius: 50%;
  position: relative;
  flex-shrink: 0;
  transition: all 0.2s;
  
  &::after {
    content: '';
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    width: 8px;
    height: 8px;
    background: white;
    border-radius: 50%;
    opacity: 0;
    transition: opacity 0.2s;
  }
}

// 输入框
.input {
  width: 100%;
  height: 42px;
  padding: 0 12px;
  border: 1px solid #e2e8f0;
  border-radius: 8px;
  font-size: 14px;
  color: #1e293b;
  background: white;
  transition: all 0.2s;
  
  &::placeholder {
    color: #94a3b8;
  }
  
  &:focus {
    outline: none;
    border-color: #2563eb;
    box-shadow: 0 0 0 3px rgba(37, 99, 235, 0.1);
  }
}

// 下拉框
.select-wrapper {
  position: relative;
  
  .select-icon {
    position: absolute;
    right: 12px;
    top: 50%;
    transform: translateY(-50%);
    color: #94a3b8;
    font-size: 20px;
    pointer-events: none;
  }
}

.select {
  width: 100%;
  height: 42px;
  padding: 0 36px 0 12px;
  border: 1px solid #e2e8f0;
  border-radius: 8px;
  font-size: 14px;
  color: #1e293b;
  background: white;
  cursor: pointer;
  appearance: none;
  transition: all 0.2s;
  
  &:focus {
    outline: none;
    border-color: #2563eb;
    box-shadow: 0 0 0 3px rgba(37, 99, 235, 0.1);
  }
}

// 地理位置输入
.geo-inputs {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 12px;
}

// 窗口大小输入
.size-inputs {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 16px;
}

.size-input-group {
  display: flex;
  align-items: center;
  gap: 8px;
  
  .size-label {
    min-width: 24px;
    font-size: 14px;
    color: #64748b;
  }
  
  .size-input {
    flex: 1;
  }
}
</style>
