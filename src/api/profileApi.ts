import { invoke } from '@tauri-apps/api/core'
import type { Profile, CreateProfileDTO, UpdateProfileDTO, FingerprintConfig, ProxyConfig } from '@/types'

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
        userAgent: fp.user_agent,  // snake_case → camelCase
        hardwareConcurrency: fp.hardware_concurrency,
        deviceMemory: fp.device_memory,
        screenResolution: fp.screen_resolution,
        timezone: fp.timezone,
        language: fp.language,
        canvasNoise: fp.canvas_noise,
        webglNoise: fp.webgl_noise,
        audioNoise: fp.audio_noise,
        // 高级字段（如果存在）
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
    } as FingerprintConfig
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
        preferences: dto.preferences as any,
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
 * 将前端 Fingerprint 转换为后端格式
 * 支持两种输入格式：
 * 1. FingerprintConfig（从后端生成的复杂嵌套结构）
 * 2. 简单表单格式（扁平结构）
 */
function fingerprintToDto(fp: any): any {
    if (!fp) return fp
    
    // 处理复杂的 FingerprintConfig 格式（从后端生成的）
    if (fp.navigator && fp.screen) {
        return {
            seed: Date.now(),
            platform: fp.navigator.platform || 'windows',
            browser: 'chrome',
            user_agent: fp.navigator.user_agent,
            hardware_concurrency: fp.navigator.hardware_concurrency,
            device_memory: fp.navigator.device_memory,
            screen_resolution: `${fp.screen.width}x${fp.screen.height}`,
            timezone: fp.timezone?.id || 'America/New_York',
            language: fp.navigator.language,
            canvas_noise: Array.isArray(fp.canvas?.rgb_noise) && fp.canvas.rgb_noise.length > 0,
            webgl_noise: !!fp.webgl?.vendor,
            audio_noise: typeof fp.audio?.noise_factor === 'number',
        }
    }
    
    // 简单格式（直接从表单来的）- 包含所有新增字段
    return {
        seed: fp.seed || Date.now(),
        platform: fp.platform,
        navigator_platform: fp.navigatorPlatform,  // navigator.platform 值
        os_version: fp.osVersion,  // 操作系统版本
        browser: fp.browser || 'chrome',
        browser_version: fp.browserVersion,  // 浏览器版本
        user_agent: fp.userAgent,
        hardware_concurrency: fp.hardwareConcurrency,
        device_memory: fp.deviceMemory,
        screen_width: fp.screenWidth,
        screen_height: fp.screenHeight,
        screen_resolution: fp.screenResolution || `${fp.screenWidth}x${fp.screenHeight}`,
        timezone: fp.timezone,
        language: fp.language,
        
        // Canvas & WebGL & Audio
        canvas_noise: fp.canvas === 'noise',
        webgl_noise: fp.webgl === 'noise',
        webgl_vendor: fp.webglVendor,
        webgl_renderer: fp.webglRenderer,
        webgpu: fp.webgpu,  // 'webgl' | 'real' | 'disable'
        audio_noise: fp.audioContext === 'noise',
        
        // WebRTC
        webrtc: fp.webrtc,  // 'replace' | 'real' | 'disable'
        public_ip: fp.publicIp,
        local_ip: fp.localIp,
        
        // 隐私保护
        do_not_track: fp.doNotTrack,  // 'enable' | 'disable' | 'unspecified'
        client_rects: fp.clientRects,
        media_devices: fp.mediaDevices,  // 'real' | 'fake' | 'disable'
        port_scan_protection: fp.portScanProtection,
        port_scan_whitelist: fp.portScanWhitelist,
        fonts: fp.fonts,  // 'system' | 'custom'
        custom_fonts: fp.customFonts,
        speech_voices: fp.speechVoices,  // 'match' | 'disable'
        ignore_cert_errors: fp.ignoreCertErrors,
        custom_plugins: fp.customPlugins,
        cloudflare_optimize: fp.cloudflareOptimize,
        
        // 设备信息
        device_name: fp.deviceName,
        mac_address: fp.macAddress,
        
        // 性能设置
        hardware_acceleration: fp.hardwareAcceleration,
        disable_sandbox: fp.disableSandbox,
        launch_args: fp.launchArgs,
    }
}

/**
 * 将前端表单数据转换为浏览器配置文件所需的嵌套结构
 * 这是完整的 FingerprintFileConfig 结构，用于生成浏览器配置文件
 */
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
            preferences: data.preferences,
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
        // 转换指纹字段为后端格式（如果存在）
        const backendData = {
            ...data,
            fingerprint: data.fingerprint ? fingerprintToDto(data.fingerprint) : undefined,
            preferences: data.preferences,
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
