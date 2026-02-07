import { invoke } from '@tauri-apps/api/core'

/**
 * 获取智能默认用户数据目录
 * 优先使用非系统盘(D:, E:等)，如果只有C盘则使用AppData
 */
export async function getDefaultUserDataDir(): Promise<string> {
    try {
        return await invoke<string>('get_smart_default_user_data_dir')
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
        const value = await invoke<string | null>('get_setting_value', { key })
        console.log(`📖 读取设置: ${key} = ${value}`)
        return value
    } catch (error) {
        console.error(`❌ 读取设置失败 [${key}]:`, error)
        throw new Error(`获取设置失败: ${error}`)
    }
}

/**
 * 设置单个设置值
 */
export async function setSettingValue(key: string, value: string): Promise<void> {
    try {
        console.log(`💾 保存设置: ${key} = ${value}`)
        await invoke('set_setting_value', { key, value })
        console.log(`✅ 设置保存成功: ${key}`)
    } catch (error) {
        console.error(`❌ 保存设置失败 [${key}]:`, error)
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
