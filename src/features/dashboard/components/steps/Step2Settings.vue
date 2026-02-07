<script setup lang="ts">
/**
 * @description 创建窗口 - 第二步：基础设置
 */
import { ref, watch } from 'vue'
import type { CreateProfileFormData } from '../../composables/useCreateProfile'
import { 
  LANGUAGE_OPTIONS, 
  TIMEZONE_OPTIONS, 
  GEOLOCATION_PROMPT_OPTIONS, 
  WINDOW_SIZE_OPTIONS 
} from '../../composables/useCreateProfile'
import { IP_GEO_API_URL } from '@/config'

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

// IP 地理位置查询状态
const ipGeoLoading = ref(false)
const ipGeoError = ref('')
const ipGeoInfo = ref<{
  ip: string
  country: string
  city: string
  lat: number
  lon: number
  timezone: string
} | null>(null)

// 根据 IP 获取地理位置
async function fetchGeoByIP() {
  ipGeoLoading.value = true
  ipGeoError.value = ''
  
  try {
    // 使用统一配置的 IP 查询 API
    const response = await fetch(`${IP_GEO_API_URL}/?fields=status,message,country,city,lat,lon,timezone,query`)
    const data = await response.json()
    
    if (data.status === 'success') {
      ipGeoInfo.value = {
        ip: data.query,
        country: data.country,
        city: data.city,
        lat: data.lat,
        lon: data.lon,
        timezone: data.timezone
      }
      
      // 自动填充经纬度
      emit('update:modelValue', {
        ...props.modelValue,
        geolocationLatitude: data.lat,
        geolocationLongitude: data.lon
      })
    } else {
      ipGeoError.value = data.message || '获取失败'
    }
  } catch (err) {
    ipGeoError.value = '网络请求失败'
    console.error('IP 地理位置查询失败:', err)
  } finally {
    ipGeoLoading.value = false
  }
}

// 当选择"基于 IP"时自动查询
watch(() => props.modelValue.geolocation, (newVal) => {
  if (newVal === 'ip') {
    fetchGeoByIP()
  }
})
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
          <option value="ip">基于 IP 匹配</option>
          <option value="custom">自定义</option>
        </select>
      </div>
      
      <!-- 基于 IP 匹配结果 -->
      <div v-if="modelValue.geolocation === 'ip'" class="form-row geo-ip-result">
        <div v-if="ipGeoLoading" class="geo-loading">
          <span class="loading-spinner"></span>
          <span>正在获取 IP 地理位置...</span>
        </div>
        <div v-else-if="ipGeoError" class="geo-error">
          <span>❌ {{ ipGeoError }}</span>
          <button type="button" class="btn-retry" @click="fetchGeoByIP">重试</button>
        </div>
        <div v-else-if="ipGeoInfo" class="geo-info">
          <div class="geo-info-row">
            <span class="geo-label">IP:</span>
            <span class="geo-value">{{ ipGeoInfo.ip }}</span>
          </div>
          <div class="geo-info-row">
            <span class="geo-label">位置:</span>
            <span class="geo-value">{{ ipGeoInfo.country }} - {{ ipGeoInfo.city }}</span>
          </div>
          <div class="geo-info-row">
            <span class="geo-label">经纬度:</span>
            <span class="geo-value">{{ ipGeoInfo.lat }}, {{ ipGeoInfo.lon }}</span>
          </div>
          <div class="geo-info-row">
            <span class="geo-label">时区:</span>
            <span class="geo-value">{{ ipGeoInfo.timezone }}</span>
          </div>
          <button type="button" class="btn-refresh" @click="fetchGeoByIP">重新获取</button>
        </div>
      </div>
      
      <!-- 自定义经纬度输入 -->
      <div v-if="modelValue.geolocation === 'custom'" class="form-row geo-inputs">
        <div class="geo-input-group">
          <label class="form-label-small">纬度</label>
          <input 
            type="number" 
            step="0.0001"
            :value="modelValue.geolocationLatitude"
            class="form-input geo-input"
            placeholder="如: 31.2304"
            @input="updateField('geolocationLatitude', Number(($event.target as HTMLInputElement).value))"
          />
        </div>
        <div class="geo-input-group">
          <label class="form-label-small">经度</label>
          <input 
            type="number" 
            step="0.0001"
            :value="modelValue.geolocationLongitude"
            class="form-input geo-input"
            placeholder="如: 121.4737"
            @input="updateField('geolocationLongitude', Number(($event.target as HTMLInputElement).value))"
          />
        </div>
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
