<template>
  <div class="settings-view">
    <!-- 设置表单 -->
    <div class="settings-container">
      <!-- 浏览器内核设置 -->
      <div class="setting-section">
        <div class="section-header">
          <h2 class="section-title">浏览器内核</h2>
          <p class="section-description">配置浏览器内核可执行文件路径</p>
        </div>

        <div class="setting-item">
          <label class="setting-label">
            内核路径
            <span class="required">*</span>
          </label>
          <div class="setting-input-group">
            <input
              v-model="settings.kernelPath"
              type="text"
              class="setting-input"
              placeholder="请选择浏览器内核可执行文件"
              readonly
            />
            <button class="btn-select" @click="selectKernelPath">
              <span class="icon">📁</span>
              选择文件
            </button>
          </div>
          <p class="setting-hint">
            例如：C:\Program Files\Google\Chrome\Application\chrome.exe
          </p>
        </div>

        <div class="setting-item">
          <label class="setting-label">用户数据目录</label>
          <div class="setting-input-group">
            <input
              v-model="settings.userDataDir"
              type="text"
              class="setting-input"
              placeholder="请选择用户数据存储目录"
              readonly
            />
            <button class="btn-select" @click="selectUserDataDir">
              <span class="icon">📁</span>
              选择目录
            </button>
          </div>
          <p class="setting-hint">
            用于存储浏览器配置文件和用户数据，留空则使用默认目录
          </p>
        </div>
      </div>

      <!-- 代理设置 -->
      <div class="setting-section">
        <div class="section-header">
          <h2 class="section-title">默认代理</h2>
          <p class="section-description">配置新窗口的默认代理设置</p>
        </div>

        <div class="setting-item">
          <label class="setting-label">代理服务器</label>
          <input
            v-model="settings.defaultProxy"
            type="text"
            class="setting-input"
            placeholder="例如：http://127.0.0.1:8080"
          />
          <p class="setting-hint">
            格式：协议://主机:端口，留空表示不使用代理
          </p>
        </div>
      </div>

      <!-- 操作按钮 -->
      <div class="setting-actions">
        <button class="btn-cancel" @click="handleReset">
          <span class="icon">↺</span>
          重置
        </button>
        <button class="btn-save" @click="handleSave" :disabled="!isValid || isSaving">
          <span v-if="isSaving" class="icon loading">⏳</span>
          <span v-else class="icon">✓</span>
          {{ isSaving ? '保存中...' : '保存设置' }}
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { ElMessage } from 'element-plus'
import { open as openDialog } from '@tauri-apps/plugin-dialog'
import * as settingsApi from '@/api/settingsApi'

// 设置数据
const settings = ref({
  kernelPath: '',
  userDataDir: '',
  defaultProxy: ''
})

// 原始设置（用于重置）
const originalSettings = ref({ ...settings.value })

// 加载和保存状态
const isLoading = ref(false)
const isSaving = ref(false)

// 验证设置是否有效
const isValid = computed(() => {
  // 内核路径必填
  return settings.value.kernelPath.trim() !== ''
})

// 选择内核路径
const selectKernelPath = async () => {
  try {
    const selected = await openDialog({
      multiple: false,
      directory: false,
      filters: [{
        name: 'Executable',
        extensions: ['exe']
      }]
    })
    
    if (selected) {
      settings.value.kernelPath = selected
    }
  } catch (error) {
    console.error('选择文件失败:', error)
    ElMessage.error('选择文件失败')
  }
}

// 选择用户数据目录
const selectUserDataDir = async () => {
  try {
    const selected = await openDialog({
      multiple: false,
      directory: true
    })
    
    if (selected) {
      settings.value.userDataDir = selected
    }
  } catch (error) {
    console.error('选择目录失败:', error)
    ElMessage.error('选择目录失败')
  }
}

// 加载设置
const loadSettings = async () => {
  isLoading.value = true
  try {
    const allSettings = await settingsApi.getAllSettings()
    settings.value = {
      kernelPath: allSettings.kernel_path || '',
      userDataDir: allSettings.user_data_dir || '',
      defaultProxy: allSettings.default_proxy || ''
    }
    originalSettings.value = { ...settings.value }
  } catch (error) {
    console.error('加载设置失败:', error)
    ElMessage.warning('加载设置失败，使用默认值')
  } finally {
    isLoading.value = false
  }
}

// 保存设置
const handleSave = async () => {
  if (!isValid.value) {
    ElMessage.warning('请填写必填项')
    return
  }

  isSaving.value = true
  try {
    // 保存各项设置
    await settingsApi.setSettingValue('kernel_path', settings.value.kernelPath)
    
    if (settings.value.userDataDir) {
      await settingsApi.setSettingValue('user_data_dir', settings.value.userDataDir)
    }
    
    if (settings.value.defaultProxy) {
      await settingsApi.setSettingValue('default_proxy', settings.value.defaultProxy)
    }

    originalSettings.value = { ...settings.value }
    ElMessage.success('设置保存成功')
  } catch (error) {
    console.error('保存设置失败:', error)
    ElMessage.error('保存设置失败：' + error)
  } finally {
    isSaving.value = false
  }
}

// 重置设置
const handleReset = () => {
  settings.value = { ...originalSettings.value }
  ElMessage.info('已重置为上次保存的设置')
}

// 初始化
onMounted(() => {
  loadSettings()
})
</script>

<style scoped lang="scss">
.settings-view {
  padding: var(--spacing-lg);
  max-width: 800px;
  margin: 0 auto;
}

.page-header {
  margin-bottom: var(--spacing-xl);
}

.page-title {
  font-size: 24px;
  font-weight: 600;
  color: var(--text-primary);
  margin: 0 0 var(--spacing-xs);
}

.page-subtitle {
  font-size: 14px;
  color: var(--text-secondary);
  margin: 0;
}

.settings-container {
  background: var(--bg-primary);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-lg);
  overflow: hidden;
}

.setting-section {
  padding: var(--spacing-lg);
  border-bottom: 1px solid var(--border-color);

  &:last-child {
    border-bottom: none;
  }
}

.section-header {
  margin-bottom: var(--spacing-lg);
}

.section-title {
  font-size: 16px;
  font-weight: 600;
  color: var(--text-primary);
  margin: 0 0 var(--spacing-xs);
}

.section-description {
  font-size: 13px;
  color: var(--text-secondary);
  margin: 0;
}

.setting-item {
  margin-bottom: var(--spacing-lg);

  &:last-child {
    margin-bottom: 0;
  }
}

.setting-label {
  display: block;
  font-size: 14px;
  font-weight: 500;
  color: var(--text-primary);
  margin-bottom: var(--spacing-sm);

  .required {
    color: var(--color-danger);
    margin-left: 2px;
  }
}

.setting-input-group {
  display: flex;
  gap: var(--spacing-sm);
}

.setting-input {
  flex: 1;
  height: 36px;
  padding: 0 var(--spacing-md);
  font-size: 13px;
  color: var(--text-primary);
  background: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-md);
  transition: all 0.2s;

  &:focus {
    outline: none;
    border-color: var(--color-primary);
    background: var(--bg-primary);
  }

  &::placeholder {
    color: var(--text-tertiary);
  }

  &[readonly] {
    cursor: pointer;
  }
}

.btn-select {
  height: 36px;
  padding: 0 var(--spacing-md);
  font-size: 13px;
  font-weight: 500;
  color: var(--text-primary);
  background: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-md);
  cursor: pointer;
  transition: all 0.2s;
  display: flex;
  align-items: center;
  gap: var(--spacing-xs);
  white-space: nowrap;

  &:hover {
    background: var(--bg-hover);
    border-color: var(--color-primary);
  }

  &:active {
    transform: translateY(1px);
  }

  .icon {
    font-size: 16px;
  }
}

.setting-hint {
  font-size: 12px;
  color: var(--text-tertiary);
  margin: var(--spacing-xs) 0 0;
}

.setting-actions {
  padding: var(--spacing-lg);
  background: var(--bg-secondary);
  display: flex;
  justify-content: flex-end;
  gap: var(--spacing-md);
}

.btn-cancel,
.btn-save {
  height: 36px;
  padding: 0 var(--spacing-lg);
  font-size: 14px;
  font-weight: 500;
  border-radius: var(--radius-md);
  cursor: pointer;
  transition: all 0.2s;
  display: flex;
  align-items: center;
  gap: var(--spacing-xs);

  &:active {
    transform: translateY(1px);
  }

  .icon {
    font-size: 16px;

    &.loading {
      animation: spin 1s linear infinite;
    }
  }
}

.btn-cancel {
  color: var(--text-secondary);
  background: transparent;
  border: 1px solid var(--border-color);

  &:hover {
    color: var(--text-primary);
    background: var(--bg-hover);
  }
}

.btn-save {
  color: white;
  background: var(--color-primary);
  border: none;

  &:hover {
    background: var(--color-primary-hover);
  }

  &:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
}

@keyframes spin {
  from {
    transform: rotate(0deg);
  }
  to {
    transform: rotate(360deg);
  }
}
</style>
