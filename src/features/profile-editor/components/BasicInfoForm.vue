<script setup lang="ts">
/**
 * @description 基础信息表单
 */

import { ref, onMounted, computed, watch } from 'vue'
import type { ProfileCreateDTO } from '@/types'
import { useGroupStore } from '@/stores/groupStore'
import { getTags, getProfileTags, setProfileTags, type Tag } from '@/api/tagApi'

interface Props {
  modelValue: ProfileCreateDTO
  profileId?: string  // 编辑模式时传入
}

interface Emits {
  (e: 'update:modelValue', value: ProfileCreateDTO): void
}

const props = defineProps<Props>()
const emit = defineEmits<Emits>()

const formRef = ref()
const groupStore = useGroupStore()

// 标签相关状态
const tagList = ref<Tag[]>([])
const selectedTagIds = ref<string[]>([])
const tagLoading = ref(false)
const showTagDropdown = ref(false)

// 标签颜色配置
const tagColors: Record<string, { bg: string; text: string }> = {
  blue: { bg: '#dbeafe', text: '#2563eb' },
  green: { bg: '#dcfce7', text: '#16a34a' },
  purple: { bg: '#f3e8ff', text: '#9333ea' },
  orange: { bg: '#ffedd5', text: '#ea580c' },
  red: { bg: '#fee2e2', text: '#dc2626' },
  cyan: { bg: '#cffafe', text: '#0891b2' },
  pink: { bg: '#fce7f3', text: '#db2777' },
  yellow: { bg: '#fef9c3', text: '#ca8a04' },
}

const getTagColor = (tag: Tag) => {
  const color = (tag as any).color || 'blue'
  return tagColors[color] || tagColors.blue
}

// 已选中的标签对象列表
const selectedTags = computed(() => {
  if (!selectedTagIds.value.length || !tagList.value.length) return []
  return tagList.value.filter(tag => selectedTagIds.value.includes(tag.id))
})

// 可选的标签（未选中的）
const availableTags = computed(() => {
  if (!tagList.value.length) return []
  return tagList.value.filter(tag => !selectedTagIds.value.includes(tag.id))
})

// 初始化数据
onMounted(async () => {
  groupStore.initGroups()
  
  // 加载标签列表
  try {
    tagLoading.value = true
    tagList.value = await getTags()
  } catch (error) {
    console.error('Failed to load tags:', error)
  } finally {
    tagLoading.value = false
  }
})

// 监听 profileId 变化（切换编辑窗口时）
watch(() => props.profileId, async (newId) => {
  console.log('[BasicInfoForm] profileId 变化:', newId)
  if (newId) {
    try {
      const profileTags = await getProfileTags(newId)
      selectedTagIds.value = profileTags.map(t => t.id)
      console.log('[BasicInfoForm] 已加载窗口标签:', selectedTagIds.value, '标签对象:', profileTags)
    } catch (error) {
      console.error('Failed to load profile tags:', error)
      selectedTagIds.value = []
    }
  } else {
    selectedTagIds.value = []
  }
}, { immediate: true })

// 添加标签
const addTag = (tagId: string) => {
  if (!selectedTagIds.value.includes(tagId)) {
    selectedTagIds.value.push(tagId)
  }
  showTagDropdown.value = false
}

// 移除标签
const removeTag = (tagId: string) => {
  const index = selectedTagIds.value.indexOf(tagId)
  if (index > -1) {
    selectedTagIds.value.splice(index, 1)
  }
}

// 更新表单数据
const updateField = (field: keyof ProfileCreateDTO, value: any) => {
  emit('update:modelValue', {
    ...props.modelValue,
    [field]: value,
  })
}

// 更新偏好设置字段
const updatePreferenceField = (field: string, value: any) => {
  emit('update:modelValue', {
    ...props.modelValue,
    preferences: {
      ...props.modelValue.preferences,
      [field]: value,
    },
  })
}

// 计算启动页类型（默认为 blank）
const startupPageType = computed(() => {
  return props.modelValue.preferences?.startupPage || 'blank'
})

// 保存标签
const saveTags = async () => {
  if (props.profileId) {
    try {
      await setProfileTags(props.profileId, selectedTagIds.value)
    } catch (error) {
      console.error('Failed to save tags:', error)
      throw error
    }
  }
}

// 验证表单
const validate = () => {
  return formRef.value?.validate()
}

// 获取选中的标签 ID
const getSelectedTagIds = () => selectedTagIds.value

// 暴露方法
defineExpose({
  validate,
  saveTags,
  getSelectedTagIds,
})
</script>

<template>
  <div class="step-content">
    <div class="form-section">
      <!-- 窗口名称 -->
      <div class="form-row">
        <label class="form-label">
          窗口名称 <span class="required">*</span>
        </label>
        <div class="form-control">
          <input 
            :value="modelValue.name"
            @input="updateField('name', ($event.target as HTMLInputElement).value)"
            type="text" 
            class="input"
            placeholder="请输入窗口名称，例如: FB-Account-01"
          />
        </div>
      </div>
      
      <!-- 分组 -->
      <div class="form-row">
        <label class="form-label">分组</label>
        <div class="form-control">
          <div class="select-wrapper">
            <select 
              :value="modelValue.group"
              @change="updateField('group', ($event.target as HTMLSelectElement).value)"
              class="select"
            >
              <option value="">请选择分组</option>
              <option 
                v-for="option in groupStore.groupOptions" 
                :key="option.value" 
                :value="option.value"
              >
                {{ option.label }}
              </option>
            </select>
            <span class="material-symbols-outlined select-icon">expand_more</span>
          </div>
        </div>
      </div>
      
      <!-- 标签 -->
      <div class="form-row">
        <label class="form-label">标签 <span class="optional">(选填)</span></label>
        <div class="form-control">
          <div class="tag-input-wrapper">
            <!-- 已选标签列表 -->
            <div 
              v-for="tag in selectedTags" 
              :key="tag.id"
              class="tag-badge"
              :style="{
                '--tag-bg': getTagColor(tag).bg,
                '--tag-color': getTagColor(tag).text
              }"
            >
              <span class="tag-text">{{ tag.name }}</span>
              <span class="tag-remove" @click="removeTag(tag.id)">
                <span class="material-symbols-outlined">close</span>
              </span>
            </div>
            
            <!-- 添加按钮 -->
            <div class="tag-add-wrapper" v-if="availableTags.length > 0">
              <button 
                type="button" 
                class="tag-add-btn"
                @click="showTagDropdown = !showTagDropdown"
              >
                <span class="material-symbols-outlined">add</span>
                <span>添加标签</span>
              </button>
              
              <!-- 下拉选择器 -->
              <div v-if="showTagDropdown" class="tag-dropdown">
                <div 
                  v-for="tag in availableTags" 
                  :key="tag.id"
                  class="tag-option"
                  :style="{
                    '--tag-bg': getTagColor(tag).bg,
                    '--tag-color': getTagColor(tag).text
                  }"
                  @click="addTag(tag.id)"
                >
                  {{ tag.name }}
                </div>
              </div>
            </div>
            
            <!-- 无标签提示 -->
            <span v-if="tagList.length === 0 && !tagLoading" class="tag-empty-hint">
              暂无标签
            </span>
          </div>
        </div>
      </div>
      
      <!-- 备注 -->
      <div class="form-row">
        <label class="form-label">备注</label>
        <div class="form-control">
          <textarea 
            :value="modelValue.remark"
            @input="updateField('remark', ($event.target as HTMLTextAreaElement).value)"
            class="textarea"
            rows="4"
            placeholder="选填，输入备注信息"
          ></textarea>
        </div>
      </div>
      
      <!-- 启动页地址 -->
      <div class="form-row">
        <label class="form-label">
          <span class="material-symbols-outlined label-icon">home</span>
          启动页地址
        </label>
        <div class="form-control">
          <div class="radio-group">
            <label class="radio-item" :class="{ active: startupPageType === 'blank' }">
              <input
                type="radio"
                name="startupPage"
                :checked="startupPageType === 'blank'"
                @change="updatePreferenceField('startupPage', 'blank')"
              />
              <span class="radio-indicator"></span>
              <span class="radio-label">空白页</span>
            </label>
            <label class="radio-item" :class="{ active: startupPageType === 'url' }">
              <input
                type="radio"
                name="startupPage"
                :checked="startupPageType === 'url'"
                @change="updatePreferenceField('startupPage', 'url')"
              />
              <span class="radio-indicator"></span>
              <span class="radio-label">指定网页</span>
            </label>
          </div>
          <input
            v-if="startupPageType === 'url'"
            :value="modelValue.preferences?.startupUrl"
            @input="updatePreferenceField('startupUrl', ($event.target as HTMLInputElement).value)"
            type="text"
            class="input mt-2"
            placeholder="输入网址，例如：https://www.google.com"
          />
        </div>
      </div>
      
      <!-- 导入 Cookie -->
      <div class="form-row">
        <label class="form-label">导入 Cookie</label>
        <div class="form-control">
          <div class="cookie-header">
            <span></span>
            <div class="cookie-actions">
              <a href="#" class="link">格式说明</a>
              <a href="#" class="link link-primary">检测格式</a>
            </div>
          </div>
          <textarea 
            :value="modelValue.cookies"
            @input="updateField('cookies', ($event.target as HTMLTextAreaElement).value)"
            class="textarea code-textarea"
            rows="6"
            placeholder='[{"domain": ".facebook.com", "expirationDate": 1718915444, "name": "c_user", "path": "/", "value": "1000384728472"}, ...]'
          ></textarea>
          <div class="form-hint">
            <span class="material-symbols-outlined hint-icon">info</span>
            支持 JSON 或 Netscape 格式的 Cookie 数据，导入后将自动同步到云端。
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
  color: var(--color-text-primary, #1e293b);
  padding-top: 10px;
  display: flex;
  align-items: center;
  gap: 6px;
  
  .required {
    color: #ef4444;
    margin-left: 2px;
  }
  
  .label-icon {
    font-size: 18px;
    color: #64748b;
  }
}

.form-control {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

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

.textarea {
  width: 100%;
  padding: 12px;
  border: 1px solid #e2e8f0;
  border-radius: 8px;
  font-size: 14px;
  color: #1e293b;
  background: white;
  resize: vertical;
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

.code-textarea {
  font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
  font-size: 13px;
  line-height: 1.5;
  background: #f8fafc;
}

.cookie-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.cookie-actions {
  display: flex;
  gap: 16px;
}

.link {
  font-size: 12px;
  color: #94a3b8;
  text-decoration: none;
  
  &:hover {
    text-decoration: underline;
  }
  
  &.link-primary {
    color: #2563eb;
    font-weight: 500;
  }
}

.form-hint {
  display: flex;
  align-items: flex-start;
  gap: 6px;
  font-size: 12px;
  color: #94a3b8;
  
  .hint-icon {
    font-size: 14px;
    color: #2563eb;
    flex-shrink: 0;
    margin-top: 1px;
  }
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
}

.radio-indicator {
  width: 18px;
  height: 18px;
  border: 2px solid #cbd5e1;
  border-radius: 50%;
  position: relative;
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

.radio-label {
  font-size: 14px;
  color: #374151;
}

.mt-2 {
  margin-top: 8px;
}

// 标签输入器
.tag-input-wrapper {
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  gap: 8px;
  min-height: 36px;
}

// 已选标签徽章
.tag-badge {
  display: inline-flex;
  align-items: center;
  gap: 4px;
  padding: 4px 8px 4px 12px;
  background: var(--tag-bg);
  color: var(--tag-color);
  border-radius: 16px;
  font-size: 13px;
  font-weight: 500;
  position: relative;
  
  .tag-text {
    line-height: 1.2;
  }
  
  .tag-remove {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 18px;
    height: 18px;
    border-radius: 50%;
    cursor: pointer;
    opacity: 0;
    transition: all 0.2s;
    background: rgba(0, 0, 0, 0.1);
    
    .material-symbols-outlined {
      font-size: 14px;
    }
  }
  
  &:hover .tag-remove {
    opacity: 1;
  }
}

// 添加按钮包装器
.tag-add-wrapper {
  position: relative;
}

// 添加按钮
.tag-add-btn {
  display: inline-flex;
  align-items: center;
  gap: 4px;
  padding: 4px 12px;
  background: #f1f5f9;
  border: 1px dashed #cbd5e1;
  border-radius: 16px;
  font-size: 13px;
  color: #64748b;
  cursor: pointer;
  transition: all 0.2s;
  
  .material-symbols-outlined {
    font-size: 16px;
  }
  
  &:hover {
    background: #e2e8f0;
    border-color: #94a3b8;
    color: #475569;
  }
}

// 下拉选择器
.tag-dropdown {
  position: absolute;
  top: calc(100% + 4px);
  left: 0;
  min-width: 150px;
  max-height: 200px;
  overflow-y: auto;
  background: white;
  border: 1px solid #e2e8f0;
  border-radius: 8px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
  z-index: 100;
}

// 下拉选项
.tag-option {
  padding: 8px 12px;
  font-size: 13px;
  cursor: pointer;
  transition: background 0.15s;
  
  &:hover {
    background: var(--tag-bg);
    color: var(--tag-color);
  }
  
  &:first-child {
    border-radius: 7px 7px 0 0;
  }
  
  &:last-child {
    border-radius: 0 0 7px 7px;
  }
  
  &:only-child {
    border-radius: 7px;
  }
}

.tag-empty-hint {
  font-size: 13px;
  color: #94a3b8;
}

.optional {
  font-weight: 400;
  color: #94a3b8;
  font-size: 12px;
}
</style>
