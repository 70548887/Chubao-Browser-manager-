// Profile 相关类型定义

/** 环境状态 */
export type ProfileStatus = 'stopped' | 'running' | 'launching' | 'error'

/** 代理类型 */
export type ProxyType = 'http' | 'https' | 'socks5'

/** 代理配置 */
export interface ProxyConfig {
    type: ProxyType
    host: string
    port: number
    username?: string
    password?: string
}

/** 基础设置配置 */
export interface BasicSettingsConfig {
    /** 语言 */
    language: 'auto' | 'custom'
    languageValue?: string
    /** 界面语言 */
    displayLanguage: 'auto' | 'custom'
    displayLanguageValue?: string
    /** 时区 */
    timezone: 'auto' | 'custom'
    timezoneValue?: string
    /** 地理位置提示 */
    geolocationPrompt: 'ask' | 'allow' | 'block'
    /** 地理位置 */
    geolocation: 'auto' | 'custom'
    geolocationLatitude?: number
    geolocationLongitude?: number
    /** 声音 */
    audio: boolean
    /** 图片 */
    image: boolean
    /** 视频 */
    video: boolean
    /** 窗口大小 */
    windowSize: 'custom' | 'fullscreen'
    windowWidth?: number
    windowHeight?: number
}

/** 偏好设置配置 */
export interface PreferencesConfig {
    /** 窗口名称 */
    windowName: boolean
    /** 自定义书签 */
    customBookmarks: boolean
    /** 扩展管理 */
    extensions?: string[]  // 已选择的扩展ID列表
    customExtensions?: string[]  // 自定义扩展路径列表
    /** 启动页设置 */
    startupPage: 'blank' | 'url'
    startupUrl?: string
    /** 同步选项 */
    syncBookmarks: boolean
    syncHistory: boolean
    syncTabs: boolean
    syncCookies: boolean
    syncExtensions: boolean
    syncPasswords: boolean
    syncIndexedDB: boolean
    syncLocalStorage: boolean
    syncSessionStorage: boolean
    /** 启动前清理 */
    clearCacheOnStart: boolean
    clearCookiesOnStart: boolean
    clearLocalStorageOnStart: boolean
    /** 退出自动清理 */
    clearHistoryOnExit: boolean
    clearCookiesOnExit: boolean
    clearCacheOnExit: boolean
    /** 云端同步 */
    cloudSync: boolean
    cloudSyncExtensions: boolean
    cloudSyncBookmarks: boolean
    /** 启动时随机指纹 */
    randomFingerprintOnStart: boolean
    /** 弹出保存密码提示 */
    showPasswordSavePrompt: boolean
    /** 停止打开条件 */
    stopOnNetworkError: boolean
    stopOnIpChange: boolean
    stopOnCountryChange: boolean
    /** 打开工作台 */
    openWorkbench: boolean
    /** IP 变化提醒 */
    ipChangeNotification: boolean
    /** 是否开启谷歌登录 */
    enableGoogleLogin: boolean
    /** 网址黑名单 */
    urlBlacklist?: string
    /** 网址白名单 */
    urlWhitelist?: string
}

/** 指纹配置 */
export interface FingerprintConfig {
    seed: number
    platform: 'windows' | 'macos' | 'linux' | 'android' | 'ios'
    browser: 'chrome' | 'edge' | 'brave'
    userAgent: string
    brand?: string
    version?: string
    hardwareConcurrency: number
    deviceMemory: number
    screenResolution: string
    screenWidth?: number
    screenHeight?: number
    timezone: string
    language: string
    canvasNoise: boolean
    webglNoise: boolean
    audioNoise: boolean
    
    // 高级指纹参数
    /** 分辨率 */
    resolution?: string
    /** 字体指纹 */
    fonts?: string[]
    /** WebRTC */
    webrtc: 'real' | 'fake' | 'disabled'
    webrtcPublicIp?: string
    webrtcLocalIp?: string
    /** WebGL 图像 */
    webglImage?: string
    /** WebGL Info */
    webglVendor: string
    webglRenderer: string
    /** WebGL Unmasked 模式: 'mask' | 'real' */
    webglUnmaskedMode?: 'mask' | 'real'
    /** WebGL Vendor Unmasked */
    webglVendorUnmasked?: string
    /** WebGL Renderer Unmasked */
    webglRendererUnmasked?: string
    /** WebGpu */
    webgpu: boolean
    /** WebGPU 模式: 'webgl' | 'real' | 'disabled' */
    webgpuMode?: 'webgl' | 'real' | 'disabled'
    /** Canvas */
    canvas: 'noise' | 'block' | 'off'
    /** AudioContext */
    audioContext: 'noise' | 'block' | 'off'
    /** Speech Voices */
    speechVoices?: string[]
    /** Do Not Track */
    doNotTrack: '1' | '0' | 'unspecified'
    /** Client Rects */
    clientRects: boolean
    /** 媒体设备 */
    mediaDevices: 'real' | 'fake' | 'disabled'
    /** 设备名称 */
    deviceName?: string
    /** MAC地址 */
    macAddress?: string
    /** SSL指纹 */
    sslFingerprint?: string
    /** 端口扫描保护 */
    portScanProtection: boolean
    /** 扫描白名单 */
    portWhitelist?: string
    /** 硬件加速 */
    hardwareAcceleration: boolean
    /** 禁用沙盒 */
    disableSandbox: boolean
    /** 启动参数 */
    launchArgs?: string
    
    // 字体配置
    /** 字体模式 */
    fontsMode?: 'subset' | 'real' | 'custom' | 'random'
    /** 字体列表 */
    fontsList?: string[]
    /** 自定义字体 */
    customFonts?: string
    
    // Variations 配置
    /** 启用 Variations */
    variationsEnabled?: boolean
    /** Variations Seed ID */
    variationsSeedId?: string
    
    /** 地理位置模式: 'auto' | 'custom' | 'disabled' */
    geolocationMode?: string
    /** 纬度 */
    geolocationLatitude?: number
    /** 经度 */
    geolocationLongitude?: number
    /** 精度 (米) */
    geolocationAccuracy?: number
    /** 地理位置权限提示: 'ask' | 'allow' | 'block' */
    geolocationPrompt?: string
}

/** 环境/配置文件 */
export interface Profile {
    id: string
    name: string
    group?: string
    status: ProfileStatus
    dataDir?: string
    cdpPort?: number
    fingerprint: FingerprintConfig
    proxy?: ProxyConfig
    basicSettings?: BasicSettingsConfig
    preferences?: PreferencesConfig
    remark?: string
    cookies?: string
    lastOpenTime?: number
    createdAt: number
    updatedAt: number
}

/** 创建环境 DTO */
export interface CreateProfileDTO {
    name: string
    group?: string
    fingerprint?: Partial<FingerprintConfig>
    proxy?: ProxyConfig
    basicSettings?: Partial<BasicSettingsConfig>
    preferences?: Partial<PreferencesConfig>
    remark?: string
    cookies?: string
}

/** 更新环境 DTO */
export interface UpdateProfileDTO {
    name?: string
    group?: string
    fingerprint?: Partial<FingerprintConfig>
    proxy?: ProxyConfig
    preferences?: Partial<PreferencesConfig>
    remark?: string
    cookies?: string
}

export type ProfileCreateDTO = CreateProfileDTO
export type ProfileUpdateDTO = UpdateProfileDTO
export type Fingerprint = FingerprintConfig
