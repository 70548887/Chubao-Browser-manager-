<template>
  <div class="step-content">
    <h3 class="step-title">代理设置</h3>
    <p class="step-desc">配置代理服务器以保护您的真实网络身份</p>

    <div class="form-section">
      <!-- 启用代理开关 -->
      <div class="form-row">
        <label class="form-label">启用代理</label>
        <div class="form-control">
          <label class="toggle-switch">
            <input type="checkbox" v-model="proxyEnabled" />
            <span class="toggle-slider"></span>
          </label>
        </div>
      </div>
      
      <!-- 代理配置（只在开启时显示） -->
      <template v-if="proxyEnabled">
        <!-- 分隔线 -->
        <div class="divider-dashed"></div>
        
        <!-- 代理协议 -->
        <div class="form-row">
          <label class="form-label">代理协议</label>
          <div class="form-control">
            <div class="btn-group protocol-group">
              <button 
                type="button"
                class="btn-option first"
                :class="{ active: model.proxyProtocol === 'socks5' }"
                @click="model.proxyProtocol = 'socks5'"
              >SOCKS5</button>
              <button 
                type="button"
                class="btn-option"
                :class="{ active: model.proxyProtocol === 'http' }"
                @click="model.proxyProtocol = 'http'"
              >HTTP</button>
              <button 
                type="button"
                class="btn-option last"
                :class="{ active: model.proxyProtocol === 'https' }"
                @click="model.proxyProtocol = 'https'"
              >HTTPS</button>
            </div>
          </div>
        </div>

      <!-- 分隔线 -->
      <div class="divider-dashed"></div>

      <!-- 代理服务器 -->
      <div class="form-row">
        <label class="form-label">代理服务器</label>
        <div class="form-control">
          <div class="proxy-server-row">
            <div class="input-group flex-1">
              <span class="input-icon">
                <span class="material-symbols-outlined">dns</span>
              </span>
              <input 
                type="text" 
                class="form-input with-icon"
                v-model="model.proxyHost"
                placeholder="主机IP或域名"
              />
            </div>
            <div class="input-group port-input">
              <input 
                type="text" 
                class="form-input"
                v-model="model.proxyPort"
                placeholder="端口"
              />
            </div>
          </div>
        </div>
      </div>

      <!-- 认证信息 -->
      <div class="form-row">
        <label class="form-label">认证信息</label>
        <div class="form-control">
          <div class="auth-row">
            <div class="input-group flex-1">
              <span class="input-icon">
                <span class="material-symbols-outlined">person</span>
              </span>
              <input 
                type="text" 
                class="form-input with-icon"
                v-model="model.proxyUsername"
                placeholder="用户名（可选）"
              />
            </div>
            <div class="input-group flex-1">
              <span class="input-icon">
                <span class="material-symbols-outlined">lock</span>
              </span>
              <input 
                type="password" 
                class="form-input with-icon"
                v-model="model.proxyPassword"
                placeholder="密码（可选）"
              />
            </div>
          </div>
        </div>
      </div>

      <!-- UDP 支持（仅 SOCKS5） -->
      <div class="form-row" v-if="model.proxyProtocol === 'socks5'">
        <label class="form-label">
          UDP 支持
          <span class="label-hint" title="启用 SOCKS5 UDP ASSOCIATE，让 WebRTC 流量通过代理">
            <span class="material-symbols-outlined">info</span>
          </span>
        </label>
        <div class="form-control">
          <label class="toggle-switch">
            <input type="checkbox" v-model="model.enableUdp" />
            <span class="toggle-slider"></span>
          </label>
          <span class="toggle-label">{{ model.enableUdp ? '已启用（WebRTC UDP 将通过代理）' : '未启用（WebRTC UDP 将被禁用）' }}</span>
        </div>
      </div>

      <!-- 操作按钮 -->
      <div class="form-row">
        <label class="form-label"></label>
        <div class="form-control">
          <div class="action-buttons">
            <button type="button" class="btn-link">
              <span class="material-symbols-outlined">inventory_2</span>
              从代理池选择
            </button>
            <button type="button" class="btn-check" @click="checkProxy" :disabled="checking">
              <span class="material-symbols-outlined" :class="{ spinning: checking }">{{ checking ? 'sync' : 'wifi_tethering' }}</span>
              {{ checking ? '检测中...' : '检查代理' }}
            </button>
          </div>
        </div>
      </div>

      <!-- 连接测试结果 -->
      <div class="form-row" v-if="proxyResult">
        <label class="form-label"></label>
        <div class="form-control">
          <div class="proxy-result" :class="proxyResult.success ? 'success' : 'error'">
            <div class="result-icon">
              <span class="material-symbols-outlined">{{ proxyResult.success ? 'check_circle' : 'error' }}</span>
            </div>
            <div class="result-content">
              <template v-if="proxyResult.success">
                <div class="result-item">
                  <span class="result-label">IP地址</span>
                  <span class="result-value">{{ proxyResult.ip }}</span>
                </div>
                <div class="result-item">
                  <span class="result-label">地区</span>
                  <span class="result-value">{{ proxyResult.region }}</span>
                </div>
                <div class="result-item">
                  <span class="result-label">延迟</span>
                  <span class="result-value latency">{{ proxyResult.latency }}ms</span>
                </div>
              </template>
              <template v-else>
                <div class="result-error">{{ proxyResult.error }}</div>
              </template>
            </div>
          </div>
        </div>
      </div>
      </template>
      
      <!-- 未启用提示 -->
      <div v-if="!proxyEnabled" class="info-alert">
        <span class="material-symbols-outlined alert-icon">info</span>
        <div class="alert-content">
          <span class="alert-title">未启用代理</span>
          <span class="alert-desc">浏览器将使用直连方式访问网络</span>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue'
import { testProxyConfig } from '@/api/proxyApi'

interface FormModel {
  proxyProtocol: 'socks5' | 'http' | 'https' | 'ssh'
  proxyHost: string
  proxyPort: string
  proxyUsername: string
  proxyPassword: string
  enableUdp: boolean  // 启用 UDP 支持（用于 WebRTC）
}

const model = defineModel<FormModel>({ required: true })

// 代理开关
const proxyEnabled = ref(false)

// 监听开关状态，关闭时清空代理配置
watch(proxyEnabled, (enabled) => {
  if (!enabled) {
    model.value.proxyHost = ''
    model.value.proxyPort = ''
    model.value.proxyUsername = ''
    model.value.proxyPassword = ''
  }
})

interface ProxyResult {
  success: boolean
  ip?: string
  region?: string
  latency?: number
  error?: string
}

const checking = ref(false)
const proxyResult = ref<ProxyResult | null>(null)

const checkProxy = async () => {
  if (!model.value.proxyHost || !model.value.proxyPort) {
    proxyResult.value = {
      success: false,
      error: '请先填写代理服务器地址和端口'
    }
    return
  }

  checking.value = true
  proxyResult.value = null

  try {
    // 调用真实的后端代理检测 API
    const result = await testProxyConfig(
      model.value.proxyProtocol,
      model.value.proxyHost,
      model.value.proxyPort,
      model.value.proxyUsername || undefined,
      model.value.proxyPassword || undefined
    )
    
    if (result.success) {
      proxyResult.value = {
        success: true,
        ip: result.ip || model.value.proxyHost,
        region: result.location || result.country || '未知地区',
        latency: result.latency || 0
      }
    } else {
      proxyResult.value = {
        success: false,
        error: result.error || '代理连接失败'
      }
    }
  } catch (error) {
    proxyResult.value = {
      success: false,
      error: `检测失败: ${error}`
    }
  } finally {
    checking.value = false
  }
}
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

.form-section {
  display: flex;
  flex-direction: column;
  gap: 20px;
}

.form-row {
  display: grid;
  grid-template-columns: 100px 1fr;
  gap: 16px;
  align-items: start;
}

.form-label {
  font-size: 13px;
  font-weight: 500;
  color: #475569;
  padding-top: 8px;
  text-align: right;
}

.form-control {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

// 按钮组样式
.btn-group {
  display: inline-flex;
  border-radius: 6px;
  overflow: hidden;
  border: 1px solid #e2e8f0;

  .btn-option {
    padding: 8px 16px;
    font-size: 13px;
    font-weight: 500;
    background: white;
    color: #475569;
    border: none;
    border-right: 1px solid #e2e8f0;
    cursor: pointer;
    transition: all 0.15s ease;

    &:last-child {
      border-right: none;
    }

    &:hover:not(.active) {
      background: #f8fafc;
    }

    &.active {
      background: #2563eb;
      color: white;
    }
  }
}

.protocol-group {
  .btn-option {
    min-width: 70px;
  }
}

// 分隔线
.divider-dashed {
  border-top: 1px dashed #e2e8f0;
  margin: 4px 0;
}

// 代理服务器输入行
.proxy-server-row {
  display: flex;
  gap: 12px;
}

.port-input {
  width: 120px;
}

// 认证信息行
.auth-row {
  display: flex;
  gap: 12px;
}

// 输入框组
.input-group {
  position: relative;
  display: flex;
  align-items: center;

  .input-icon {
    position: absolute;
    left: 12px;
    color: #94a3b8;
    display: flex;
    align-items: center;

    .material-symbols-outlined {
      font-size: 18px;
    }
  }

  .form-input {
    width: 100%;
    height: 38px;
    padding: 0 12px;
    font-size: 13px;
    border: 1px solid #e2e8f0;
    border-radius: 6px;
    background: white;
    color: #1e293b;
    transition: all 0.15s ease;

    &::placeholder {
      color: #94a3b8;
    }

    &:focus {
      outline: none;
      border-color: #2563eb;
      box-shadow: 0 0 0 3px rgba(37, 99, 235, 0.1);
    }

    &.with-icon {
      padding-left: 40px;
    }
  }
}

.flex-1 {
  flex: 1;
}

// 操作按钮
.action-buttons {
  display: flex;
  align-items: center;
  gap: 16px;
}

.btn-link {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  padding: 8px 0;
  font-size: 13px;
  font-weight: 500;
  color: #2563eb;
  background: none;
  border: none;
  cursor: pointer;
  transition: color 0.15s ease;

  .material-symbols-outlined {
    font-size: 18px;
  }

  &:hover {
    color: #1d4ed8;
  }
}

.btn-check {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  padding: 8px 16px;
  font-size: 13px;
  font-weight: 500;
  color: white;
  background: linear-gradient(135deg, #22c55e 0%, #16a34a 100%);
  border: none;
  border-radius: 6px;
  cursor: pointer;
  transition: all 0.15s ease;
  box-shadow: 0 2px 4px rgba(34, 197, 94, 0.2);

  .material-symbols-outlined {
    font-size: 18px;
  }

  &:hover:not(:disabled) {
    background: linear-gradient(135deg, #16a34a 0%, #15803d 100%);
    box-shadow: 0 4px 8px rgba(34, 197, 94, 0.3);
  }

  &:disabled {
    opacity: 0.7;
    cursor: not-allowed;
  }

  .spinning {
    animation: spin 1s linear infinite;
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

// 代理检测结果
.proxy-result {
  display: flex;
  align-items: flex-start;
  gap: 12px;
  padding: 16px;
  border-radius: 8px;
  animation: fadeIn 0.3s ease;

  &.success {
    background: linear-gradient(135deg, #f0fdf4 0%, #dcfce7 100%);
    border: 1px solid #bbf7d0;

    .result-icon {
      color: #22c55e;
    }
  }

  &.error {
    background: linear-gradient(135deg, #fef2f2 0%, #fee2e2 100%);
    border: 1px solid #fecaca;

    .result-icon {
      color: #ef4444;
    }
  }

  .result-icon {
    .material-symbols-outlined {
      font-size: 24px;
    }
  }

  .result-content {
    flex: 1;
    display: flex;
    flex-wrap: wrap;
    gap: 16px;
  }

  .result-item {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .result-label {
    font-size: 11px;
    color: #64748b;
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }

  .result-value {
    font-size: 14px;
    font-weight: 600;
    color: #1e293b;

    &.latency {
      color: #22c55e;
    }
  }

  .result-error {
    font-size: 13px;
    color: #dc2626;
  }
}

@keyframes fadeIn {
  from {
    opacity: 0;
    transform: translateY(-10px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

// 开关样式
.toggle-switch {
  position: relative;
  display: inline-block;
  width: 48px;
  height: 26px;
  cursor: pointer;
  
  input {
    opacity: 0;
    width: 0;
    height: 0;
    
    &:checked + .toggle-slider {
      background: linear-gradient(135deg, #2563eb 0%, #3b82f6 100%);
      
      &::before {
        transform: translateX(22px);
      }
    }
  }
  
  .toggle-slider {
    position: absolute;
    inset: 0;
    background: #cbd5e1;
    border-radius: 26px;
    transition: all 0.3s ease;
    
    &::before {
      content: '';
      position: absolute;
      height: 20px;
      width: 20px;
      left: 3px;
      bottom: 3px;
      background: white;
      border-radius: 50%;
      transition: all 0.3s ease;
      box-shadow: 0 2px 4px rgba(0, 0, 0, 0.2);
    }
  }
}

// 提示框
.info-alert {
  display: flex;
  align-items: flex-start;
  gap: 12px;
  padding: 16px;
  background: linear-gradient(135deg, #f0f9ff 0%, #e0f2fe 100%);
  border: 1px solid #bae6fd;
  border-radius: 8px;
  margin-top: 8px;
  
  .alert-icon {
    font-size: 20px;
    color: #0284c7;
    flex-shrink: 0;
  }
  
  .alert-content {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }
  
  .alert-title {
    font-size: 14px;
    font-weight: 600;
    color: #0c4a6e;
  }
  
  .alert-desc {
    font-size: 12px;
    color: #075985;
  }
}
</style>
