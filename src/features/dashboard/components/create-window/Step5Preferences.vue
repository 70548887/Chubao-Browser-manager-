<template>
  <div class="step-content">
    <h3 class="step-title">偏好设置</h3>
    <p class="step-desc">配置浏览器扩展、数据清理和云端同步选项</p>

    <!-- 扩展管理 -->
    <div class="form-section">
      <div class="form-row full-width">
        <div class="section-header">
          <label class="section-label">
            <span class="material-symbols-outlined">extension</span>
            扩展管理
          </label>
          <button type="button" class="btn-link">
            <span class="material-symbols-outlined">upload</span>
            上传自定义扩展
          </button>
        </div>
        <div class="extensions-grid">
          <label 
            v-for="ext in extensions" 
            :key="ext.id"
            class="extension-card"
            :class="{ active: model.extensions.includes(ext.id) }"
          >
            <input 
              type="checkbox" 
              :value="ext.id"
              v-model="model.extensions"
              class="ext-checkbox"
            />
            <div class="ext-icon" :style="{ background: ext.color }">
              <span class="material-symbols-outlined">{{ ext.icon }}</span>
            </div>
            <div class="ext-info">
              <span class="ext-name">{{ ext.name }}</span>
              <span class="ext-desc">{{ ext.desc }}</span>
            </div>
            <div class="ext-check">
              <span class="material-symbols-outlined">check_circle</span>
            </div>
          </label>
        </div>
      </div>
    </div>

    <!-- 退出自动清理 -->
    <div class="form-section">
      <div class="section-title-row">
        <span class="material-symbols-outlined section-icon">cleaning_services</span>
        <span class="section-title">退出自动清理</span>
      </div>
      <div class="switch-grid">
        <label class="switch-item">
          <span class="switch-label">清理历史记录</span>
          <div class="switch-toggle" :class="{ active: model.clearHistoryOnExit }">
            <input type="checkbox" v-model="model.clearHistoryOnExit" />
            <span class="switch-slider"></span>
          </div>
        </label>
        <label class="switch-item">
          <span class="switch-label">清理 Cookies</span>
          <div class="switch-toggle" :class="{ active: model.clearCookiesOnExit }">
            <input type="checkbox" v-model="model.clearCookiesOnExit" />
            <span class="switch-slider"></span>
          </div>
        </label>
        <label class="switch-item">
          <span class="switch-label">清理缓存文件</span>
          <div class="switch-toggle" :class="{ active: model.clearCacheOnExit }">
            <input type="checkbox" v-model="model.clearCacheOnExit" />
            <span class="switch-slider"></span>
          </div>
        </label>
      </div>
    </div>

    <!-- 启动前清理 -->
    <div class="form-section">
      <div class="section-title-row">
        <span class="material-symbols-outlined section-icon">restart_alt</span>
        <span class="section-title">启动前清理</span>
      </div>
      <div class="switch-grid">
        <label class="switch-item">
          <span class="switch-label">启动前删除缓存文件</span>
          <div class="switch-toggle" :class="{ active: model.clearCacheOnStart }">
            <input type="checkbox" v-model="model.clearCacheOnStart" />
            <span class="switch-slider"></span>
          </div>
        </label>
        <label class="switch-item">
          <span class="switch-label">启动前删除Cookie</span>
          <div class="switch-toggle" :class="{ active: model.clearCookiesOnStart }">
            <input type="checkbox" v-model="model.clearCookiesOnStart" />
            <span class="switch-slider"></span>
          </div>
        </label>
        <label class="switch-item">
          <span class="switch-label">启动前删除Local Storage</span>
          <div class="switch-toggle" :class="{ active: model.clearLocalStorageOnStart }">
            <input type="checkbox" v-model="model.clearLocalStorageOnStart" />
            <span class="switch-slider"></span>
          </div>
        </label>
      </div>
    </div>

    <!-- 同步选项 -->
    <div class="form-section">
      <div class="section-title-row">
        <span class="material-symbols-outlined section-icon">sync</span>
        <span class="section-title">同步选项</span>
      </div>
      <div class="switch-grid three-cols">
        <label class="switch-item">
          <span class="switch-label">同步书签</span>
          <div class="switch-toggle" :class="{ active: model.syncBookmarks }">
            <input type="checkbox" v-model="model.syncBookmarks" />
            <span class="switch-slider"></span>
          </div>
        </label>
        <label class="switch-item">
          <span class="switch-label">同步历史记录</span>
          <div class="switch-toggle" :class="{ active: model.syncHistory }">
            <input type="checkbox" v-model="model.syncHistory" />
            <span class="switch-slider"></span>
          </div>
        </label>
        <label class="switch-item">
          <span class="switch-label">同步标签页</span>
          <div class="switch-toggle" :class="{ active: model.syncTabs }">
            <input type="checkbox" v-model="model.syncTabs" />
            <span class="switch-slider"></span>
          </div>
        </label>
        <label class="switch-item">
          <span class="switch-label">同步Cookie</span>
          <div class="switch-toggle" :class="{ active: model.syncCookies }">
            <input type="checkbox" v-model="model.syncCookies" />
            <span class="switch-slider"></span>
          </div>
        </label>
        <label class="switch-item">
          <span class="switch-label">同步扩展程序</span>
          <div class="switch-toggle" :class="{ active: model.syncExtensions }">
            <input type="checkbox" v-model="model.syncExtensions" />
            <span class="switch-slider"></span>
          </div>
        </label>
        <label class="switch-item">
          <span class="switch-label">同步密码</span>
          <div class="switch-toggle" :class="{ active: model.syncPasswords }">
            <input type="checkbox" v-model="model.syncPasswords" />
            <span class="switch-slider"></span>
          </div>
        </label>
        <label class="switch-item">
          <span class="switch-label">同步IndexedDB</span>
          <div class="switch-toggle" :class="{ active: model.syncIndexedDB }">
            <input type="checkbox" v-model="model.syncIndexedDB" />
            <span class="switch-slider"></span>
          </div>
        </label>
        <label class="switch-item">
          <span class="switch-label">同步Local Storage</span>
          <div class="switch-toggle" :class="{ active: model.syncLocalStorage }">
            <input type="checkbox" v-model="model.syncLocalStorage" />
            <span class="switch-slider"></span>
          </div>
        </label>
        <label class="switch-item">
          <span class="switch-label">同步Session Storage</span>
          <div class="switch-toggle" :class="{ active: model.syncSessionStorage }">
            <input type="checkbox" v-model="model.syncSessionStorage" />
            <span class="switch-slider"></span>
          </div>
        </label>
      </div>
    </div>

    <!-- 云端同步 -->
    <div class="form-section">
      <div class="section-title-row">
        <span class="material-symbols-outlined section-icon">cloud_sync</span>
        <span class="section-title">云端同步</span>
      </div>
      <div class="cloud-sync-card">
        <label class="switch-item main-switch">
          <div class="switch-info">
            <span class="switch-label">开启云端同步</span>
            <span class="switch-desc">同步扩展、书签和密码到云端</span>
          </div>
          <div class="switch-toggle" :class="{ active: model.cloudSync }">
            <input type="checkbox" v-model="model.cloudSync" />
            <span class="switch-slider"></span>
          </div>
        </label>
        <div v-if="model.cloudSync" class="sub-switches">
          <label class="switch-item sub">
            <span class="switch-label">云端同步扩展</span>
            <div class="switch-toggle small" :class="{ active: model.cloudSyncExtensions }">
              <input type="checkbox" v-model="model.cloudSyncExtensions" />
              <span class="switch-slider"></span>
            </div>
          </label>
          <label class="switch-item sub">
            <span class="switch-label">云端同步书签</span>
            <div class="switch-toggle small" :class="{ active: model.cloudSyncBookmarks }">
              <input type="checkbox" v-model="model.cloudSyncBookmarks" />
              <span class="switch-slider"></span>
            </div>
          </label>
        </div>
      </div>
    </div>

    <!-- 其他选项 -->
    <div class="form-section">
      <div class="section-title-row">
        <span class="material-symbols-outlined section-icon">tune</span>
        <span class="section-title">其他选项</span>
      </div>
      <div class="switch-grid">
        <label class="switch-item">
          <span class="switch-label">启动时随机指纹</span>
          <div class="switch-toggle" :class="{ active: model.randomFingerprintOnStart }">
            <input type="checkbox" v-model="model.randomFingerprintOnStart" />
            <span class="switch-slider"></span>
          </div>
        </label>
        <label class="switch-item">
          <span class="switch-label">弹出保存密码提示</span>
          <div class="switch-toggle" :class="{ active: model.showPasswordSavePrompt }">
            <input type="checkbox" v-model="model.showPasswordSavePrompt" />
            <span class="switch-slider"></span>
          </div>
        </label>
        <label class="switch-item">
          <span class="switch-label">网络不通停止打开</span>
          <div class="switch-toggle" :class="{ active: model.stopOnNetworkError }">
            <input type="checkbox" v-model="model.stopOnNetworkError" />
            <span class="switch-slider"></span>
          </div>
        </label>
        <label class="switch-item">
          <span class="switch-label">IP变化停止打开</span>
          <div class="switch-toggle" :class="{ active: model.stopOnIpChange }">
            <input type="checkbox" v-model="model.stopOnIpChange" />
            <span class="switch-slider"></span>
          </div>
        </label>
        <label class="switch-item">
          <span class="switch-label">IP国家变化停止打开</span>
          <div class="switch-toggle" :class="{ active: model.stopOnCountryChange }">
            <input type="checkbox" v-model="model.stopOnCountryChange" />
            <span class="switch-slider"></span>
          </div>
        </label>
        <label class="switch-item">
          <span class="switch-label">打开工作台</span>
          <div class="switch-toggle" :class="{ active: model.openWorkbench }">
            <input type="checkbox" v-model="model.openWorkbench" />
            <span class="switch-slider"></span>
          </div>
        </label>
        <label class="switch-item">
          <span class="switch-label">IP变化提醒</span>
          <div class="switch-toggle" :class="{ active: model.ipChangeNotification }">
            <input type="checkbox" v-model="model.ipChangeNotification" />
            <span class="switch-slider"></span>
          </div>
        </label>
        <label class="switch-item">
          <span class="switch-label">开启谷歌登录</span>
          <div class="switch-toggle" :class="{ active: model.enableGoogleLogin }">
            <input type="checkbox" v-model="model.enableGoogleLogin" />
            <span class="switch-slider"></span>
          </div>
        </label>
      </div>
    </div>

    <!-- 网址访问控制 -->
    <div class="form-section">
      <div class="section-title-row">
        <span class="material-symbols-outlined section-icon">language</span>
        <span class="section-title">网址访问控制</span>
      </div>
      <div class="url-control-grid">
        <div class="url-control-item">
          <label class="url-label">网址黑名单</label>
          <textarea
            v-model="model.urlBlacklist"
            class="url-textarea"
            rows="3"
            placeholder="每行一个 URL，换行以添加多个"
          ></textarea>
        </div>
        <div class="url-control-item">
          <label class="url-label">网址白名单</label>
          <textarea
            v-model="model.urlWhitelist"
            class="url-textarea"
            rows="3"
            placeholder="每行一个 URL，换行以添加多个"
          ></textarea>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
/**
 * @description 偏好设置表单 - 完整版（与编辑窗口功能一致）
 */

interface FormModel {
  // 扩展管理
  extensions: string[]
  
  // 退出自动清理
  clearHistoryOnExit: boolean
  clearCookiesOnExit: boolean
  clearCacheOnExit: boolean
  
  // 启动前清理
  clearCacheOnStart: boolean
  clearCookiesOnStart: boolean
  clearLocalStorageOnStart: boolean
  
  // 同步选项
  syncBookmarks: boolean
  syncHistory: boolean
  syncTabs: boolean
  syncCookies: boolean
  syncExtensions: boolean
  syncPasswords: boolean
  syncIndexedDB: boolean
  syncLocalStorage: boolean
  syncSessionStorage: boolean
  
  // 云端同步
  cloudSync: boolean
  cloudSyncExtensions: boolean
  cloudSyncBookmarks: boolean
  
  // 其他选项
  randomFingerprintOnStart: boolean
  showPasswordSavePrompt: boolean
  stopOnNetworkError: boolean
  stopOnIpChange: boolean
  stopOnCountryChange: boolean
  openWorkbench: boolean
  ipChangeNotification: boolean
  enableGoogleLogin: boolean
  
  // 网址访问控制
  urlBlacklist: string
  urlWhitelist: string
}

const model = defineModel<FormModel>({ required: true })

const extensions = [
  {
    id: 'metamask',
    name: 'MetaMask',
    desc: '以太坊钱包扩展',
    icon: 'account_balance_wallet',
    color: 'linear-gradient(135deg, #f6851b 0%, #e2761b 100%)'
  },
  {
    id: 'google-translate',
    name: 'Google Translate',
    desc: '网页翻译工具',
    icon: 'translate',
    color: 'linear-gradient(135deg, #4285f4 0%, #3367d6 100%)'
  },
  {
    id: 'editthiscookie',
    name: 'EditThisCookie',
    desc: 'Cookie 编辑管理',
    icon: 'cookie',
    color: 'linear-gradient(135deg, #8b5cf6 0%, #7c3aed 100%)'
  },
  {
    id: 'adblock-plus',
    name: 'AdBlock Plus',
    desc: '广告拦截工具',
    icon: 'block',
    color: 'linear-gradient(135deg, #ef4444 0%, #dc2626 100%)'
  }
]
</script>

<style scoped lang="scss">
.step-content {
  padding: 0;
}

.step-title {
  font-size: 18px;
  font-weight: 600;
  color: #1e293b;
  margin: 0 0 6px 0;
}

.step-desc {
  font-size: 13px;
  color: #64748b;
  margin: 0 0 24px 0;
}

// 表单区块
.form-section {
  margin-bottom: 24px;
  padding-bottom: 24px;
  border-bottom: 1px solid #f1f5f9;
  
  &:last-child {
    border-bottom: none;
    margin-bottom: 0;
    padding-bottom: 0;
  }
}

// 区块标题行
.section-title-row {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-bottom: 16px;
}

.section-icon {
  font-size: 20px;
  color: #2563eb;
}

.section-title {
  font-size: 15px;
  font-weight: 600;
  color: #1e293b;
}

// 扩展管理区域头部
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
  font-size: 14px;
  font-weight: 600;
  color: #1e293b;

  .material-symbols-outlined {
    font-size: 20px;
    color: #2563eb;
  }
}

.btn-link {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  padding: 6px 12px;
  font-size: 13px;
  font-weight: 500;
  color: #2563eb;
  background: #eff6ff;
  border: none;
  border-radius: 6px;
  cursor: pointer;
  transition: all 0.15s ease;

  .material-symbols-outlined {
    font-size: 16px;
  }

  &:hover {
    background: #dbeafe;
  }
}

// 扩展网格
.extensions-grid {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 12px;
}

.extension-card {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 14px;
  background: white;
  border: 1px solid #e2e8f0;
  border-radius: 10px;
  cursor: pointer;
  transition: all 0.2s ease;
  position: relative;

  .ext-checkbox {
    position: absolute;
    opacity: 0;
    width: 0;
    height: 0;
  }

  &:hover {
    border-color: #cbd5e1;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.05);
  }

  &.active {
    border-color: #2563eb;
    background: #f8faff;

    .ext-check {
      opacity: 1;
      transform: scale(1);
    }
  }
}

.ext-icon {
  width: 40px;
  height: 40px;
  border-radius: 10px;
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;

  .material-symbols-outlined {
    font-size: 22px;
    color: white;
  }
}

.ext-info {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 2px;
  min-width: 0;
}

.ext-name {
  font-size: 13px;
  font-weight: 600;
  color: #1e293b;
}

.ext-desc {
  font-size: 11px;
  color: #64748b;
}

.ext-check {
  opacity: 0;
  transform: scale(0.8);
  transition: all 0.2s ease;

  .material-symbols-outlined {
    font-size: 22px;
    color: #2563eb;
  }
}

// 开关网格布局
.switch-grid {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 12px;
  
  &.three-cols {
    grid-template-columns: repeat(3, 1fr);
  }
}

.switch-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 12px 14px;
  background: #f8fafc;
  border-radius: 8px;
  border: 1px solid #e2e8f0;
  cursor: pointer;
  transition: all 0.2s;

  &:hover {
    background: #f1f5f9;
    border-color: #cbd5e1;
  }

  &.main-switch {
    padding: 16px;
    background: white;
    border: 2px solid #e2e8f0;
  }

  &.sub {
    padding: 10px 14px;
    background: white;
    
    .switch-label {
      color: #64748b;
      font-size: 12px;
    }
  }
}

.switch-info {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.switch-label {
  font-size: 13px;
  font-weight: 500;
  color: #475569;
}

.switch-desc {
  font-size: 11px;
  color: #94a3b8;
}

.switch-toggle {
  position: relative;
  width: 36px;
  height: 20px;
  flex-shrink: 0;

  &.small {
    width: 32px;
    height: 18px;

    .switch-slider::after {
      width: 14px;
      height: 14px;
    }
  }

  input {
    opacity: 0;
    width: 0;
    height: 0;
    position: absolute;
  }

  .switch-slider {
    position: absolute;
    cursor: pointer;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: #cbd5e1;
    border-radius: 999px;
    transition: all 0.2s ease;

    &::after {
      content: '';
      position: absolute;
      top: 2px;
      left: 2px;
      width: 16px;
      height: 16px;
      background: white;
      border-radius: 50%;
      transition: all 0.2s ease;
      box-shadow: 0 1px 3px rgba(0, 0, 0, 0.15);
    }
  }

  &.active .switch-slider {
    background: #2563eb;

    &::after {
      transform: translateX(16px);
    }
  }

  &.small.active .switch-slider::after {
    transform: translateX(14px);
  }
}

// 云端同步卡片
.cloud-sync-card {
  background: #f8fafc;
  border: 1px solid #e2e8f0;
  border-radius: 10px;
  padding: 16px;
}

.sub-switches {
  display: flex;
  flex-direction: column;
  gap: 10px;
  padding-top: 12px;
  margin-top: 12px;
  border-top: 1px solid #e2e8f0;
  animation: fadeIn 0.2s ease;
}

// 网址访问控制
.url-control-grid {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 16px;
}

.url-control-item {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.url-label {
  font-size: 13px;
  font-weight: 500;
  color: #475569;
}

.url-textarea {
  width: 100%;
  padding: 10px 12px;
  border: 1px solid #e2e8f0;
  border-radius: 8px;
  font-size: 13px;
  font-family: inherit;
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

.form-row {
  &.full-width {
    width: 100%;
  }
}

@keyframes fadeIn {
  from {
    opacity: 0;
    transform: translateY(-5px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}
</style>
