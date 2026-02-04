<script setup lang="ts">
/**
 * @description 环境配置抽屉 - 主组件 (桌面原生化)
 * @author DeepAgent
 */

import { ref, watch, computed } from 'vue'
import { Message } from '@/utils/message'
import { useProfileStore } from '@/stores/profile.store'
import { useProfileForm } from './composables/useProfileForm'
import BasicInfoForm from './components/BasicInfoForm.vue'
import BasicSettingsForm from './components/BasicSettingsForm.vue'
import AdvancedFingerprintForm from './components/AdvancedFingerprintForm.vue'
import ProxyForm from './components/ProxyForm.vue'
import PreferencesForm from './components/PreferencesForm.vue'
import type { Profile } from '@/types'
import { filterFingerprintWhitelist, detectFingerprintBlacklist } from '@/config/fingerprint.config'
import { calculateFingerprintPatch } from '@/utils/fingerprintDiff'
import { getProfileTags } from '@/api/tagApi'

interface Props {
  visible: boolean
  profile?: Profile
}

interface Emits {
  (e: 'update:visible', value: boolean): void
  (e: 'success'): void
}

const props = defineProps<Props>()
const emit = defineEmits<Emits>()

const profileStore = useProfileStore()

// 保存原始数据（用于 diff）
const originalFingerprint = ref<any>(undefined)
const originalName = ref<string>('')
const originalGroup = ref<string>('')
const originalRemark = ref<string>('')

// 表单逻辑
const {
  formData,
  currentStep,
  // isEditMode 不再使用 composable 中的，因为它不是响应式的
  regenerateFingerprint,
  nextStep,
  prevStep,
  resetForm,
} = useProfileForm(props.profile)

// 直接依赖 props.profile 计算编辑模式（响应式）
const isEditMode = computed(() => !!props.profile)

// 表单引用
const basicFormRef = ref()

// 加载状态
const loading = ref(false)

// 判断基础信息是否已填写（实时）
const isBasicInfoValid = computed(() => {
  return !!(formData.value.name && formData.value.name.trim())
})

// 步骤配置
const steps = [
  { title: '窗口信息', icon: 'Document' },
  { title: '基础设置', icon: 'Setting' },
  { title: '高级指纹设置', icon: 'Fingerprint' },
  { title: '代理设置', icon: 'Connection' },
  { title: '偏好设置', icon: 'Tools' },
]

// 跳转到指定步骤
const goToStep = async (index: number) => {
  // 第一步始终可以跳转
  if (index === 0) {
    currentStep.value = index
    return
  }

  // 跳转到其他步骤前，检查窗口信息是否填写
  if (!formData.value.name || !formData.value.name.trim()) {
    Message.warning('请先填写窗口名称')
    return
  }

  // 验证通过，允许跳转
  currentStep.value = index
}

// 关闭抽屉
const handleClose = () => {
  emit('update:visible', false)
  setTimeout(() => {
    // 如果抽屉已经重新打开，不要重置表单（避免清空新加载的数据）
    if (!props.visible) {
      resetForm()
    }
  }, 300)
}

// 下一步
const handleNext = async () => {
  // 第一步需要验证
  if (currentStep.value === 0) {
    try {
      await basicFormRef.value?.validate()
      nextStep()
    } catch (error) {
      Message.warning('请填写必填项')
    }
  } else {
    nextStep()
  }
}

// 上一步
const handlePrev = () => {
  prevStep()
}

// 检查标签是否有变化
const checkTagChanges = async () => {
  if (!basicFormRef.value || !props.profile) return false
  
  try {
    // 获取当前选中的标签
    const currentTagIds = basicFormRef.value.getSelectedTagIds()
    
    // 获取原始标签（从数据库查询）
    const originalTags = await getProfileTags(props.profile.id)
    const originalIds = originalTags.map(t => t.id)
    
    // 比较数组
    if (currentTagIds.length !== originalIds.length) {
      console.log('[标签变化] 数量不同:', currentTagIds.length, 'vs', originalIds.length)
      return true
    }
    
    // 排序后比较
    const sortedCurrent = [...currentTagIds].sort()
    const sortedOriginal = [...originalIds].sort()
    
    const hasChanges = sortedCurrent.some((id, index) => id !== sortedOriginal[index])
    if (hasChanges) {
      console.log('[标签变化] 内容不同:', sortedCurrent, 'vs', sortedOriginal)
    }
    
    return hasChanges
  } catch (error) {
    console.error('Failed to check tag changes:', error)
    return false
  }
}

// 保存
const handleSave = async () => {
  try {
    loading.value = true

    // 检查指纹黑名单字段
    if (formData.value.fingerprint) {
      const blacklistFields = detectFingerprintBlacklist(formData.value.fingerprint as any)
      if (blacklistFields.length > 0) {
        Message.error(`检测到危险字段：${blacklistFields.join(', ')}，已自动过滤`)
      }

      // 过滤指纹字段，只保留白名单
      formData.value.fingerprint = filterFingerprintWhitelist(formData.value.fingerprint as any) as any
    }

    if (isEditMode.value && props.profile) {
      // 编辑模式 - 增量更新，只提交有变化的字段

      // 检查指纹是否有变更
      const fingerprintPatch = calculateFingerprintPatch(
        originalFingerprint.value,
        formData.value.fingerprint
      )

      // 构建更新数据 - 只添加有变化的字段
      const updateData: any = {}

      // 检查名称是否有变化
      if (formData.value.name !== originalName.value) {
        updateData.name = formData.value.name
        console.log('[Update] 名称有变化:', originalName.value, '->', formData.value.name)
      }

      // 检查分组是否有变化
      if (formData.value.group !== originalGroup.value) {
        updateData.group = formData.value.group
        console.log('[Update] 分组有变化:', originalGroup.value, '->', formData.value.group)
      }

      // 检查备注是否有变化
      if (formData.value.remark !== originalRemark.value) {
        updateData.remark = formData.value.remark
        console.log('[Update] 备注有变化')
      }

      // 如果指纹有变更，提交完整的指纹数据（后端需要完整结构）
      if (fingerprintPatch) {
        // 提交完整的 fingerprint，不是 patch
        updateData.fingerprint = formData.value.fingerprint
        console.log('[Fingerprint Update] 提交完整指纹数据')
      }

      // proxy相关（暂时全量提交）
      if (formData.value.proxy) {
        updateData.proxy = formData.value.proxy
      }

      // 检查标签是否有变化
      const hasTagChanges = await checkTagChanges()

      // 检查是否有任何变化
      if (Object.keys(updateData).length === 0 && !hasTagChanges) {
        Message.info('没有修改内容')
        handleClose()
        return
      }

      // 如果有 profile 字段变化，才调用 updateProfile
      if (Object.keys(updateData).length > 0) {
        await profileStore.updateProfile(props.profile.id, updateData)
      }
      
      // 保存标签关联（如果有变化）
      if (hasTagChanges) {
        try {
          await basicFormRef.value?.saveTags()
          console.log('[编辑] 标签已保存')
        } catch (error) {
          console.error('Failed to save tags:', error)
          Message.error('标签保存失败')
        }
      }
      
      // 注意：不在这里显示成功消息，eventListener.ts 会监听 profile:updated 事件并显示
    } else {
      // 创建模式
      await profileStore.createProfile(formData.value)
      // 注意：不在这里显示成功消息，eventListener.ts 会监听 profile:created 事件并显示
    }

    emit('success')
    handleClose()
  } catch (error: any) {
    Message.error(error.message || '操作失败')
  } finally {
    loading.value = false
  }
}

// 默认基础设置
const defaultBasicSettings = {
  language: 'auto' as const,
  displayLanguage: 'auto' as const,
  timezone: 'auto' as const,
  geolocationPrompt: 'ask' as const,
  geolocation: 'auto' as const,
  audio: true,
  image: true,
  video: true,
  windowSize: 'custom' as const,
  windowWidth: 1200,
  windowHeight: 800,
}

// 默认偏好设置
const defaultPreferences = {
  windowName: false,
  customBookmarks: false,
  extensions: [] as string[],
  startupPage: 'blank' as const,
  startupUrl: '',
  syncBookmarks: false,
  syncHistory: false,
  syncTabs: false,
  syncCookies: false,
  syncExtensions: false,
  syncPasswords: false,
  syncIndexedDB: false,
  syncLocalStorage: false,
  syncSessionStorage: false,
  clearCacheOnStart: false,
  clearCookiesOnStart: false,
  clearLocalStorageOnStart: false,
  clearHistoryOnExit: false,
  clearCookiesOnExit: false,
  clearCacheOnExit: false,
  cloudSync: false,
  cloudSyncExtensions: false,
  cloudSyncBookmarks: false,
  randomFingerprintOnStart: false,
  showPasswordSavePrompt: true,
  stopOnNetworkError: false,
  stopOnIpChange: false,
  stopOnCountryChange: false,
  openWorkbench: false,
  ipChangeNotification: false,
  enableGoogleLogin: false,
}

// 监听 profile 变化，重新初始化表单
watch(() => props.profile, (newProfile) => {
  if (newProfile) {
    formData.value = {
      name: newProfile.name,
      group: newProfile.group,
      remark: newProfile.remark,
      fingerprint: newProfile.fingerprint,
      proxy: newProfile.proxy,
      basicSettings: newProfile.basicSettings || { ...defaultBasicSettings },
      preferences: newProfile.preferences || { ...defaultPreferences },
    }
    // 保存原始数据（用于 diff）
    originalFingerprint.value = JSON.parse(JSON.stringify(newProfile.fingerprint || {}))
    originalName.value = newProfile.name || ''
    originalGroup.value = newProfile.group || ''
    originalRemark.value = newProfile.remark || ''
    console.log('[Edit] 加载窗口数据:', newProfile.name)
  }
}, { immediate: true })
</script>

<template>
  <el-drawer :model-value="visible" @update:model-value="emit('update:visible', $event)"
    :title="isEditMode ? '编辑窗口' : '新建窗口'" size="600px" :close-on-click-modal="false" class="native-drawer"
    @close="handleClose">
    <div class="drawer-content">
      <!-- 步骤指示器 - 可点击 -->
      <div class="steps-wrapper">
        <div v-for="(step, index) in steps" :key="index" class="step-item" :class="{
          active: currentStep === index,
          completed: currentStep > index,
          clickable: index === 0 || isBasicInfoValid
        }" @click="goToStep(index)">
          <div class="step-indicator">
            <el-icon v-if="currentStep > index" class="check-icon">
              <Check />
            </el-icon>
            <span v-else class="step-number">{{ index + 1 }}</span>
          </div>
          <span class="step-title">{{ step.title }}</span>
          <div v-if="index < steps.length - 1" class="step-connector"></div>
        </div>
      </div>

      <!-- 表单内容 -->
      <div class="form-container">
        <!-- 第1步：窗口信息 -->
        <div v-show="currentStep === 0" class="form-section">
          <BasicInfoForm ref="basicFormRef" v-model="formData" :profile-id="profile?.id" />
        </div>

        <!-- 第2步：基础设置 -->
        <div v-show="currentStep === 1" class="form-section">
          <BasicSettingsForm v-model="formData.basicSettings!" />
        </div>

        <!-- 第3步：高级指纹设置 -->
        <div v-show="currentStep === 2" class="form-section">
          <AdvancedFingerprintForm v-model="formData" @regenerate="regenerateFingerprint" />
        </div>

        <!-- 第4步：代理设置 -->
        <div v-show="currentStep === 3" class="form-section">
          <ProxyForm v-model="formData" />
        </div>

        <!-- 第5步：偏好设置 -->
        <div v-show="currentStep === 4" class="form-section">
          <PreferencesForm v-model="formData.preferences!" />
        </div>
      </div>
    </div>

    <!-- 底部操作按钮 -->
    <template #footer>
      <div class="drawer-footer">
        <button class="footer-btn cancel" @click="handleClose">取消</button>
        <button v-if="currentStep > 0" class="footer-btn secondary" @click="handlePrev">
          <el-icon>
            <ArrowLeft />
          </el-icon>
          <span>上一步</span>
        </button>
        <button v-if="currentStep < 4" class="footer-btn primary" @click="handleNext">
          <span>下一步</span>
          <el-icon>
            <ArrowRight />
          </el-icon>
        </button>
        <button v-else class="footer-btn primary" :disabled="loading" @click="handleSave">
          <el-icon v-if="loading" class="is-loading">
            <Loading />
          </el-icon>
          <span>{{ isEditMode ? '保存更改' : '创建环境' }}</span>
        </button>
      </div>
    </template>
  </el-drawer>
</template>

<style scoped lang="scss">
.drawer-content {
  display: flex;
  flex-direction: column;
  height: 100%;
  background: white; // 白色背景
}

// 步骤指示器（横向布局）
.steps-wrapper {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: var(--spacing-xl) var(--spacing-2xl);
  border-bottom: 1px solid #E5E5E5;
  background: white;
  position: relative;
}

.step-item {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: var(--spacing-sm);
  position: relative;
  flex: 1;
  transition: all var(--duration-fast) var(--ease-out-quart);

  // 可点击样式
  &.clickable {
    cursor: pointer;

    &:hover {
      .step-indicator {
        transform: scale(1.1);
        box-shadow: 0 2px 8px rgba(22, 119, 255, 0.3);
      }
    }

    &:active {
      .step-indicator {
        transform: scale(0.95);
      }
    }
  }

  // 圆形指示器
  .step-indicator {
    width: 36px;
    height: 36px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: #F5F5F5; // 默认灰色
    border: none;
    border-radius: var(--radius-full);
    color: #999; // 灰色文字
    font-size: var(--text-md);
    font-weight: var(--font-semibold);
    transition: all var(--duration-fast) var(--ease-out-quart);
    z-index: 2;
  }

  // 步骤标题
  .step-title {
    font-size: var(--text-sm);
    font-weight: var(--font-medium);
    color: #999; // 默认灰色
    transition: color var(--duration-fast);
    white-space: nowrap;
  }

  // 连接线（绝对定位）
  .step-connector {
    position: absolute;
    top: 18px; // 居中对齐圆形
    left: calc(50% + 18px); // 从圆形右边开始
    width: calc(100% - 36px); // 到下一个圆形左边
    height: 2px;
    background: #E5E5E5; // 默认灰色
    transition: background var(--duration-fast);
    z-index: 1;
  }

  // 最后一个步骤无连接线
  &:last-child .step-connector {
    display: none;
  }

  // 激活状态
  &.active {
    .step-indicator {
      background: var(--color-accent-blue); // 蓝色
      color: white;
      box-shadow: 0 4px 12px rgba(22, 119, 255, 0.3);
    }

    .step-title {
      color: var(--color-accent-blue); // 蓝色文字
      font-weight: var(--font-semibold);
    }
  }

  // 已完成状态
  &.completed {
    .step-indicator {
      background: var(--color-accent-blue);
      color: white;
    }

    .step-title {
      color: var(--color-text-primary); // 深色文字
    }

    .step-connector {
      background: var(--color-accent-blue); // 蓝色连接线
    }

    .check-icon {
      font-size: 16px;
    }
  }
}

// 表单容器
.form-container {
  flex: 1;
  overflow-y: auto;
  padding: 0;
  background: white;

  // 自定义滚动条
  &::-webkit-scrollbar {
    width: 6px;
  }

  &::-webkit-scrollbar-track {
    background: transparent;
  }

  &::-webkit-scrollbar-thumb {
    background: rgba(0, 0, 0, 0.1);
    border-radius: 3px;

    &:hover {
      background: rgba(0, 0, 0, 0.2);
    }
  }
}

.form-section {
  animation: fade-in var(--duration-normal) var(--ease-out-quart);
}

@keyframes fade-in {
  from {
    opacity: 0;
    transform: translateY(10px);
  }

  to {
    opacity: 1;
    transform: translateY(0);
  }
}

// 底部按钮栏
.drawer-footer {
  display: flex;
  justify-content: flex-end;
  gap: var(--spacing-md);
  padding: var(--spacing-lg) var(--spacing-2xl);
  border-top: 1px solid #E5E5E5;
  background: white;
  box-shadow: 0 -2px 8px rgba(0, 0, 0, 0.04); // 微妙阴影
}

.footer-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: var(--spacing-sm);
  height: 36px;
  padding: 0 var(--spacing-xl);
  min-width: 80px;
  border: none;
  border-radius: var(--radius-md);
  font-size: var(--text-base);
  font-weight: var(--font-medium);
  cursor: pointer;
  transition: all var(--duration-fast) var(--ease-out-quart);

  .el-icon {
    font-size: 14px;
  }

  // 取消按钮（灰色边框）
  &.cancel {
    background: white;
    color: var(--color-text-secondary);
    border: 1px solid #D9D9D9;

    &:hover {
      background: #FAFAFA;
      border-color: #999;
      color: var(--color-text-primary);
    }

    &:active {
      transform: scale(0.98);
    }
  }

  // 上一步按钮
  &.secondary {
    background: white;
    color: var(--color-text-primary);
    border: 1px solid #D9D9D9;

    &:hover {
      background: #FAFAFA;
      border-color: #999;
    }

    &:active {
      transform: scale(0.98);
    }
  }

  // 下一步/完成按钮（蓝色）
  &.primary {
    background: var(--color-accent-blue);
    color: white;
    box-shadow: 0 2px 6px rgba(22, 119, 255, 0.3);
    min-width: 100px;

    &:hover:not(:disabled) {
      background: var(--color-accent-blue-hover);
      transform: translateY(-1px);
      box-shadow: 0 4px 12px rgba(22, 119, 255, 0.4);
    }

    &:active:not(:disabled) {
      transform: translateY(0);
    }

    &:disabled {
      opacity: 0.6;
      cursor: not-allowed;
      box-shadow: none;
    }
  }
}

// Drawer 头部样式（简化）
:deep(.el-drawer__header) {
  margin-bottom: 0;
  padding: var(--spacing-lg) var(--spacing-2xl);
  border-bottom: 1px solid #E5E5E5;
  background: white;

  .el-drawer__title {
    font-size: var(--text-xl);
    font-weight: var(--font-semibold);
    color: var(--color-text-primary);
  }

  .el-drawer__close-btn {
    color: var(--color-text-tertiary);
    font-size: 20px;

    &:hover {
      color: var(--color-text-primary);
    }
  }
}

:deep(.el-drawer__body) {
  padding: 0;
  background: white;
  display: flex;
  flex-direction: column;
}

:deep(.el-drawer__footer) {
  padding: 0;
}
</style>
