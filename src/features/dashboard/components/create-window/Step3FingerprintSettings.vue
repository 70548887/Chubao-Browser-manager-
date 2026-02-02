<script setup lang="ts">
/**
 * @description 步骤3 - 高级指纹设置
 */
import { ref, onMounted, watch, computed } from 'vue'
import { getTemplateList, generateRandomFingerprint, validateFingerprint, type TemplateInfo, type FingerprintConfig, type ValidationResult, type PlatformType, type BrowserVersionType, BROWSER_VERSION_OPTIONS } from '@/api/fingerprintApi'

const model = defineModel<any>({ required: true })

// 模板相关
const templates = ref<TemplateInfo[]>([])
// 使用 computed 将 selectedTemplateId 绑定到 model，这样切换步骤后不会丢失
const selectedTemplateId = computed({
  get: () => model.value.selectedTemplateId || '',
  set: (val) => { model.value.selectedTemplateId = val }
})
const loadingTemplates = ref(false)
const generatingFingerprint = ref(false)

// 校验相关
const validationResult = ref<ValidationResult | null>(null)
const showValidation = ref(false)

// 生成的完整指纹（用于最终提交）
const generatedFingerprint = ref<FingerprintConfig | null>(null)

// 加载模板列表
onMounted(async () => {
  try {
    loadingTemplates.value = true
    templates.value = await getTemplateList()
    console.log('模板列表加载成功:', templates.value)
    
    // 初始化WebGL高级选项默认值
    if (!model.value.webglUnmaskedMode) {
      model.value.webglUnmaskedMode = 'mask' // 默认伪装模式
    }
    if (model.value.webglNoise === undefined) {
      model.value.webglNoise = true // 默认开启噪声
    }
    if (!model.value.platform) {
      model.value.platform = 'windows' // 默认Windows平台
    }
    if (!model.value.browserVersion) {
      model.value.browserVersion = '146' // 默认Chrome 146
    }
    
    // 只有在没有已生成的指纹数据时才自动生成
    // 避免切换步骤返回时覆盖用户已选择的配置
    // 检查 webglRenderer 是否为生成的值（包含 ANGLE 且不是默认的 GTX 1080）
    const hasExistingFingerprint = model.value.webglRenderer && 
                                    model.value.webglRenderer.includes('ANGLE') &&
                                    model.value.hardwareConcurrency &&
                                    model.value.deviceMemory
    
    if (!hasExistingFingerprint) {
      await generateFingerprintForPreview()
      console.log('默认指纹已生成')
    } else {
      console.log('已有指纹数据，跳过自动生成:', {
        webglRenderer: model.value.webglRenderer,
        hardwareConcurrency: model.value.hardwareConcurrency,
        deviceMemory: model.value.deviceMemory
      })
    }
    
  } catch (error) {
    console.error('加载模板失败:', error)
    // 模板加载失败不影响正常使用，只记录日志
  } finally {
    loadingTemplates.value = false
  }
})

// 选择模板并生成指纹
async function selectTemplate(templateId: string) {
  console.log('👉 点击选择模板:', templateId)
  selectedTemplateId.value = templateId
  
  // 根据模板的操作系统自动更新平台选择
  const template = templates.value.find(t => t.id === templateId)
  if (template) {
    const platformMap: Record<string, string> = {
      'Windows': 'windows',
      'MacOS': 'macos',
      'macOS': 'macos',
      'Linux': 'linux',
      'Android': 'android',
      'iOS': 'ios'
    }
    const newPlatform = platformMap[template.os_name] || 'windows'
    console.log(`💻 模板操作系统: ${template.os_name} -> 平台: ${newPlatform}`)
    model.value.platform = newPlatform
  }
  
  console.log('📌 开始生成新指纹...')
  await generateFingerprintForPreview()
  console.log('✅ 指纹生成完成')
}

// 随机选择模板
async function randomTemplate() {
  if (templates.value.length === 0) return
  
  const randomIndex = Math.floor(Math.random() * templates.value.length)
  await selectTemplate(templates.value[randomIndex].id)
}

// 生成指纹（用于预览）
async function generateFingerprintForPreview() {
  try {
    generatingFingerprint.value = true
    showValidation.value = false
    
    // 使用临时ID生成指纹预览
    const previewId = 'preview-' + Date.now()
    // 根据选择的平台生成对应的指纹
    const currentPlatform = (model.value.platform || 'windows') as PlatformType
    const currentVersion = (model.value.browserVersion || '146') as BrowserVersionType
    const fingerprint = await generateRandomFingerprint(previewId, currentPlatform, currentVersion)
    
    generatedFingerprint.value = fingerprint
    
    // 将生成的指纹填充到表单（用于UI显示）
    applyFingerprintToForm(fingerprint)
    
    // 校验指纹
    await validateGeneratedFingerprint(fingerprint)
    
    console.log(`指纹生成成功 [平台: ${currentPlatform}, 内核: Chrome ${currentVersion}]:`, fingerprint)
    
  } catch (error) {
    console.error('生成指纹失败:', error)
    alert(`生成指纹失败: ${error}`)
  } finally {
    generatingFingerprint.value = false
  }
}

// 将指纹应用到表单
function applyFingerprintToForm(fingerprint: FingerprintConfig) {
  console.log('📥 应用指纹到表单, 原始数据:', JSON.stringify(fingerprint, null, 2))
  
  // 基础指纹 - Navigator
  if (fingerprint.navigator) {
    model.value.userAgent = fingerprint.navigator.user_agent || model.value.userAgent
    model.value.hardwareConcurrency = fingerprint.navigator.hardware_concurrency || model.value.hardwareConcurrency
    model.value.deviceMemory = fingerprint.navigator.device_memory || model.value.deviceMemory
    // 语言和时区保持“基于IP”模式，不受模板影响
    console.log('✅ Navigator 已更新:', {
      userAgent: model.value.userAgent?.substring(0, 50) + '...',
      hardwareConcurrency: model.value.hardwareConcurrency,
      deviceMemory: model.value.deviceMemory
    })
  }
  
  // 屏幕分辨率
  if (fingerprint.screen) {
    model.value.screenWidth = fingerprint.screen.width || model.value.screenWidth
    model.value.screenHeight = fingerprint.screen.height || model.value.screenHeight
    console.log('✅ Screen 已更新:', {
      width: model.value.screenWidth,
      height: model.value.screenHeight
    })
  }
  
  // WebGL
  if (fingerprint.webgl) {
    model.value.webglVendor = fingerprint.webgl.vendor || model.value.webglVendor
    model.value.webglRenderer = fingerprint.webgl.renderer || model.value.webglRenderer
    console.log('✅ WebGL 已更新:', {
      vendor: model.value.webglVendor,
      renderer: model.value.webglRenderer
    })
  }
  
  // WebRTC
  if (fingerprint.webrtc) {
    model.value.localIp = fingerprint.webrtc.local_ip || '192.168.1.' + Math.floor(Math.random() * 254 + 1)
  } else {
    model.value.localIp = '192.168.1.' + Math.floor(Math.random() * 254 + 1)
  }
  
  // 时区 - 保持“基于IP”模式，不受模板影响
  // 只更新实际时区ID作为备用值，不改变模式
  if (fingerprint.timezone?.id) {
    model.value.timezoneId = fingerprint.timezone.id
    console.log('✅ Timezone 备用值已更新:', model.value.timezoneId)
  }
  
  // 设备信息 - 每次都重新生成
  model.value.deviceName = 'DESKTOP-' + Math.random().toString(36).substring(2, 10).toUpperCase()
  model.value.macAddress = generateRandomMacAddress()
  console.log('✅ 设备信息已更新:', {
    deviceName: model.value.deviceName,
    macAddress: model.value.macAddress
  })
  
  console.log('🎉 表单数据更新完成')
}

// 生成随机MAC地址
function generateRandomMacAddress(): string {
  const randomByte = () => Math.floor(Math.random() * 256).toString(16).padStart(2, '0').toUpperCase()
  return `${randomByte()}-${randomByte()}-${randomByte()}-${randomByte()}-${randomByte()}-${randomByte()}`
}

// 校验指纹
async function validateGeneratedFingerprint(fingerprint: FingerprintConfig) {
  try {
    const result = await validateFingerprint(fingerprint)
    validationResult.value = result
    showValidation.value = !result.valid || result.warnings.length > 0
    
    if (!result.valid) {
      console.warn('指纹配置有错误:', result.errors)
    }
    if (result.warnings.length > 0) {
      console.warn('指纹配置有警告:', result.warnings)
    }
  } catch (error) {
    console.error('校验指纹失败:', error)
  }
}

// 根据平台、操作系统版本、浏览器版本生成 User-Agent
const generateUserAgent = (platform: string, osVersion: string, browserVersion: string): string => {
  const chromeVersion = browserVersion || '146'
  
  // 操作系统映射
  const osMap: Record<string, string> = {
    // Windows
    'Windows 7': 'Windows NT 6.1; Win64; x64',
    'Windows 8': 'Windows NT 6.2; Win64; x64',
    'Windows 8.1': 'Windows NT 6.3; Win64; x64',
    'Windows 10': 'Windows NT 10.0; Win64; x64',
    'Windows 11': 'Windows NT 10.0; Win64; x64',  // Win11 仍用 NT 10.0
    // macOS
    'macOS 12': 'Macintosh; Intel Mac OS X 12_0_0',
    'macOS 13': 'Macintosh; Intel Mac OS X 13_0_0',
    'macOS 14': 'Macintosh; Intel Mac OS X 14_0_0',
    'macOS 15': 'Macintosh; Intel Mac OS X 15_0_0',
    // Linux
    'Ubuntu 20.04': 'X11; Linux x86_64',
    'Ubuntu 22.04': 'X11; Linux x86_64',
    'Ubuntu 24.04': 'X11; Linux x86_64',
    'Debian 11': 'X11; Linux x86_64',
    'Debian 12': 'X11; Linux x86_64',
    // Android
    'Android 11': 'Linux; Android 11; SM-G991B',
    'Android 12': 'Linux; Android 12; SM-G998B',
    'Android 13': 'Linux; Android 13; SM-S918B',
    'Android 14': 'Linux; Android 14; SM-S928B',
    // iOS
    'iOS 15': 'iPhone; CPU iPhone OS 15_0 like Mac OS X',
    'iOS 16': 'iPhone; CPU iPhone OS 16_0 like Mac OS X',
    'iOS 17': 'iPhone; CPU iPhone OS 17_0 like Mac OS X',
  }
  
  const osString = osMap[osVersion] || 'Windows NT 10.0; Win64; x64'
  
  // 根据平台生成不同格式的 UA
  if (platform === 'ios') {
    return `Mozilla/5.0 (${osString}) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.0 Mobile/15E148 Safari/604.1`
  } else if (platform === 'android') {
    return `Mozilla/5.0 (${osString}) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/${chromeVersion}.0.0.0 Mobile Safari/537.36`
  } else {
    return `Mozilla/5.0 (${osString}) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/${chromeVersion}.0.0.0 Safari/537.36`
  }
}

// 更新 User-Agent
const updateUserAgent = () => {
  model.value.userAgent = generateUserAgent(
    model.value.platform,
    model.value.osVersion,
    model.value.browserVersion
  )
}

// 刷新UA（随机微调版本号）
const refreshUserAgent = () => {
  // 基于当前配置重新生成，并添加随机的小版本号
  const baseUA = generateUserAgent(
    model.value.platform,
    model.value.osVersion,
    model.value.browserVersion
  )
  // 替换版本号中的 .0.0.0 为随机版本
  const randomPatch = Math.floor(Math.random() * 200)
  model.value.userAgent = baseUA.replace(/\.0\.0\.0/, `.0.${randomPatch}.0`)
}

// 刷新MAC地址
const refreshMacAddress = () => {
  const randomByte = () => Math.floor(Math.random() * 256).toString(16).padStart(2, '0').toUpperCase()
  model.value.macAddress = `${randomByte()}-${randomByte()}-${randomByte()}-${randomByte()}-${randomByte()}-${randomByte()}`
}

// 刷新设备名称
const refreshDeviceName = () => {
  model.value.deviceName = 'DESKTOP-' + Math.random().toString(36).substring(2, 10).toUpperCase()
}

// WebGL Vendor选项
const vendorOptions = [
  { value: 'Google Inc. (NVIDIA)', label: 'Google Inc. (NVIDIA)' },
  { value: 'Google Inc. (AMD)', label: 'Google Inc. (AMD)' },
  { value: 'Google Inc. (Intel)', label: 'Google Inc. (Intel)' }
]

// WebGL Renderer选项
// 操作系统版本选项
const osVersionOptions = [
  // Windows
  { value: 'Windows 7', label: 'Windows 7', platform: 'windows' },
  { value: 'Windows 8', label: 'Windows 8', platform: 'windows' },
  { value: 'Windows 8.1', label: 'Windows 8.1', platform: 'windows' },
  { value: 'Windows 10', label: 'Windows 10', platform: 'windows' },
  { value: 'Windows 11', label: 'Windows 11', platform: 'windows' },
  // macOS
  { value: 'macOS 12', label: 'macOS 12 Monterey', platform: 'macos' },
  { value: 'macOS 13', label: 'macOS 13 Ventura', platform: 'macos' },
  { value: 'macOS 14', label: 'macOS 14 Sonoma', platform: 'macos' },
  { value: 'macOS 15', label: 'macOS 15 Sequoia', platform: 'macos' },
  // Linux
  { value: 'Ubuntu 20.04', label: 'Ubuntu 20.04 LTS', platform: 'linux' },
  { value: 'Ubuntu 22.04', label: 'Ubuntu 22.04 LTS', platform: 'linux' },
  { value: 'Ubuntu 24.04', label: 'Ubuntu 24.04 LTS', platform: 'linux' },
  { value: 'Debian 11', label: 'Debian 11', platform: 'linux' },
  { value: 'Debian 12', label: 'Debian 12', platform: 'linux' },
  // Android
  { value: 'Android 11', label: 'Android 11', platform: 'android' },
  { value: 'Android 12', label: 'Android 12', platform: 'android' },
  { value: 'Android 13', label: 'Android 13', platform: 'android' },
  { value: 'Android 14', label: 'Android 14', platform: 'android' },
  // iOS
  { value: 'iOS 15', label: 'iOS 15', platform: 'ios' },
  { value: 'iOS 16', label: 'iOS 16', platform: 'ios' },
  { value: 'iOS 17', label: 'iOS 17', platform: 'ios' },
]

// 根据平台过滤操作系统版本选项
const filteredOsVersionOptions = computed(() => {
  const platform = model.value.platform
  return osVersionOptions.filter(opt => opt.platform === platform)
})

// 监听平台变化，自动设置默认操作系统版本并更新UA
watch(() => model.value.platform, (newPlatform) => {
  const options = osVersionOptions.filter(opt => opt.platform === newPlatform)
  if (options.length > 0) {
    // 检查当前选中的版本是否属于新平台
    const currentValid = options.some(opt => opt.value === model.value.osVersion)
    if (!currentValid) {
      // 设置为该平台的最新版本（列表最后一个）
      model.value.osVersion = options[options.length - 1].value
    }
  }
  
  // 自动设置 navigator.platform 值
  const platformMap: Record<string, string> = {
    'windows': 'Win32',
    'macos': 'MacIntel',
    'linux': 'Linux x86_64',
    'android': 'Linux armv7l',
    'ios': 'iPhone'
  }
  model.value.navigatorPlatform = platformMap[newPlatform] || 'Win32'
  
  // 更新 User-Agent
  updateUserAgent()
  console.log('🔄 平台变化，已更新:', {
    platform: model.value.platform,
    osVersion: model.value.osVersion,
    navigatorPlatform: model.value.navigatorPlatform,
    ua: model.value.userAgent?.substring(0, 60) + '...'
  })
})

// 监听操作系统版本变化，更新UA
watch(() => model.value.osVersion, () => {
  updateUserAgent()
  console.log('🔄 操作系统版本变化，UA已更新:', model.value.osVersion, model.value.userAgent?.substring(0, 60) + '...')
})

// 监听浏览器版本变化，更新UA
watch(() => model.value.browserVersion, () => {
  updateUserAgent()
  console.log('🔄 浏览器版本变化，UA已更新:', model.value.browserVersion, model.value.userAgent?.substring(0, 60) + '...')
})

const rendererOptions = [
  // NVIDIA
  { value: 'ANGLE (NVIDIA, NVIDIA GeForce GTX 1650 Direct3D11 vs_5_0 ps_5_0, D3D11)', label: 'NVIDIA GeForce GTX 1650' },
  { value: 'ANGLE (NVIDIA, NVIDIA GeForce GTX 1660 Ti Direct3D11 vs_5_0 ps_5_0, D3D11)', label: 'NVIDIA GeForce GTX 1660 Ti' },
  { value: 'ANGLE (NVIDIA, NVIDIA GeForce RTX 3060 Direct3D11 vs_5_0 ps_5_0, D3D11)', label: 'NVIDIA GeForce RTX 3060' },
  { value: 'ANGLE (NVIDIA, NVIDIA GeForce RTX 3070 Direct3D11 vs_5_0 ps_5_0, D3D11)', label: 'NVIDIA GeForce RTX 3070' },
  { value: 'ANGLE (NVIDIA, NVIDIA GeForce RTX 4060 Direct3D11 vs_5_0 ps_5_0, D3D11)', label: 'NVIDIA GeForce RTX 4060' },
  { value: 'ANGLE (NVIDIA, NVIDIA GeForce RTX 4080 Direct3D11 vs_5_0 ps_5_0, D3D11)', label: 'NVIDIA GeForce RTX 4080' },
  { value: 'ANGLE (NVIDIA, NVIDIA GeForce RTX 4090 Direct3D11 vs_5_0 ps_5_0, D3D11)', label: 'NVIDIA GeForce RTX 4090' },
  // AMD
  { value: 'ANGLE (AMD, AMD Radeon RX 6600 Direct3D11 vs_5_0 ps_5_0, D3D11)', label: 'AMD Radeon RX 6600' },
  { value: 'ANGLE (AMD, AMD Radeon RX 6700 XT Direct3D11 vs_5_0 ps_5_0, D3D11)', label: 'AMD Radeon RX 6700 XT' },
]

// 获取平台显示标签
const getPlatformLabel = (platform: string): string => {
  const labels: Record<string, string> = {
    'windows': 'Windows',
    'macos': 'macOS',
    'android': 'Android',
    'ios': 'iOS',
    'linux': 'Linux'
  }
  return labels[platform] || 'Windows'
}

// 根据平台和版本生成 User-Agent
const generateUserAgentForPlatform = (platform: string, version: string): string => {
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

// 监听平台和版本变化，自动更新 User-Agent
watch(
  () => [model.value.platform, model.value.browserVersion],
  ([platform, version]) => {
    if (platform && version) {
      model.value.userAgent = generateUserAgentForPlatform(platform, version)
      console.log(`平台/版本变化，更新 UA: ${platform} Chrome ${version}`)
    }
  },
  { immediate: false }
)

// 将生成的完整指纹暴露给父组件（用于最终提交）
watch(generatedFingerprint, (newVal) => {
  if (newVal) {
    model.value._generatedFingerprint = newVal
  }
})

</script>

<template>
  <div class="step-content">
    <!-- ========== 设备模板选择器 ========== -->
    <div class="template-section">
      <div class="section-header">
        <h3 class="section-title">
          <span class="material-symbols-outlined">devices</span>
          智能设备模板
        </h3>
        <button 
          class="btn-random" 
          @click="randomTemplate" 
          :disabled="loadingTemplates || generatingFingerprint"
          title="随机生成指纹"
        >
          <span class="material-symbols-outlined" :class="{ spinning: generatingFingerprint }">
            {{ generatingFingerprint ? 'refresh' : 'casino' }}
          </span>
          {{ generatingFingerprint ? '生成中...' : '随机生成' }}
        </button>
      </div>
      
      <!-- 加载状态 -->
      <div v-if="loadingTemplates" class="loading-state">
        <span class="material-symbols-outlined spinning">progress_activity</span>
        加载模板中...
      </div>
      
      <!-- 模板列表 -->
      <div v-else class="template-list">
        <div 
          v-for="template in templates" 
          :key="template.id"
          class="template-card"
          :class="{ active: selectedTemplateId === template.id, disabled: generatingFingerprint }"
          @click="!generatingFingerprint && selectTemplate(template.id)"
        >
          <div class="template-header">
            <h4>{{ template.description }}</h4>
            <span class="template-weight">{{ (template.weight * 100).toFixed(0) }}%</span>
          </div>
          <div class="template-specs">
            <span class="spec-item">
              <span class="material-symbols-outlined">computer</span>
              {{ template.os_name }} {{ template.os_version }}
            </span>
            <span class="spec-item">
              <span class="material-symbols-outlined">memory</span>
              {{ template.cpu_vendor }}
            </span>
            <span class="spec-item">
              <span class="material-symbols-outlined">sports_esports</span>
              {{ template.gpu_vendor }}
            </span>
          </div>
        </div>
      </div>
    </div>
    
    <!-- ========== 校验警告 ========== -->
    <div v-if="showValidation && validationResult" class="validation-section">
      <!-- 错误提示 -->
      <div v-if="validationResult.errors.length > 0" class="validation-alert error">
        <div class="alert-header">
          <span class="material-symbols-outlined">error</span>
          <strong>配置错误（{{ validationResult.errors.length }}个）</strong>
        </div>
        <ul class="alert-list">
          <li v-for="(error, index) in validationResult.errors" :key="index">
            <strong>{{ error.code }}</strong>: {{ error.message }}
          </li>
        </ul>
      </div>
      
      <!-- 警告提示 -->
      <div v-if="validationResult.warnings.length > 0" class="validation-alert warning">
        <div class="alert-header">
          <span class="material-symbols-outlined">warning</span>
          <strong>配置警告（{{ validationResult.warnings.length }}个）</strong>
        </div>
        <ul class="alert-list">
          <li v-for="(warning, index) in validationResult.warnings" :key="index">
            <strong>{{ warning.code }}</strong>: {{ warning.message }}
          </li>
        </ul>
      </div>
    </div>
    
    <div class="divider"></div>
    
    <div class="form-section">
      <!-- ========== 基础指纹 ========== -->
      <h3 class="section-title">基础指纹</h3>
      
      <!-- 平台选择 -->
      <div class="form-row">
        <label class="form-label">
          平台选择
          <span class="material-symbols-outlined help-icon" title="选择目标操作系统平台">help</span>
        </label>
        <div class="form-control">
          <div class="btn-group platform-group">
            <button 
              type="button"
              class="btn-option"
              :class="{ active: model.platform === 'windows' }"
              @click="model.platform = 'windows'"
            >
              <svg class="platform-svg-icon" fill="currentColor" viewBox="0 0 24 24">
                <path d="M0 3.449L9.75 2.1V11.719H0V3.449ZM0 12.281V20.551L9.75 19.2V12.281H0ZM10.547 1.983L24 0V11.719H10.547V1.983ZM10.547 12.281V19.017L24 20.893V12.281H10.547Z"/>
              </svg>
              Windows
            </button>
            <button 
              type="button"
              class="btn-option"
              :class="{ active: model.platform === 'macos' }"
              @click="model.platform = 'macos'"
            >
              <svg class="platform-svg-icon" fill="currentColor" viewBox="0 0 24 24">
                <path d="M18.71 19.5c-.83 1.24-1.71 2.45-3.1 2.48-1.34.03-1.77-.79-3.29-.79-1.53 0-2 .77-3.27.82-1.31.05-2.31-1.32-3.14-2.53C4.25 17 2.94 12.45 4.7 9.39c.87-1.52 2.43-2.48 4.12-2.51 1.28-.02 2.5.87 3.29.87.78 0 2.26-1.07 3.81-.91.65.03 2.47.26 3.64 1.98-.09.06-2.17 1.28-2.15 3.81.03 3.02 2.65 4.03 2.68 4.04-.03.07-.42 1.44-1.38 2.83M13 3.5c.73-.83 1.94-1.46 2.94-1.5.13 1.17-.36 2.35-1.04 3.19-.69.85-1.83 1.51-2.95 1.42-.15-1.15.41-2.35 1.05-3.11z"/>
              </svg>
              macOS
            </button>
            <button 
              type="button"
              class="btn-option"
              :class="{ active: model.platform === 'android' }"
              @click="model.platform = 'android'"
            >
              <svg class="platform-svg-icon" fill="currentColor" viewBox="0 0 24 24">
                <path d="M17.523 15.341c-.5 0-.91-.41-.91-.91s.41-.91.91-.91.91.41.91.91-.41.91-.91.91m-11.046 0c-.5 0-.91-.41-.91-.91s.41-.91.91-.91.91.41.91.91-.41.91-.91.91m11.4-6.039l1.94-3.36c.11-.19.04-.43-.15-.54-.19-.11-.43-.04-.54.15l-1.96 3.4c-1.45-.66-3.08-1.03-4.82-1.03s-3.37.37-4.82 1.03l-1.96-3.4c-.11-.19-.35-.26-.54-.15-.19.11-.26.35-.15.54l1.94 3.36C3.03 10.94 0 14.81 0 19.34h24c0-4.53-3.03-8.4-7.12-10.04"/>
              </svg>
              Android
            </button>
            <button 
              type="button"
              class="btn-option"
              :class="{ active: model.platform === 'ios' }"
              @click="model.platform = 'ios'"
            >
              <svg class="platform-svg-icon" fill="currentColor" viewBox="0 0 24 24">
                <path d="M18.71 19.5c-.83 1.24-1.71 2.45-3.1 2.48-1.34.03-1.77-.79-3.29-.79-1.53 0-2 .77-3.27.82-1.31.05-2.31-1.32-3.14-2.53C4.25 17 2.94 12.45 4.7 9.39c.87-1.52 2.43-2.48 4.12-2.51 1.28-.02 2.5.87 3.29.87.78 0 2.26-1.07 3.81-.91.65.03 2.47.26 3.64 1.98-.09.06-2.17 1.28-2.15 3.81.03 3.02 2.65 4.03 2.68 4.04-.03.07-.42 1.44-1.38 2.83M13 3.5c.73-.83 1.94-1.46 2.94-1.5.13 1.17-.36 2.35-1.04 3.19-.69.85-1.83 1.51-2.95 1.42-.15-1.15.41-2.35 1.05-3.11z"/>
              </svg>
              iOS
            </button>
            <button 
              type="button"
              class="btn-option"
              :class="{ active: model.platform === 'linux' }"
              @click="model.platform = 'linux'"
            >
              <svg class="platform-svg-icon" fill="currentColor" viewBox="0 0 24 24">
                <path d="M12.504 0c-.155 0-.311.003-.467.011-.859.048-1.694.251-2.463.666-.651.338-1.203.825-1.609 1.453-.453.697-.742 1.521-.831 2.417-.046.442-.044.894.004 1.351.042.407.108.812.2 1.211.164.683.386 1.354.665 1.997.263.607.578 1.184.944 1.716-.173.211-.338.436-.486.676-.372.6-.617 1.28-.703 1.992-.038.304-.048.612-.032.918.042.82.296 1.603.738 2.266.227.34.504.644.823.902.133.106.273.204.42.294-.009.039-.019.078-.027.117-.147.726-.056 1.443.297 2.048.18.308.423.579.719.801.176.13.365.24.564.33.082.035.166.067.251.095-.25.254-.45.557-.593.889-.227.53-.295 1.099-.198 1.63.048.263.127.52.237.766.109.242.249.472.42.681.301.369.692.666 1.151.868.385.168.804.266 1.237.29.103.006.206.008.309.007.337-.006.672-.048.996-.125.458-.109.893-.285 1.292-.52.244-.144.472-.31.681-.498.051.053.102.106.155.157.422.404.915.722 1.456.929.497.191 1.027.291 1.56.295.327.003.655-.032.972-.103.313-.068.614-.174.9-.313.373-.181.713-.42 1.008-.708.217-.21.405-.446.559-.702.039-.062.074-.127.108-.192.133.047.27.086.41.117.417.092.845.104 1.264.034.504-.086.983-.267 1.404-.532.332-.21.622-.477.859-.79.188-.249.338-.525.445-.821.106-.295.168-.607.184-.922.027-.491-.067-.983-.274-1.43-.156-.337-.375-.644-.646-.903.033-.019.066-.039.098-.06.303-.195.572-.434.796-.716.258-.322.452-.687.573-1.075.121-.389.168-.8.14-1.207-.035-.507-.178-.999-.421-1.443-.178-.327-.411-.621-.69-.87-.278-.25-.6-.45-.953-.596.14-.191.27-.392.389-.602.31-.546.531-1.14.651-1.756.081-.414.124-.837.128-1.261.004-.41-.03-.82-.102-1.223-.117-.648-.325-1.27-.62-1.844-.274-.533-.625-1.017-1.045-1.433-.416-.412-.894-.748-1.418-.994-.66-.31-1.376-.505-2.103-.574-.155-.015-.311-.023-.467-.025zm-.108 1.181c.126.001.252.007.377.02.622.06 1.217.217 1.76.475.426.202.81.473 1.135.803.324.33.59.715.784 1.135.212.46.35.955.408 1.462.049.427.051.86.005 1.288-.055.51-.181 1.006-.376 1.471-.162.388-.368.754-.616 1.089l-.057.078c-.218-.161-.449-.3-.69-.417-.486-.234-1.011-.365-1.544-.388-.145-.006-.291-.005-.436.003-.423.025-.836.123-1.22.29-.36.158-.689.375-.976.645-.025-.138-.054-.276-.087-.413-.117-.492-.299-.965-.539-1.402-.208-.377-.461-.726-.755-1.036.254-.387.455-.811.594-1.258.127-.41.2-.838.215-1.27.015-.44-.028-.882-.129-1.311-.056-.238-.129-.471-.218-.698.343-.065.693-.096 1.044-.096zm1.073 5.132c.108 0 .216.005.324.016.457.045.898.174 1.298.378.327.167.621.39.87.659.248.268.45.579.596.921.17.398.26.829.264 1.263.003.305-.038.61-.122.903-.084.294-.212.573-.382.825-.153.228-.343.431-.564.601-.22.17-.469.305-.737.401.018-.162.027-.326.027-.49 0-.405-.056-.807-.165-1.194-.148-.525-.397-1.011-.732-1.429-.336-.419-.757-.765-1.236-1.017-.192-.101-.393-.185-.601-.252.264-.32.576-.595.926-.816.35-.222.734-.392 1.14-.506.235-.067.476-.115.72-.143.138-.015.276-.022.414-.022zm-3.016.2c.094 0 .187.006.28.017.333.04.659.129.964.266.235.107.453.247.648.418.195.17.367.369.51.59.178.274.312.576.396.895.068.257.106.522.114.789.007.25-.015.5-.068.744-.05.233-.13.458-.238.67-.087.17-.193.331-.316.479-.045-.053-.092-.105-.14-.155-.297-.307-.643-.563-1.024-.758-.395-.202-.824-.336-1.266-.394-.139-.019-.279-.029-.419-.031-.265-.004-.529.025-.785.087-.255.061-.501.155-.731.28-.113.061-.221.129-.325.202-.028-.112-.051-.225-.071-.34-.071-.412-.086-.834-.045-1.25.036-.367.116-.728.24-1.073.115-.32.27-.624.462-.905.169-.246.37-.47.598-.663.247-.21.524-.383.823-.513.219-.095.448-.164.683-.205.117-.02.235-.032.352-.034.027-.001.054-.001.081-.001z"/>
              </svg>
              Linux
            </button>
          </div>
          <span class="form-hint">选择平台将影响 User-Agent 和相关指纹特征</span>
        </div>
      </div>
      
      <!-- 操作系统版本 -->
      <div class="form-row">
        <label class="form-label">
          操作系统版本
          <span class="material-symbols-outlined help-icon" title="操作系统版本信息">help</span>
        </label>
        <div class="form-control">
          <div class="select-wrapper">
            <select v-model="model.osVersion" class="select">
              <option v-for="opt in filteredOsVersionOptions" :key="opt.value" :value="opt.value">
                {{ opt.label }}
              </option>
            </select>
            <span class="material-symbols-outlined select-icon">expand_more</span>
          </div>
        </div>
      </div>
      
      <!-- 内核版本 -->
      <div class="form-row">
        <label class="form-label">
          内核版本
          <span class="material-symbols-outlined help-icon" title="选择浏览器内核版本">help</span>
        </label>
        <div class="form-control">
          <div class="select-wrapper">
            <select v-model="model.browserVersion" class="select">
              <option 
                v-for="opt in BROWSER_VERSION_OPTIONS" 
                :key="opt.value" 
                :value="opt.value"
              >
                {{ opt.label }}{{ opt.description ? ` - ${opt.description}` : '' }}
              </option>
            </select>
            <span class="material-symbols-outlined select-icon">expand_more</span>
          </div>
          <span class="form-hint">选择内核版本将影响 User-Agent 和浏览器特征信息</span>
        </div>
      </div>
      
      <!-- User-Agent -->
      <div class="form-row">
        <label class="form-label">
          User-Agent
          <span class="material-symbols-outlined help-icon" title="浏览器标识字符串">help</span>
        </label>
        <div class="form-control">
          <div class="ua-input-wrapper">
            <input 
              v-model="model.userAgent"
              type="text" 
              class="input ua-input"
              readonly
            />
            <button type="button" class="refresh-btn" @click="refreshUserAgent" title="刷新">
              <span class="material-symbols-outlined">refresh</span>
            </button>
          </div>
          <div class="ua-tags">
            <span class="ua-tag">{{ getPlatformLabel(model.platform) }}</span>
            <span class="ua-tag ua-tag-primary">Chrome {{ model.browserVersion || '146' }}</span>
          </div>
        </div>
      </div>
      
      <!-- 分辨率 -->
      <div class="form-row">
        <label class="form-label">
          分辨率
        </label>
        <div class="form-control">
          <div class="resolution-inputs">
            <input 
              v-model.number="model.screenWidth"
              type="number" 
              class="input resolution-input"
              placeholder="1920"
            />
            <span class="resolution-separator">x</span>
            <input 
              v-model.number="model.screenHeight"
              type="number" 
              class="input resolution-input"
              placeholder="1080"
            />
          </div>
        </div>
      </div>
      
      <!-- 硬件并发数 -->
      <div class="form-row">
        <label class="form-label">
          硬件并发数
        </label>
        <div class="form-control">
          <div class="select-wrapper">
            <select v-model.number="model.hardwareConcurrency" class="select">
              <option :value="2">2</option>
              <option :value="4">4</option>
              <option :value="6">6</option>
              <option :value="8">8</option>
              <option :value="12">12</option>
              <option :value="16">16</option>
            </select>
            <span class="material-symbols-outlined select-icon">expand_more</span>
          </div>
        </div>
      </div>
      
      <!-- 设备内存 -->
      <div class="form-row">
        <label class="form-label">
          设备内存
        </label>
        <div class="form-control">
          <div class="memory-wrapper">
            <div class="select-wrapper flex-1">
              <select v-model.number="model.deviceMemory" class="select">
                <option :value="2">2</option>
                <option :value="4">4</option>
                <option :value="8">8</option>
                <option :value="16">16</option>
                <option :value="32">32</option>
                <option :value="64">64</option>
              </select>
              <span class="material-symbols-outlined select-icon">expand_more</span>
            </div>
            <span class="memory-unit">GB</span>
          </div>
        </div>
      </div>
      
      <div class="divider"></div>
      
      <!-- ========== WebRTC & WebGL ========== -->
      <h3 class="section-title">WebRTC & WebGL</h3>
      
      <!-- WebRTC -->
      <div class="form-row">
        <label class="form-label">
          WebRTC
          <span class="material-symbols-outlined help-icon" title="WebRTC IP保护">help</span>
        </label>
        <div class="form-control">
          <div class="btn-group">
            <button 
              type="button"
              class="btn-option"
              :class="{ active: model.webrtc === 'real' }"
              @click="model.webrtc = 'real'"
            >真实</button>
            <button 
              type="button"
              class="btn-option"
              :class="{ active: model.webrtc === 'replace' }"
              @click="model.webrtc = 'replace'"
            >伪造</button>
            <button 
              type="button"
              class="btn-option"
              :class="{ active: model.webrtc === 'disable' }"
              @click="model.webrtc = 'disable'"
            >禁用</button>
          </div>
          
          <div class="webrtc-details" v-if="model.webrtc !== 'disable'">
            <div class="webrtc-row">
              <div class="webrtc-item">
                <div class="sub-label">公网 IP 处理</div>
                <input 
                  type="text" 
                  class="input input-disabled"
                  value="跟随代理 IP"
                  disabled
                />
              </div>
              <div class="webrtc-item">
                <div class="sub-label">局域网 IP</div>
                <input 
                  v-model="model.localIp"
                  type="text" 
                  class="input"
                  placeholder="192.168.1.15"
                />
              </div>
            </div>
          </div>
        </div>
      </div>
      
      <!-- WebGL 厂商 -->
      <div class="form-row">
        <label class="form-label">
          WebGL 厂商
        </label>
        <div class="form-control">
          <div class="select-wrapper">
            <select v-model="model.webglVendor" class="select">
              <option v-for="opt in vendorOptions" :key="opt.value" :value="opt.value">
                {{ opt.label }}
              </option>
            </select>
            <span class="material-symbols-outlined select-icon">expand_more</span>
          </div>
        </div>
      </div>
      
      <!-- WebGL 渲染器 -->
      <div class="form-row">
        <label class="form-label">
          WebGL 渲染器
        </label>
        <div class="form-control">
          <div class="select-wrapper">
            <select v-model="model.webglRenderer" class="select">
              <option v-for="opt in rendererOptions" :key="opt.value" :value="opt.value">
                {{ opt.label }}
              </option>
            </select>
            <span class="material-symbols-outlined select-icon">expand_more</span>
          </div>
        </div>
      </div>
      
      <!-- WebGL 高级选项 -->
      <div class="form-row webgl-advanced">
        <label class="form-label">
          WebGL 高级选项
          <span class="material-symbols-outlined help-icon" title="Unmasked字段和噪声配置">help</span>
        </label>
        <div class="form-control">
          <!-- Unmasked 模式选择 -->
          <div class="advanced-option">
            <div class="option-header">
              <span class="option-label">Unmasked 字段</span>
              <div class="btn-group btn-group-sm">
                <button 
                  type="button"
                  class="btn-option"
                  :class="{ active: model.webglUnmaskedMode === 'mask' }"
                  @click="model.webglUnmaskedMode = 'mask'"
                >
                  伪装（推荐）
                </button>
                <button 
                  type="button"
                  class="btn-option"
                  :class="{ active: model.webglUnmaskedMode === 'real' }"
                  @click="model.webglUnmaskedMode = 'real'"
                >
                  真实
                </button>
              </div>
            </div>
            <div v-if="model.webglUnmaskedMode === 'real'" class="unmasked-info">
              <div class="info-item">
                <span class="info-label">Vendor Unmasked:</span>
                <span class="info-value">{{ model.webglVendorUnmasked || 'NVIDIA Corporation' }}</span>
              </div>
              <div class="info-item">
                <span class="info-label">Renderer Unmasked:</span>
                <span class="info-value">{{ model.webglRendererUnmasked || 'NVIDIA GeForce GTX 1660' }}</span>
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
                <button 
                  type="button"
                  class="btn-option"
                  :class="{ active: model.webglNoise === true }"
                  @click="model.webglNoise = true"
                >
                  开启
                </button>
                <button 
                  type="button"
                  class="btn-option"
                  :class="{ active: model.webglNoise === false }"
                  @click="model.webglNoise = false"
                >
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
      
      <!-- WebGPU -->
      <div class="form-row">
        <label class="form-label">
          WebGPU
          <span class="material-symbols-outlined help-icon" title="WebGPU 配置">help</span>
        </label>
        <div class="form-control">
          <div class="btn-group">
            <button 
              type="button"
              class="btn-option"
              :class="{ active: model.webgpu === 'webgl' }"
              @click="model.webgpu = 'webgl'"
            >基于WebGL</button>
            <button 
              type="button"
              class="btn-option"
              :class="{ active: model.webgpu === 'real' }"
              @click="model.webgpu = 'real'"
            >真实</button>
            <button 
              type="button"
              class="btn-option"
              :class="{ active: model.webgpu === 'disable' }"
              @click="model.webgpu = 'disable'"
            >禁用</button>
          </div>
          <span class="form-hint">使用与 WebGL 元数据相匹配的 WebGPU，推荐选择“基于WebGL”</span>
        </div>
      </div>
      
      <div class="divider"></div>
      
      <!-- ========== Canvas & Audio ========== -->
      <h3 class="section-title">Canvas & Audio</h3>
      
      <!-- Canvas -->
      <div class="form-row">
        <label class="form-label">
          Canvas
          <span class="material-symbols-outlined help-icon" title="Canvas指纹保护">help</span>
        </label>
        <div class="form-control">
          <div class="btn-group">
            <button 
              type="button"
              class="btn-option"
              :class="{ active: model.canvas === 'noise' }"
              @click="model.canvas = 'noise'"
            >添加噪声</button>
            <button 
              type="button"
              class="btn-option"
              :class="{ active: model.canvas === 'block' }"
              @click="model.canvas = 'block'"
            >阻止</button>
            <button 
              type="button"
              class="btn-option"
              :class="{ active: model.canvas === 'off' }"
              @click="model.canvas = 'off'"
            >关闭</button>
          </div>
        </div>
      </div>
      
      <!-- AudioContext -->
      <div class="form-row">
        <label class="form-label">
          AudioContext
          <span class="material-symbols-outlined help-icon" title="音频指纹保护">help</span>
        </label>
        <div class="form-control">
          <div class="btn-group">
            <button 
              type="button"
              class="btn-option"
              :class="{ active: model.audioContext === 'noise' }"
              @click="model.audioContext = 'noise'"
            >添加噪声</button>
            <button 
              type="button"
              class="btn-option"
              :class="{ active: model.audioContext === 'block' }"
              @click="model.audioContext = 'block'"
            >阻止</button>
            <button 
              type="button"
              class="btn-option"
              :class="{ active: model.audioContext === 'off' }"
              @click="model.audioContext = 'off'"
            >关闭</button>
          </div>
        </div>
      </div>
      
      <div class="divider"></div>
      
      <!-- ========== 隐私保护 ========== -->
      <h3 class="section-title">隐私保护</h3>
      
      <!-- Do Not Track -->
      <div class="form-row">
        <label class="form-label">
          Do Not Track
        </label>
        <div class="form-control">
          <div class="btn-group">
            <button 
              type="button"
              class="btn-option"
              :class="{ active: model.doNotTrack === 'enable' }"
              @click="model.doNotTrack = 'enable'"
            >启用</button>
            <button 
              type="button"
              class="btn-option"
              :class="{ active: model.doNotTrack === 'disable' }"
              @click="model.doNotTrack = 'disable'"
            >禁用</button>
            <button 
              type="button"
              class="btn-option"
              :class="{ active: model.doNotTrack === 'unspecified' }"
              @click="model.doNotTrack = 'unspecified'"
            >未指定</button>
          </div>
        </div>
      </div>
      
      <!-- Client Rects -->
      <div class="form-row">
        <label class="form-label">
          Client Rects
          <span class="material-symbols-outlined help-icon" title="元素位置信息保护">help</span>
        </label>
        <div class="form-control">
          <label class="toggle-switch">
            <input type="checkbox" v-model="model.clientRects" />
            <span class="toggle-slider"></span>
          </label>
        </div>
      </div>
      
      <!-- 媒体设备 -->
      <div class="form-row">
        <label class="form-label">
          媒体设备
        </label>
        <div class="form-control">
          <div class="btn-group">
            <button 
              type="button"
              class="btn-option"
              :class="{ active: model.mediaDevices === 'real' }"
              @click="model.mediaDevices = 'real'"
            >真实</button>
            <button 
              type="button"
              class="btn-option"
              :class="{ active: model.mediaDevices === 'fake' }"
              @click="model.mediaDevices = 'fake'"
            >伪造</button>
            <button 
              type="button"
              class="btn-option"
              :class="{ active: model.mediaDevices === 'disable' }"
              @click="model.mediaDevices = 'disable'"
            >禁用</button>
          </div>
        </div>
      </div>
      
      <!-- 端口扫描保护 -->
      <div class="form-row">
        <label class="form-label">
          端口扫描保护
          <span class="material-symbols-outlined help-icon" title="防止端口扫描攻击">help</span>
        </label>
        <div class="form-control">
          <label class="toggle-switch">
            <input type="checkbox" v-model="model.portScanProtection" />
            <span class="toggle-slider"></span>
          </label>
        </div>
      </div>
      
      <!-- 扫描白名单 -->
      <div class="form-row">
        <label class="form-label">
          扫描白名单
          <span class="material-symbols-outlined help-icon" title="允许被网站扫描的端口">help</span>
        </label>
        <div class="form-control">
          <input 
            v-model="model.portScanWhitelist"
            type="text" 
            class="input"
            placeholder="输入整数，多个用英文逗号隔开"
          />
          <span class="form-hint">允许被网站扫描的端口，多个用英文逗号隔开</span>
        </div>
      </div>
      
      <!-- 字体指纹 -->
      <div class="form-row">
        <label class="form-label">
          字体指纹
          <span class="material-symbols-outlined help-icon" title="自定义字体列表">help</span>
        </label>
        <div class="form-control">
          <div class="btn-group">
            <button 
              type="button"
              class="btn-option"
              :class="{ active: model.fonts === 'system' }"
              @click="model.fonts = 'system'"
            >系统字体</button>
            <button 
              type="button"
              class="btn-option"
              :class="{ active: model.fonts === 'custom' }"
              @click="model.fonts = 'custom'"
            >自定义</button>
          </div>
          <input 
            v-if="model.fonts === 'custom'"
            v-model="model.customFonts"
            type="text" 
            class="input mt-8"
            placeholder="Arial, Helvetica, 微软雅黑"
          />
        </div>
      </div>
      
      <!-- Speech Voices -->
      <div class="form-row">
        <label class="form-label">
          Speech Voices
          <span class="material-symbols-outlined help-icon" title="语音合成指纹">help</span>
        </label>
        <div class="form-control">
          <div class="btn-group">
            <button 
              type="button"
              class="btn-option"
              :class="{ active: model.speechVoices === 'match' }"
              @click="model.speechVoices = 'match'"
            >匹配伪装</button>
            <button 
              type="button"
              class="btn-option"
              :class="{ active: model.speechVoices === 'disable' }"
              @click="model.speechVoices = 'disable'"
            >禁用</button>
          </div>
          <span class="form-hint">使用相匹配的值代替您真实的 Speech Voices</span>
        </div>
      </div>
      
      <!-- 忽略证书错误 -->
      <div class="form-row">
        <label class="form-label">
          忽略证书错误
          <span class="material-symbols-outlined help-icon" title="忽略HTTPS证书错误直接打开网页">help</span>
        </label>
        <div class="form-control">
          <label class="toggle-switch">
            <input type="checkbox" v-model="model.ignoreCertErrors" />
            <span class="toggle-slider"></span>
          </label>
          <span class="form-hint">忽略 HTTPS 证书错误直接打开网页</span>
        </div>
      </div>
      
      <!-- 插件指纹 -->
      <div class="form-row">
        <label class="form-label">
          插件指纹
          <span class="material-symbols-outlined help-icon" title="自定义的插件可能会影响真实插件功能">help</span>
        </label>
        <div class="form-control">
          <label class="toggle-switch">
            <input type="checkbox" v-model="model.customPlugins" />
            <span class="toggle-slider"></span>
          </label>
          <span class="form-hint">开启后，自定义的插件可能会影响真实插件功能</span>
        </div>
      </div>
      
      <!-- Cloudflare 优化 -->
      <div class="form-row">
        <label class="form-label">
          Cloudflare 优化
          <span class="material-symbols-outlined help-icon" title="Cloudflare 验证优化">help</span>
        </label>
        <div class="form-control">
          <label class="toggle-switch">
            <input type="checkbox" v-model="model.cloudflareOptimize" />
            <span class="toggle-slider"></span>
          </label>
          <span class="form-hint">完全保留反检测功能，但可能导致 Cloudflare 验证页面卡住无法跳过</span>
        </div>
      </div>
      
      <div class="divider"></div>
      
      <!-- ========== 设备信息 ========== -->
      <h3 class="section-title">设备信息</h3>
      
      <!-- 设备名称 -->
      <div class="form-row">
        <label class="form-label">
          设备名称
        </label>
        <div class="form-control">
          <div class="input-with-btn">
            <input 
              v-model="model.deviceName"
              type="text" 
              class="input"
              placeholder="DESKTOP-W0KJT6V0"
            />
            <button type="button" class="input-btn" @click="refreshDeviceName" title="随机生成">
              <span class="material-symbols-outlined">refresh</span>
            </button>
          </div>
        </div>
      </div>
      
      <!-- MAC地址 -->
      <div class="form-row">
        <label class="form-label">
          MAC地址
        </label>
        <div class="form-control">
          <div class="input-with-btn">
            <input 
              v-model="model.macAddress"
              type="text" 
              class="input"
              placeholder="64-2B-7A-4D-96-E1"
            />
            <button type="button" class="input-btn" @click="refreshMacAddress" title="随机生成">
              <span class="material-symbols-outlined">refresh</span>
            </button>
          </div>
        </div>
      </div>
      
      <div class="divider"></div>
      
      <!-- ========== 性能设置 ========== -->
      <h3 class="section-title">性能设置</h3>
      
      <!-- 硬件加速模式 -->
      <div class="form-row">
        <label class="form-label">
          硬件加速模式
        </label>
        <div class="form-control">
          <label class="toggle-switch">
            <input type="checkbox" v-model="model.hardwareAcceleration" />
            <span class="toggle-slider"></span>
          </label>
        </div>
      </div>
      
      <!-- 禁用沙箱 -->
      <div class="form-row">
        <label class="form-label">
          禁用沙箱
          <span class="material-symbols-outlined help-icon" title="不推荐启用，可能存在安全风险">help</span>
        </label>
        <div class="form-control">
          <label class="toggle-switch">
            <input type="checkbox" v-model="model.disableSandbox" />
            <span class="toggle-slider"></span>
          </label>
        </div>
      </div>
      
      <!-- 启动参数 -->
      <div class="form-row">
        <label class="form-label">
          启动参数
          <span class="material-symbols-outlined help-icon" title="Chromium启动命令行参数">help</span>
        </label>
        <div class="form-control">
          <textarea 
            v-model="model.launchArgs"
            class="textarea"
            rows="3"
            placeholder="--disable-blink-features=AutomationControlled&#10;--disable-dev-shm-usage"
          ></textarea>
          <div class="form-hint">
            <span class="material-symbols-outlined hint-icon">info</span>
            每行一个参数，如：--disable-blink-features=AutomationControlled
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped lang="scss">
.step-content {
  max-width: 700px;
  margin: 0 auto;
}

.form-section {
  display: flex;
  flex-direction: column;
  gap: 24px;
}

// 分组标题
.section-title {
  font-size: 16px;
  font-weight: 600;
  color: #1e293b;
  margin: 8px 0 0 0;
  padding-bottom: 8px;
  border-bottom: 2px solid #e2e8f0;
}

.form-row {
  display: grid;
  grid-template-columns: 120px 1fr;
  gap: 16px;
  align-items: start;
}

.form-label {
  font-size: 14px;
  font-weight: 500;
  color: var(--color-text-primary);
  display: flex;
  align-items: center;
  gap: 4px;
  padding-top: 10px;
  
  .help-icon {
    font-size: 14px;
    color: var(--color-text-tertiary);
    cursor: help;
  }
}

.form-control {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.sub-label {
  font-size: 12px;
  color: var(--color-text-tertiary);
  margin-bottom: 4px;
}

.divider {
  height: 1px;
  background: var(--color-border-default);
  margin: 8px 0;
}

.input {
  width: 100%;
  height: 42px;
  padding: 0 12px;
  border: 1px solid var(--color-border-default);
  border-radius: 8px;
  font-size: 14px;
  color: var(--color-text-primary);
  background: white;
  transition: all 0.2s;
  
  .dark & {
    background: var(--color-bg-elevated);
    border-color: var(--color-border-dark);
  }
  
  &:focus {
    outline: none;
    border-color: #2563eb;
    box-shadow: 0 0 0 3px rgba(37, 99, 235, 0.1);
  }
  
  &.input-disabled {
    background: #f1f5f9;
    color: var(--color-text-tertiary);
    cursor: not-allowed;
    
    .dark & {
      background: rgba(30, 41, 59, 0.5);
    }
  }
}

.ua-input-wrapper {
  display: flex;
  gap: 8px;
  
  .ua-input {
    flex: 1;
    font-size: 13px;
    font-family: 'Monaco', 'Menlo', monospace;
  }
  
  .refresh-btn {
    width: 42px;
    height: 42px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: white;
    border: 1px solid var(--color-border-default);
    border-radius: 8px;
    color: var(--color-text-tertiary);
    cursor: pointer;
    transition: all 0.2s;
    
    .dark & {
      background: var(--color-bg-elevated);
      border-color: var(--color-border-dark);
    }
    
    &:hover {
      color: #2563eb;
      border-color: #2563eb;
    }
    
    .material-symbols-outlined {
      font-size: 20px;
    }
  }
}

.ua-tags {
  display: flex;
  gap: 8px;
}

.ua-tag {
  display: inline-flex;
  align-items: center;
  padding: 4px 10px;
  font-size: 12px;
  font-weight: 500;
  background: #f1f5f9;
  color: var(--color-text-secondary);
  border-radius: 6px;
  
  .dark & {
    background: rgba(30, 41, 59, 0.5);
  }
  
  &.ua-tag-primary {
    background: rgba(37, 99, 235, 0.1);
    color: #2563eb;
  }
}

.select-wrapper {
  position: relative;
  
  .select-icon {
    position: absolute;
    right: 12px;
    top: 50%;
    transform: translateY(-50%);
    color: var(--color-text-tertiary);
    font-size: 20px;
    pointer-events: none;
  }
}

.select {
  width: 100%;
  height: 42px;
  padding: 0 36px 0 12px;
  border: 1px solid var(--color-border-default);
  border-radius: 8px;
  font-size: 14px;
  color: var(--color-text-primary);
  background: white;
  cursor: pointer;
  appearance: none;
  transition: all 0.2s;
  
  .dark & {
    background: var(--color-bg-elevated);
    border-color: var(--color-border-dark);
  }
  
  &:focus {
    outline: none;
    border-color: #2563eb;
    box-shadow: 0 0 0 3px rgba(37, 99, 235, 0.1);
  }
}

.form-hint {
  font-size: 12px;
  color: var(--color-text-tertiary);
  line-height: 1.5;
}

.btn-group {
  display: inline-flex;
  border-radius: 8px;
  overflow: hidden;
  box-shadow: 0 1px 2px rgba(0, 0, 0, 0.05);
  
  &.platform-group {
    flex-wrap: wrap;
    gap: 8px;
    
    .btn-option {
      border-radius: 8px;
      margin-left: 0;
      
      &:first-child {
        border-radius: 8px;
      }
      
      &:last-child {
        border-radius: 8px;
      }
    }
  }
}

.btn-option {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  padding: 8px 16px;
  font-size: 14px;
  font-weight: 500;
  border: 1px solid var(--color-border-default);
  background: white;
  color: var(--color-text-secondary);
  cursor: pointer;
  transition: all 0.2s;
  
  .dark & {
    background: var(--color-bg-elevated);
    border-color: var(--color-border-dark);
  }
  
  &:not(:first-child) {
    margin-left: -1px;
  }
  
  &:first-child {
    border-radius: 8px 0 0 8px;
  }
  
  &:last-child {
    border-radius: 0 8px 8px 0;
  }
  
  &:hover:not(.active) {
    background: var(--color-hover-bg);
  }
  
  &.active {
    background: #2563eb;
    border-color: #2563eb;
    color: white;
    position: relative;
    z-index: 1;
  }
  
  // 平台 SVG 图标
  .platform-svg-icon {
    width: 16px;
    height: 16px;
    flex-shrink: 0;
  }
}

.webgl-details,
.webrtc-details {
  margin-top: 12px;
}

.webgl-row,
.webrtc-row {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 16px;
}

.webgl-item,
.webrtc-item {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

// 分辨率输入
.resolution-inputs {
  display: flex;
  align-items: center;
  gap: 12px;
  
  .resolution-input {
    flex: 1;
  }
  
  .resolution-separator {
    font-size: 18px;
    font-weight: 500;
    color: #64748b;
  }
}

// 内存输入
.memory-wrapper {
  display: flex;
  align-items: center;
  gap: 12px;
  
  .memory-unit {
    font-size: 14px;
    font-weight: 500;
    color: #64748b;
    min-width: 30px;
  }
}

// WebGL输入包装器
.webgl-input-wrapper {
  display: flex;
  gap: 8px;
  align-items: center;
}

// 内联刷新按钮
.refresh-btn-inline {
  width: 42px;
  height: 42px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: white;
  border: 1px solid var(--color-border-default);
  border-radius: 8px;
  color: var(--color-text-tertiary);
  cursor: pointer;
  transition: all 0.2s;
  flex-shrink: 0;
  
  &:hover {
    color: #2563eb;
    border-color: #2563eb;
    background: #eff6ff;
  }
  
  .material-symbols-outlined {
    font-size: 20px;
  }
}

.flex-1 {
  flex: 1;
}

.mt-8 {
  margin-top: 8px;
}

// 输入框带按钮统一样式
.input-with-btn {
  display: flex;
  gap: 8px;
  align-items: center;
  
  .input {
    flex: 1;
  }
}

.input-btn {
  width: 42px;
  height: 42px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: white;
  border: 1px solid var(--color-border-default);
  border-radius: 8px;
  color: var(--color-text-tertiary);
  cursor: pointer;
  transition: all 0.2s;
  flex-shrink: 0;
  
  .dark & {
    background: var(--color-bg-elevated);
    border-color: var(--color-border-dark);
  }
  
  &:hover {
    color: #2563eb;
    border-color: #2563eb;
    background: #eff6ff;
    
    .dark & {
      background: rgba(37, 99, 235, 0.1);
    }
  }
  
  .material-symbols-outlined {
    font-size: 20px;
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

// 文本域
.textarea {
  width: 100%;
  padding: 12px;
  border: 1px solid var(--color-border-default);
  border-radius: 8px;
  font-size: 13px;
  font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
  color: var(--color-text-primary);
  background: #f8fafc;
  resize: vertical;
  transition: all 0.2s;
  line-height: 1.6;
  
  .dark & {
    background: rgba(30, 41, 59, 0.5);
    border-color: var(--color-border-dark);
  }
  
  &::placeholder {
    color: var(--color-text-tertiary);
  }
  
  &:focus {
    outline: none;
    border-color: #2563eb;
    box-shadow: 0 0 0 3px rgba(37, 99, 235, 0.1);
    background: white;
  }
}

.form-hint {
  display: flex;
  align-items: flex-start;
  gap: 6px;
  font-size: 12px;
  color: var(--color-text-tertiary);
  margin-top: 8px;
  
  .hint-icon {
    font-size: 14px;
    color: #2563eb;
    flex-shrink: 0;
    margin-top: 1px;
  }
}

// ===== 模板选择器样式 =====
.template-section {
  margin-bottom: 32px;
  padding: 24px;
  background: linear-gradient(135deg, #f6f8fb 0%, #ffffff 100%);
  border-radius: 16px;
  border: 1px solid #e2e8f0;
  
  .dark & {
    background: linear-gradient(135deg, rgba(30, 41, 59, 0.4) 0%, rgba(30, 41, 59, 0.2) 100%);
    border-color: var(--color-border-dark);
  }
}

.section-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 20px;
  
  .section-title {
    display: flex;
    align-items: center;
    gap: 10px;
    font-size: 18px;
    font-weight: 700;
    color: var(--color-text-primary);
    margin: 0;
    
    .material-symbols-outlined {
      font-size: 24px;
      color: #2563eb;
    }
  }
}

.btn-random {
  display: inline-flex;
  align-items: center;
  gap: 8px;
  padding: 10px 20px;
  font-size: 14px;
  font-weight: 600;
  color: white;
  background: linear-gradient(135deg, #2563eb 0%, #3b82f6 100%);
  border: none;
  border-radius: 10px;
  cursor: pointer;
  transition: all 0.3s;
  box-shadow: 0 4px 12px rgba(37, 99, 235, 0.3);
  
  &:hover:not(:disabled) {
    transform: translateY(-2px);
    box-shadow: 0 6px 20px rgba(37, 99, 235, 0.4);
  }
  
  &:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }
  
  .material-symbols-outlined {
    font-size: 20px;
  }
}

.loading-state {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 12px;
  padding: 40px;
  font-size: 15px;
  color: var(--color-text-secondary);
  
  .material-symbols-outlined {
    font-size: 24px;
    color: #2563eb;
  }
}

.template-list {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
  gap: 16px;
}

.template-card {
  padding: 18px;
  background: white;
  border: 2px solid #e2e8f0;
  border-radius: 12px;
  cursor: pointer;
  transition: all 0.3s;
  
  .dark & {
    background: var(--color-bg-elevated);
    border-color: var(--color-border-dark);
  }
  
  &:hover:not(.disabled) {
    border-color: #2563eb;
    transform: translateY(-3px);
    box-shadow: 0 8px 20px rgba(37, 99, 235, 0.15);
  }
  
  &.active {
    border-color: #2563eb;
    background: linear-gradient(135deg, rgba(37, 99, 235, 0.08) 0%, rgba(59, 130, 246, 0.05) 100%);
    box-shadow: 0 8px 20px rgba(37, 99, 235, 0.2);
    
    .template-weight {
      background: #2563eb;
      color: white;
      font-weight: 700;
    }
  }
  
  &.disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
  
  .template-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 14px;
    
    h4 {
      font-size: 15px;
      font-weight: 600;
      color: var(--color-text-primary);
      margin: 0;
    }
  }
  
  .template-weight {
    display: inline-block;
    padding: 4px 10px;
    font-size: 11px;
    font-weight: 600;
    background: #f1f5f9;
    color: var(--color-text-tertiary);
    border-radius: 6px;
    transition: all 0.3s;
  }
  
  .template-specs {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }
  
  .spec-item {
    display: flex;
    align-items: center;
    gap: 8px;
    font-size: 13px;
    color: var(--color-text-secondary);
    
    .material-symbols-outlined {
      font-size: 16px;
      color: #2563eb;
    }
  }
}

// ===== 校验警告样式 =====
.validation-section {
  margin-bottom: 24px;
}

.validation-alert {
  padding: 16px 20px;
  border-radius: 10px;
  margin-bottom: 12px;
  
  &.error {
    background: #fef2f2;
    border: 1px solid #fca5a5;
    
    .dark & {
      background: rgba(220, 38, 38, 0.15);
      border-color: rgba(220, 38, 38, 0.3);
    }
    
    .alert-header {
      color: #dc2626;
      
      .material-symbols-outlined {
        color: #dc2626;
      }
    }
    
    .alert-list {
      li {
        color: #991b1b;
        
        .dark & {
          color: #fca5a5;
        }
      }
    }
  }
  
  &.warning {
    background: #fffbeb;
    border: 1px solid #fcd34d;
    
    .dark & {
      background: rgba(245, 158, 11, 0.15);
      border-color: rgba(245, 158, 11, 0.3);
    }
    
    .alert-header {
      color: #d97706;
      
      .material-symbols-outlined {
        color: #f59e0b;
      }
    }
    
    .alert-list {
      li {
        color: #92400e;
        
        .dark & {
          color: #fcd34d;
        }
      }
    }
  }
  
  .alert-header {
    display: flex;
    align-items: center;
    gap: 10px;
    font-size: 14px;
    margin-bottom: 12px;
    
    .material-symbols-outlined {
      font-size: 20px;
    }
  }
  
  .alert-list {
    margin: 0;
    padding-left: 30px;
    font-size: 13px;
    line-height: 1.6;
    
    li {
      margin-bottom: 6px;
      
      strong {
        font-weight: 600;
        font-family: 'Monaco', 'Menlo', monospace;
        font-size: 12px;
      }
    }
  }
}

// 旋转动画
@keyframes spin {
  from {
    transform: rotate(0deg);
  }
  to {
    transform: rotate(360deg);
  }
}

.spinning {
  animation: spin 1s linear infinite;
}

// ===== WebGL 高级选项样式 =====
.webgl-advanced {
  .form-control {
    gap: 16px;
  }
}

.advanced-option {
  padding: 16px;
  background: #f8fafc;
  border: 1px solid #e2e8f0;
  border-radius: 10px;
  
  .dark & {
    background: rgba(30, 41, 59, 0.3);
    border-color: var(--color-border-dark);
  }
  
  .option-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 12px;
  }
  
  .option-label {
    font-size: 14px;
    font-weight: 600;
    color: var(--color-text-primary);
  }
  
  .btn-group-sm {
    .btn-option {
      padding: 6px 14px;
      font-size: 13px;
    }
  }
}

.unmasked-info {
  margin-top: 12px;
  padding: 12px;
  background: white;
  border: 1px solid #e2e8f0;
  border-radius: 8px;
  
  .dark & {
    background: rgba(30, 41, 59, 0.5);
    border-color: var(--color-border-dark);
  }
  
  .info-item {
    display: flex;
    align-items: center;
    padding: 6px 0;
    font-size: 13px;
    
    &:not(:last-child) {
      border-bottom: 1px solid #f1f5f9;
      
      .dark & {
        border-color: var(--color-border-dark);
      }
    }
    
    .info-label {
      flex-shrink: 0;
      width: 150px;
      font-weight: 500;
      color: var(--color-text-secondary);
    }
    
    .info-value {
      flex: 1;
      color: var(--color-text-primary);
      font-family: 'Consolas', 'Monaco', monospace;
      font-size: 12px;
      word-break: break-all;
    }
  }
}

.hint-text {
  margin-top: 8px;
  padding: 8px 12px;
  font-size: 13px;
  line-height: 1.5;
  color: var(--color-text-tertiary);
  background: rgba(37, 99, 235, 0.05);
  border-left: 3px solid #2563eb;
  border-radius: 4px;
  
  .dark & {
    background: rgba(37, 99, 235, 0.1);
  }
}
</style>
