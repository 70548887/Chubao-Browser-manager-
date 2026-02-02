<script setup lang="ts">
/**
 * @description 偏好设置表单
 */

import { computed } from 'vue'
import type { PreferencesConfig } from '@/types'

interface Props {
  modelValue: Partial<PreferencesConfig>
}

interface Emits {
  (e: 'update:modelValue', value: Partial<PreferencesConfig>): void
}

const props = defineProps<Props>()
const emit = defineEmits<Emits>()

// 预设扩展列表
const availableExtensions = [
  {
    id: 'metamask',
    name: 'MetaMask',
    description: '以太坊钱包扩展',
    icon: 'account_balance_wallet',
    gradient: 'linear-gradient(135deg, #f6851b 0%, #e2761b 100%)',
  },
  {
    id: 'google-translate',
    name: 'Google Translate',
    description: '网页翻译工具',
    icon: 'translate',
    gradient: 'linear-gradient(135deg, #4285f4 0%, #3367d6 100%)',
  },
  {
    id: 'editthiscookie',
    name: 'EditThisCookie',
    description: 'Cookie 编辑管理',
    icon: 'cookie',
    gradient: 'linear-gradient(135deg, #8b5cf6 0%, #7c3aed 100%)',
  },
  {
    id: 'adblock-plus',
    name: 'AdBlock Plus',
    description: '广告拦截工具',
    icon: 'block',
    gradient: 'linear-gradient(135deg, #ef4444 0%, #dc2626 100%)',
  },
]

// 默认值
const defaultPreferences: Partial<PreferencesConfig> = {
  windowName: true,
  customBookmarks: false,
  extensions: [],
  startupPage: 'blank',
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
  showPasswordSavePrompt: false,
  stopOnNetworkError: false,
  stopOnIpChange: false,
  stopOnCountryChange: false,
  openWorkbench: false,
  ipChangeNotification: false,
  enableGoogleLogin: false,
}

// 合并默认值
const preferences = computed(() => ({
  ...defaultPreferences,
  ...props.modelValue,
}))

// 更新字段
const updateField = (field: keyof PreferencesConfig, value: any) => {
  emit('update:modelValue', {
    ...props.modelValue,
    [field]: value,
  })
}

// 切换扩展选择
const toggleExtension = (extensionId: string) => {
  const currentExtensions = preferences.value.extensions || []
  const index = currentExtensions.indexOf(extensionId)
  
  if (index > -1) {
    updateField('extensions', currentExtensions.filter(id => id !== extensionId))
  } else {
    updateField('extensions', [...currentExtensions, extensionId])
  }
}

// 检查扩展是否已选择
const isExtensionSelected = (extensionId: string) => {
  return (preferences.value.extensions || []).includes(extensionId)
}

// 上传自定义扩展
const uploadCustomExtension = () => {
  // TODO: 实现文件选择对话框
  console.log('上传自定义扩展')
}
</script>

<template>
  <div class="step-content">
    <!-- 基本设置 -->
    <div class="form-section">
      <div class="form-row">
        <label class="form-label">窗口名称</label>
        <div class="form-control">
          <label class="toggle-switch">
            <input type="checkbox" :checked="preferences.windowName" @change="updateField('windowName', !preferences.windowName)" />
            <span class="toggle-slider"></span>
          </label>
        </div>
      </div>

      <div class="form-row">
        <label class="form-label">自定义书签</label>
        <div class="form-control">
          <label class="toggle-switch">
            <input type="checkbox" :checked="preferences.customBookmarks" @change="updateField('customBookmarks', !preferences.customBookmarks)" />
            <span class="toggle-slider"></span>
          </label>
        </div>
      </div>
    </div>

    <!-- 扩展管理 -->
    <div class="form-section">
      <div class="form-row full-width">
        <div class="section-header">
          <label class="section-label">
            <span class="material-symbols-outlined">extension</span>
            扩展管理
          </label>
          <button type="button" class="btn-link" @click="uploadCustomExtension">
            <span class="material-symbols-outlined">upload</span>
            上传自定义扩展
          </button>
        </div>
        <div class="extensions-grid">
          <label
            v-for="ext in availableExtensions"
            :key="ext.id"
            class="extension-card"
            :class="{ selected: isExtensionSelected(ext.id) }"
          >
            <input
              type="checkbox"
              class="ext-checkbox"
              :checked="isExtensionSelected(ext.id)"
              @change="toggleExtension(ext.id)"
            />
            <div class="ext-icon" :style="{ background: ext.gradient }">
              <span class="material-symbols-outlined">{{ ext.icon }}</span>
            </div>
            <div class="ext-info">
              <span class="ext-name">{{ ext.name }}</span>
              <span class="ext-desc">{{ ext.description }}</span>
            </div>
            <div class="ext-check">
              <span class="material-symbols-outlined">check_circle</span>
            </div>
          </label>
        </div>
      </div>

      <div class="divider"></div>

      <!-- 退出自动清理 -->
      <div class="form-row">
        <label class="form-label">
          <span class="material-symbols-outlined">cleaning_services</span>
          退出自动清理
        </label>
        <div class="form-control">
          <div class="switch-list">
            <label class="switch-item">
              <span class="switch-label">清理历史记录</span>
              <div class="switch-toggle" :class="{ active: preferences.clearHistoryOnExit }">
                <input type="checkbox" :checked="preferences.clearHistoryOnExit" @change="updateField('clearHistoryOnExit', !preferences.clearHistoryOnExit)" />
                <span class="switch-slider"></span>
              </div>
            </label>
            <label class="switch-item">
              <span class="switch-label">清理 Cookies</span>
              <div class="switch-toggle" :class="{ active: preferences.clearCookiesOnExit }">
                <input type="checkbox" :checked="preferences.clearCookiesOnExit" @change="updateField('clearCookiesOnExit', !preferences.clearCookiesOnExit)" />
                <span class="switch-slider"></span>
              </div>
            </label>
            <label class="switch-item">
              <span class="switch-label">清理缓存文件</span>
              <div class="switch-toggle" :class="{ active: preferences.clearCacheOnExit }">
                <input type="checkbox" :checked="preferences.clearCacheOnExit" @change="updateField('clearCacheOnExit', !preferences.clearCacheOnExit)" />
                <span class="switch-slider"></span>
              </div>
            </label>
          </div>
        </div>
      </div>

      <div class="divider"></div>

      <!-- 数据同步 -->
      <div class="form-row">
        <label class="form-label">
          <span class="material-symbols-outlined">cloud_sync</span>
          数据同步
        </label>
        <div class="form-control">
          <div class="sync-section">
            <label class="switch-item main-switch">
              <div class="switch-info">
                <span class="switch-label">云端同步</span>
                <span class="switch-desc">同步扩展、书签和密码到云端</span>
              </div>
              <div class="switch-toggle" :class="{ active: preferences.cloudSync }">
                <input type="checkbox" :checked="preferences.cloudSync" @change="updateField('cloudSync', !preferences.cloudSync)" />
                <span class="switch-slider"></span>
              </div>
            </label>
            <div v-if="preferences.cloudSync" class="sub-switches">
              <label class="switch-item sub">
                <span class="switch-label">同步应用扩展</span>
                <div class="switch-toggle small" :class="{ active: preferences.cloudSyncExtensions }">
                  <input type="checkbox" :checked="preferences.cloudSyncExtensions" @change="updateField('cloudSyncExtensions', !preferences.cloudSyncExtensions)" />
                  <span class="switch-slider"></span>
                </div>
              </label>
              <label class="switch-item sub">
                <span class="switch-label">同步书签/收藏夹</span>
                <div class="switch-toggle small" :class="{ active: preferences.cloudSyncBookmarks }">
                  <input type="checkbox" :checked="preferences.cloudSyncBookmarks" @change="updateField('cloudSyncBookmarks', !preferences.cloudSyncBookmarks)" />
                  <span class="switch-slider"></span>
                </div>
              </label>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- 同步选项 -->
    <div class="form-section">
      <div class="section-title">同步选项</div>
      
      <div class="form-row">
        <label class="form-label">同步书签</label>
        <div class="form-control">
          <label class="toggle-switch">
            <input type="checkbox" :checked="preferences.syncBookmarks" @change="updateField('syncBookmarks', !preferences.syncBookmarks)" />
            <span class="toggle-slider"></span>
          </label>
        </div>
      </div>

      <div class="form-row">
        <label class="form-label">同步历史记录</label>
        <div class="form-control">
          <label class="toggle-switch">
            <input type="checkbox" :checked="preferences.syncHistory" @change="updateField('syncHistory', !preferences.syncHistory)" />
            <span class="toggle-slider"></span>
          </label>
        </div>
      </div>

      <div class="form-row">
        <label class="form-label">同步标签页</label>
        <div class="form-control">
          <label class="toggle-switch">
            <input type="checkbox" :checked="preferences.syncTabs" @change="updateField('syncTabs', !preferences.syncTabs)" />
            <span class="toggle-slider"></span>
          </label>
        </div>
      </div>

      <div class="form-row">
        <label class="form-label">同步Cookie</label>
        <div class="form-control">
          <label class="toggle-switch">
            <input type="checkbox" :checked="preferences.syncCookies" @change="updateField('syncCookies', !preferences.syncCookies)" />
            <span class="toggle-slider"></span>
          </label>
        </div>
      </div>

      <div class="form-row">
        <label class="form-label">同步扩展应用程序</label>
        <div class="form-control">
          <label class="toggle-switch">
            <input type="checkbox" :checked="preferences.syncExtensions" @change="updateField('syncExtensions', !preferences.syncExtensions)" />
            <span class="toggle-slider"></span>
          </label>
        </div>
      </div>

      <div class="form-row">
        <label class="form-label">同步已保存的用户名密码</label>
        <div class="form-control">
          <label class="toggle-switch">
            <input type="checkbox" :checked="preferences.syncPasswords" @change="updateField('syncPasswords', !preferences.syncPasswords)" />
            <span class="toggle-slider"></span>
          </label>
        </div>
      </div>

      <div class="form-row">
        <label class="form-label">同步IndexedDB</label>
        <div class="form-control">
          <label class="toggle-switch">
            <input type="checkbox" :checked="preferences.syncIndexedDB" @change="updateField('syncIndexedDB', !preferences.syncIndexedDB)" />
            <span class="toggle-slider"></span>
          </label>
        </div>
      </div>

      <div class="form-row">
        <label class="form-label">同步Local Storage</label>
        <div class="form-control">
          <label class="toggle-switch">
            <input type="checkbox" :checked="preferences.syncLocalStorage" @change="updateField('syncLocalStorage', !preferences.syncLocalStorage)" />
            <span class="toggle-slider"></span>
          </label>
        </div>
      </div>

      <div class="form-row">
        <label class="form-label">同步Session Storage</label>
        <div class="form-control">
          <label class="toggle-switch">
            <input type="checkbox" :checked="preferences.syncSessionStorage" @change="updateField('syncSessionStorage', !preferences.syncSessionStorage)" />
            <span class="toggle-slider"></span>
          </label>
        </div>
      </div>
    </div>

    <!-- 启动前清理 -->
    <div class="form-section">
      <div class="section-title">启动前清理</div>
      
      <div class="form-row">
        <label class="form-label">启动浏览器前删除缓存文件</label>
        <div class="form-control">
          <label class="toggle-switch">
            <input type="checkbox" :checked="preferences.clearCacheOnStart" @change="updateField('clearCacheOnStart', !preferences.clearCacheOnStart)" />
            <span class="toggle-slider"></span>
          </label>
        </div>
      </div>

      <div class="form-row">
        <label class="form-label">启动浏览器前删除Cookie</label>
        <div class="form-control">
          <label class="toggle-switch">
            <input type="checkbox" :checked="preferences.clearCookiesOnStart" @change="updateField('clearCookiesOnStart', !preferences.clearCookiesOnStart)" />
            <span class="toggle-slider"></span>
          </label>
        </div>
      </div>

      <div class="form-row">
        <label class="form-label">启动浏览器前删除Local Storage</label>
        <div class="form-control">
          <label class="toggle-switch">
            <input type="checkbox" :checked="preferences.clearLocalStorageOnStart" @change="updateField('clearLocalStorageOnStart', !preferences.clearLocalStorageOnStart)" />
            <span class="toggle-slider"></span>
          </label>
        </div>
      </div>
    </div>

    <!-- 其他选项 -->
    <div class="form-section">
      <div class="section-title">其他选项</div>
      
      <div class="form-row">
        <label class="form-label">启动浏览器时随机指纹</label>
        <div class="form-control">
          <label class="toggle-switch">
            <input type="checkbox" :checked="preferences.randomFingerprintOnStart" @change="updateField('randomFingerprintOnStart', !preferences.randomFingerprintOnStart)" />
            <span class="toggle-slider"></span>
          </label>
        </div>
      </div>

      <div class="form-row">
        <label class="form-label">弹出保存密码提示</label>
        <div class="form-control">
          <label class="toggle-switch">
            <input type="checkbox" :checked="preferences.showPasswordSavePrompt" @change="updateField('showPasswordSavePrompt', !preferences.showPasswordSavePrompt)" />
            <span class="toggle-slider"></span>
          </label>
        </div>
      </div>

      <div class="form-row">
        <label class="form-label">网络不通停止打开</label>
        <div class="form-control">
          <label class="toggle-switch">
            <input type="checkbox" :checked="preferences.stopOnNetworkError" @change="updateField('stopOnNetworkError', !preferences.stopOnNetworkError)" />
            <span class="toggle-slider"></span>
          </label>
        </div>
      </div>

      <div class="form-row">
        <label class="form-label">IP发生变化停止打开</label>
        <div class="form-control">
          <label class="toggle-switch">
            <input type="checkbox" :checked="preferences.stopOnIpChange" @change="updateField('stopOnIpChange', !preferences.stopOnIpChange)" />
            <span class="toggle-slider"></span>
          </label>
        </div>
      </div>

      <div class="form-row">
        <label class="form-label">IP对应国家/地区发生改变停止打开</label>
        <div class="form-control">
          <label class="toggle-switch">
            <input type="checkbox" :checked="preferences.stopOnCountryChange" @change="updateField('stopOnCountryChange', !preferences.stopOnCountryChange)" />
            <span class="toggle-slider"></span>
          </label>
        </div>
      </div>

      <div class="form-row">
        <label class="form-label">打开工作台</label>
        <div class="form-control">
          <label class="toggle-switch">
            <input type="checkbox" :checked="preferences.openWorkbench" @change="updateField('openWorkbench', !preferences.openWorkbench)" />
            <span class="toggle-slider"></span>
          </label>
        </div>
      </div>

      <div class="form-row">
        <label class="form-label">IP 变化提醒</label>
        <div class="form-control">
          <label class="toggle-switch">
            <input type="checkbox" :checked="preferences.ipChangeNotification" @change="updateField('ipChangeNotification', !preferences.ipChangeNotification)" />
            <span class="toggle-slider"></span>
          </label>
        </div>
      </div>

      <div class="form-row">
        <label class="form-label">是否开启谷歌登录</label>
        <div class="form-control">
          <label class="toggle-switch">
            <input type="checkbox" :checked="preferences.enableGoogleLogin" @change="updateField('enableGoogleLogin', !preferences.enableGoogleLogin)" />
            <span class="toggle-slider"></span>
          </label>
        </div>
      </div>
    </div>

    <!-- 网址访问控制 -->
    <div class="form-section">
      <div class="section-title">网址访问控制</div>
      
      <div class="form-row align-top">
        <label class="form-label">网址访问黑名单</label>
        <div class="form-control">
          <textarea
            :value="preferences.urlBlacklist"
            @input="updateField('urlBlacklist', ($event.target as HTMLTextAreaElement).value)"
            class="textarea"
            rows="4"
            placeholder="每行一个 URL，换行以添加多个"
          ></textarea>
        </div>
      </div>

      <div class="form-row align-top">
        <label class="form-label">网址访问白名单</label>
        <div class="form-control">
          <textarea
            :value="preferences.urlWhitelist"
            @input="updateField('urlWhitelist', ($event.target as HTMLTextAreaElement).value)"
            class="textarea"
            rows="4"
            placeholder="每行一个 URL，换行以添加多个"
          ></textarea>
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
  gap: 20px;
  padding-bottom: 24px;
  margin-bottom: 24px;
  border-bottom: 1px solid #f1f5f9;
  
  &:last-child {
    border-bottom: none;
    margin-bottom: 0;
    padding-bottom: 0;
  }
}

.section-title {
  font-size: 15px;
  font-weight: 600;
  color: #1e293b;
  padding-bottom: 8px;
  border-bottom: 2px solid #2563eb;
  display: inline-block;
  margin-bottom: 4px;
}

.form-row {
  display: grid;
  grid-template-columns: 200px 1fr;
  gap: 16px;
  align-items: center;
  
  &.align-top {
    align-items: start;
  }
  
  &.full-width {
    grid-template-columns: 1fr;
  }
}

.form-label {
  font-size: 14px;
  color: #374151;
  display: flex;
  align-items: center;
  gap: 6px;
  
  .material-symbols-outlined {
    font-size: 18px;
    color: #64748b;
  }
}

.form-control {
  display: flex;
  align-items: center;
  flex-direction: column;
  align-items: stretch;
  gap: 12px;
}

// 扩展管理
.section-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 16px;
}

.section-label {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 15px;
  font-weight: 600;
  color: #1e293b;
  
  .material-symbols-outlined {
    font-size: 20px;
    color: #2563eb;
  }
}

.btn-link {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 6px 12px;
  background: transparent;
  border: 1px solid #e2e8f0;
  border-radius: 6px;
  font-size: 13px;
  color: #64748b;
  cursor: pointer;
  transition: all 0.2s;
  
  .material-symbols-outlined {
    font-size: 16px;
  }
  
  &:hover {
    background: #f8fafc;
    border-color: #cbd5e1;
    color: #2563eb;
  }
}

.extensions-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
  gap: 12px;
}

.extension-card {
  position: relative;
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: 16px;
  background: white;
  border: 2px solid #e2e8f0;
  border-radius: 12px;
  cursor: pointer;
  transition: all 0.2s;
  
  &:hover {
    border-color: #cbd5e1;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.06);
  }
  
  &.selected {
    border-color: #2563eb;
    background: #eff6ff;
    
    .ext-check {
      opacity: 1;
    }
  }
}

.ext-checkbox {
  display: none;
}

.ext-icon {
  width: 48px;
  height: 48px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 12px;
  margin-bottom: 12px;
  
  .material-symbols-outlined {
    font-size: 28px;
    color: white;
  }
}

.ext-info {
  text-align: center;
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.ext-name {
  font-size: 14px;
  font-weight: 600;
  color: #1e293b;
}

.ext-desc {
  font-size: 12px;
  color: #64748b;
}

.ext-check {
  position: absolute;
  top: 8px;
  right: 8px;
  opacity: 0;
  transition: opacity 0.2s;
  
  .material-symbols-outlined {
    font-size: 20px;
    color: #2563eb;
  }
}

.divider {
  height: 1px;
  background: #e2e8f0;
  margin: 16px 0;
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

.mt-2 {
  margin-top: 8px;
}

// 开关列表
.switch-list {
  display: flex;
  flex-direction: column;
  gap: 12px;
  width: 100%;
}

.switch-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12px;
  background: #f8fafc;
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.2s;
  
  &:hover {
    background: #f1f5f9;
  }
  
  &.main-switch {
    background: white;
    border: 2px solid #e2e8f0;
    padding: 16px;
  }
  
  &.sub {
    background: white;
    border: 1px solid #e2e8f0;
    padding: 10px 12px;
  }
}

.switch-label {
  font-size: 14px;
  color: #374151;
}

.switch-info {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.switch-desc {
  font-size: 12px;
  color: #64748b;
}

.switch-toggle {
  position: relative;
  width: 44px;
  height: 24px;
  flex-shrink: 0;
  
  input {
    display: none;
  }
  
  .switch-slider {
    position: absolute;
    inset: 0;
    background: #cbd5e1;
    border-radius: 12px;
    cursor: pointer;
    transition: all 0.2s;
    
    &::before {
      content: '';
      position: absolute;
      left: 2px;
      top: 2px;
      width: 20px;
      height: 20px;
      background: white;
      border-radius: 50%;
      transition: all 0.2s;
    }
  }
  
  &.active .switch-slider {
    background: #2563eb;
    
    &::before {
      left: 22px;
    }
  }
  
  &.small {
    width: 36px;
    height: 20px;
    
    .switch-slider::before {
      width: 16px;
      height: 16px;
    }
    
    &.active .switch-slider::before {
      left: 18px;
    }
  }
}

// 同步区域
.sync-section {
  display: flex;
  flex-direction: column;
  gap: 12px;
  width: 100%;
}

.sub-switches {
  display: flex;
  flex-direction: column;
  gap: 8px;
  padding-left: 16px;
  margin-top: 8px;
  border-left: 2px solid #e2e8f0;
}

// 开关
.toggle-switch {
  position: relative;
  width: 44px;
  height: 24px;
  display: inline-block;
  
  input {
    display: none;
  }
  
  .toggle-slider {
    position: absolute;
    inset: 0;
    background: #cbd5e1;
    border-radius: 12px;
    cursor: pointer;
    transition: all 0.2s;
    
    &::before {
      content: '';
      position: absolute;
      left: 2px;
      top: 2px;
      width: 20px;
      height: 20px;
      background: white;
      border-radius: 50%;
      transition: all 0.2s;
    }
  }
  
  input:checked + .toggle-slider {
    background: #2563eb;
    
    &::before {
      left: 22px;
    }
  }
}

// 文本域
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
  font-family: inherit;
  
  &::placeholder {
    color: #94a3b8;
  }
  
  &:focus {
    outline: none;
    border-color: #2563eb;
    box-shadow: 0 0 0 3px rgba(37, 99, 235, 0.1);
  }
}
</style>
