// Kernel Download API
import { invoke } from '@tauri-apps/api/core'
import { listen, type UnlistenFn } from '@tauri-apps/api/event'
import { FALLBACK_KERNEL_URL } from '@/config'

// Download progress interface
export interface DownloadProgress {
    downloaded: number
    total: number | null
    speed: number
    status: 'Idle' | 'Downloading' | 'Extracting' | 'Completed' | 'Failed'
    message: string
}

// Kernel version info interface
export interface KernelVersionInfo {
    version: string
    build_date: string
    platform: string
    source: string
    files_count: number
    total_size_bytes: number
}

// 内核下载源接口
export interface KernelDownloadSource {
    id: string
    name: string
    url: string
    priority: number
}

// 内核下载信息接口
export interface KernelDownloadInfo {
    has_update: boolean
    version: string
    release_date?: string
    changelog?: string
    file_size?: number
    file_hash?: string
    min_launcher_version?: string
    downloads?: KernelDownloadSource[]
    current_is_latest?: boolean
}

// API 响应接口
export interface ApiResponse<T> {
    code: number
    message: string
    data: T | null
}

/**
 * 获取内核下载信息（通过后端代理请求，安全架构）
 * @param platform 平台：windows / darwin / linux
 * @param arch 架构：x86_64 / aarch64
 * @returns 内核下载信息
 */
export async function getKernelDownloadInfo(
    platform: string = 'windows',
    arch: string = 'x86_64'
): Promise<KernelDownloadInfo> {
    try {
        // ✅ 通过 IPC 调用后端接口，避免前端暴露 API 地址
        const result = await invoke<any>('get_kernel_download_info_api', {
            platform,
            arch,
        })

        // 处理错误码
        if (result.code !== 0) {
            switch (result.code) {
                case 1001:
                    throw new Error('缺少必要参数')
                case 1002:
                    throw new Error('未找到适配的内核版本')
                case 1003:
                    throw new Error('启动器版本过低，请先更新启动器')
                case 5000:
                    throw new Error('服务器内部错误，请稍后重试')
                default:
                    throw new Error(result.message || '获取内核下载信息失败')
            }
        }

        if (!result.data) {
            throw new Error('响应数据为空')
        }

        return result.data
    } catch (error) {
        console.error('Failed to get kernel download info:', error)
        // 降级：返回默认下载信息
        return {
            has_update: true,
            version: '146',
            downloads: [{
                id: 'default',
                name: '默认下载源',
                url: DEFAULT_KERNEL_URL,
                priority: 1
            }]
        }
    }
}

/**
 * Check if kernel is installed
 */
export async function isKernelInstalled(): Promise<boolean> {
    try {
        return await invoke<boolean>('is_kernel_installed')
    } catch (error) {
        console.error('Failed to check kernel status:', error)
        return false
    }
}

/**
 * Get installed kernel version info
 */
export async function getKernelVersion(): Promise<KernelVersionInfo | null> {
    try {
        return await invoke<KernelVersionInfo | null>('get_kernel_version')
    } catch (error) {
        console.error('Failed to get kernel version:', error)
        return null
    }
}

/**
 * Get kernel download progress
 */
export async function getKernelDownloadProgress(): Promise<DownloadProgress> {
    try {
        return await invoke<DownloadProgress>('get_kernel_download_progress')
    } catch (error) {
        console.error('Failed to get download progress:', error)
        throw error
    }
}

/**
 * Start kernel download
 */
export async function downloadKernel(downloadUrl: string): Promise<void> {
    try {
        await invoke('download_kernel', { downloadUrl })
    } catch (error) {
        console.error('Failed to start kernel download:', error)
        throw new Error(`下载内核失败: ${error}`)
    }
}

/**
 * Uninstall kernel
 */
export async function uninstallKernel(): Promise<void> {
    try {
        await invoke('uninstall_kernel')
    } catch (error) {
        console.error('Failed to uninstall kernel:', error)
        throw new Error(`卸载内核失败: ${error}`)
    }
}

/**
 * Get kernel executable path
 */
export async function getKernelPath(): Promise<string> {
    try {
        return await invoke<string>('get_kernel_path')
    } catch (error) {
        console.error('Failed to get kernel path:', error)
        throw error
    }
}

/**
 * Get bundled kernel path (from resources)
 */
export async function getBundledKernelPath(): Promise<string | null> {
    try {
        return await invoke<string | null>('get_bundled_kernel_path')
    } catch (error) {
        console.error('Failed to get bundled kernel path:', error)
        return null
    }
}

/**
 * List all bundled kernel versions
 * @returns Array of version strings (e.g. ['146.0.7652.0', '145.0.0.0'])
 */
export async function listBundledKernelVersions(): Promise<string[]> {
    try {
        return await invoke<string[]>('list_bundled_kernel_versions')
    } catch (error) {
        console.error('Failed to list bundled kernel versions:', error)
        return []
    }
}

/**
 * Get bundled kernel path for specific version
 * @param version Version string (e.g. '146.0.7652.0')
 */
export async function getBundledKernelPathByVersion(version: string): Promise<string | null> {
    try {
        return await invoke<string | null>('get_bundled_kernel_path_by_version', { version })
    } catch (error) {
        console.error('Failed to get bundled kernel path by version:', error)
        return null
    }
}

/**
 * 手动触发内核检查和解压 (用于登录后延迟检查)
 * 
 * 当用户登录/注册完成进入主页面时,如果启动时内核解压未触发或未完成,
 * 可以调用此函数手动触发检查和解压流程
 * 
 * @returns true: 触发了解压流程, false: 内核已存在或无需解压
 */
export async function triggerKernelExtraction(): Promise<boolean> {
    try {
        return await invoke<boolean>('trigger_kernel_extraction')
    } catch (error) {
        console.error('Failed to trigger kernel extraction:', error)
        throw new Error(`触发内核解压失败: ${error}`)
    }
}

/**
 * Listen to download progress events
 */
export function onDownloadProgress(callback: (progress: DownloadProgress) => void): Promise<UnlistenFn> {
    return listen<DownloadProgress>('kernel:download-progress', (event) => {
        callback(event.payload)
    })
}

/**
 * Listen to download complete event
 */
export function onDownloadComplete(callback: () => void): Promise<UnlistenFn> {
    return listen('kernel:download-complete', () => {
        callback()
    })
}

/**
 * Listen to download error event
 */
export function onDownloadError(callback: (error: string) => void): Promise<UnlistenFn> {
    return listen<string>('kernel:download-error', (event) => {
        callback(event.payload)
    })
}

// 默认内核下载地址（使用统一配置）
export const DEFAULT_KERNEL_URL = FALLBACK_KERNEL_URL
