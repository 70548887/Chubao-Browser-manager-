<script setup lang="ts">
/**
 * @description 创建窗口 - 第四步：代理设置
 */
import type { CreateProfileFormData } from '../../composables/useCreateProfile'
import { PROXY_TYPE_OPTIONS, PROXY_MODE_OPTIONS } from '../../composables/useCreateProfile'

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
        <label class="form-label">每次启动检测</label>
        <div 
          class="toggle-switch" 
          :class="{ active: modelValue.checkIpOnStart }" 
          @click="updateField('checkIpOnStart', !modelValue.checkIpOnStart)"
        >
          <div class="toggle-handle"></div>
        </div>
        <span class="form-help">开启IP检测会影响环境启动速度，如非动态代理无需启动</span>
      </div>
      
      <div class="form-row">
        <label class="form-label">代理方式</label>
        <div class="radio-group">
          <label 
            v-for="item in PROXY_MODE_OPTIONS" 
            :key="item.value" 
            class="radio-item" 
            :class="{ active: modelValue.proxyMode === item.value }"
          >
            <input 
              type="radio" 
              :value="item.value" 
              :checked="modelValue.proxyMode === item.value"
              @change="updateField('proxyMode', item.value)"
            />
            <span>{{ item.label }}</span>
          </label>
        </div>
      </div>
      
      <div class="form-row">
        <label class="form-label">代理类型</label>
        <select 
          :value="modelValue.proxyType"
          class="form-select"
          @change="updateField('proxyType', ($event.target as HTMLSelectElement).value)"
        >
          <option v-for="item in PROXY_TYPE_OPTIONS" :key="item.value" :value="item.value">
            {{ item.label }}
          </option>
        </select>
      </div>
      
      <template v-if="modelValue.proxyType !== 'direct'">
        <div class="form-row">
          <label class="form-label">代理主机</label>
          <input 
            :value="modelValue.proxyHost"
            class="form-input" 
            type="text" 
            placeholder="代理服务器地址"
            @input="updateField('proxyHost', ($event.target as HTMLInputElement).value)"
          />
        </div>
        
        <div class="form-row">
          <label class="form-label">端口</label>
          <input 
            :value="modelValue.proxyPort"
            class="form-input" 
            type="text" 
            placeholder="代理端口"
            @input="updateField('proxyPort', ($event.target as HTMLInputElement).value)"
          />
        </div>
        
        <div class="form-row">
          <label class="form-label">用户名</label>
          <input 
            :value="modelValue.proxyUsername"
            class="form-input" 
            type="text" 
            placeholder="代理认证用户名（可选）"
            @input="updateField('proxyUsername', ($event.target as HTMLInputElement).value)"
          />
        </div>
        
        <div class="form-row">
          <label class="form-label">密码</label>
          <input 
            :value="modelValue.proxyPassword"
            class="form-input" 
            type="password" 
            placeholder="代理认证密码（可选）"
            @input="updateField('proxyPassword', ($event.target as HTMLInputElement).value)"
          />
        </div>
      </template>
      
      <div class="form-row">
        <label class="form-label">使用系统代理</label>
        <div 
          class="toggle-switch" 
          :class="{ active: modelValue.useSystemProxy }" 
          @click="updateField('useSystemProxy', !modelValue.useSystemProxy)"
        >
          <div class="toggle-handle"></div>
        </div>
      </div>
      
      <div class="section-header">
        <h3 class="section-title">直连白名单</h3>
      </div>
      
      <div class="form-row">
        <label class="form-label">跟随全局设置</label>
        <div 
          class="toggle-switch" 
          :class="{ active: modelValue.followGlobal }" 
          @click="updateField('followGlobal', !modelValue.followGlobal)"
        >
          <div class="toggle-handle"></div>
        </div>
      </div>
      
      <div class="form-row">
        <label class="form-label">启用白名单</label>
        <div 
          class="toggle-switch" 
          :class="{ active: modelValue.enableDirectWhitelist }" 
          @click="updateField('enableDirectWhitelist', !modelValue.enableDirectWhitelist)"
        >
          <div class="toggle-handle"></div>
        </div>
        <span class="form-help">开启后允许部分域名不走代理</span>
      </div>
      
      <div v-if="modelValue.enableDirectWhitelist" class="form-row">
        <label class="form-label">白名单域名</label>
        <textarea 
          :value="modelValue.directWhitelist"
          class="form-textarea" 
          rows="4" 
          placeholder="可以输入多个域名，通过换行进行间隔"
          @input="updateField('directWhitelist', ($event.target as HTMLTextAreaElement).value)"
        ></textarea>
      </div>
    </div>
  </div>
</template>

<style scoped lang="scss">
@import './_step-form.scss';
</style>
