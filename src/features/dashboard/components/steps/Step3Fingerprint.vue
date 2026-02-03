<script setup lang="ts">
/**
 * @description 创建窗口 - 第三步：高级指纹设置
 */
import { ref, computed } from 'vue'
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

// ==================== 字体配置 ====================

// 常用字体列表（Windows 系统常见字体）
const COMMON_FONTS = [
  // 基础字体
  'Arial', 'Arial Black', 'Calibri', 'Cambria', 'Courier New',
  'Georgia', 'Helvetica', 'Impact', 'Microsoft YaHei', 'SimSun',
  'SimHei', 'Tahoma', 'Times New Roman', 'Trebuchet MS', 'Verdana',
  // 扩展字体
  'Comic Sans MS', 'Consolas', 'Lucida Console', 'Segoe UI', 'Palatino Linotype',
  'Book Antiqua', 'Garamond', 'Century Gothic', 'Franklin Gothic Medium',
  // 中文字体
  'KaiTi', 'FangSong', 'NSimSun', 'Microsoft JhengHei', 'DengXian'
]

// 字体模式选项
const FONTS_MODE_OPTIONS = [
  { value: 'subset', label: '精简子集', desc: '使用常用字体，风控低' },
  { value: 'real', label: '真实字体', desc: '显示系统真实字体' },
  { value: 'custom', label: '自定义', desc: '手动选择字体列表' },
  { value: 'random', label: '随机子集', desc: '随机选择字体组合' }
]

// 字体搜索关键词
const fontSearchKeyword = ref('')

// 过滤后的字体列表
const filteredFonts = computed(() => {
  if (!fontSearchKeyword.value) return COMMON_FONTS
  const keyword = fontSearchKeyword.value.toLowerCase()
  return COMMON_FONTS.filter(font => font.toLowerCase().includes(keyword))
})

// 切换字体选中状态
const toggleFont = (font: string) => {
  const currentList = [...props.modelValue.fontsList]
  const index = currentList.indexOf(font)
  if (index > -1) {
    currentList.splice(index, 1)
  } else {
    currentList.push(font)
  }
  updateField('fontsList', currentList)
}

// 全选/取消全选
const toggleAllFonts = () => {
  if (props.modelValue.fontsList.length === COMMON_FONTS.length) {
    updateField('fontsList', [])
  } else {
    updateField('fontsList', [...COMMON_FONTS])
  }
}

// 随机选择字体
const randomSelectFonts = () => {
  const count = Math.floor(Math.random() * 10) + 10 // 10-20个字体
  const shuffled = [...COMMON_FONTS].sort(() => Math.random() - 0.5)
  updateField('fontsList', shuffled.slice(0, count))
}

// 生成 Variations ID
const generateVariationsId = () => {
  const id = crypto.randomUUID().replace(/-/g, '').substring(0, 16)
  updateField('variationsSeedId', id)
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
      
      <!-- 字体配置 -->
      <div class="section-header">
        <h3 class="section-title">字体配置</h3>
      </div>
      
      <div class="form-row">
        <label class="form-label">字体模式</label>
        <div class="radio-group-vertical">
          <label 
            v-for="option in FONTS_MODE_OPTIONS" 
            :key="option.value"
            class="radio-item-card" 
            :class="{ active: modelValue.fontsMode === option.value }"
          >
            <input 
              type="radio" 
              :value="option.value" 
              :checked="modelValue.fontsMode === option.value" 
              @change="updateField('fontsMode', option.value as any)" 
            />
            <div class="radio-content">
              <span class="radio-label">{{ option.label }}</span>
              <span class="radio-desc">{{ option.desc }}</span>
            </div>
          </label>
        </div>
      </div>
      
      <!-- 自定义字体选择器 -->
      <div v-if="modelValue.fontsMode === 'custom'" class="form-row">
        <label class="form-label">选择字体</label>
        <div class="fonts-selector">
          <div class="fonts-toolbar">
            <input 
              v-model="fontSearchKeyword"
              class="form-input fonts-search" 
              type="text" 
              placeholder="搜索字体..."
            />
            <button type="button" class="action-btn" @click="toggleAllFonts" title="全选/取消">
              <svg width="14" height="14" viewBox="0 0 16 16" fill="currentColor">
                <path d="M2 4a2 2 0 012-2h8a2 2 0 012 2v8a2 2 0 01-2 2H4a2 2 0 01-2-2V4zm2-1a1 1 0 00-1 1v8a1 1 0 001 1h8a1 1 0 001-1V4a1 1 0 00-1-1H4z"/>
                <path d="M10.97 4.97a.75.75 0 011.07 1.05l-3.99 4.99a.75.75 0 01-1.08.02L4.324 8.384a.75.75 0 111.06-1.06l2.094 2.093 3.473-4.425a.267.267 0 01.02-.022z"/>
              </svg>
            </button>
            <button type="button" class="action-btn" @click="randomSelectFonts" title="随机选择">
              <svg width="14" height="14" viewBox="0 0 16 16" fill="currentColor">
                <path d="M8 3a5 5 0 100 10A5 5 0 008 3zM2 8a6 6 0 1110.89 3.476l2.317 2.317a.5.5 0 01-.708.708l-2.317-2.317A6 6 0 012 8z"/>
              </svg>
            </button>
          </div>
          <div class="fonts-list">
            <label 
              v-for="font in filteredFonts" 
              :key="font"
              class="font-item"
              :class="{ selected: modelValue.fontsList.includes(font) }"
            >
              <input 
                type="checkbox" 
                :checked="modelValue.fontsList.includes(font)"
                @change="toggleFont(font)"
              />
              <span class="font-name" :style="{ fontFamily: font }">{{ font }}</span>
            </label>
          </div>
          <div class="fonts-count">
            已选: {{ modelValue.fontsList.length }} / {{ COMMON_FONTS.length }}
          </div>
        </div>
      </div>
      
      <!-- 随机子集时显示数量配置 -->
      <div v-if="modelValue.fontsMode === 'random'" class="form-row">
        <label class="form-label">当前随机字体</label>
        <div class="random-fonts-info">
          <span class="info-text">已选择 {{ modelValue.fontsList.length }} 个字体</span>
          <button type="button" class="action-btn" @click="randomSelectFonts" title="重新随机">
            <svg width="14" height="14" viewBox="0 0 16 16" fill="currentColor">
              <path fill-rule="evenodd" d="M8 3a5 5 0 104.546 2.914.5.5 0 01.908-.418A6 6 0 118 2v1z"/>
              <path d="M8 4.466V.534a.25.25 0 01.41-.192l2.36 1.966c.12.1.12.284 0 .384L8.41 4.658A.25.25 0 018 4.466z"/>
            </svg>
            <span>重新随机</span>
          </button>
        </div>
      </div>
      
      <!-- 自定义字体输入 -->
      <div v-if="modelValue.fontsMode === 'custom'" class="form-row">
        <label class="form-label">额外字体</label>
        <textarea 
          :value="modelValue.customFonts"
          class="form-textarea" 
          rows="2" 
          placeholder="输入额外字体名称，用英文逗号分隔，如: Roboto, Open Sans"
          @input="updateField('customFonts', ($event.target as HTMLTextAreaElement).value)"
        ></textarea>
      </div>
      
      <!-- Variations 配置 -->
      <div class="section-header">
        <h3 class="section-title">Variations 配置</h3>
        <span class="section-tip">内核实验分组标识</span>
      </div>
      
      <div class="form-row">
        <label class="form-label">启用 Variations</label>
        <div 
          class="toggle-switch" 
          :class="{ active: modelValue.variationsEnabled }" 
          @click="updateField('variationsEnabled', !modelValue.variationsEnabled)"
        >
          <div class="toggle-handle"></div>
        </div>
      </div>
      
      <div v-if="modelValue.variationsEnabled" class="form-row">
        <label class="form-label">Variations ID</label>
        <div class="input-with-action">
          <input 
            :value="modelValue.variationsSeedId"
            class="form-input" 
            type="text" 
            placeholder="留空则自动生成"
            @input="updateField('variationsSeedId', ($event.target as HTMLInputElement).value)"
          />
          <button type="button" class="action-btn" @click="generateVariationsId" title="生成新 ID">
            <svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor">
              <path fill-rule="evenodd" d="M8 3a5 5 0 104.546 2.914.5.5 0 01.908-.418A6 6 0 118 2v1z"/>
              <path d="M8 4.466V.534a.25.25 0 01.41-.192l2.36 1.966c.12.1.12.284 0 .384L8.41 4.658A.25.25 0 018 4.466z"/>
            </svg>
          </button>
        </div>
        <p class="form-hint">每个 Profile 应有唯一的 Variations ID，空则自动基于 Profile ID 生成</p>
      </div>
    </div>
  </div>
</template>

<style scoped lang="scss">
@import './_step-form.scss';
</style>
