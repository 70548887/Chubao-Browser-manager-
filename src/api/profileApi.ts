import { invoke } from '@tauri-apps/api/core'
import type { Profile, CreateProfileDTO, UpdateProfileDTO, FingerprintConfig, ProxyConfig, PreferencesConfig } from '@/types'

/**
 * Profile 数据类型（后端返回）
 */
export interface ProfileDto {
    id: string
    name: string
    group: string
    remark: string
    status: 'stopped' | 'running'
    fingerprint: Record<string, any>
    proxy: Record<string, any> | null
    preferences: Record<string, any> | null
    created_at: string  // RFC3339
    updated_at: string  // RFC3339
}

/**
 * 将后端 Fingerprint 转换为前端格式（snake_case → camelCase）
 */
function fingerprintFromDto(fp: any): FingerprintConfig | undefined {
    if (!fp) return undefined
    
    return {
        seed: fp.seed,
        platform: fp.platform,
        browser: fp.browser,
        userAgent: fp.user_agent,
        navigatorPlatform: fp.navigator_platform,
        osVersion: fp.os_version,
        browserVersion: fp.browser_version,
        hardwareConcurrency: fp.hardware_concurrency,
        deviceMemory: fp.device_memory,
        screenResolution: fp.screen_resolution,
        screenWidth: fp.screen_width,
        screenHeight: fp.screen_height,
        timezone: fp.timezone,
        language: fp.language,
        canvasNoise: fp.canvas_noise,
        webglNoise: fp.webgl_noise,
        audioNoise: fp.audio_noise,
        
        // 高级字段
        resolution: fp.resolution,
        fonts: fp.fonts,
        webrtc: fp.webrtc || 'disabled',
        webrtcPublicIp: fp.webrtc_public_ip,
        webrtcLocalIp: fp.webrtc_local_ip,
        webglImage: fp.webgl_image,
        webglVendor: fp.webgl_vendor || 'Intel Inc.',
        webglRenderer: fp.webgl_renderer || 'Intel Iris OpenGL Engine',
        webgpu: fp.webgpu ?? true,
        canvas: fp.canvas || (fp.canvas_noise ? 'noise' : 'off'),
        audioContext: fp.audio_context || (fp.audio_noise ? 'noise' : 'off'),
        speechVoices: fp.speech_voices,
        doNotTrack: fp.do_not_track || 'unspecified',
        clientRects: fp.client_rects ?? true,
        mediaDevices: fp.media_devices || 'real',
        deviceName: fp.device_name,
        macAddress: fp.mac_address,
        sslFingerprint: fp.ssl_fingerprint,
        portScanProtection: fp.port_scan_protection ?? true,
        portScanWhitelist: fp.port_scan_whitelist,
        customFonts: fp.custom_fonts,
        ignoreCertErrors: fp.ignore_cert_errors ?? false,
        customPlugins: fp.custom_plugins ?? false,
        cloudflareOptimize: fp.cloudflare_optimize ?? false,
        hardwareAcceleration: fp.hardware_acceleration ?? true,
        disableSandbox: fp.disable_sandbox ?? false,
        launchArgs: fp.launch_args,
        
        // 字体配置
        fontsMode: fp.fonts_mode ?? 'subset',
        fontsList: fp.fonts_list ?? [],
        
        // Variations 配置
        variationsEnabled: fp.variations_enabled ?? true,
        variationsSeedId: fp.variations_seed_id ?? '',
        
        // 地理位置配置
        geolocationMode: fp.geolocation_mode ?? 'disabled',
        geolocationLatitude: fp.geolocation_latitude ?? undefined,
        geolocationLongitude: fp.geolocation_longitude ?? undefined,
        geolocationAccuracy: fp.geolocation_accuracy ?? undefined,
        geolocationPrompt: fp.geolocation_prompt ?? 'ask',
    } as FingerprintConfig
}

/**
 * 将后端 Preferences 转换为前端格式（snake_case → camelCase）
 */
function preferencesFromDto(pref: any): PreferencesConfig | undefined {
    if (!pref) return undefined
    
    return {
        windowName: pref.window_name ?? false,
        customBookmarks: pref.custom_bookmarks ?? false,
        extensions: pref.extensions || [],
        customExtensions: pref.custom_extensions || [],
        startupPage: pref.startup_page || 'blank',
        startupUrl: pref.startup_url,
        syncBookmarks: pref.sync_bookmarks ?? false,
        syncHistory: pref.sync_history ?? false,
        syncTabs: pref.sync_tabs ?? false,
        syncCookies: pref.sync_cookies ?? false,
        syncExtensions: pref.sync_extensions ?? false,
        syncPasswords: pref.sync_passwords ?? false,
        syncIndexedDB: pref.sync_indexed_db ?? false,
        syncLocalStorage: pref.sync_local_storage ?? false,
        syncSessionStorage: pref.sync_session_storage ?? false,
        clearCacheOnStart: pref.clear_cache_on_start ?? false,
        clearCookiesOnStart: pref.clear_cookies_on_start ?? false,
        clearLocalStorageOnStart: pref.clear_local_storage_on_start ?? false,
        clearHistoryOnExit: pref.clear_history_on_exit ?? false,
        clearCookiesOnExit: pref.clear_cookies_on_exit ?? false,
        clearCacheOnExit: pref.clear_cache_on_exit ?? false,
        cloudSync: pref.cloud_sync ?? false,
        cloudSyncExtensions: pref.cloud_sync_extensions ?? false,
        cloudSyncBookmarks: pref.cloud_sync_bookmarks ?? false,
        randomFingerprintOnStart: pref.random_fingerprint_on_start ?? false,
        showPasswordSavePrompt: pref.show_password_save_prompt ?? false,
        stopOnNetworkError: pref.stop_on_network_error ?? false,
        stopOnIpChange: pref.stop_on_ip_change ?? false,
        stopOnCountryChange: pref.stop_on_country_change ?? false,
        openWorkbench: pref.open_workbench ?? false,
        ipChangeNotification: pref.ip_change_notification ?? false,
        enableGoogleLogin: pref.enable_google_login ?? false,
        urlBlacklist: pref.url_blacklist,
        urlWhitelist: pref.url_whitelist,
    } as PreferencesConfig
}

/**
 * 将后端 DTO 转换为前端 Profile
 */
function dtoToProfile(dto: ProfileDto): Profile {
    return {
        id: dto.id,
        name: dto.name,
        group: dto.group || '',
        status: dto.status as 'stopped' | 'running',
        remark: dto.remark,
        fingerprint: fingerprintFromDto(dto.fingerprint) || {
            seed: Date.now(),
            platform: 'windows',
            browser: 'chrome',
            userAgent: 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36',
            hardwareConcurrency: 8,
            deviceMemory: 16,
            screenResolution: '1920x1080',
            timezone: 'Asia/Shanghai',
            language: 'zh-CN',
            canvasNoise: true,
            webglNoise: true,
            audioNoise: true,
            canvas: 'noise',
            audioContext: 'noise',
            webrtc: 'real',
            webglVendor: 'Intel Inc.',
            webglRenderer: 'Intel Iris OpenGL Engine',
            webgpu: true,
            doNotTrack: 'unspecified',
            clientRects: true,
            mediaDevices: 'real',
            portScanProtection: true,
            hardwareAcceleration: true,
            disableSandbox: false,
        } as FingerprintConfig,
        proxy: dto.proxy as ProxyConfig,
        preferences: preferencesFromDto(dto.preferences),
        createdAt: new Date(dto.created_at).getTime(),
        updatedAt: new Date(dto.updated_at).getTime(),
    }
}

/**
 * 获取所有窗口列表
 */
export async function getProfiles(): Promise<Profile[]> {
    try {
        const dtos = await invoke<ProfileDto[]>('get_profiles')
        return dtos.map(dtoToProfile)
    } catch (error) {
        console.error('Failed to get profiles:', error)
        throw new Error(`获取窗口列表失败: ${error}`)
    }
}

/**
 * 获取单个窗口
 */
export async function getProfile(id: string): Promise<Profile> {
    try {
        const dto = await invoke<ProfileDto>('get_profile', { id })
        return dtoToProfile(dto)
    } catch (error) {
        console.error('Failed to get profile:', error)
        throw new Error(`获取窗口失败: ${error}`)
    }
}

/**
 * 将前端 Preferences 转换为后端格式（camelCase → snake_case）
 */
function preferencesToDto(pref: any): any {
    if (!pref) return pref
    
    return {
        window_name: pref.windowName,
        custom_bookmarks: pref.customBookmarks,
        extensions: pref.extensions,
        custom_extensions: pref.customExtensions,
        startup_page: pref.startupPage,
        startup_url: pref.startupUrl,
        sync_bookmarks: pref.syncBookmarks,
        sync_history: pref.syncHistory,
        sync_tabs: pref.syncTabs,
        sync_cookies: pref.syncCookies,
        sync_extensions: pref.syncExtensions,
        sync_passwords: pref.syncPasswords,
        sync_indexed_db: pref.syncIndexedDB,
        sync_local_storage: pref.syncLocalStorage,
        sync_session_storage: pref.syncSessionStorage,
        clear_cache_on_start: pref.clearCacheOnStart,
        clear_cookies_on_start: pref.clearCookiesOnStart,
        clear_local_storage_on_start: pref.clearLocalStorageOnStart,
        clear_history_on_exit: pref.clearHistoryOnExit,
        clear_cookies_on_exit: pref.clearCookiesOnExit,
        clear_cache_on_exit: pref.clearCacheOnExit,
        cloud_sync: pref.cloudSync,
        cloud_sync_extensions: pref.cloudSyncExtensions,
        cloud_sync_bookmarks: pref.cloudSyncBookmarks,
        random_fingerprint_on_start: pref.randomFingerprintOnStart,
        show_password_save_prompt: pref.showPasswordSavePrompt,
        stop_on_network_error: pref.stopOnNetworkError,
        stop_on_ip_change: pref.stopOnIpChange,
        stop_on_country_change: pref.stopOnCountryChange,
        open_workbench: pref.openWorkbench,
        ip_change_notification: pref.ipChangeNotification,
        enable_google_login: pref.enableGoogleLogin,
        url_blacklist: pref.urlBlacklist,
        url_whitelist: pref.urlWhitelist,
    }
}

/**
 * 将前端 Fingerprint 转换为后端格式
 */
function fingerprintToDto(fp: any): any {
    if (!fp) return fp
    
    // 如果是嵌套的 FingerprintFileConfig 格式 (从 generateRandomFingerprint 返回)
    if (fp.navigator && fp.screen) {
        return {
            seed: fp.seed?.master || fp.seed || Date.now(),
            platform: fp.navigator.platform || 'windows',
            navigator_platform: fp.navigator.platform || 'Win32',
            os_version: fp.client_hints?.platform_version || '10.0.0',
            browser: 'chrome',
            browser_version: fp.client_hints?.full_version?.split('.')[0] || '139',
            user_agent: fp.navigator.user_agent,
            hardware_concurrency: fp.navigator.hardware_concurrency,
            device_memory: fp.navigator.device_memory,
            screen_width: fp.screen.width,
            screen_height: fp.screen.height,
            screen_resolution: `${fp.screen.width}x${fp.screen.height}`,
            timezone: fp.timezone?.id || 'Asia/Shanghai',
            language: fp.navigator.language,
            
            // Canvas & WebGL & Audio
            canvas_noise: fp.canvas?.mode === 'noise' || !!fp.canvas?.rgb_noise,
            webgl_noise: fp.webgl?.noise_enabled || !!fp.webgl?.vendor,
            webgl_vendor: fp.webgl?.unmasked_vendor || fp.webgl?.vendor,
            webgl_renderer: fp.webgl?.unmasked_renderer || fp.webgl?.renderer,
            webgpu: true,
            audio_noise: fp.audio?.mode === 'noise' || typeof fp.audio?.noise_factor === 'number',
            audio_context: fp.audio?.mode || 'noise',
            canvas: fp.canvas?.mode || 'noise',
            
            // WebRTC
            webrtc: fp.webrtc?.mode || 'disabled',
            webrtc_public_ip: fp.webrtc?.public_ip,
            webrtc_local_ip: fp.webrtc?.local_ip,
            
            // 隐私保护
            do_not_track: fp.navigator?.do_not_track || 'unspecified',
            client_rects: fp.privacy?.client_rects_noise !== false,
            media_devices: fp.mediaDevices || 'real',
            port_scan_protection: fp.privacy?.port_scan_protection !== false,
            port_scan_whitelist: fp.portScanWhitelist || null,
            fonts: Array.isArray(fp.fonts?.list) ? fp.fonts.list : [],
            custom_fonts: [],
            speech_voices: [],
            ignore_cert_errors: fp.ignore_cert_errors || false,
            custom_plugins: fp.custom_plugins,
            cloudflare_optimize: fp.cloudflare_optimize,
            
            // 设备信息
            device_name: fp.device?.name,
            mac_address: fp.device?.mac_address,
            ssl_fingerprint: fp.ssl_fingerprint,
            
            // 性能设置
            hardware_acceleration: fp.hardware_acceleration !== false,
            disable_sandbox: fp.disable_sandbox || false,
            launch_args: fp.launch_args,
        }
    }
    
    // 如果是扁平的 FingerprintConfig 格式 (从表单或 profileApi 返回)
    return {
        seed: fp.seed || Date.now(),
        platform: fp.platform || 'windows',  // 确保必填字段有默认值
        navigator_platform: fp.navigator_platform || fp.navigatorPlatform || 'Win32',
        os_version: fp.os_version || fp.osVersion || 'Windows 10',
        browser: fp.browser || 'chrome',  // 确保必填字段有默认值
        browser_version: fp.browser_version || fp.browserVersion || '120',
        user_agent: fp.user_agent || fp.userAgent || 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36',
        hardware_concurrency: fp.hardware_concurrency || fp.hardwareConcurrency || 8,
        device_memory: fp.device_memory || fp.deviceMemory || 8,
        screen_width: fp.screen_width || fp.screenWidth || 1920,
        screen_height: fp.screen_height || fp.screenHeight || 1080,
        screen_resolution: fp.screen_resolution || fp.screenResolution || `${fp.screen_width || fp.screenWidth || 1920}x${fp.screen_height || fp.screenHeight || 1080}`,
        timezone: fp.timezone || 'Asia/Shanghai',
        language: fp.language || 'zh-CN',
        
        // Canvas & WebGL & Audio - 确保布尔字段有默认值
        canvas_noise: fp.canvas === 'noise' || fp.canvas_noise || fp.canvasNoise || true,
        webgl_noise: fp.webgl === 'noise' || fp.webgl_noise || fp.webglNoise || true,
        webgl_vendor: fp.webgl_vendor || fp.webglVendor || 'Google Inc. (NVIDIA)',
        webgl_renderer: fp.webgl_renderer || fp.webglRenderer || 'ANGLE (NVIDIA GeForce GTX 1660)',
        webgpu: fp.webgpu === 'real' || fp.webgpu === 'webgl' || fp.webgpu === true || false,
        audio_noise: fp.audio_context === 'noise' || fp.audioContext === 'noise' || fp.audio_noise || fp.audioNoise || true,
        audio_context: fp.audio_context || fp.audioContext,
        canvas: fp.canvas,
        
        // WebRTC
        webrtc: fp.webrtc,
        webrtc_public_ip: fp.webrtc_public_ip || fp.webrtcPublicIp,
        webrtc_local_ip: fp.webrtc_local_ip || fp.webrtcLocalIp,
        
        // 隐私保护
        do_not_track: fp.do_not_track || fp.doNotTrack || 'unspecified',
        client_rects: fp.client_rects || fp.clientRects,
        media_devices: fp.media_devices || fp.mediaDevices,
        port_scan_protection: fp.port_scan_protection || fp.portScanProtection,
        port_scan_whitelist: fp.port_scan_whitelist || fp.portScanWhitelist || null,
        fonts: Array.isArray(fp.fonts) ? fp.fonts : (typeof fp.fonts === 'string' ? fp.fonts.split(',').filter(Boolean) : []),
        custom_fonts: Array.isArray(fp.customFonts) ? fp.customFonts : (typeof fp.customFonts === 'string' ? fp.customFonts.split(',').filter(Boolean) : []),
        speech_voices: Array.isArray(fp.speechVoices) ? fp.speechVoices : (typeof fp.speechVoices === 'string' ? fp.speechVoices.split(',').filter(Boolean) : []),
        ignore_cert_errors: fp.ignore_cert_errors || fp.ignoreCertErrors || false,
        custom_plugins: fp.custom_plugins || fp.customPlugins,
        cloudflare_optimize: fp.cloudflare_optimize || fp.cloudflareOptimize,
        
        // 设备信息
        device_name: fp.device_name || fp.deviceName,
        mac_address: fp.mac_address || fp.macAddress,
        ssl_fingerprint: fp.ssl_fingerprint || fp.sslFingerprint,
        
        // 性能设置
        hardware_acceleration: fp.hardware_acceleration || fp.hardwareAcceleration,
        disable_sandbox: fp.disable_sandbox || fp.disableSandbox,
        launch_args: fp.launch_args || fp.launchArgs,
        
        // 字体配置
        fonts_mode: fp.fonts_mode || fp.fontsMode || 'subset',
        fonts_list: fp.fonts_list || fp.fontsList || [],
        
        // Variations 配置
        variations_enabled: fp.variations_enabled ?? fp.variationsEnabled ?? true,
        variations_seed_id: fp.variations_seed_id || fp.variationsSeedId || '',
        
        // 地理位置配置
        geolocation_mode: fp.geolocation_mode || fp.geolocationMode || 'disabled',
        geolocation_latitude: fp.geolocation_latitude ?? fp.geolocationLatitude ?? null,
        geolocation_longitude: fp.geolocation_longitude ?? fp.geolocationLongitude ?? null,
        geolocation_accuracy: fp.geolocation_accuracy ?? fp.geolocationAccuracy ?? null,
        geolocation_prompt: fp.geolocation_prompt || fp.geolocationPrompt || 'ask',
    }
}

/**
 * 将前端表单数据转换为浏览器配置文件所需的嵌套结构
 */
// @ts-ignore
// eslint-disable-next-line @typescript-eslint/no-unused-vars
function formToBrowserConfig(form: any): any {
    if (!form) return null;
    
    // 解析屏幕分辨率
    const [screenWidth, screenHeight] = form.screenResolution?.split('x').map(Number) || [1920, 1080];
    
    // 生成种子值
    const seed = form.seed || Date.now();
    
    return {
        "$schema": "bm_fingerprint_v2",
        schema_version: 2,
        profile_id: crypto.randomUUID().replace(/-/g, ''),
        created_at: new Date().toISOString(),
        
        seed: {
            master: seed,
            canvas: seed,
            webgl: seed,
            audio: seed,
        },
        
        navigator: {
            user_agent: form.userAgent || form.navigator?.user_agent || 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/139.0.0.0 Safari/537.36',
            platform: form.navigatorPlatform || form.navigator?.platform || 'Win32',
            vendor: 'Google Inc.',
            app_version: '5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36',
            language: form.language || form.navigator?.language || 'zh-CN',
            languages: [form.language || form.navigator?.language || 'zh-CN', 'en-US', 'en'],
            hardware_concurrency: form.hardwareConcurrency || form.navigator?.hardware_concurrency || 8,
            device_memory: form.deviceMemory || form.navigator?.device_memory || 8,
            max_touch_points: 0,
            do_not_track: form.doNotTrack || form.navigator?.do_not_track || null,
            webdriver: false,
            pdf_viewer_enabled: true,
            cookie_enabled: true,
        },
        
        screen: {
            width: form.screenWidth || screenWidth || 1920,
            height: form.screenHeight || screenHeight || 1080,
            avail_width: form.screenWidth || screenWidth || 1920,
            avail_height: (form.screenHeight || screenHeight || 1080) - 40, // 通常减去任务栏高度
            color_depth: 24,
            pixel_depth: 24,
            device_pixel_ratio: 1.0,
            orientation_type: 'landscape-primary',
            orientation_angle: 0,
        },
        
        webgl: {
            vendor: form.webglVendor || form.webgl?.vendor || 'Google Inc. (NVIDIA)',
            renderer: form.webglRenderer || form.webgl?.renderer || 'ANGLE (NVIDIA, NVIDIA GeForce GTX 1660 Direct3D11 vs_5_0 ps_5_0, D3D11)',
            unmasked_vendor: form.webglVendor || form.webgl?.vendor || 'NVIDIA Corporation',
            unmasked_renderer: form.webglRenderer || form.webgl?.renderer || 'NVIDIA GeForce GTX 1660/PCIe/SSE2',
            version: 'WebGL 1.0 (OpenGL ES 2.0 Chromium)',
            shading_language_version: 'WebGL GLSL ES 1.0 (OpenGL ES GLSL ES 1.0 Chromium)',
            max_texture_size: 16384,
            max_vertex_attribs: 16,
            noise_enabled: form.webgl === 'noise' || form.webgl_noise || form.webgl?.noise_enabled || true,
        },
        
        canvas: {
            mode: form.canvas || form.canvas?.mode || 'noise',
            noise_level: 0.0001,
        },
        
        audio: {
            mode: form.audioContext || form.audio?.mode || 'noise',
            sample_rate: 44100,
            max_channel_count: 2,
        },
        
        timezone: {
            id: form.timezone || form.timezone?.id || 'Asia/Shanghai',
            offset_minutes: getTimezoneOffset(form.timezone || form.timezone?.id || 'Asia/Shanghai'),
        },
        
        geolocation: {
            mode: 'disabled',
            latitude: null,
            longitude: null,
            accuracy: null,
        },
        
        webrtc: {
            mode: form.webrtc || form.webrtc?.mode || 'disabled',
            public_ip: form.publicIp || form.webrtc?.public_ip || null,
            local_ip: form.localIp || form.webrtc?.local_ip || null,
        },
        
        fonts: {
            mode: form.fonts || form.fonts?.mode || 'subset',
            list: [
                'Arial', 'Arial Black', 'Calibri', 'Cambria', 'Courier New',
                'Georgia', 'Helvetica', 'Impact', 'Microsoft YaHei', 'SimSun',
                'SimHei', 'Tahoma', 'Times New Roman', 'Trebuchet MS', 'Verdana'
            ],
        },
        
        client_hints: {
            brands: [
                { brand: 'Not_A Brand', version: '8' },
                { brand: 'Chromium', version: form.browserVersion || '139' },
                { brand: 'Google Chrome', version: form.browserVersion || '139' },
            ],
            full_version: `${form.browserVersion || '139'}.0.0.0`,
            platform: form.platform === 'windows' ? 'Windows' : form.platform === 'macos' ? 'macOS' : 'Linux',
            platform_version: form.osVersion || '10.0.0',
            architecture: 'x86',
            bitness: '64',
            model: '',
            mobile: false,
            wow64: false,
        },
        
        battery: {
            charging: true,
            charging_time: null,
            discharging_time: null,
            level: 1.0,
        },
        
        network: {
            effective_type: '4g',
            downlink: 10.0,
            rtt: 50,
            save_data: false,
        },
        
        privacy: {
            client_rects_noise: form.clientRects || form.privacy?.client_rects_noise || true,
            port_scan_protection: form.portScanProtection || form.privacy?.port_scan_protection || true,
        },
        
        device: {
            name: form.deviceName || form.device?.name || 'DESKTOP-' + Math.random().toString(36).substr(2, 10).toUpperCase(),
            mac_address: form.macAddress || form.device?.mac_address || '64-2B-7A-4D-96-E1',
        }
    };
}

// 辅助函数：获取时区偏移量
function getTimezoneOffset(timezone: string): number {
    switch (timezone) {
        case 'Asia/Shanghai':
        case 'Asia/Hong_Kong':
        case 'Asia/Taipei':
            return -480; // UTC+8
        case 'Asia/Tokyo':
            return -540; // UTC+9
        case 'America/New_York':
            return 300;  // UTC-5
        case 'America/Los_Angeles':
            return 480;  // UTC-8
        case 'Europe/London':
            return 0;    // UTC+0
        case 'Europe/Paris':
        case 'Europe/Berlin':
            return -60;  // UTC+1
        default:
            return -480; // 默认 UTC+8
    }
}

/**
 * 将前端代理配置转换为后端格式
 */
function proxyToDto(proxy: any): any {
    if (!proxy) return undefined
    
    // 转换代理类型为后端期望的格式（首字母大写）
    const typeMap: Record<string, string> = {
        'http': 'Http',
        'https': 'Https',
        'socks5': 'Socks5',
    }
    
    return {
        type: typeMap[proxy.type] || 'Socks5',
        host: proxy.host,
        port: proxy.port,
        username: proxy.username || null,
        password: proxy.password || null,
    }
}

/**
 * 创建窗口
 */
export async function createProfile(data: CreateProfileDTO): Promise<Profile> {
    try {
        // 转换字段为后端格式
        const backendData = {
            ...data,
            fingerprint: fingerprintToDto(data.fingerprint),
            proxy: proxyToDto(data.proxy),
            preferences: preferencesToDto(data.preferences),
        }
        const dto = await invoke<ProfileDto>('create_profile', { data: backendData })
        return dtoToProfile(dto)
    } catch (error) {
        console.error('Failed to create profile:', error)
        throw new Error(`创建窗口失败: ${error}`)
    }
}

/**
 * 更新窗口
 */
export async function updateProfile(id: string, data: UpdateProfileDTO): Promise<Profile> {
    try {
        // 转换字段为后端格式
        const backendData = {
            ...data,
            fingerprint: data.fingerprint ? fingerprintToDto(data.fingerprint) : undefined,
            proxy: data.proxy ? proxyToDto(data.proxy) : undefined,
            preferences: data.preferences ? preferencesToDto(data.preferences) : undefined,
        }
        const dto = await invoke<ProfileDto>('update_profile', { id, data: backendData })
        return dtoToProfile(dto)
    } catch (error) {
        console.error('Failed to update profile:', error)
        throw new Error(`更新窗口失败: ${error}`)
    }
}

/**
 * 删除窗口
 */
export async function deleteProfile(id: string): Promise<void> {
    try {
        await invoke('delete_profile', { id })
    } catch (error) {
        console.error('Failed to delete profile:', error)
        throw new Error(`删除窗口失败: ${error}`)
    }
}

/**
 * 批量删除窗口
 */
export async function batchDeleteProfiles(ids: string[]): Promise<import('@/types').BatchResult> {
    try {
        const result = await invoke<import('@/types').BatchResult>('batch_delete_profiles', { ids })
        return result
    } catch (error) {
        console.error('Failed to batch delete profiles:', error)
        throw new Error(`批量删除窗口失败: ${error}`)
    }
}

/**
 * 批量移动到分组
 */
export async function batchMoveToGroup(profileIds: string[], targetGroupId: string): Promise<import('@/types').BatchResult> {
    try {
        const result = await invoke<import('@/types').BatchResult>('batch_move_to_group', { profileIds, targetGroupId })
        return result
    } catch (error) {
        console.error('Failed to batch move to group:', error)
        throw new Error(`批量移动分组失败: ${error}`)
    }
}

/**
 * 批量复制环境
 */
export async function batchDuplicateProfiles(profileIds: string[]): Promise<import('@/types').BatchResult> {
    try {
        const result = await invoke<import('@/types').BatchResult>('batch_duplicate_profiles', { profileIds })
        return result
    } catch (error) {
        console.error('Failed to batch duplicate profiles:', error)
        throw new Error(`批量复制环境失败: ${error}`)
    }
}

/**
 * 宫格排列窗口
 * @param columns 列数（默认自动计算）
 */
export async function arrangeWindowsGrid(columns: number = 3): Promise<void> {
    try {
        await invoke('arrange_windows_grid', { columns })
    } catch (error) {
        console.error('Failed to arrange windows:', error)
        throw new Error(`排列窗口失败: ${error}`)
    }
}
