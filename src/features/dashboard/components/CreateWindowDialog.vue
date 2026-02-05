<script setup lang="ts">
/**
 * @description 新建/编辑窗口对话框 - 5步骤向导
 */
import { ref, computed, watch } from 'vue'
import StepIndicator from './create-window/StepIndicator.vue'
import Step1WindowInfo from './create-window/Step1WindowInfo.vue'
import Step2BasicSettings from './create-window/Step2BasicSettings.vue'
import Step3FingerprintSettings from './create-window/Step3FingerprintSettings.vue'
import Step4ProxySettings from './create-window/Step4ProxySettings.vue'
import Step5Preferences from './create-window/Step5Preferences.vue'
import { generateRandomFingerprint, type PlatformType, type BrowserVersionType } from '@/api/fingerprintApi'
import { useProfileStore } from '@/stores/profile.store'

interface Props {
  visible: boolean
  editData?: any  // 编辑模式时传入的窗口数据
}

const props = defineProps<Props>()
const emit = defineEmits(['close', 'submit'])

// 获取 Profile Store
const profileStore = useProfileStore()

// 是否为编辑模式
const isEditMode = computed(() => !!props.editData)

// 当前步骤 (1-5)
const currentStep = ref(1)

// 指纹生成状态
const generatingFingerprint = ref(false)
const fingerprintError = ref('')

// 防止重复提交状态
const isSubmitting = ref(false)

// 表单数据
const formData = ref({
  // Step 1 - 窗口信息
  name: '',
  groupId: 'default',
  remark: '',
  cookies: '',

  // Step 2 - 基础设置
  language: 'ip' as 'ip' | 'custom',
  languageValue: 'en-US',  // 实际语言值
  uiLanguage: 'ip' as 'ip' | 'custom',
  timezone: 'ip' as 'ip' | 'custom',
  timezoneId: 'Asia/Shanghai',  // 实际时区ID
  geolocationPrompt: 'ask' as 'ask' | 'allow' | 'deny',
  geolocation: 'ip' as 'ip' | 'custom',
  sound: true,
  images: true,
  video: true,
  windowSize: 'custom' as 'custom' | 'fullscreen',
  width: 1200,
  height: 800,

  // Step 3 - 高级指纹设置
  platform: 'windows' as 'windows' | 'macos' | 'android' | 'ios' | 'linux',  // 平台选择
  navigatorPlatform: 'Win32',  // navigator.platform 值
  osVersion: 'Windows 10',  // 操作系统版本
  browserVersion: '146' as '146' | '145' | '144' | '143',  // 内核版本
  userAgent: 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/146.0.0.0 Safari/537.36',
  screenWidth: 1920,
  screenHeight: 1080,
  hardwareConcurrency: 4,
  deviceMemory: 8,
  canvas: 'noise',
  webgl: 'noise',
  webglVendor: 'Google Inc. (NVIDIA)',
  webglRenderer: 'ANGLE (NVIDIA, NVIDIA GeForce GTX 1080)',
  webgpu: 'webgl' as 'webgl' | 'real' | 'disable',  // WebGPU 配置
  audioContext: 'noise' as 'noise' | 'off',
  webrtc: 'replace',
  publicIp: '',
  localIp: '192.168.1.15',

  // 新增指纹功能
  ignoreCertErrors: false,  // 忽略HTTPS证书错误
  fonts: 'system' as 'system' | 'custom',  // 字体指纹
  customFonts: '',  // 自定义字体列表
  speechVoices: 'match' as 'match' | 'disable',  // Speech Voices
  portScanWhitelist: '',  // 端口扫描白名单
  customPlugins: false,  // 自定义插件指纹
  cloudflareOptimize: true,  // Cloudflare验证优化

  // 隐私保护
  doNotTrack: 'unspecified' as 'enable' | 'disable' | 'unspecified',
  clientRects: true,
  mediaDevices: 'real' as 'real' | 'fake' | 'disable',
  portScanProtection: true,

  // 设备信息
  deviceName: 'DESKTOP-W0KJT6V0',
  macAddress: '64-2B-7A-4D-96-E1',

  // 性能设置
  hardwareAcceleration: true,
  disableSandbox: false,
  launchArgs: '',

  // Step 4 - 代理设置
  proxyProtocol: 'socks5' as 'socks5' | 'http' | 'https' | 'ssh',
  proxyHost: '',
  proxyPort: '',
  proxyUsername: '',
  proxyPassword: '',
  enableUdp: true,

  // Step 5 - 偏好设置（完整版）
  extensions: [] as string[],

  // 退出自动清理
  clearHistoryOnExit: false,
  clearCookiesOnExit: false,
  clearCacheOnExit: false,

  // 启动前清理
  clearCacheOnStart: false,
  clearCookiesOnStart: false,
  clearLocalStorageOnStart: false,

  // 同步选项
  syncBookmarks: false,
  syncHistory: false,
  syncTabs: false,
  syncCookies: false,
  syncExtensions: false,
  syncPasswords: false,
  syncIndexedDB: false,
  syncLocalStorage: false,
  syncSessionStorage: false,

  // 云端同步
  cloudSync: false,
  cloudSyncExtensions: false,
  cloudSyncBookmarks: false,

  // 其他选项
  randomFingerprintOnStart: false,
  showPasswordSavePrompt: false,
  stopOnNetworkError: false,
  stopOnIpChange: false,
  stopOnCountryChange: false,
  openWorkbench: false,
  ipChangeNotification: false,
  enableGoogleLogin: false,

  // 网址访问控制
  urlBlacklist: '',
  urlWhitelist: '',

  // 兼容旧字段（映射到新字段）
  startupPage: 'blank' as 'blank' | 'url',
  startupUrl: 'https://www.google.com',
  clearHistory: false,
  clearCookies: true,
  clearCache: false,
})

// 重置表单数据
const resetFormData = () => {
  formData.value = {
    name: '',
    groupId: 'default',
    remark: '',
    cookies: '',
    language: 'ip',
    languageValue: 'en-US',
    uiLanguage: 'ip',
    timezone: 'ip',
    timezoneId: 'Asia/Shanghai',
    geolocationPrompt: 'ask',
    geolocation: 'ip',
    sound: true,
    images: true,
    video: true,
    windowSize: 'custom',
    width: 1200,
    height: 800,
    platform: 'windows',
    navigatorPlatform: 'Win32',
    osVersion: 'Windows 10',
    browserVersion: '146',
    userAgent: 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/146.0.0.0 Safari/537.36',
    screenWidth: 1920,
    screenHeight: 1080,
    hardwareConcurrency: 4,
    deviceMemory: 8,
    canvas: 'noise',
    webgl: 'noise',
    webglVendor: 'Google Inc. (NVIDIA)',
    webglRenderer: 'ANGLE (NVIDIA, NVIDIA GeForce GTX 1080)',
    webgpu: 'webgl',
    audioContext: 'noise',
    webrtc: 'replace',
    publicIp: '',
    localIp: '192.168.1.15',
    ignoreCertErrors: false,
    fonts: 'system',
    customFonts: '',
    speechVoices: 'match',
    portScanWhitelist: '',
    customPlugins: false,
    cloudflareOptimize: true,
    doNotTrack: 'unspecified',
    clientRects: true,
    mediaDevices: 'real',
    portScanProtection: true,
    // 设备信息
    deviceName: 'DESKTOP-' + Math.random().toString(36).substring(2, 10).toUpperCase(),
    macAddress: Array.from({ length: 6 }, () => Math.floor(Math.random() * 256).toString(16).padStart(2, '0').toUpperCase()).join('-'),
    hardwareAcceleration: true,
    disableSandbox: false,
    launchArgs: '',
    proxyProtocol: 'socks5',
    proxyHost: '',
    proxyPort: '',
    proxyUsername: '',
    proxyPassword: '',
    enableUdp: true,
    // Step 5 - 偏好设置（完整版）
    extensions: [],

    // 退出自动清理
    clearHistoryOnExit: false,
    clearCookiesOnExit: false,
    clearCacheOnExit: false,

    // 启动前清理
    clearCacheOnStart: false,
    clearCookiesOnStart: false,
    clearLocalStorageOnStart: false,

    // 同步选项
    syncBookmarks: false,
    syncHistory: false,
    syncTabs: false,
    syncCookies: false,
    syncExtensions: false,
    syncPasswords: false,
    syncIndexedDB: false,
    syncLocalStorage: false,
    syncSessionStorage: false,

    // 云端同步
    cloudSync: false,
    cloudSyncExtensions: false,
    cloudSyncBookmarks: false,

    // 其他选项
    randomFingerprintOnStart: false,
    showPasswordSavePrompt: false,
    stopOnNetworkError: false,
    stopOnIpChange: false,
    stopOnCountryChange: false,
    openWorkbench: false,
    ipChangeNotification: false,
    enableGoogleLogin: false,

    // 网址访问控制
    urlBlacklist: '',
    urlWhitelist: '',

    // 兼容旧字段
    startupPage: 'blank',
    startupUrl: 'https://www.google.com',
    clearHistory: false,
    clearCookies: true,
    clearCache: false,
  }
}

// 初始化全新的随机指纹（新建模式用）
const initNewFingerprint = async () => {
  try {
    generatingFingerprint.value = true
    const profileId = crypto.randomUUID()
    const platform = formData.value.platform as PlatformType
    const browserVersion = formData.value.browserVersion as BrowserVersionType

    const fingerprint = await generateRandomFingerprint(profileId, platform, browserVersion)
    console.log('🎲 已生成初始化随机指纹:', fingerprint)

    // 填充到表单字段（模仿 Step3 的应用逻辑）
    if (fingerprint.navigator) {
      formData.value.userAgent = fingerprint.navigator.user_agent
      formData.value.hardwareConcurrency = fingerprint.navigator.hardware_concurrency
      formData.value.deviceMemory = fingerprint.navigator.device_memory
    }
    if (fingerprint.screen) {
      formData.value.screenWidth = fingerprint.screen.width
      formData.value.screenHeight = fingerprint.screen.height
    }
    if (fingerprint.webgl) {
      formData.value.webglVendor = fingerprint.webgl.vendor
      formData.value.webglRenderer = fingerprint.webgl.renderer
    }
  } catch (error) {
    console.error('初始化随机指纹失败:', error)
  } finally {
    generatingFingerprint.value = false
  }
}

// 监听编辑数据变化，回显到表单
watch(() => props.editData, (newData) => {
  if (newData) {
    console.log('📝 编辑模式，加载数据:', newData)

    // 基本信息
    formData.value.name = newData.name || ''
    formData.value.groupId = newData.group || 'default'
    formData.value.remark = newData.remark || ''

    // 解析指纹数据
    const fp = newData.fingerprint || {}

    // 平台和系统
    formData.value.platform = fp.platform || 'windows'
    formData.value.navigatorPlatform = fp.navigatorPlatform || 'Win32'
    formData.value.osVersion = fp.osVersion || 'Windows 10'
    formData.value.browserVersion = fp.browserVersion || '146'
    formData.value.userAgent = fp.userAgent || ''

    // 硬件配置
    formData.value.screenWidth = fp.screenWidth || 1920
    formData.value.screenHeight = fp.screenHeight || 1080
    formData.value.hardwareConcurrency = fp.hardwareConcurrency || 4
    formData.value.deviceMemory = fp.deviceMemory || 8

    // WebGL
    formData.value.webglVendor = fp.webglVendor || 'Google Inc. (NVIDIA)'
    formData.value.webglRenderer = fp.webglRenderer || ''
    formData.value.webgpu = fp.webgpu ? 'real' : 'webgl'
    formData.value.canvas = fp.canvas || (fp.canvasNoise ? 'noise' : 'off')
    formData.value.webgl = fp.webglNoise ? 'noise' : 'off'
    formData.value.audioContext = fp.audioContext || (fp.audioNoise ? 'noise' : 'off')

    // WebRTC
    formData.value.webrtc = fp.webrtc || 'replace'
    formData.value.publicIp = fp.webrtcPublicIp || ''
    formData.value.localIp = fp.webrtcLocalIp || '192.168.1.15'

    // 隐私保护
    formData.value.doNotTrack = fp.doNotTrack || 'unspecified'
    formData.value.clientRects = fp.clientRects !== false
    formData.value.mediaDevices = fp.mediaDevices || 'real'
    formData.value.portScanProtection = fp.portScanProtection !== false
    formData.value.portScanWhitelist = fp.portScanWhitelist || ''
    formData.value.fonts = fp.fonts?.length ? 'custom' : 'system'
    formData.value.customFonts = fp.customFonts || ''
    formData.value.speechVoices = fp.speechVoices || 'match'
    formData.value.ignoreCertErrors = fp.ignoreCertErrors || false
    formData.value.customPlugins = fp.customPlugins || false
    formData.value.cloudflareOptimize = fp.cloudflareOptimize !== false

    // 设备信息
    formData.value.deviceName = fp.deviceName || 'DESKTOP-' + Math.random().toString(36).substring(2, 10).toUpperCase()
    formData.value.macAddress = fp.macAddress || ''

    // 性能设置
    formData.value.hardwareAcceleration = fp.hardwareAcceleration !== false
    formData.value.disableSandbox = fp.disableSandbox || false
    formData.value.launchArgs = fp.launchArgs || ''

    // 代理配置
    const proxy = newData.proxy
    if (proxy) {
      formData.value.proxyProtocol = proxy.type?.toLowerCase() || 'socks5'
      formData.value.proxyHost = proxy.host || ''
      formData.value.proxyPort = proxy.port?.toString() || ''
      formData.value.proxyUsername = proxy.username || ''
      formData.value.proxyPassword = proxy.password || ''
    }

    // 偏好设置
    const prefs = newData.preferences || {}

    // 扩展
    formData.value.extensions = prefs.extensions || []

    // 退出自动清理
    formData.value.clearHistoryOnExit = prefs.clearHistoryOnExit || false
    formData.value.clearCookiesOnExit = prefs.clearCookiesOnExit || false
    formData.value.clearCacheOnExit = prefs.clearCacheOnExit || false

    // 启动前清理
    formData.value.clearCacheOnStart = prefs.clearCacheOnStart || false
    formData.value.clearCookiesOnStart = prefs.clearCookiesOnStart || false
    formData.value.clearLocalStorageOnStart = prefs.clearLocalStorageOnStart || false

    // 同步选项
    formData.value.syncBookmarks = prefs.syncBookmarks || false
    formData.value.syncHistory = prefs.syncHistory || false
    formData.value.syncTabs = prefs.syncTabs || false
    formData.value.syncCookies = prefs.syncCookies || false
    formData.value.syncExtensions = prefs.syncExtensions || false
    formData.value.syncPasswords = prefs.syncPasswords || false
    formData.value.syncIndexedDB = prefs.syncIndexedDB || false
    formData.value.syncLocalStorage = prefs.syncLocalStorage || false
    formData.value.syncSessionStorage = prefs.syncSessionStorage || false

    // 云端同步
    formData.value.cloudSync = prefs.cloudSync || false
    formData.value.cloudSyncExtensions = prefs.cloudSyncExtensions || false
    formData.value.cloudSyncBookmarks = prefs.cloudSyncBookmarks || false

    // 其他选项
    formData.value.randomFingerprintOnStart = prefs.randomFingerprintOnStart || false
    formData.value.showPasswordSavePrompt = prefs.showPasswordSavePrompt || false
    formData.value.stopOnNetworkError = prefs.stopOnNetworkError || false
    formData.value.stopOnIpChange = prefs.stopOnIpChange || false
    formData.value.stopOnCountryChange = prefs.stopOnCountryChange || false
    formData.value.openWorkbench = prefs.openWorkbench || false
    formData.value.ipChangeNotification = prefs.ipChangeNotification || false
    formData.value.enableGoogleLogin = prefs.enableGoogleLogin || false

    // 网址访问控制
    formData.value.urlBlacklist = prefs.urlBlacklist || ''
    formData.value.urlWhitelist = prefs.urlWhitelist || ''

    console.log('✅ 表单数据已加载')
  } else {
    // ✨ 核心修复：如果切换到创建模式（newData 为空），主动重置表单
    console.log('✨ 切换到新建模式，执行重置')
    resetFormData()
  }
}, { immediate: true })

// 打开弹窗时重置状态
watch(() => props.visible, async (visible) => {
  if (visible) {
    currentStep.value = 1
    if (!props.editData) {
      console.log('✨ 新建模式：重置并初始化随机指纹')
      resetFormData()
      await initNewFingerprint()
    }
  } else {
    // 对话框关闭时，重置提交状态
    isSubmitting.value = false
  }
})

// 步骤配置
const steps = [
  { id: 1, label: '窗口信息' },
  { id: 2, label: '基础设置' },
  { id: 3, label: '高级指纹设置' },
  { id: 4, label: '代理设置' },
  { id: 5, label: '偏好设置' }
]

// 是否是最后一步
const isLastStep = computed(() => currentStep.value === 5)
const isFirstStep = computed(() => currentStep.value === 1)

// 下一步
const handleNext = () => {
  if (currentStep.value < 5) {
    currentStep.value++
  }
}

// 上一步
const handlePrev = () => {
  if (currentStep.value > 1) {
    currentStep.value--
  }
}

// 取消
const handleCancel = () => {
  // 提交期间不允许取消
  if (isSubmitting.value) {
    console.log('❗ 正在提交中，不允许取消')
    return
  }
  emit('close')
}

// 完成创建
const handleSubmit = async () => {
  // 防止重复提交
  if (isSubmitting.value) {
    console.log('❗ 正在提交中，忽略重复点击')
    return
  }

  isSubmitting.value = true

  try {
    generatingFingerprint.value = true
    fingerprintError.value = ''

    // 处理窗口名称：如果为空，自动生成"未命名+序号"
    let windowName = formData.value.name.trim()
    if (!windowName) {
      // 计算序号：统计现有"未命名"开头的窗口数量
      const unnamedProfiles = profileStore.profiles.filter(p =>
        p.name.startsWith('未命名')
      )
      const nextNumber = unnamedProfiles.length + 1
      windowName = `未命名${nextNumber}`
      console.log(`窗口名称为空，自动生成: ${windowName}`)
    }

    // 构建偏好设置对象
    const preferences = {
      extensions: formData.value.extensions || [],

      // 退出自动清理
      clearHistoryOnExit: formData.value.clearHistoryOnExit,
      clearCookiesOnExit: formData.value.clearCookiesOnExit,
      clearCacheOnExit: formData.value.clearCacheOnExit,

      // 启动前清理
      clearCacheOnStart: formData.value.clearCacheOnStart,
      clearCookiesOnStart: formData.value.clearCookiesOnStart,
      clearLocalStorageOnStart: formData.value.clearLocalStorageOnStart,

      // 同步选项
      syncBookmarks: formData.value.syncBookmarks,
      syncHistory: formData.value.syncHistory,
      syncTabs: formData.value.syncTabs,
      syncCookies: formData.value.syncCookies,
      syncExtensions: formData.value.syncExtensions,
      syncPasswords: formData.value.syncPasswords,
      syncIndexedDB: formData.value.syncIndexedDB,
      syncLocalStorage: formData.value.syncLocalStorage,
      syncSessionStorage: formData.value.syncSessionStorage,

      // 云端同步
      cloudSync: formData.value.cloudSync,
      cloudSyncExtensions: formData.value.cloudSyncExtensions,
      cloudSyncBookmarks: formData.value.cloudSyncBookmarks,

      // 其他选项
      randomFingerprintOnStart: formData.value.randomFingerprintOnStart,
      showPasswordSavePrompt: formData.value.showPasswordSavePrompt,
      stopOnNetworkError: formData.value.stopOnNetworkError,
      stopOnIpChange: formData.value.stopOnIpChange,
      stopOnCountryChange: formData.value.stopOnCountryChange,
      openWorkbench: formData.value.openWorkbench,
      ipChangeNotification: formData.value.ipChangeNotification,
      enableGoogleLogin: formData.value.enableGoogleLogin,

      // 网址访问控制
      urlBlacklist: formData.value.urlBlacklist,
      urlWhitelist: formData.value.urlWhitelist,
    }

    // 将表单数据合并到提交对象
    // 注意：我们直接使用 formData.value 作为指纹基础，
    // 因为 Step 3 已经将生成的指纹应用到了 formData 中
    const submitData = {
      ...formData.value,
      name: windowName,
      fingerprint: { ...formData.value }, // 使用表单中的指纹数据
      preferences: preferences,
    }

    console.log('提交表单数据:', submitData)
    emit('submit', submitData)
    // 注意：不在这里 emit('close')，让父组件在 API 完成后控制关闭
    // 这样可以防止用户在 API 调用期间再次打开对话框并点击

  } catch (error) {
    console.error('提交失败:', error)
    fingerprintError.value = `提交失败: ${error}`
    alert(`操作失败：${error}`)
    // 只有出错时才重置提交状态，允许用户重试
    isSubmitting.value = false
  } finally {
    generatingFingerprint.value = false
    // isSubmitting 不在这里重置，由父组件关闭对话框时自动重置
  }
}

// 关闭对话框
const handleClose = () => {
  // 提交期间不允许关闭
  if (isSubmitting.value) {
    console.log('❗ 正在提交中，不允许关闭')
    return
  }
  emit('close')
  // 关闭时如果是新建模式，重置一下，防止残留
  if (!props.editData) {
    resetFormData()
  }
}

// 移除此处重复的 watch，逻辑已整合到上方 visible 监听中

</script>

<template>
  <Teleport to="body">
    <Transition name="dialog-fade">
      <div v-if="visible" class="dialog-overlay" @click.self="handleClose">
        <div class="dialog-container">
          <!-- 头部 -->
          <div class="dialog-header">
            <h2 class="dialog-title">{{ isEditMode ? '编辑窗口' : '新建窗口' }}</h2>
            <button class="close-btn" @click="handleClose">
              <span class="material-symbols-outlined">close</span>
            </button>
          </div>

          <!-- 步骤条 -->
          <div class="step-area">
            <StepIndicator :steps="steps" :current="currentStep" @select="currentStep = $event" />
          </div>

          <!-- 内容区域 -->
          <div class="dialog-content">
            <Step1WindowInfo v-if="currentStep === 1" v-model="formData" />
            <Step2BasicSettings v-else-if="currentStep === 2" v-model="formData" />
            <Step3FingerprintSettings v-else-if="currentStep === 3" v-model="formData" />
            <Step4ProxySettings v-else-if="currentStep === 4" v-model="formData" />
            <Step5Preferences v-else-if="currentStep === 5" v-model="formData" />
          </div>

          <!-- 底部按钮 -->
          <div class="dialog-footer">
            <button class="btn btn-cancel" @click="handleCancel">
              取消
            </button>
            <button v-if="!isFirstStep" class="btn btn-prev" @click="handlePrev">
              <span class="material-symbols-outlined">chevron_left</span>
              上一步
            </button>
            <button v-if="!isLastStep" class="btn btn-next" @click="handleNext">
              下一步
              <span class="material-symbols-outlined">chevron_right</span>
            </button>
            <button v-else class="btn btn-submit" @click="handleSubmit"
              :disabled="generatingFingerprint || isSubmitting">
              <span v-if="generatingFingerprint" class="material-symbols-outlined spinning">refresh</span>
              <span v-else class="material-symbols-outlined">check_circle</span>
              {{ generatingFingerprint ? '正在生成指纹...' : (isEditMode ? '保存更改' : '完成并创建') }}
            </button>
          </div>
        </div>
      </div>
    </Transition>
  </Teleport>
</template>

<style scoped lang="scss">
.dialog-overlay {
  position: fixed;
  inset: 0;
  z-index: 50;
  display: flex;
  align-items: center;
  justify-content: center;
  background: rgba(15, 23, 42, 0.5);
  backdrop-filter: blur(4px);
  padding: 16px;
}

.dialog-container {
  width: 100%;
  max-width: 800px;
  height: 85vh;
  background: white;
  border-radius: 12px;
  box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.25);
  display: flex;
  flex-direction: column;
  overflow: hidden;
  border: 1px solid var(--color-border-default);

  .dark & {
    background: var(--color-bg-elevated);
    border-color: var(--color-border-dark);
  }
}

.dialog-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 16px 24px;
  border-bottom: 1px solid var(--color-border-default);

  .dialog-title {
    font-size: 18px;
    font-weight: 700;
    color: var(--color-text-primary);
    margin: 0;
  }

  .close-btn {
    width: 32px;
    height: 32px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: transparent;
    border: none;
    border-radius: 6px;
    color: var(--color-text-tertiary);
    cursor: pointer;
    transition: all 0.2s;

    &:hover {
      background: var(--color-hover-bg);
      color: var(--color-text-primary);
    }

    .material-symbols-outlined {
      font-size: 20px;
    }
  }
}

.step-area {
  padding: 24px 32px;
  border-bottom: 1px solid var(--color-border-default);
  background: rgba(248, 250, 252, 0.5);

  .dark & {
    background: rgba(30, 41, 59, 0.3);
  }
}

.dialog-content {
  flex: 1;
  overflow-y: auto;
  padding: 32px;

  &::-webkit-scrollbar {
    width: 6px;
  }

  &::-webkit-scrollbar-track {
    background: transparent;
  }

  &::-webkit-scrollbar-thumb {
    background: #cbd5e1;
    border-radius: 3px;

    &:hover {
      background: #94a3b8;
    }
  }
}

.dialog-footer {
  display: flex;
  align-items: center;
  justify-content: flex-end;
  gap: 12px;
  padding: 16px 24px;
  border-top: 1px solid var(--color-border-default);
  background: rgba(248, 250, 252, 0.5);

  .dark & {
    background: rgba(30, 41, 59, 0.3);
  }
}

.btn {
  display: inline-flex;
  align-items: center;
  gap: 4px;
  padding: 8px 16px;
  font-size: 14px;
  font-weight: 500;
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.2s;
  border: 1px solid transparent;

  .material-symbols-outlined {
    font-size: 18px;
  }
}

.btn-cancel {
  background: white;
  border-color: var(--color-border-default);
  color: var(--color-text-secondary);

  &:hover {
    background: var(--color-hover-bg);
    color: var(--color-text-primary);
  }

  .dark & {
    background: var(--color-bg-elevated);
  }
}

.btn-prev {
  background: white;
  border-color: var(--color-border-default);
  color: var(--color-text-secondary);

  &:hover {
    background: var(--color-hover-bg);
    color: var(--color-text-primary);
  }

  .dark & {
    background: var(--color-bg-elevated);
  }
}

.btn-next {
  background: #2563eb;
  color: white;

  &:hover {
    background: #1d4ed8;
  }
}

.btn-submit {
  background: linear-gradient(to right, #2563eb, #3b82f6);
  color: white;
  padding: 8px 24px;
  box-shadow: 0 10px 15px -3px rgba(37, 99, 235, 0.3);

  &:hover {
    background: linear-gradient(to right, #1d4ed8, #2563eb);
    transform: translateY(-1px);
    box-shadow: 0 20px 25px -5px rgba(37, 99, 235, 0.3);
  }
}

// 动画
.dialog-fade-enter-active,
.dialog-fade-leave-active {
  transition: all 0.3s ease;

  .dialog-container {
    transition: all 0.3s ease;
  }
}

.dialog-fade-enter-from,
.dialog-fade-leave-to {
  opacity: 0;

  .dialog-container {
    transform: scale(0.95) translateY(20px);
    opacity: 0;
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
</style>
