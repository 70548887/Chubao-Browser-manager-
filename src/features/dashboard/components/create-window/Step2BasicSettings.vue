<script setup lang="ts">
/**
 * @description 步骤2 - 基础设置
 */
import { watch, onMounted } from 'vue'
import { COMMON_TIMEZONES } from '@/config/timezones'

const model = defineModel<any>({ required: true })

// 初始化时区ID（如果未设置）
onMounted(() => {
  if (!model.value.timezoneId) {
    model.value.timezoneId = 'Asia/Shanghai' // 默认中国时区
  }
})

// 当切换回"跟随IP"时，清空自定义时区
watch(() => model.value.timezone, (newVal) => {
  if (newVal === 'ip') {
    model.value.timezoneId = undefined
  }
})
</script>

<template>
  <div class="step-content">
    <div class="form-section">
      <!-- 语言 -->
      <div class="form-row">
        <label class="form-label">
          语言
          <span class="material-symbols-outlined help-icon" title="浏览器语言设置">help</span>
        </label>
        <div class="form-control">
          <div class="btn-group">
            <button 
              type="button"
              class="btn-option"
              :class="{ active: model.language === 'ip' }"
              @click="model.language = 'ip'"
            >基于 IP 匹配</button>
            <button 
              type="button"
              class="btn-option"
              :class="{ active: model.language === 'custom' }"
              @click="model.language = 'custom'"
            >自定义</button>
          </div>
          
          <!-- 自定义语言选择器 -->
          <div v-if="model.language === 'custom'" class="language-selector">
            <select v-model="model.languageValue" class="timezone-select">
              <option value="en-US">English (US)</option>
              <option value="en-GB">English (UK)</option>
              <option value="zh-CN">中文 (简体)</option>
              <option value="zh-TW">中文 (繁體)</option>
              <option value="ja-JP">日本語</option>
              <option value="ko-KR">한국어</option>
              <option value="de-DE">Deutsch</option>
              <option value="fr-FR">Français</option>
              <option value="es-ES">Español</option>
              <option value="pt-BR">Português (Brasil)</option>
              <option value="ru-RU">Русский</option>
            </select>
          </div>
        </div>
      </div>
      
      <!-- 界面语言 -->
      <div class="form-row">
        <label class="form-label">
          界面语言
          <span class="material-symbols-outlined help-icon" title="浏览器界面语言">help</span>
        </label>
        <div class="form-control">
          <div class="btn-group">
            <button 
              type="button"
              class="btn-option"
              :class="{ active: model.uiLanguage === 'ip' }"
              @click="model.uiLanguage = 'ip'"
            >基于 IP 匹配</button>
            <button 
              type="button"
              class="btn-option"
              :class="{ active: model.uiLanguage === 'custom' }"
              @click="model.uiLanguage = 'custom'"
            >自定义</button>
          </div>
        </div>
      </div>
      
      <!-- 时区 -->
      <div class="form-row">
        <label class="form-label">时区</label>
        <div class="form-control">
          <div class="btn-group">
            <button 
              type="button"
              class="btn-option"
              :class="{ active: model.timezone === 'ip' }"
              @click="model.timezone = 'ip'"
            >基于 IP 匹配</button>
            <button 
              type="button"
              class="btn-option"
              :class="{ active: model.timezone === 'custom' }"
              @click="model.timezone = 'custom'"
            >自定义</button>
          </div>
          
          <!-- 时区选择器 -->
          <div v-if="model.timezone === 'custom'" class="timezone-selector">
            <select v-model="model.timezoneId" class="timezone-select">
              <option 
                v-for="tz in COMMON_TIMEZONES" 
                :key="tz.id" 
                :value="tz.id"
              >
                {{ tz.label }}
              </option>
            </select>
          </div>
        </div>
      </div>
      
      <!-- 地理位置提示 -->
      <div class="form-row">
        <label class="form-label">地理位置提示</label>
        <div class="form-control">
          <div class="btn-group">
            <button 
              type="button"
              class="btn-option"
              :class="{ active: model.geolocationPrompt === 'ask' }"
              @click="model.geolocationPrompt = 'ask'"
            >询问</button>
            <button 
              type="button"
              class="btn-option"
              :class="{ active: model.geolocationPrompt === 'allow' }"
              @click="model.geolocationPrompt = 'allow'"
            >允许</button>
            <button 
              type="button"
              class="btn-option"
              :class="{ active: model.geolocationPrompt === 'deny' }"
              @click="model.geolocationPrompt = 'deny'"
            >禁止</button>
          </div>
        </div>
      </div>
      
      <!-- 地理位置 -->
      <div class="form-row">
        <label class="form-label">地理位置</label>
        <div class="form-control">
          <div class="btn-group">
            <button 
              type="button"
              class="btn-option"
              :class="{ active: model.geolocation === 'ip' }"
              @click="model.geolocation = 'ip'"
            >基于 IP 匹配</button>
            <button 
              type="button"
              class="btn-option"
              :class="{ active: model.geolocation === 'custom' }"
              @click="model.geolocation = 'custom'"
            >自定义</button>
          </div>
        </div>
      </div>
      
      <!-- 声音 -->
      <div class="form-row">
        <label class="form-label">声音</label>
        <div class="form-control">
          <div class="btn-group">
            <button 
              type="button"
              class="btn-option"
              :class="{ active: model.sound === true }"
              @click="model.sound = true"
            >开启</button>
            <button 
              type="button"
              class="btn-option"
              :class="{ active: model.sound === false }"
              @click="model.sound = false"
            >关闭</button>
          </div>
        </div>
      </div>
      
      <!-- 图片 -->
      <div class="form-row">
        <label class="form-label">图片</label>
        <div class="form-control">
          <div class="btn-group">
            <button 
              type="button"
              class="btn-option"
              :class="{ active: model.images === true }"
              @click="model.images = true"
            >开启</button>
            <button 
              type="button"
              class="btn-option"
              :class="{ active: model.images === false }"
              @click="model.images = false"
            >关闭</button>
          </div>
        </div>
      </div>
      
      <!-- 视频 -->
      <div class="form-row">
        <label class="form-label">视频</label>
        <div class="form-control">
          <div class="btn-group">
            <button 
              type="button"
              class="btn-option"
              :class="{ active: model.video === true }"
              @click="model.video = true"
            >开启</button>
            <button 
              type="button"
              class="btn-option"
              :class="{ active: model.video === false }"
              @click="model.video = false"
            >关闭</button>
          </div>
        </div>
      </div>
      
      <!-- 窗口大小 -->
      <div class="form-row">
        <label class="form-label">窗口大小</label>
        <div class="form-control">
          <div class="btn-group">
            <button 
              type="button"
              class="btn-option"
              :class="{ active: model.windowSize === 'custom' }"
              @click="model.windowSize = 'custom'"
            >自定义</button>
            <button 
              type="button"
              class="btn-option"
              :class="{ active: model.windowSize === 'fullscreen' }"
              @click="model.windowSize = 'fullscreen'"
            >全屏</button>
          </div>
        </div>
      </div>
      
      <!-- 宽度和高度 -->
      <div v-if="model.windowSize === 'custom'" class="form-row">
        <label class="form-label"></label>
        <div class="form-control">
          <div class="size-inputs">
            <div class="size-input-group">
              <span class="size-label">宽</span>
              <input 
                v-model.number="model.width"
                type="number" 
                class="size-input"
              />
            </div>
            <div class="size-input-group">
              <span class="size-label">高</span>
              <input 
                v-model.number="model.height"
                type="number" 
                class="size-input"
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
}

.form-section {
  display: flex;
  flex-direction: column;
  gap: 20px;
}

.form-row {
  display: grid;
  grid-template-columns: 110px 1fr;
  gap: 16px;
  align-items: center;
}

.form-label {
  font-size: 14px;
  font-weight: 500;
  color: var(--color-text-primary);
  display: flex;
  align-items: center;
  gap: 4px;
  
  .help-icon {
    font-size: 14px;
    color: var(--color-text-tertiary);
    cursor: help;
  }
}

.form-control {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.btn-group {
  display: inline-flex;
  border-radius: 8px;
  overflow: hidden;
  box-shadow: 0 1px 2px rgba(0, 0, 0, 0.05);
}

.btn-option {
  padding: 8px 16px;
  font-size: 14px;
  font-weight: 500;
  border: 1px solid var(--color-border-default);
  background: white;
  color: var(--color-text-secondary);
  cursor: pointer;
  transition: all 0.2s;
  
  .dark & {
    background: var(--color-bg-elevated);
    border-color: var(--color-border-dark);
  }
  
  &:not(:first-child) {
    margin-left: -1px;
  }
  
  &:first-child {
    border-radius: 8px 0 0 8px;
  }
  
  &:last-child {
    border-radius: 0 8px 8px 0;
  }
  
  &:only-child {
    border-radius: 8px;
  }
  
  &:hover:not(.active) {
    background: var(--color-hover-bg);
    color: var(--color-text-primary);
  }
  
  &.active {
    background: #2563eb;
    border-color: #2563eb;
    color: white;
    position: relative;
    z-index: 1;
  }
}

.size-inputs {
  display: flex;
  gap: 16px;
}

.size-input-group {
  display: flex;
  align-items: center;
  gap: 8px;
  
  .size-label {
    font-size: 14px;
    color: var(--color-text-tertiary);
  }
  
  .size-input {
    width: 100px;
    height: 38px;
    padding: 0 12px;
    border: 1px solid var(--color-border-default);
    border-radius: 8px;
    font-size: 14px;
    color: var(--color-text-primary);
    background: white;
    
    .dark & {
      background: var(--color-bg-elevated);
      border-color: var(--color-border-dark);
    }
    
    &:focus {
      outline: none;
      border-color: #2563eb;
      box-shadow: 0 0 0 3px rgba(37, 99, 235, 0.1);
    }
  }
}

// 时区选择器
.timezone-selector,
.language-selector {
  margin-top: 8px;
}

.timezone-select {
  width: 100%;
  height: 38px;
  padding: 0 32px 0 12px;
  border: 1px solid var(--color-border-default);
  border-radius: 8px;
  font-size: 14px;
  color: var(--color-text-primary);
  background: white url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='12' height='12' viewBox='0 0 12 12'%3E%3Cpath fill='%23666' d='M6 9L1 4h10z'/%3E%3C/svg%3E") no-repeat right 12px center;
  background-size: 12px;
  appearance: none;
  cursor: pointer;
  transition: all 0.2s;
  
  .dark & {
    background-color: var(--color-bg-elevated);
    background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='12' height='12' viewBox='0 0 12 12'%3E%3Cpath fill='%23aaa' d='M6 9L1 4h10z'/%3E%3C/svg%3E");
    border-color: var(--color-border-dark);
  }
  
  &:hover {
    border-color: #2563eb;
  }
  
  &:focus {
    outline: none;
    border-color: #2563eb;
    box-shadow: 0 0 0 3px rgba(37, 99, 235, 0.1);
  }
}
</style>
