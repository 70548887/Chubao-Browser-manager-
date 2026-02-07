import { invoke } from '@tauri-apps/api/core'
import { appDataDir } from '@tauri-apps/api/path'

/**
 * 获取默认用户数据目录
 */
export async function getDefaultUserDataDir(): Promise<string> {
    try {
        const dataDir = await appDataDir()
        return dataDir
    } catch (error) {
        console.error('Failed to get default user data dir:', error)
        return ''
    }
}

/**
 * 获取单个设置值
 */
export async function getSettingValue(key: string): Promise<string | null> {
    try {
        return await invoke<string | null>('get_setting_value', { key })
    } catch (error) {
        console.error('Failed to get setting:', error)
        throw new Error(`获取设置失败: ${error}`)
    }
}

/**
 * 设置单个设置值
 */
export async function setSettingValue(key: string, value: string): Promise<void> {
    try {
        await invoke('set_setting_value', { key, value })
    } catch (error) {
        console.error('Failed to set setting:', error)
        throw new Error(`保存设置失败: ${error}`)
    }
}

/**
 * 获取所有设置
 */
export async function getAllSettings(): Promise<Record<string, string>> {
    try {
        return await invoke<Record<string, string>>('get_all_settings')
    } catch (error) {
        console.error('Failed to get all settings:', error)
        throw new Error(`获取所有设置失败: ${error}`)
    }
}
