import { invoke } from '@tauri-apps/api/core'
import type { BatchResult } from '@/types'

/**
 * 启动浏览器
 */
export async function launchBrowser(profileId: string): Promise<void> {
    try {
        await invoke('launch_browser', { profileId })
    } catch (error) {
        console.error('Failed to launch browser:', error)
        throw new Error(`启动浏览器失败: ${error}`)
    }
}

/**
 * 停止浏览器
 */
export async function stopBrowser(profileId: string): Promise<void> {
    try {
        await invoke('stop_browser', { profileId })
    } catch (error) {
        console.error('Failed to stop browser:', error)
        throw new Error(`停止浏览器失败: ${error}`)
    }
}

/**
 * 批量启动浏览器
 */
export async function batchLaunchBrowsers(profileIds: string[]): Promise<BatchResult> {
    try {
        const result = await invoke<BatchResult>('batch_launch_browsers', { profileIds })
        return result
    } catch (error) {
        console.error('Failed to batch launch browsers:', error)
        throw new Error(`批量启动浏览器失败: ${error}`)
    }
}

/**
 * 批量停止浏览器
 */
export async function batchStopBrowsers(profileIds: string[]): Promise<BatchResult> {
    try {
        const result = await invoke<BatchResult>('batch_stop_browsers', { profileIds })
        return result
    } catch (error) {
        console.error('Failed to batch stop browsers:', error)
        throw new Error(`批量停止浏览器失败: ${error}`)
    }
}

/**
 * 宫格排列窗口（仅 Windows）
 */
export async function arrangeWindowsGrid(columns: number): Promise<void> {
    try {
        await invoke('arrange_windows_grid', { columns })
    } catch (error) {
        console.error('Failed to arrange windows:', error)
        throw new Error(`排列窗口失败: ${error}`)
    }
}

/**
 * 隐藏所有窗口（老板键，仅 Windows）
 */
export async function hideAllWindows(): Promise<void> {
    try {
        await invoke('hide_all_windows')
    } catch (error) {
        console.error('Failed to hide windows:', error)
        throw new Error(`隐藏窗口失败: ${error}`)
    }
}

/**
 * 显示所有窗口（仅 Windows）
 */
export async function showAllWindows(): Promise<void> {
    try {
        await invoke('show_all_windows')
    } catch (error) {
        console.error('Failed to show windows:', error)
        throw new Error(`显示窗口失败: ${error}`)
    }
}
