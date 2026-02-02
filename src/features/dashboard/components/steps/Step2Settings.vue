<script setup lang="ts">
/**
 * @description 创建窗口 - 第二步：基础设置
 */
import type { CreateProfileFormData } from '../../composables/useCreateProfile'
import { 
  LANGUAGE_OPTIONS, 
  TIMEZONE_OPTIONS, 
  GEOLOCATION_PROMPT_OPTIONS, 
  WINDOW_SIZE_OPTIONS 
} from '../../composables/useCreateProfile'

const props = defineProps<{
  modelValue: CreateProfileFormData
}>()

const emit = defineEmits<{
  (e: 'update:modelValue', value: CreateProfileFormData): void
}>()

// 双向绑定辅助
const updateField = <K extends keyof CreateProfileFormData>(key: K, value: CreateProfileFormData[K]) => {
  emit('update:modelValue', { ...props.modelValue, [key]: value })
}
</script>

<template>
  <div class="step-pane">
    <div class="form-section">
      <div class="form-row">
        <label class="form-label">语言</label>
        <select 
          :value="modelValue.language"
          class="form-select"
          @change="updateField('language', ($event.target as HTMLSelectElement).value)"
        >
          <option v-for="item in LANGUAGE_OPTIONS" :key="item.value" :value="item.value">
            {{ item.label }}
          </option>
        </select>
      </div>
      
      <div class="form-row">
        <label class="form-label">界面语言</label>
        <select 
          :value="modelValue.interfaceLanguage"
          class="form-select"
          @change="updateField('interfaceLanguage', ($event.target as HTMLSelectElement).value)"
        >
          <option v-for="item in LANGUAGE_OPTIONS" :key="item.value" :value="item.value">
            {{ item.label }}
          </option>
        </select>
      </div>
      
      <div class="form-row">
        <label class="form-label">时区</label>
        <select 
          :value="modelValue.timezone"
          class="form-select"
          @change="updateField('timezone', ($event.target as HTMLSelectElement).value)"
        >
          <option v-for="item in TIMEZONE_OPTIONS" :key="item.value" :value="item.value">
            {{ item.label }}
          </option>
        </select>
      </div>
      
      <div class="form-row">
        <label class="form-label">地理位置提示</label>
        <select 
          :value="modelValue.geolocationPrompt"
          class="form-select"
          @change="updateField('geolocationPrompt', ($event.target as HTMLSelectElement).value)"
        >
          <option v-for="item in GEOLOCATION_PROMPT_OPTIONS" :key="item.value" :value="item.value">
            {{ item.label }}
          </option>
        </select>
      </div>
      
      <div class="form-row">
        <label class="form-label">地理位置</label>
        <select 
          :value="modelValue.geolocation"
          class="form-select"
          @change="updateField('geolocation', ($event.target as HTMLSelectElement).value)"
        >
          <option value="auto">自动获取</option>
          <option value="custom">自定义</option>
        </select>
      </div>
      
      <div class="form-row">
        <label class="form-label">媒体权限</label>
        <div class="checkbox-group">
          <label class="checkbox-item">
            <input 
              type="checkbox" 
              :checked="modelValue.sound"
              @change="updateField('sound', ($event.target as HTMLInputElement).checked)"
            />
            <span>声音</span>
          </label>
          <label class="checkbox-item">
            <input 
              type="checkbox" 
              :checked="modelValue.images"
              @change="updateField('images', ($event.target as HTMLInputElement).checked)"
            />
            <span>图片</span>
          </label>
          <label class="checkbox-item">
            <input 
              type="checkbox" 
              :checked="modelValue.video"
              @change="updateField('video', ($event.target as HTMLInputElement).checked)"
            />
            <span>视频</span>
          </label>
        </div>
      </div>
      
      <div class="form-row">
        <label class="form-label">窗口大小</label>
        <select 
          :value="modelValue.windowSize"
          class="form-select"
          @change="updateField('windowSize', ($event.target as HTMLSelectElement).value)"
        >
          <option v-for="item in WINDOW_SIZE_OPTIONS" :key="item.value" :value="item.value">
            {{ item.label }}
          </option>
        </select>
      </div>
      
      <div v-if="modelValue.windowSize === 'custom'" class="form-row">
        <label class="form-label">自定义尺寸</label>
        <div class="size-inputs">
          <input 
            :value="modelValue.customWidth"
            class="form-input small" 
            type="number" 
            min="800" 
            max="3840" 
            placeholder="宽度"
            @input="updateField('customWidth', parseInt(($event.target as HTMLInputElement).value) || 1920)"
          />
          <span class="size-separator">×</span>
          <input 
            :value="modelValue.customHeight"
            class="form-input small" 
            type="number" 
            min="600" 
            max="2160" 
            placeholder="高度"
            @input="updateField('customHeight', parseInt(($event.target as HTMLInputElement).value) || 1080)"
          />
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped lang="scss">
@import './_step-form.scss';
</style>
