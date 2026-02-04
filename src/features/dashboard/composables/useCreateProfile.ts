/**
 * @description 创建窗口抽屉业务逻辑 Composable
 * @author DeepAgent
 */

import { ref, reactive, computed, onMounted } from 'vue'
import { Message } from '@/utils/message'
import { useGroupStore } from '@/stores/groupStore'
import { useProfileStore } from '@/stores/profile.store'
import { filterFingerprintWhitelist, FINGERPRINT_SCHEMA_VERSION } from '@/config/fingerprint.config'
import { setProfileTags } from '@/api/tagApi'

// 表单数据类型
export interface CreateProfileFormData {
  // 第一步：窗口信息
  name: string
  group: string
  tagIds: string[]  // 标签 ID 列表
  cookie: string
  remark: string
  startupPage: 'blank' | 'url'
  startupUrl: string
  
  // 第二步：基础设置
  language: string
  interfaceLanguage: string
  timezone: string
  geolocationPrompt: string
  geolocation: string
  geolocationLatitude: number | null
  geolocationLongitude: number | null
  sound: boolean
  images: boolean
  video: boolean
  windowSize: string
  customWidth: number
  customHeight: number
  
  // 第三步：高级指纹设置
  resolution: string
  hardwareConcurrency: number
  deviceMemory: number
  webrtc: string
  webrtcPublicIp: string
  webrtcLocalIp: string
  webglVendor: string
  webglRenderer: string
  webgpu: string
  canvas: string
  audioContext: string
  doNotTrack: boolean | null
  clientRects: string
  mediaDevices: string
  portScanProtection: boolean
  scanWhitelist: string
  deviceName: string
  macAddress: string
  hardwareAcceleration: boolean
  disableSandbox: boolean
  launchArgs: string
  
  // 字体配置
  fontsMode: 'subset' | 'real' | 'custom' | 'random'
  fontsList: string[]  // 选中的字体列表
  customFonts: string  // 自定义字体（逗号分隔）
  
  // Variations 配置（内核实验分组）
  variationsEnabled: boolean
  variationsSeedId: string  // 自动基于 Profile ID 生成
  
  // 第四步：代理设置
  checkIpOnStart: boolean
  proxyMode: string
  proxyType: string
  proxyHost: string
  proxyPort: string
  proxyUsername: string
  proxyPassword: string
  useSystemProxy: boolean
  directWhitelist: string
  followGlobal: boolean
  enableDirectWhitelist: boolean
  enableProxyUdp: boolean  // 启用 UDP 支持（用于 WebRTC）
  
  // 第五步：偏好设置
  windowTitle: string
  customBookmarks: string
  syncBookmarks: boolean
  syncHistory: boolean
  syncTabs: boolean
  syncCookies: boolean
  syncExtensions: boolean
  syncPasswords: boolean
  syncIndexedDB: boolean
  syncLocalStorage: boolean
  syncSessionStorage: boolean
  clearCacheOnStart: boolean
  clearCookiesOnStart: boolean
  clearLocalStorageOnStart: boolean
  randomFingerprintOnStart: boolean
  showPasswordSavePrompt: boolean
  stopOnNetworkError: boolean
  stopOnIpChange: boolean
  stopOnCountryChange: boolean
  openWorkbench: boolean
  ipChangeAlert: boolean
  enableGoogleLogin: boolean
  urlBlacklist: string
  urlWhitelist: string
}

// 选项配置
export const LANGUAGE_OPTIONS = [
  { label: '自动匹配', value: 'auto' },
  { label: '中文(简体)', value: 'zh-CN' },
  { label: '中文(繁体)', value: 'zh-TW' },
  { label: 'English', value: 'en-US' },
  { label: '日本語', value: 'ja-JP' },
  { label: '한국어', value: 'ko-KR' }
]

export const TIMEZONE_OPTIONS = [
  { label: '自动匹配', value: 'auto' },
  { label: 'UTC+8 北京', value: 'Asia/Shanghai' },
  { label: 'UTC+9 东京', value: 'Asia/Tokyo' },
  { label: 'UTC+0 伦敦', value: 'Europe/London' },
  { label: 'UTC-5 纽约', value: 'America/New_York' }
]

export const GEOLOCATION_PROMPT_OPTIONS = [
  { label: '询问', value: 'ask' },
  { label: '允许', value: 'allow' },
  { label: '拒绝', value: 'deny' }
]

export const WINDOW_SIZE_OPTIONS = [
  { label: '最大化', value: 'maximize' },
  { label: '自定义', value: 'custom' },
  { label: '跟随设备', value: 'device' }
]

export const PROXY_TYPE_OPTIONS = [
  { label: '直连模式', value: 'direct' },
  { label: 'HTTP', value: 'http' },
  { label: 'HTTPS', value: 'https' },
  { label: 'SOCKS5', value: 'socks5' }
]

export const PROXY_MODE_OPTIONS = [
  { label: '自定义填写', value: 'custom' },
  { label: '从分组选取', value: 'group' }
]

export const STEP_TITLES = [
  '窗口信息',
  '基础设置',
  '高级指纹设置',
  '代理设置',
  '偏好设置'
]

export function useCreateProfile(emit: {
  (e: 'update:visible', value: boolean): void
  (e: 'created', profile: unknown): void
}) {
  // ==================== Stores ====================
  const groupStore = useGroupStore()
  const profileStore = useProfileStore()

  // ==================== 状态 ====================
  const currentStep = ref(1)
  const totalSteps = 5

  // 表单数据
  const formData = reactive<CreateProfileFormData>({
    // 第一步：窗口信息
    name: '',
    group: 'default',
    tagIds: [],  // 标签 ID 列表
    cookie: '',
    remark: '',
    startupPage: 'blank',
    startupUrl: '',
    
    // 第二步：基础设置
    language: 'auto',
    interfaceLanguage: 'zh-CN',
    timezone: 'auto',
    geolocationPrompt: 'ask',
    geolocation: 'auto',
    geolocationLatitude: null,
    geolocationLongitude: null,
    sound: true,
    images: true,
    video: true,
    windowSize: 'maximize',
    customWidth: 1920,
    customHeight: 1080,
    
    // 第三步：高级指纹设置
    resolution: '1920x1080',
    hardwareConcurrency: 16,
    deviceMemory: 8,
    webrtc: 'fake',
    webrtcPublicIp: '',
    webrtcLocalIp: '',
    webglVendor: 'Intel Inc.',
    webglRenderer: 'Intel Iris OpenGL Engine',
    webgpu: 'webgl',
    canvas: 'noise',
    audioContext: 'noise',
    doNotTrack: null,
    clientRects: 'fake',
    mediaDevices: 'fake',
    portScanProtection: false,
    scanWhitelist: '',
    deviceName: 'DESKTOP-W0KJT6V0',
    macAddress: '64-2B-7A-4D-96-E1',
    hardwareAcceleration: false,
    disableSandbox: false,
    launchArgs: '',
    
    // 字体配置
    fontsMode: 'subset',
    fontsList: [
      'Arial', 'Arial Black', 'Calibri', 'Cambria', 'Courier New',
      'Georgia', 'Helvetica', 'Impact', 'Microsoft YaHei', 'SimSun',
      'SimHei', 'Tahoma', 'Times New Roman', 'Trebuchet MS', 'Verdana'
    ],
    customFonts: '',
    
    // Variations 配置
    variationsEnabled: true,
    variationsSeedId: '',  // 自动生成
    
    // 第四步：代理设置
    checkIpOnStart: false,
    proxyMode: 'custom',
    proxyType: 'direct',
    proxyHost: '',
    proxyPort: '',
    proxyUsername: '',
    proxyPassword: '',
    useSystemProxy: false,
    directWhitelist: '',
    followGlobal: true,
    enableDirectWhitelist: false,
    enableProxyUdp: true,  // 默认启用 UDP 支持（用于 WebRTC）
    
    // 第五步：偏好设置
    windowTitle: '',
    customBookmarks: '',
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
    randomFingerprintOnStart: false,
    showPasswordSavePrompt: true,
    stopOnNetworkError: false,
    stopOnIpChange: false,
    stopOnCountryChange: false,
    openWorkbench: false,
    ipChangeAlert: false,
    enableGoogleLogin: true,
    urlBlacklist: '',
    urlWhitelist: ''
  })

  // ==================== 计算属性 ====================
  const groupOptions = computed(() => groupStore.groupOptions)

  const canProceed = computed(() => {
    if (currentStep.value === 1) {
      return formData.name.trim().length > 0
    }
    return true
  })

  // ==================== 方法 ====================
  const handleClose = () => {
    emit('update:visible', false)
  }

  const handlePrev = () => {
    if (currentStep.value > 1) {
      currentStep.value--
    }
  }

  const handleNext = () => {
    if (currentStep.value === 1 && !formData.name) {
      Message.warning('请输入窗口名称')
      return
    }
    
    if (currentStep.value < totalSteps) {
      currentStep.value++
    } else {
      handleSubmit()
    }
  }

  const handleSubmit = async () => {
    try {
      // 构建指纹配置（仅包含白名单字段）
      const fingerprintData = {
        platform: 'windows',
        browser: 'chrome',
        hardwareConcurrency: formData.hardwareConcurrency,
        deviceMemory: formData.deviceMemory,
        screenResolution: `${formData.customWidth}x${formData.customHeight}`,
        timezone: formData.timezone,
        language: formData.language,
        userAgent: 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36',
        canvasNoise: formData.canvas === 'noise',
        webglNoise: formData.canvas === 'noise',
        audioNoise: formData.audioContext === 'noise',
        webrtc: formData.webrtc,
        webrtcPublicIp: formData.webrtcPublicIp || undefined,
        webrtcLocalIp: formData.webrtcLocalIp || undefined,
        webglVendor: formData.webglVendor,
        webglRenderer: formData.webglRenderer,
        schemaVersion: FINGERPRINT_SCHEMA_VERSION,
        
        // 字体配置
        fontsMode: formData.fontsMode,
        fontsList: formData.fontsMode === 'custom' || formData.fontsMode === 'random' 
          ? formData.fontsList 
          : undefined,
        customFonts: formData.customFonts || undefined,
        
        // Variations 配置
        variationsEnabled: formData.variationsEnabled,
        variationsSeedId: formData.variationsSeedId || undefined,
        
        // 地理位置配置
        geolocationMode: formData.geolocation,  // 'auto' | 'ip' | 'custom'
        geolocationLatitude: (formData.geolocation === 'custom' || formData.geolocation === 'ip') 
          ? formData.geolocationLatitude : undefined,
        geolocationLongitude: (formData.geolocation === 'custom' || formData.geolocation === 'ip') 
          ? formData.geolocationLongitude : undefined,
        geolocationPrompt: formData.geolocationPrompt,
        
        // 其他配置
        deviceName: formData.deviceName,
        macAddress: formData.macAddress,
        hardwareAcceleration: formData.hardwareAcceleration,
        disableSandbox: formData.disableSandbox,
        launchArgs: formData.launchArgs || undefined,
      }
      
      // 过滤指纹字段（安全保障）
      const filteredFingerprint = filterFingerprintWhitelist(fingerprintData)
      
      // 创建环境
      const newProfile = await profileStore.createProfile({
        name: formData.name,
        group: formData.group,
        remark: formData.remark,
        fingerprint: filteredFingerprint as Record<string, unknown>,
      })
      
      if (newProfile) {
        // 如果选择了标签，设置标签关联
        if (formData.tagIds && formData.tagIds.length > 0) {
          try {
            await setProfileTags(newProfile.id, formData.tagIds)
          } catch (error) {
            console.error('Failed to set profile tags:', error)
            // 标签设置失败不影响主流程
          }
        }
        
        emit('created', newProfile)
        emit('update:visible', false)
        Message.success('创建成功')
        
        // 重置
        currentStep.value = 1
      }
    } catch (error) {
      Message.error('创建失败')
      console.error('Failed to create profile:', error)
    }
  }

  // 生成随机名称
  const generateRandomName = () => {
    const chars = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789'
    let result = 'Window_'
    for (let i = 0; i < 8; i++) {
      result += chars.charAt(Math.floor(Math.random() * chars.length))
    }
    formData.name = result
  }

  // 生成随机MAC地址
  const generateRandomMac = () => {
    const hex = '0123456789ABCDEF'
    let mac = ''
    for (let i = 0; i < 6; i++) {
      if (i > 0) mac += '-'
      mac += hex.charAt(Math.floor(Math.random() * 16))
      mac += hex.charAt(Math.floor(Math.random() * 16))
    }
    formData.macAddress = mac
  }

  // 重置表单
  const resetForm = () => {
    currentStep.value = 1
    formData.name = ''
    formData.group = 'default'
    formData.cookie = ''
    formData.remark = ''
    // ... 可以根据需要重置更多字段
  }

  // ==================== 生命周期 ====================
  onMounted(() => {
    groupStore.initGroups()
  })

  return {
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
    handleSubmit,
    generateRandomName,
    generateRandomMac,
    resetForm,
    
    // 常量（供模板使用）
    stepTitles: STEP_TITLES,
    languageOptions: LANGUAGE_OPTIONS,
    timezoneOptions: TIMEZONE_OPTIONS,
    geolocationPromptOptions: GEOLOCATION_PROMPT_OPTIONS,
    windowSizeOptions: WINDOW_SIZE_OPTIONS,
    proxyTypeOptions: PROXY_TYPE_OPTIONS,
    proxyModeOptions: PROXY_MODE_OPTIONS
  }
}
