<script setup lang="ts">
/**
 * @description 高级指纹设置表单
 */

import { computed } from 'vue'
import type { ProfileCreateDTO, FingerprintConfig } from '@/types'

interface Props {
  modelValue: ProfileCreateDTO
}

interface Emits {
  (e: 'update:modelValue', value: ProfileCreateDTO): void
  (e: 'regenerate'): void
}

const props = defineProps<Props>()
const emit = defineEmits<Emits>()

// 平台选项
const platformOptions = [
  { value: 'windows', label: 'Windows' },
  { value: 'macos', label: 'macOS' },
  { value: 'android', label: 'Android' },
  { value: 'ios', label: 'iOS' },
  { value: 'linux', label: 'Linux' },
]

// 内核版本选项
const browserVersionOptions = [
  { value: '146', label: 'Chrome 146', description: '最新稳定版' },
  { value: '145', label: 'Chrome 145' },
  { value: '144', label: 'Chrome 144' },
  { value: '143', label: 'Chrome 143' },
]

// 根据平台和版本生成 User-Agent
const generateUserAgent = (platform: string, version: string): string => {
  switch (platform) {
    case 'windows':
      return `Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/${version}.0.0.0 Safari/537.36`
    case 'macos':
      return `Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/${version}.0.0.0 Safari/537.36`
    case 'android':
      return `Mozilla/5.0 (Linux; Android 14; SM-S928B) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/${version}.0.0.0 Mobile Safari/537.36`
    case 'ios':
      return `Mozilla/5.0 (iPhone; CPU iPhone OS 17_4 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) CriOS/${version}.0.0.0 Mobile/15E148 Safari/604.1`
    default:
      return `Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/${version}.0.0.0 Safari/537.36`
  }
}

// 默认指纹配置
const defaultFingerprint: Partial<FingerprintConfig> = {
  webrtc: 'fake',
  webglVendor: 'Google Inc. (NVIDIA)',
  webglRenderer: 'ANGLE (NVIDIA, NVIDIA GeForce GTX 1650 with Max-Q Design Direct3D11 vs_5_0 ps_5_0, D3D11-27.21.14.5148)',
  webgpu: true,
  canvas: 'noise',
  audioContext: 'noise',
  doNotTrack: 'unspecified',
  clientRects: true,
  mediaDevices: 'fake',
  deviceName: 'DESKTOP-W0KJT6V0',
  macAddress: '64-2B-7A-4D-96-E1',
  portScanProtection: true,
  hardwareAcceleration: true,
  disableSandbox: false,
}

// 合并指纹配置
const fingerprint = computed(() => ({
  ...defaultFingerprint,
  ...props.modelValue.fingerprint,
}))

// 更新指纹字段
const updateFingerprint = (field: keyof FingerprintConfig, value: any) => {
  const newFingerprint = {
    ...props.modelValue.fingerprint,
    [field]: value,
  }

  // 如果是平台或版本变化，自动重新生成 User-Agent
  if (field === 'platform' || field === 'version') {
    const platform = field === 'platform' ? value : (newFingerprint.platform || 'windows')
    const version = field === 'version' ? value : (newFingerprint.version || '146')
    newFingerprint.userAgent = generateUserAgent(platform, version)
  }

  emit('update:modelValue', {
    ...props.modelValue,
    fingerprint: newFingerprint,
  })
}
</script>

<template>
  <div class="step-content">
    <!-- 快捷操作 -->
    <div class="quick-actions">
      <button class="btn-regenerate" @click="emit('regenerate')">
        <span class="material-symbols-outlined">refresh</span>
        <span>随机生成指纹</span>
      </button>
    </div>

    <!-- 基础指纹 -->
    <div class="form-section">
      <div class="section-title">基础指纹</div>

      <!-- 平台选择 -->
      <div class="form-row">
        <label class="form-label">
          平台选择
          <span class="help-icon" title="选择平台将影响 User-Agent 和相关指纹特征">
            <span class="material-symbols-outlined">help</span>
          </span>
        </label>
        <div class="form-control">
          <div class="platform-selector">
            <label v-for="opt in platformOptions" :key="opt.value" class="platform-option"
              :class="{ active: fingerprint.platform === opt.value }">
              <input type="radio" :checked="fingerprint.platform === opt.value"
                @change="updateFingerprint('platform', opt.value)" />
              <!-- Windows 图标 -->
              <svg v-if="opt.value === 'windows'" class="platform-icon" fill="currentColor" viewBox="0 0 24 24">
                <path
                  d="M0 3.449L9.75 2.1V11.719H0V3.449ZM0 12.281V20.551L9.75 19.2V12.281H0ZM10.547 1.983L24 0V11.719H10.547V1.983ZM10.547 12.281V19.017L24 20.893V12.281H10.547Z" />
              </svg>
              <!-- macOS/iOS 图标 -->
              <svg v-else-if="opt.value === 'macos' || opt.value === 'ios'" class="platform-icon" fill="currentColor"
                viewBox="0 0 24 24">
                <path
                  d="M18.71 19.5c-.83 1.24-1.71 2.45-3.1 2.48-1.34.03-1.77-.79-3.29-.79-1.53 0-2 .77-3.27.82-1.31.05-2.31-1.32-3.14-2.53C4.25 17 2.94 12.45 4.7 9.39c.87-1.52 2.43-2.48 4.12-2.51 1.28-.02 2.5.87 3.29.87.78 0 2.26-1.07 3.81-.91.65.03 2.47.26 3.64 1.98-.09.06-2.17 1.28-2.15 3.81.03 3.02 2.65 4.03 2.68 4.04-.03.07-.42 1.44-1.38 2.83M13 3.5c.73-.83 1.94-1.46 2.94-1.5.13 1.17-.36 2.35-1.04 3.19-.69.85-1.83 1.51-2.95 1.42-.15-1.15.41-2.35 1.05-3.11z" />
              </svg>
              <!-- Android 图标 -->
              <svg v-else-if="opt.value === 'android'" class="platform-icon" fill="currentColor" viewBox="0 0 24 24">
                <path
                  d="M17.523 15.341c-.5 0-.91-.41-.91-.91s.41-.91.91-.91.91.41.91.91-.41.91-.91.91m-11.046 0c-.5 0-.91-.41-.91-.91s.41-.91.91-.91.91.41.91.91-.41.91-.91.91m11.4-6.039l1.94-3.36c.11-.19.04-.43-.15-.54-.19-.11-.43-.04-.54.15l-1.96 3.4c-1.45-.66-3.08-1.03-4.82-1.03s-3.37.37-4.82 1.03l-1.96-3.4c-.11-.19-.35-.26-.54-.15-.19.11-.26.35-.15.54l1.94 3.36C3.03 10.94 0 14.81 0 19.34h24c0-4.53-3.03-8.4-7.12-10.04" />
              </svg>
              <!-- Linux 图标 -->
              <svg v-else-if="opt.value === 'linux'" class="platform-icon" fill="currentColor" viewBox="0 0 24 24">
                <path
                  d="M12.504 0c-.155 0-.311.003-.467.011-.859.048-1.694.251-2.463.666-.651.338-1.203.825-1.609 1.453-.453.697-.742 1.521-.831 2.417-.046.442-.044.894.004 1.351.042.407.108.812.2 1.211.164.683.386 1.354.665 1.997.263.607.578 1.184.944 1.716-.173.211-.338.436-.486.676-.372.6-.617 1.28-.703 1.992-.038.304-.048.612-.032.918.042.82.296 1.603.738 2.266.227.34.504.644.823.902.133.106.273.204.42.294-.009.039-.019.078-.027.117-.147.726-.056 1.443.297 2.048.18.308.423.579.719.801.176.13.365.24.564.33.082.035.166.067.251.095-.25.254-.45.557-.593.889-.227.53-.295 1.099-.198 1.63.048.263.127.52.237.766.109.242.249.472.42.681.301.369.692.666 1.151.868.385.168.804.266 1.237.29.103.006.206.008.309.007.337-.006.672-.048.996-.125.458-.109.893-.285 1.292-.52.244-.144.472-.31.681-.498.051.053.102.106.155.157.422.404.915.722 1.456.929.497.191 1.027.291 1.56.295.327.003.655-.032.972-.103.313-.068.614-.174.9-.313.373-.181.713-.42 1.008-.708.217-.21.405-.446.559-.702.039-.062.074-.127.108-.192.133.047.27.086.41.117.417.092.845.104 1.264.034.504-.086.983-.267 1.404-.532.332-.21.622-.477.859-.79.188-.249.338-.525.445-.821.106-.295.168-.607.184-.922.027-.491-.067-.983-.274-1.43-.156-.337-.375-.644-.646-.903.033-.019.066-.039.098-.06.303-.195.572-.434.796-.716.258-.322.452-.687.573-1.075.121-.389.168-.8.14-1.207-.035-.507-.178-.999-.421-1.443-.178-.327-.411-.621-.69-.87-.278-.25-.6-.45-.953-.596.14-.191.27-.392.389-.602.31-.546.531-1.14.651-1.756.081-.414.124-.837.128-1.261.004-.41-.03-.82-.102-1.223-.117-.648-.325-1.27-.62-1.844-.274-.533-.625-1.017-1.045-1.433-.416-.412-.894-.748-1.418-.994-.66-.31-1.376-.505-2.103-.574-.155-.015-.311-.023-.467-.025z" />
              </svg>
              <span>{{ opt.label }}</span>
            </label>
          </div>
          <p class="form-hint">选择平台将影响 User-Agent 和相关指纹特征</p>
        </div>
      </div>

      <!-- 内核版本 -->
      <div class="form-row">
        <label class="form-label">
          内核版本
          <span class="help-icon" title="选择浏览器内核版本">
            <span class="material-symbols-outlined">help</span>
          </span>
        </label>
        <div class="form-control">
          <select class="select" :value="fingerprint.version || '146'"
            @change="updateFingerprint('version', ($event.target as HTMLSelectElement).value)">
            <option v-for="opt in browserVersionOptions" :key="opt.value" :value="opt.value">
              {{ opt.label }}{{ opt.description ? ` - ${opt.description}` : '' }}
            </option>
          </select>
        </div>
      </div>

      <!-- User-Agent -->
      <div class="form-row">
        <label class="form-label">User-Agent</label>
        <div class="form-control">
          <div class="ua-input-wrapper">
            <input :value="fingerprint.userAgent" type="text" class="input ua-input" readonly />
            <button type="button" class="refresh-btn" title="刷新User-Agent" @click="emit('regenerate')">
              <span class="material-symbols-outlined">refresh</span>
            </button>
          </div>
          <div class="ua-tags">
            <!-- 平台标签 -->
            <span class="ua-tag">
              <svg v-if="fingerprint.platform === 'windows'" class="ua-tag-icon" fill="currentColor"
                viewBox="0 0 24 24">
                <path
                  d="M0 3.449L9.75 2.1V11.719H0V3.449ZM0 12.281V20.551L9.75 19.2V12.281H0ZM10.547 1.983L24 0V11.719H10.547V1.983ZM10.547 12.281V19.017L24 20.893V12.281H10.547Z" />
              </svg>
              <svg v-else-if="fingerprint.platform === 'macos' || fingerprint.platform === 'ios'" class="ua-tag-icon"
                fill="currentColor" viewBox="0 0 24 24">
                <path
                  d="M18.71 19.5c-.83 1.24-1.71 2.45-3.1 2.48-1.34.03-1.77-.79-3.29-.79-1.53 0-2 .77-3.27.82-1.31.05-2.31-1.32-3.14-2.53C4.25 17 2.94 12.45 4.7 9.39c.87-1.52 2.43-2.48 4.12-2.51 1.28-.02 2.5.87 3.29.87.78 0 2.26-1.07 3.81-.91.65.03 2.47.26 3.64 1.98-.09.06-2.17 1.28-2.15 3.81.03 3.02 2.65 4.03 2.68 4.04-.03.07-.42 1.44-1.38 2.83M13 3.5c.73-.83 1.94-1.46 2.94-1.5.13 1.17-.36 2.35-1.04 3.19-.69.85-1.83 1.51-2.95 1.42-.15-1.15.41-2.35 1.05-3.11z" />
              </svg>
              <svg v-else-if="fingerprint.platform === 'android'" class="ua-tag-icon" fill="currentColor"
                viewBox="0 0 24 24">
                <path
                  d="M17.523 15.341c-.5 0-.91-.41-.91-.91s.41-.91.91-.91.91.41.91.91-.41.91-.91.91m-11.046 0c-.5 0-.91-.41-.91-.91s.41-.91.91-.91.91.41.91.91-.41.91-.91.91m11.4-6.039l1.94-3.36c.11-.19.04-.43-.15-.54-.19-.11-.43-.04-.54.15l-1.96 3.4c-1.45-.66-3.08-1.03-4.82-1.03s-3.37.37-4.82 1.03l-1.96-3.4c-.11-.19-.35-.26-.54-.15-.19.11-.26.35-.15.54l1.94 3.36C3.03 10.94 0 14.81 0 19.34h24c0-4.53-3.03-8.4-7.12-10.04" />
              </svg>
              <svg v-else-if="fingerprint.platform === 'linux'" class="ua-tag-icon" fill="currentColor"
                viewBox="0 0 24 24">
                <path
                  d="M12.504 0c-.155 0-.311.003-.467.011-.859.048-1.694.251-2.463.666-.651.338-1.203.825-1.609 1.453-.453.697-.742 1.521-.831 2.417-.046.442-.044.894.004 1.351.042.407.108.812.2 1.211.164.683.386 1.354.665 1.997.263.607.578 1.184.944 1.716-.173.211-.338.436-.486.676-.372.6-.617 1.28-.703 1.992-.038.304-.048.612-.032.918.042.82.296 1.603.738 2.266.227.34.504.644.823.902.133.106.273.204.42.294-.009.039-.019.078-.027.117-.147.726-.056 1.443.297 2.048.18.308.423.579.719.801.176.13.365.24.564.33.082.035.166.067.251.095-.25.254-.45.557-.593.889-.227.53-.295 1.099-.198 1.63.048.263.127.52.237.766.109.242.249.472.42.681.301.369.692.666 1.151.868.385.168.804.266 1.237.29.103.006.206.008.309.007.337-.006.672-.048.996-.125.458-.109.893-.285 1.292-.52.244-.144.472-.31.681-.498.051.053.102.106.155.157.422.404.915.722 1.456.929.497.191 1.027.291 1.56.295.327.003.655-.032.972-.103.313-.068.614-.174.9-.313.373-.181.713-.42 1.008-.708.217-.21.405-.446.559-.702.039-.062.074-.127.108-.192.133.047.27.086.41.117.417.092.845.104 1.264.034.504-.086.983-.267 1.404-.532.332-.21.622-.477.859-.79.188-.249.338-.525.445-.821.106-.295.168-.607.184-.922.027-.491-.067-.983-.274-1.43-.156-.337-.375-.644-.646-.903.033-.019.066-.039.098-.06.303-.195.572-.434.796-.716.258-.322.452-.687.573-1.075.121-.389.168-.8.14-1.207-.035-.507-.178-.999-.421-1.443-.178-.327-.411-.621-.69-.87-.278-.25-.6-.45-.953-.596.14-.191.27-.392.389-.602.31-.546.531-1.14.651-1.756.081-.414.124-.837.128-1.261.004-.41-.03-.82-.102-1.223-.117-.648-.325-1.27-.62-1.844-.274-.533-.625-1.017-1.045-1.433-.416-.412-.894-.748-1.418-.994-.66-.31-1.376-.505-2.103-.574-.155-.015-.311-.023-.467-.025z" />
              </svg>
              <svg v-else class="ua-tag-icon" fill="currentColor" viewBox="0 0 24 24">
                <path
                  d="M0 3.449L9.75 2.1V11.719H0V3.449ZM0 12.281V20.551L9.75 19.2V12.281H0ZM10.547 1.983L24 0V11.719H10.547V1.983ZM10.547 12.281V19.017L24 20.893V12.281H10.547Z" />
              </svg>
              {{ fingerprint.platform === 'windows' ? 'Windows' : fingerprint.platform === 'macos' ? 'macOS' :
                fingerprint.platform === 'android' ? 'Android' : fingerprint.platform === 'ios' ? 'iOS' :
                  fingerprint.platform === 'linux' ? 'Linux' : 'Windows' }}
            </span>
            <span class="ua-tag ua-tag-primary">Chrome {{ fingerprint.version || '146' }}</span>
          </div>
        </div>
      </div>

      <div class="divider"></div>

      <div class="form-row">
        <label class="form-label">分辨率</label>
        <div class="form-control">
          <input :value="fingerprint.resolution || '1920x1080'"
            @input="updateFingerprint('resolution', ($event.target as HTMLInputElement).value)" type="text"
            class="input" placeholder="例如：1920x1080" />
        </div>
      </div>

      <div class="form-row">
        <label class="form-label">硬件并发数</label>
        <div class="form-control">
          <input :value="fingerprint.hardwareConcurrency"
            @input="updateFingerprint('hardwareConcurrency', Number(($event.target as HTMLInputElement).value))"
            type="number" class="input small-input" min="2" max="32" />
        </div>
      </div>

      <div class="form-row">
        <label class="form-label">设备内存</label>
        <div class="form-control">
          <input :value="fingerprint.deviceMemory"
            @input="updateFingerprint('deviceMemory', Number(($event.target as HTMLInputElement).value))" type="number"
            class="input small-input" min="4" max="64" />
          <span class="unit">GB</span>
        </div>
      </div>
    </div>

    <!-- WebRTC & WebGL -->
    <div class="form-section">
      <div class="section-title">WebRTC & WebGL</div>

      <div class="form-row">
        <label class="form-label">WebRTC</label>
        <div class="form-control">
          <div class="control-with-refresh">
            <div class="radio-group">
              <label class="radio-item" :class="{ active: fingerprint.webrtc === 'real' }">
                <input type="radio" :checked="fingerprint.webrtc === 'real'"
                  @change="updateFingerprint('webrtc', 'real')" />
                <span>真实</span>
              </label>
              <label class="radio-item" :class="{ active: fingerprint.webrtc === 'fake' }">
                <input type="radio" :checked="fingerprint.webrtc === 'fake'"
                  @change="updateFingerprint('webrtc', 'fake')" />
                <span>伪造</span>
              </label>
              <label class="radio-item" :class="{ active: fingerprint.webrtc === 'disabled' }">
                <input type="radio" :checked="fingerprint.webrtc === 'disabled'"
                  @change="updateFingerprint('webrtc', 'disabled')" />
                <span>禁用</span>
              </label>
            </div>
            <button type="button" class="refresh-btn-small" title="刷新WebRTC配置" @click="emit('regenerate')">
              <span class="material-symbols-outlined">refresh</span>
            </button>
          </div>
        </div>
      </div>

      <div v-if="fingerprint.webrtc === 'fake'" class="form-row sub-row">
        <label class="form-label">公网IP</label>
        <div class="form-control">
          <input :value="fingerprint.webrtcPublicIp"
            @input="updateFingerprint('webrtcPublicIp', ($event.target as HTMLInputElement).value)" type="text"
            class="input" placeholder="公网IP（可选）" />
        </div>
      </div>

      <div v-if="fingerprint.webrtc === 'fake'" class="form-row sub-row">
        <label class="form-label">本地IP</label>
        <div class="form-control">
          <input :value="fingerprint.webrtcLocalIp"
            @input="updateFingerprint('webrtcLocalIp', ($event.target as HTMLInputElement).value)" type="text"
            class="input" placeholder="本地IP（可选）" />
        </div>
      </div>

      <div class="form-row">
        <label class="form-label">WebGL 厂商</label>
        <div class="form-control">
          <div class="input-with-refresh">
            <input :value="fingerprint.webglVendor"
              @input="updateFingerprint('webglVendor', ($event.target as HTMLInputElement).value)" type="text"
              class="input" placeholder="Google Inc. (NVIDIA)" />
            <button type="button" class="refresh-btn-small" title="刷新WebGL厂商" @click="emit('regenerate')">
              <span class="material-symbols-outlined">refresh</span>
            </button>
          </div>
        </div>
      </div>

      <div class="form-row">
        <label class="form-label">WebGL 渲染</label>
        <div class="form-control">
          <div class="input-with-refresh">
            <textarea :value="fingerprint.webglRenderer"
              @input="updateFingerprint('webglRenderer', ($event.target as HTMLTextAreaElement).value)" class="textarea"
              rows="2" placeholder="ANGLE (NVIDIA, NVIDIA GeForce GTX 1650...)"></textarea>
            <button type="button" class="refresh-btn-small" title="刷新WebGL渲染器" @click="emit('regenerate')">
              <span class="material-symbols-outlined">refresh</span>
            </button>
          </div>
        </div>
      </div>

      <!-- WebGL 高级选项 -->
      <div class="form-row webgl-advanced">
        <label class="form-label">
          WebGL 高级选项
          <span class="help-icon" title="Unmasked字段和噪声配置">
            <span class="material-symbols-outlined">help</span>
          </span>
        </label>
        <div class="form-control">
          <!-- Unmasked 模式选择 -->
          <div class="advanced-option">
            <div class="option-header">
              <span class="option-label">Unmasked 字段</span>
              <div class="btn-group btn-group-sm">
                <button type="button" class="btn-option"
                  :class="{ active: (fingerprint.webglUnmaskedMode || 'mask') === 'mask' }"
                  @click="updateFingerprint('webglUnmaskedMode', 'mask')">
                  伪装（推荐）
                </button>
                <button type="button" class="btn-option" :class="{ active: fingerprint.webglUnmaskedMode === 'real' }"
                  @click="updateFingerprint('webglUnmaskedMode', 'real')">
                  真实
                </button>
              </div>
            </div>
            <div v-if="fingerprint.webglUnmaskedMode === 'real'" class="unmasked-info">
              <div class="info-item">
                <span class="info-label">Vendor Unmasked:</span>
                <span class="info-value">{{ fingerprint.webglVendorUnmasked || 'NVIDIA Corporation' }}</span>
              </div>
              <div class="info-item">
                <span class="info-label">Renderer Unmasked:</span>
                <span class="info-value">{{ fingerprint.webglRendererUnmasked || 'NVIDIA GeForce GTX 1660' }}</span>
              </div>
            </div>
            <div v-else class="hint-text">
              ℹ️ 伪装模式下 unmasked 字段与 masked 字段相同，更安全
            </div>
          </div>

          <!-- WebGL 噪声开关 -->
          <div class="advanced-option">
            <div class="option-header">
              <span class="option-label">WebGL 噪声</span>
              <div class="btn-group btn-group-sm">
                <button type="button" class="btn-option" :class="{ active: fingerprint.webglNoise !== false }"
                  @click="updateFingerprint('webglNoise', true)">
                  开启
                </button>
                <button type="button" class="btn-option" :class="{ active: fingerprint.webglNoise === false }"
                  @click="updateFingerprint('webglNoise', false)">
                  关闭
                </button>
              </div>
            </div>
            <div class="hint-text">
              💡 开启噪声可增加指纹随机性，防止跟踪
            </div>
          </div>
        </div>
      </div>

      <div class="form-row">
        <label class="form-label">
          WebGPU
          <span class="help-icon" title="WebGPU 配置">
            <span class="material-symbols-outlined">help</span>
          </span>
        </label>
        <div class="form-control">
          <div class="radio-group">
            <label class="radio-item"
              :class="{ active: fingerprint.webgpuMode === 'webgl' || (!fingerprint.webgpuMode && fingerprint.webgpu !== false) }">
              <input type="radio"
                :checked="fingerprint.webgpuMode === 'webgl' || (!fingerprint.webgpuMode && fingerprint.webgpu !== false)"
                @change="updateFingerprint('webgpuMode', 'webgl'); updateFingerprint('webgpu', true)" />
              <span>基于WebGL</span>
            </label>
            <label class="radio-item" :class="{ active: fingerprint.webgpuMode === 'real' }">
              <input type="radio" :checked="fingerprint.webgpuMode === 'real'"
                @change="updateFingerprint('webgpuMode', 'real'); updateFingerprint('webgpu', true)" />
              <span>真实</span>
            </label>
            <label class="radio-item"
              :class="{ active: fingerprint.webgpuMode === 'disabled' || fingerprint.webgpu === false }">
              <input type="radio" :checked="fingerprint.webgpuMode === 'disabled' || fingerprint.webgpu === false"
                @change="updateFingerprint('webgpuMode', 'disabled'); updateFingerprint('webgpu', false)" />
              <span>禁用</span>
            </label>
          </div>
        </div>
      </div>
    </div>

    <!-- Canvas & Audio -->
    <div class="form-section">
      <div class="section-title">Canvas & Audio</div>

      <div class="form-row">
        <label class="form-label">Canvas</label>
        <div class="form-control">
          <div class="control-with-refresh">
            <div class="radio-group">
              <label class="radio-item" :class="{ active: fingerprint.canvas === 'noise' }">
                <input type="radio" :checked="fingerprint.canvas === 'noise'"
                  @change="updateFingerprint('canvas', 'noise')" />
                <span>添加噪声</span>
              </label>
              <label class="radio-item" :class="{ active: fingerprint.canvas === 'block' }">
                <input type="radio" :checked="fingerprint.canvas === 'block'"
                  @change="updateFingerprint('canvas', 'block')" />
                <span>阻止</span>
              </label>
              <label class="radio-item" :class="{ active: fingerprint.canvas === 'off' }">
                <input type="radio" :checked="fingerprint.canvas === 'off'"
                  @change="updateFingerprint('canvas', 'off')" />
                <span>关闭</span>
              </label>
            </div>
            <button type="button" class="refresh-btn-small" title="刷新Canvas噪声" @click="emit('regenerate')">
              <span class="material-symbols-outlined">refresh</span>
            </button>
          </div>
        </div>
      </div>

      <div class="form-row">
        <label class="form-label">AudioContext</label>
        <div class="form-control">
          <div class="control-with-refresh">
            <div class="radio-group">
              <label class="radio-item" :class="{ active: fingerprint.audioContext === 'noise' }">
                <input type="radio" :checked="fingerprint.audioContext === 'noise'"
                  @change="updateFingerprint('audioContext', 'noise')" />
                <span>添加噪声</span>
              </label>
              <label class="radio-item" :class="{ active: fingerprint.audioContext === 'block' }">
                <input type="radio" :checked="fingerprint.audioContext === 'block'"
                  @change="updateFingerprint('audioContext', 'block')" />
                <span>阻止</span>
              </label>
              <label class="radio-item" :class="{ active: fingerprint.audioContext === 'off' }">
                <input type="radio" :checked="fingerprint.audioContext === 'off'"
                  @change="updateFingerprint('audioContext', 'off')" />
                <span>关闭</span>
              </label>
            </div>
            <button type="button" class="refresh-btn-small" title="刷新Audio噪声" @click="emit('regenerate')">
              <span class="material-symbols-outlined">refresh</span>
            </button>
          </div>
        </div>
      </div>
    </div>

    <!-- 隐私保护 -->
    <div class="form-section">
      <div class="section-title">隐私保护</div>

      <div class="form-row">
        <label class="form-label">Do Not Track</label>
        <div class="form-control">
          <div class="radio-group">
            <label class="radio-item" :class="{ active: fingerprint.doNotTrack === '1' }">
              <input type="radio" :checked="fingerprint.doNotTrack === '1'"
                @change="updateFingerprint('doNotTrack', '1')" />
              <span>启用</span>
            </label>
            <label class="radio-item" :class="{ active: fingerprint.doNotTrack === '0' }">
              <input type="radio" :checked="fingerprint.doNotTrack === '0'"
                @change="updateFingerprint('doNotTrack', '0')" />
              <span>禁用</span>
            </label>
            <label class="radio-item" :class="{ active: fingerprint.doNotTrack === 'unspecified' }">
              <input type="radio" :checked="fingerprint.doNotTrack === 'unspecified'"
                @change="updateFingerprint('doNotTrack', 'unspecified')" />
              <span>未指定</span>
            </label>
          </div>
        </div>
      </div>

      <div class="form-row">
        <label class="form-label">Client Rects</label>
        <div class="form-control">
          <label class="toggle-switch">
            <input type="checkbox" :checked="fingerprint.clientRects"
              @change="updateFingerprint('clientRects', !fingerprint.clientRects)" />
            <span class="toggle-slider"></span>
          </label>
        </div>
      </div>

      <div class="form-row">
        <label class="form-label">媒体设备</label>
        <div class="form-control">
          <div class="radio-group">
            <label class="radio-item" :class="{ active: fingerprint.mediaDevices === 'real' }">
              <input type="radio" :checked="fingerprint.mediaDevices === 'real'"
                @change="updateFingerprint('mediaDevices', 'real')" />
              <span>真实</span>
            </label>
            <label class="radio-item" :class="{ active: fingerprint.mediaDevices === 'fake' }">
              <input type="radio" :checked="fingerprint.mediaDevices === 'fake'"
                @change="updateFingerprint('mediaDevices', 'fake')" />
              <span>伪造</span>
            </label>
            <label class="radio-item" :class="{ active: fingerprint.mediaDevices === 'disabled' }">
              <input type="radio" :checked="fingerprint.mediaDevices === 'disabled'"
                @change="updateFingerprint('mediaDevices', 'disabled')" />
              <span>禁用</span>
            </label>
          </div>
        </div>
      </div>

      <div class="form-row">
        <label class="form-label">端口扫描保护</label>
        <div class="form-control">
          <label class="toggle-switch">
            <input type="checkbox" :checked="fingerprint.portScanProtection"
              @change="updateFingerprint('portScanProtection', !fingerprint.portScanProtection)" />
            <span class="toggle-slider"></span>
          </label>
        </div>
      </div>
    </div>

    <!-- 设备信息 -->
    <div class="form-section">
      <div class="section-title">设备信息</div>

      <div class="form-row">
        <label class="form-label">设备名称</label>
        <div class="form-control">
          <input :value="fingerprint.deviceName"
            @input="updateFingerprint('deviceName', ($event.target as HTMLInputElement).value)" type="text"
            class="input" placeholder="DESKTOP-W0KJT6V0" />
        </div>
      </div>

      <div class="form-row">
        <label class="form-label">MAC地址</label>
        <div class="form-control">
          <input :value="fingerprint.macAddress"
            @input="updateFingerprint('macAddress', ($event.target as HTMLInputElement).value)" type="text"
            class="input" placeholder="64-2B-7A-4D-96-E1" />
        </div>
      </div>
    </div>

    <!-- 性能设置 -->
    <div class="form-section">
      <div class="section-title">性能设置</div>

      <div class="form-row">
        <label class="form-label">硬件加速模式</label>
        <div class="form-control">
          <label class="toggle-switch">
            <input type="checkbox" :checked="fingerprint.hardwareAcceleration"
              @change="updateFingerprint('hardwareAcceleration', !fingerprint.hardwareAcceleration)" />
            <span class="toggle-slider"></span>
          </label>
        </div>
      </div>

      <div class="form-row">
        <label class="form-label">禁用沙箱</label>
        <div class="form-control">
          <label class="toggle-switch">
            <input type="checkbox" :checked="fingerprint.disableSandbox"
              @change="updateFingerprint('disableSandbox', !fingerprint.disableSandbox)" />
            <span class="toggle-slider"></span>
          </label>
        </div>
      </div>

      <div class="form-row">
        <label class="form-label">启动参数</label>
        <div class="form-control">
          <textarea :value="fingerprint.launchArgs"
            @input="updateFingerprint('launchArgs', ($event.target as HTMLTextAreaElement).value)" class="textarea"
            rows="3" placeholder="额外的Chromium启动参数，每行一个"></textarea>
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

// 快捷操作
.quick-actions {
  display: flex;
  justify-content: center;
  margin-bottom: 24px;
}

.btn-regenerate {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 10px 20px;
  background: linear-gradient(135deg, #2563eb 0%, #1d4ed8 100%);
  color: white;
  border: none;
  border-radius: 8px;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;

  .material-symbols-outlined {
    font-size: 18px;
  }

  &:hover {
    transform: translateY(-1px);
    box-shadow: 0 4px 12px rgba(37, 99, 235, 0.3);
  }
}

// 表单分区
.form-section {
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
  margin-bottom: 16px;
}

.form-row {
  display: grid;
  grid-template-columns: 120px 1fr;
  gap: 16px;
  align-items: start;
  margin-bottom: 16px;

  &:last-child {
    margin-bottom: 0;
  }

  &.sub-row {
    margin-left: 20px;

    .form-label {
      color: #64748b;
    }
  }
}

.form-label {
  font-size: 14px;
  font-weight: 500;
  color: #1e293b;
  padding-top: 10px;
  display: flex;
  align-items: center;
  gap: 4px;

  .help-icon {
    display: inline-flex;
    cursor: help;

    .material-symbols-outlined {
      font-size: 14px;
      color: #94a3b8;
    }
  }
}

.form-hint {
  width: 100%;
  font-size: 12px;
  color: #94a3b8;
  margin-top: 4px;
}

.form-control {
  display: flex;
  align-items: center;
  gap: 8px;
  flex-wrap: wrap;
}

// User-Agent 输入框包装
.ua-input-wrapper {
  position: relative;
  width: 100%;
  display: flex;
  align-items: center;
  gap: 8px;
}

.ua-input {
  flex: 1;
  padding-right: 48px !important;
}

.refresh-btn {
  position: absolute;
  right: 8px;
  width: 32px;
  height: 32px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: #f1f5f9;
  border: none;
  border-radius: 6px;
  cursor: pointer;
  transition: all 0.2s;

  .material-symbols-outlined {
    font-size: 18px;
    color: #64748b;
  }

  &:hover {
    background: #e2e8f0;

    .material-symbols-outlined {
      color: #2563eb;
    }
  }
}

.ua-tags {
  display: flex;
  gap: 8px;
  width: 100%;
}

.ua-tag {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  padding: 4px 10px;
  background: #f1f5f9;
  color: #64748b;
  border-radius: 4px;
  font-size: 12px;
  font-weight: 500;

  .ua-tag-icon {
    width: 12px;
    height: 12px;
  }

  &.ua-tag-primary {
    background: #dbeafe;
    color: #2563eb;
  }
}

.divider {
  height: 1px;
  background: #e2e8f0;
  margin: 16px 0;
}

// 带刷新按钮的控件包装
.control-with-refresh {
  display: flex;
  align-items: center;
  gap: 8px;
  width: 100%;
}

.input-with-refresh {
  position: relative;
  display: flex;
  align-items: start;
  gap: 8px;
  width: 100%;

  .input,
  .textarea {
    flex: 1;
  }
}

.refresh-btn-small {
  flex-shrink: 0;
  width: 42px;
  height: 42px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: #f1f5f9;
  border: none;
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.2s;

  .material-symbols-outlined {
    font-size: 18px;
    color: #64748b;
  }

  &:hover {
    background: #e2e8f0;

    .material-symbols-outlined {
      color: #2563eb;
      transform: rotate(90deg);
    }
  }
}

// 输入框
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

  &.small-input {
    width: 120px;
  }
}

// 下拉框
.select {
  width: 100%;
  height: 42px;
  padding: 0 12px;
  border: 1px solid #e2e8f0;
  border-radius: 8px;
  font-size: 14px;
  color: #1e293b;
  background: white;
  cursor: pointer;
  transition: all 0.2s;

  &:focus {
    outline: none;
    border-color: #2563eb;
    box-shadow: 0 0 0 3px rgba(37, 99, 235, 0.1);
  }
}

// 平台选择器
.platform-selector {
  display: flex;
  gap: 0;
  border: 1px solid #e2e8f0;
  border-radius: 8px;
  overflow: hidden;
  width: 100%;
}

.platform-option {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 6px;
  padding: 12px 8px;
  background: white;
  border-right: 1px solid #e2e8f0;
  cursor: pointer;
  transition: all 0.2s;

  &:last-child {
    border-right: none;
  }

  input {
    display: none;
  }

  .platform-icon {
    width: 20px;
    height: 20px;
    color: #64748b;
    transition: color 0.2s;
  }

  span {
    font-size: 12px;
    font-weight: 500;
    color: #64748b;
    transition: color 0.2s;
  }

  &:hover {
    background: #f8fafc;
  }

  &.active {
    background: #eff6ff;

    .platform-icon {
      color: #2563eb;
    }

    span {
      color: #2563eb;
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

// 单位
.unit {
  font-size: 14px;
  color: #64748b;
}

// 单选按钮组
.radio-group {
  display: flex;
  gap: 0;
  border: 1px solid #e2e8f0;
  border-radius: 8px;
  overflow: hidden;
}

.radio-item {
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 8px 14px;
  font-size: 13px;
  color: #64748b;
  background: white;
  cursor: pointer;
  border-right: 1px solid #e2e8f0;
  transition: all 0.2s;

  &:last-child {
    border-right: none;
  }

  input {
    display: none;
  }

  &:hover {
    background: #f8fafc;
  }

  &.active {
    background: #2563eb;
    color: white;
  }
}

// 单选按钮组（在控件组中的情况）
.control-with-refresh .radio-group {
  flex: 1;
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

  input:checked+.toggle-slider {
    background: #2563eb;

    &::before {
      left: 22px;
    }
  }
}

// WebGL 高级选项
.webgl-advanced {
  .form-control {
    flex-direction: column;
    align-items: stretch;
    gap: 12px;
  }
}

.advanced-option {
  background: #f8fafc;
  border-radius: 8px;
  padding: 12px;

  .option-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 8px;
  }

  .option-label {
    font-size: 13px;
    font-weight: 500;
    color: #475569;
  }
}

.btn-group {
  display: flex;
  gap: 0;
  border: 1px solid #e2e8f0;
  border-radius: 6px;
  overflow: hidden;

  &.btn-group-sm {
    .btn-option {
      padding: 6px 12px;
      font-size: 12px;
    }
  }
}

.btn-option {
  padding: 8px 14px;
  font-size: 13px;
  color: #64748b;
  background: white;
  border: none;
  border-right: 1px solid #e2e8f0;
  cursor: pointer;
  transition: all 0.2s;

  &:last-child {
    border-right: none;
  }

  &:hover {
    background: #f1f5f9;
  }

  &.active {
    background: #2563eb;
    color: white;
  }
}

.unmasked-info {
  background: white;
  border: 1px solid #e2e8f0;
  border-radius: 6px;
  padding: 10px;
  margin-top: 8px;

  .info-item {
    display: flex;
    align-items: center;
    gap: 8px;
    margin-bottom: 6px;

    &:last-child {
      margin-bottom: 0;
    }
  }

  .info-label {
    font-size: 12px;
    color: #64748b;
    flex-shrink: 0;
  }

  .info-value {
    font-size: 12px;
    color: #1e293b;
    font-family: monospace;
    word-break: break-all;
  }
}

.hint-text {
  font-size: 12px;
  color: #94a3b8;
  margin-top: 4px;
}
</style>
