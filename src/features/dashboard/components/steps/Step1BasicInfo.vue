<script setup lang="ts">
/**
 * @description 创建窗口 - 第一步：窗口信息
 */
import type { CreateProfileFormData } from '../../composables/useCreateProfile'

const props = defineProps<{
  modelValue: CreateProfileFormData
  groupOptions: Array<{ label: string; value: string }>
}>()

const emit = defineEmits<{
  (e: 'update:modelValue', value: CreateProfileFormData): void
  (e: 'generateRandomName'): void
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
        <label class="form-label required">窗口名称</label>
        <div class="input-with-action">
          <input 
            :value="modelValue.name"
            class="form-input" 
            type="text" 
            placeholder="自定义浏览器窗口名称"
            @input="updateField('name', ($event.target as HTMLInputElement).value)"
          />
          <button type="button" class="action-btn" @click="emit('generateRandomName')" title="随机生成">
            <svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor">
              <path d="M8 3a5 5 0 1 0 0 10A5 5 0 0 0 8 3zm0 9a4 4 0 1 1 0-8 4 4 0 0 1 0 8z"/>
              <path d="M8 7v2l2 1"/>
            </svg>
          </button>
        </div>
      </div>
      
      <div class="form-row">
        <label class="form-label">选择分组</label>
        <select 
          :value="modelValue.group"
          class="form-select"
          @change="updateField('group', ($event.target as HTMLSelectElement).value)"
        >
          <option value="default">默认分组</option>
          <option v-for="item in groupOptions" :key="item.value" :value="item.value">
            {{ item.label }}
          </option>
        </select>
      </div>
      
      <div class="form-row">
        <label class="form-label">Cookie</label>
        <textarea 
          :value="modelValue.cookie"
          class="form-textarea" 
          rows="4" 
          placeholder="粘贴Cookie内容，支持JSON格式和Netscape格式"
          @input="updateField('cookie', ($event.target as HTMLTextAreaElement).value)"
        ></textarea>
      </div>
      
      <div class="form-row">
        <label class="form-label">备注</label>
        <textarea 
          :value="modelValue.remark"
          class="form-textarea" 
          rows="3" 
          placeholder="添加备注信息"
          @input="updateField('remark', ($event.target as HTMLTextAreaElement).value)"
        ></textarea>
      </div>
      
      <!-- 启动页地址 -->
      <div class="form-row">
        <label class="form-label">
          <span class="material-symbols-outlined label-icon">home</span>
          启动页地址
        </label>
        <div class="radio-btn-group">
          <label class="radio-btn-item" :class="{ active: modelValue.startupPage === 'blank' }">
            <input 
              type="radio" 
              :checked="modelValue.startupPage === 'blank'"
              @change="updateField('startupPage', 'blank')" 
            />
            <span class="radio-btn-indicator"></span>
            <span class="radio-btn-label">空白页</span>
          </label>
          <label class="radio-btn-item" :class="{ active: modelValue.startupPage === 'url' }">
            <input 
              type="radio" 
              :checked="modelValue.startupPage === 'url'"
              @change="updateField('startupPage', 'url')" 
            />
            <span class="radio-btn-indicator"></span>
            <span class="radio-btn-label">指定网页</span>
          </label>
        </div>
      </div>
      
      <!-- 启动页网址输入框 -->
      <div v-if="modelValue.startupPage === 'url'" class="form-row">
        <label class="form-label"></label>
        <input 
          :value="modelValue.startupUrl"
          class="form-input" 
          type="text" 
          placeholder="输入网址，例如：https://www.google.com"
          @input="updateField('startupUrl', ($event.target as HTMLInputElement).value)"
        />
      </div>
    </div>
  </div>
</template>

<style scoped lang="scss">
@import './_step-form.scss';
</style>
