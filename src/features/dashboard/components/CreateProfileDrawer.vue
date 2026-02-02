<script setup lang="ts">
/**
 * @description 创建窗口抽屉组件 - 五步流程（主容器）
 * @author DeepAgent
 */

import { useCreateProfile } from '../composables/useCreateProfile'
import Step1BasicInfo from './steps/Step1BasicInfo.vue'
import Step2Settings from './steps/Step2Settings.vue'
import Step3Fingerprint from './steps/Step3Fingerprint.vue'
import Step4Proxy from './steps/Step4Proxy.vue'
import Step5Preferences from './steps/Step5Preferences.vue'

const props = defineProps<{
  visible: boolean
}>()

const emit = defineEmits(['update:visible', 'created'])

// 使用 composable 管理所有业务逻辑
const {
  // 状态
  currentStep,
  totalSteps,
  formData,
  
  // 计算属性
  groupOptions,
  canProceed,
  
  // 方法
  handleClose,
  handlePrev,
  handleNext,
  generateRandomName,
  generateRandomMac,
  
  // 常量
  stepTitles
} = useCreateProfile(emit)
</script>

<template>
  <el-drawer
    :model-value="props.visible"
    title="创建窗口"
    direction="rtl"
    size="680px"
    class="create-profile-drawer"
    :close-on-click-modal="false"
    @close="handleClose"
  >
    <template #header>
      <div class="drawer-header">
        <h3 class="drawer-title">创建窗口</h3>
        <div class="step-indicator">
          <span class="step-current">{{ currentStep }}</span>
          <span class="step-separator">/</span>
          <span class="step-total">{{ totalSteps }}</span>
          <span class="step-title">{{ stepTitles[currentStep - 1] }}</span>
        </div>
      </div>
    </template>
    
    <div class="create-profile-content">
      <!-- 步骤进度条 -->
      <div class="steps-progress">
        <div
          v-for="(title, index) in stepTitles"
          :key="index"
          class="step-item"
          :class="{ 
            active: currentStep === index + 1,
            completed: currentStep > index + 1
          }"
        >
          <div class="step-number">{{ index + 1 }}</div>
          <span class="step-label">{{ title }}</span>
        </div>
      </div>
      
      <!-- 步骤内容 -->
      <div class="step-content">
        <!-- 第一步：窗口信息 -->
        <Step1BasicInfo 
          v-show="currentStep === 1"
          v-model="formData"
          :group-options="groupOptions"
          @generate-random-name="generateRandomName"
        />
        
        <!-- 第二步：基础设置 -->
        <Step2Settings 
          v-show="currentStep === 2"
          v-model="formData"
        />
        
        <!-- 第三步：高级指纹设置 -->
        <Step3Fingerprint 
          v-show="currentStep === 3"
          v-model="formData"
          @generate-random-mac="generateRandomMac"
        />
        
        <!-- 第四步：代理设置 -->
        <Step4Proxy 
          v-show="currentStep === 4"
          v-model="formData"
        />
        
        <!-- 第五步：偏好设置 -->
        <Step5Preferences 
          v-show="currentStep === 5"
          v-model="formData"
        />
      </div>
    </div>
    
    <!-- 底部按钮 -->
    <template #footer>
      <div class="drawer-footer">
        <button class="footer-btn cancel" @click="handleClose">取 消</button>
        <button v-if="currentStep > 1" class="footer-btn prev" @click="handlePrev">上一步</button>
        <button class="footer-btn primary" :disabled="!canProceed" @click="handleNext">
          {{ currentStep === totalSteps ? '立即创建' : '下一步' }}
        </button>
      </div>
    </template>
  </el-drawer>
</template>

<style scoped lang="scss">
.drawer-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  width: 100%;
}

.drawer-title {
  font-size: var(--text-lg);
  font-weight: var(--font-semibold);
  color: var(--color-text-primary);
  margin: 0;
}

.step-indicator {
  display: flex;
  align-items: center;
  gap: 4px;
  font-size: var(--text-sm);
  color: var(--color-text-secondary);
  
  .step-current {
    color: var(--color-accent-blue);
    font-weight: var(--font-semibold);
  }
  
  .step-title {
    margin-left: 8px;
    color: var(--color-text-primary);
  }
}

.create-profile-content {
  display: flex;
  flex-direction: column;
  height: 100%;
}

// 步骤进度条
.steps-progress {
  display: flex;
  justify-content: space-between;
  padding: var(--spacing-lg) var(--spacing-xl);
  border-bottom: 1px solid var(--color-border-default);
  background: var(--color-bg-secondary);
}

.step-item {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 6px;
  flex: 1;
  position: relative;
  
  &::after {
    content: '';
    position: absolute;
    top: 12px;
    left: 60%;
    width: 80%;
    height: 2px;
    background: var(--color-border-default);
  }
  
  &:last-child::after {
    display: none;
  }
  
  &.completed::after {
    background: var(--color-accent-blue);
  }
  
  .step-number {
    width: 24px;
    height: 24px;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: 50%;
    font-size: 12px;
    font-weight: var(--font-semibold);
    background: var(--color-bg-overlay);
    color: var(--color-text-tertiary);
    position: relative;
    z-index: 1;
  }
  
  .step-label {
    font-size: 11px;
    color: var(--color-text-tertiary);
    white-space: nowrap;
  }
  
  &.active {
    .step-number {
      background: var(--color-accent-blue);
      color: white;
    }
    .step-label {
      color: var(--color-accent-blue);
      font-weight: var(--font-medium);
    }
  }
  
  &.completed {
    .step-number {
      background: var(--color-accent-blue);
      color: white;
    }
    .step-label {
      color: var(--color-text-secondary);
    }
  }
}

// 步骤内容
.step-content {
  flex: 1;
  overflow: hidden;
}

// 底部按钮
.drawer-footer {
  display: flex;
  justify-content: flex-end;
  gap: var(--spacing-md);
  padding: var(--spacing-lg) var(--spacing-xl);
  border-top: 1px solid var(--color-border-default);
  background: var(--color-bg-secondary);
}

.footer-btn {
  height: 36px;
  padding: 0 var(--spacing-xl);
  border: none;
  border-radius: var(--radius-md);
  font-size: var(--text-base);
  font-weight: var(--font-medium);
  cursor: pointer;
  transition: all var(--duration-fast);
  
  &.cancel {
    background: var(--color-bg-primary);
    color: var(--color-text-secondary);
    border: 1px solid var(--color-border-default);
    
    &:hover {
      border-color: var(--color-border-strong);
    }
  }
  
  &.prev {
    background: var(--color-bg-primary);
    color: var(--color-text-primary);
    border: 1px solid var(--color-border-default);
    
    &:hover {
      border-color: var(--color-accent-blue);
      color: var(--color-accent-blue);
    }
  }
  
  &.primary {
    background: var(--color-accent-blue);
    color: white;
    
    &:hover:not(:disabled) {
      background: var(--color-accent-blue-hover);
    }
    
    &:disabled {
      opacity: 0.5;
      cursor: not-allowed;
    }
  }
}
</style>
