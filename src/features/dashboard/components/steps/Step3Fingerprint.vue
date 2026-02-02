<script setup lang="ts">
/**
 * @description 创建窗口 - 第三步：高级指纹设置
 */
import type { CreateProfileFormData } from '../../composables/useCreateProfile'

const props = defineProps<{
  modelValue: CreateProfileFormData
}>()

const emit = defineEmits<{
  (e: 'update:modelValue', value: CreateProfileFormData): void
  (e: 'generateRandomMac'): void
}>()

// 双向绑定辅助
const updateField = <K extends keyof CreateProfileFormData>(key: K, value: CreateProfileFormData[K]) => {
  emit('update:modelValue', { ...props.modelValue, [key]: value })
}
</script>

<template>
  <div class="step-pane scrollable">
    <div class="form-section">
      <!-- 基础指纹 -->
      <div class="section-header">
        <h3 class="section-title">基础指纹</h3>
      </div>
      
      <div class="form-row">
        <label class="form-label">分辨率</label>
        <select 
          :value="modelValue.resolution"
          class="form-select"
          @change="updateField('resolution', ($event.target as HTMLSelectElement).value)"
        >
          <option value="1920x1080">1920x1080</option>
          <option value="1366x768">1366x768</option>
          <option value="2560x1440">2560x1440</option>
          <option value="1440x900">1440x900</option>
          <option value="1280x720">1280x720</option>
        </select>
      </div>
      
      <div class="form-row">
        <label class="form-label">硬件并发数</label>
        <input 
          :value="modelValue.hardwareConcurrency"
          class="form-input" 
          type="number" 
          min="2" 
          max="32" 
          placeholder="16"
          @input="updateField('hardwareConcurrency', parseInt(($event.target as HTMLInputElement).value) || 16)"
        />
      </div>
      
      <div class="form-row">
        <label class="form-label">设备内存</label>
        <div class="memory-input-group">
          <select 
            :value="modelValue.deviceMemory"
            class="form-select"
            @change="updateField('deviceMemory', parseInt(($event.target as HTMLSelectElement).value))"
          >
            <option :value="4">4 GB</option>
            <option :value="8">8 GB</option>
            <option :value="16">16 GB</option>
            <option :value="32">32 GB</option>
            <option :value="64">64 GB</option>
          </select>
        </div>
      </div>
      
      <!-- WebRTC & WebGL -->
      <div class="section-header">
        <h3 class="section-title">WebRTC & WebGL</h3>
      </div>
      
      <div class="form-row">
        <label class="form-label">WebRTC</label>
        <div class="radio-group">
          <label class="radio-item" :class="{ active: modelValue.webrtc === 'real' }">
            <input type="radio" value="real" :checked="modelValue.webrtc === 'real'" @change="updateField('webrtc', 'real')" />
            <span>真实</span>
          </label>
          <label class="radio-item" :class="{ active: modelValue.webrtc === 'fake' }">
            <input type="radio" value="fake" :checked="modelValue.webrtc === 'fake'" @change="updateField('webrtc', 'fake')" />
            <span>伪造</span>
          </label>
          <label class="radio-item" :class="{ active: modelValue.webrtc === 'disabled' }">
            <input type="radio" value="disabled" :checked="modelValue.webrtc === 'disabled'" @change="updateField('webrtc', 'disabled')" />
            <span>禁用</span>
          </label>
        </div>
      </div>
      
      <div v-if="modelValue.webrtc === 'fake'" class="form-row sub-row">
        <label class="form-label">公网IP</label>
        <input 
          :value="modelValue.webrtcPublicIp"
          class="form-input" 
          type="text" 
          placeholder="公网IP地址（可选）"
          @input="updateField('webrtcPublicIp', ($event.target as HTMLInputElement).value)"
        />
      </div>
      
      <div v-if="modelValue.webrtc === 'fake'" class="form-row sub-row">
        <label class="form-label">本地IP</label>
        <input 
          :value="modelValue.webrtcLocalIp"
          class="form-input" 
          type="text" 
          placeholder="本地IP地址（可选）"
          @input="updateField('webrtcLocalIp', ($event.target as HTMLInputElement).value)"
        />
      </div>
      
      <div class="form-row">
        <label class="form-label">WebGL 厂商</label>
        <input 
          :value="modelValue.webglVendor"
          class="form-input" 
          type="text" 
          placeholder="Intel Inc."
          @input="updateField('webglVendor', ($event.target as HTMLInputElement).value)"
        />
      </div>
      
      <div class="form-row">
        <label class="form-label">WebGL 渲染</label>
        <input 
          :value="modelValue.webglRenderer"
          class="form-input" 
          type="text" 
          placeholder="Intel Iris OpenGL Engine"
          @input="updateField('webglRenderer', ($event.target as HTMLInputElement).value)"
        />
      </div>
      
      <div class="form-row">
        <label class="form-label">WebGpu</label>
        <div 
          class="toggle-switch" 
          :class="{ active: modelValue.webgpu === 'real' }" 
          @click="updateField('webgpu', modelValue.webgpu === 'real' ? 'webgl' : 'real')"
        >
          <div class="toggle-handle"></div>
        </div>
      </div>
      
      <!-- Canvas & Audio -->
      <div class="section-header">
        <h3 class="section-title">Canvas & Audio</h3>
      </div>
      
      <div class="form-row">
        <label class="form-label">Canvas</label>
        <div class="radio-group">
          <label class="radio-item" :class="{ active: modelValue.canvas === 'noise' }">
            <input type="radio" value="noise" :checked="modelValue.canvas === 'noise'" @change="updateField('canvas', 'noise')" />
            <span>添加噪声</span>
          </label>
          <label class="radio-item" :class="{ active: modelValue.canvas === 'block' }">
            <input type="radio" value="block" :checked="modelValue.canvas === 'block'" @change="updateField('canvas', 'block')" />
            <span>阻止</span>
          </label>
          <label class="radio-item" :class="{ active: modelValue.canvas === 'off' }">
            <input type="radio" value="off" :checked="modelValue.canvas === 'off'" @change="updateField('canvas', 'off')" />
            <span>关闭</span>
          </label>
        </div>
      </div>
      
      <div class="form-row">
        <label class="form-label">AudioContext</label>
        <div class="radio-group">
          <label class="radio-item" :class="{ active: modelValue.audioContext === 'noise' }">
            <input type="radio" value="noise" :checked="modelValue.audioContext === 'noise'" @change="updateField('audioContext', 'noise')" />
            <span>添加噪声</span>
          </label>
          <label class="radio-item" :class="{ active: modelValue.audioContext === 'block' }">
            <input type="radio" value="block" :checked="modelValue.audioContext === 'block'" @change="updateField('audioContext', 'block')" />
            <span>阻止</span>
          </label>
          <label class="radio-item" :class="{ active: modelValue.audioContext === 'off' }">
            <input type="radio" value="off" :checked="modelValue.audioContext === 'off'" @change="updateField('audioContext', 'off')" />
            <span>关闭</span>
          </label>
        </div>
      </div>
      
      <!-- 隐私保护 -->
      <div class="section-header">
        <h3 class="section-title">隐私保护</h3>
      </div>
      
      <div class="form-row">
        <label class="form-label">Do Not Track</label>
        <div class="radio-group">
          <label class="radio-item" :class="{ active: modelValue.doNotTrack === true }">
            <input type="radio" :value="true" :checked="modelValue.doNotTrack === true" @change="updateField('doNotTrack', true)" />
            <span>启用</span>
          </label>
          <label class="radio-item" :class="{ active: modelValue.doNotTrack === false }">
            <input type="radio" :value="false" :checked="modelValue.doNotTrack === false" @change="updateField('doNotTrack', false)" />
            <span>禁用</span>
          </label>
          <label class="radio-item" :class="{ active: modelValue.doNotTrack === null }">
            <input type="radio" :value="null" :checked="modelValue.doNotTrack === null" @change="updateField('doNotTrack', null)" />
            <span>未指定</span>
          </label>
        </div>
      </div>
      
      <div class="form-row">
        <label class="form-label">Client Rects</label>
        <div 
          class="toggle-switch" 
          :class="{ active: modelValue.clientRects === 'real' }" 
          @click="updateField('clientRects', modelValue.clientRects === 'real' ? 'fake' : 'real')"
        >
          <div class="toggle-handle"></div>
        </div>
      </div>
      
      <div class="form-row">
        <label class="form-label">媒体设备</label>
        <div class="radio-group">
          <label class="radio-item" :class="{ active: modelValue.mediaDevices === 'real' }">
            <input type="radio" value="real" :checked="modelValue.mediaDevices === 'real'" @change="updateField('mediaDevices', 'real')" />
            <span>真实</span>
          </label>
          <label class="radio-item" :class="{ active: modelValue.mediaDevices === 'fake' }">
            <input type="radio" value="fake" :checked="modelValue.mediaDevices === 'fake'" @change="updateField('mediaDevices', 'fake')" />
            <span>伪造</span>
          </label>
          <label class="radio-item" :class="{ active: modelValue.mediaDevices === 'disabled' }">
            <input type="radio" value="disabled" :checked="modelValue.mediaDevices === 'disabled'" @change="updateField('mediaDevices', 'disabled')" />
            <span>禁用</span>
          </label>
        </div>
      </div>
      
      <div class="form-row">
        <label class="form-label">端口扫描保护</label>
        <div 
          class="toggle-switch" 
          :class="{ active: modelValue.portScanProtection }" 
          @click="updateField('portScanProtection', !modelValue.portScanProtection)"
        >
          <div class="toggle-handle"></div>
        </div>
      </div>
      
      <div v-if="!modelValue.portScanProtection" class="form-row">
        <label class="form-label">扫描白名单</label>
        <textarea 
          :value="modelValue.scanWhitelist"
          class="form-textarea" 
          rows="2" 
          placeholder="允许被网站扫描的端口，多个用英文逗号隔开"
          @input="updateField('scanWhitelist', ($event.target as HTMLTextAreaElement).value)"
        ></textarea>
      </div>
      
      <!-- 设备信息 -->
      <div class="section-header">
        <h3 class="section-title">设备信息</h3>
      </div>
      
      <div class="form-row">
        <label class="form-label">设备名称</label>
        <input 
          :value="modelValue.deviceName"
          class="form-input" 
          type="text" 
          placeholder="DESKTOP-W0KJT6V0"
          @input="updateField('deviceName', ($event.target as HTMLInputElement).value)"
        />
      </div>
      
      <div class="form-row">
        <label class="form-label">MAC地址</label>
        <div class="input-with-action">
          <input 
            :value="modelValue.macAddress"
            class="form-input" 
            type="text" 
            placeholder="64-2B-7A-4D-96-E1"
            @input="updateField('macAddress', ($event.target as HTMLInputElement).value)"
          />
          <button type="button" class="action-btn" @click="emit('generateRandomMac')" title="随机生成">
            <svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor">
              <path d="M8 3a5 5 0 1 0 0 10A5 5 0 0 0 8 3zm0 9a4 4 0 1 1 0-8 4 4 0 0 1 0 8z"/>
              <path d="M8 7v2l2 1"/>
            </svg>
          </button>
        </div>
      </div>
      
      <!-- 性能设置 -->
      <div class="section-header">
        <h3 class="section-title">性能设置</h3>
      </div>
      
      <div class="form-row">
        <label class="form-label">硬件加速模式</label>
        <div 
          class="toggle-switch" 
          :class="{ active: modelValue.hardwareAcceleration }" 
          @click="updateField('hardwareAcceleration', !modelValue.hardwareAcceleration)"
        >
          <div class="toggle-handle"></div>
        </div>
      </div>
      
      <div class="form-row">
        <label class="form-label">禁用沙箱</label>
        <div 
          class="toggle-switch" 
          :class="{ active: modelValue.disableSandbox }" 
          @click="updateField('disableSandbox', !modelValue.disableSandbox)"
        >
          <div class="toggle-handle"></div>
        </div>
      </div>
      
      <div class="form-row">
        <label class="form-label">启动参数</label>
        <textarea 
          :value="modelValue.launchArgs"
          class="form-textarea" 
          rows="3" 
          placeholder="自定义启动参数"
          @input="updateField('launchArgs', ($event.target as HTMLTextAreaElement).value)"
        ></textarea>
      </div>
    </div>
  </div>
</template>

<style scoped lang="scss">
@import './_step-form.scss';
</style>
