// Kernel Download API
import { invoke } from '@tauri-apps/api/core'
import { listen, type UnlistenFn } from '@tauri-apps/api/event'

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

// Default kernel download URL
export const DEFAULT_KERNEL_URL = 'https://github.com/70548887/Chubao-Browser-manager-/releases/download/v0.2.0-kernel146/chromium-kernel-win64-v146.zip'
