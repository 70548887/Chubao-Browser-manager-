import { invoke } from '@tauri-apps/api/core'

/**
 * @file extensionApi.ts
 * @description 扩展中心 API 封装（共享目录模式）
 * @author DeepAgent
 */

// ==================== 类型定义 ====================

/**
 * Extension 数据类型（后端返回）
 */
export interface ExtensionDto {
    id: string
    name: string
    extension_id: string      // Chrome扩展ID
    version: string
    category: string
    description: string
    icon: string
    source: string            // local/store/upload
    file_path: string         // 相对于 Extensions 目录
    manifest_json: string     // manifest.json 内容
    enabled: boolean
    created_at: string
    updated_at: string
}

/**
 * 创建 Extension 的 DTO
 */
export interface CreateExtensionDto {
    name: string
    extension_id: string
    version?: string
    category?: string
    description?: string
    icon?: string
    source?: string
    file_path: string
    manifest_json?: string
}

/**
 * 更新 Extension 的 DTO
 */
export interface UpdateExtensionDto {
    name?: string
    category?: string
    description?: string
    icon?: string
    enabled?: boolean
}

/**
 * 前端使用的 Extension 类型
 */
export interface Extension {
    id: string
    name: string
    extensionId: string       // Chrome扩展ID
    version: string
    category: string
    description: string
    icon: string
    iconBg: string            // 前端样式
    iconColor: string         // 前端样式
    source: string
    filePath: string
    manifestJson: string
    enabled: boolean
    installed: boolean        // 前端计算：filePath 非空
    rating?: number           // 前端展示用
    reviews?: number          // 前端展示用
    createdAt: number
    updatedAt: number
}

// ==================== 样式映射 ====================

const CATEGORY_STYLE_MAP: Record<string, { bg: string; color: string }> = {
    'web3': { bg: 'bg-orange-50 dark:bg-orange-900/20', color: 'text-orange-600' },
    'wallet': { bg: 'bg-orange-50 dark:bg-orange-900/20', color: 'text-orange-600' },
    'adblock': { bg: 'bg-red-50 dark:bg-red-900/20', color: 'text-red-600' },
    'productivity': { bg: 'bg-blue-50 dark:bg-blue-900/20', color: 'text-blue-600' },
    'social': { bg: 'bg-pink-50 dark:bg-pink-900/20', color: 'text-pink-600' },
    'developer': { bg: 'bg-purple-50 dark:bg-purple-900/20', color: 'text-purple-600' },
    'default': { bg: 'bg-gray-50 dark:bg-gray-900/20', color: 'text-gray-600' },
}

function getCategoryStyle(category: string): { bg: string; color: string } {
    const lowerCategory = category.toLowerCase()
    for (const [key, style] of Object.entries(CATEGORY_STYLE_MAP)) {
        if (lowerCategory.includes(key)) {
            return style
        }
    }
    return CATEGORY_STYLE_MAP['default']
}

// ==================== DTO 转换 ====================

/**
 * 将后端 DTO 转换为前端 Extension
 */
function dtoToExtension(dto: ExtensionDto): Extension {
    const style = getCategoryStyle(dto.category)
    return {
        id: dto.id,
        name: dto.name,
        extensionId: dto.extension_id,
        version: dto.version,
        category: dto.category,
        description: dto.description,
        icon: dto.icon || 'extension',
        iconBg: style.bg,
        iconColor: style.color,
        source: dto.source,
        filePath: dto.file_path,
        manifestJson: dto.manifest_json,
        enabled: dto.enabled,
        installed: !!dto.file_path && dto.file_path.length > 0,
        createdAt: new Date(dto.created_at).getTime(),
        updatedAt: new Date(dto.updated_at).getTime(),
    }
}

// ==================== API 函数 ====================

/**
 * 获取所有扩展列表
 */
export async function getExtensions(): Promise<Extension[]> {
    try {
        const dtos = await invoke<ExtensionDto[]>('get_extensions')
        return dtos.map(dtoToExtension)
    } catch (error) {
        console.error('Failed to get extensions:', error)
        throw new Error(`获取扩展列表失败: ${error}`)
    }
}

/**
 * 获取已安装的扩展
 */
export async function getInstalledExtensions(): Promise<Extension[]> {
    try {
        const dtos = await invoke<ExtensionDto[]>('get_installed_extensions')
        return dtos.map(dtoToExtension)
    } catch (error) {
        console.error('Failed to get installed extensions:', error)
        throw new Error(`获取已安装扩展失败: ${error}`)
    }
}

/**
 * 获取单个扩展
 */
export async function getExtension(id: string): Promise<Extension> {
    try {
        const dto = await invoke<ExtensionDto>('get_extension', { id })
        return dtoToExtension(dto)
    } catch (error) {
        console.error('Failed to get extension:', error)
        throw new Error(`获取扩展详情失败: ${error}`)
    }
}

/**
 * 创建/注册扩展
 */
export async function createExtension(data: CreateExtensionDto): Promise<Extension> {
    try {
        const dto = await invoke<ExtensionDto>('create_extension', { data })
        return dtoToExtension(dto)
    } catch (error) {
        console.error('Failed to create extension:', error)
        throw new Error(`创建扩展失败: ${error}`)
    }
}

/**
 * 更新扩展
 */
export async function updateExtension(id: string, data: UpdateExtensionDto): Promise<Extension> {
    try {
        const dto = await invoke<ExtensionDto>('update_extension', { id, data })
        return dtoToExtension(dto)
    } catch (error) {
        console.error('Failed to update extension:', error)
        throw new Error(`更新扩展失败: ${error}`)
    }
}

/**
 * 删除扩展
 */
export async function deleteExtension(id: string): Promise<void> {
    try {
        await invoke('delete_extension', { id })
    } catch (error) {
        console.error('Failed to delete extension:', error)
        throw new Error(`删除扩展失败: ${error}`)
    }
}

/**
 * 切换扩展启用状态
 */
export async function toggleExtension(id: string, enabled: boolean): Promise<Extension> {
    try {
        const dto = await invoke<ExtensionDto>('toggle_extension', { id, enabled })
        return dtoToExtension(dto)
    } catch (error) {
        console.error('Failed to toggle extension:', error)
        throw new Error(`切换扩展状态失败: ${error}`)
    }
}

/**
 * 扫描扩展目录，自动注册新扩展
 */
export async function scanExtensions(): Promise<Extension[]> {
    try {
        const dtos = await invoke<ExtensionDto[]>('scan_extensions')
        return dtos.map(dtoToExtension)
    } catch (error) {
        console.error('Failed to scan extensions:', error)
        throw new Error(`扫描扩展目录失败: ${error}`)
    }
}

// ==================== Profile-扩展关联 API ====================

/**
 * 获取 Profile 启用的扩展
 */
export async function getProfileExtensions(profileId: string): Promise<Extension[]> {
    try {
        const dtos = await invoke<ExtensionDto[]>('get_profile_extensions', { profileId })
        return dtos.map(dtoToExtension)
    } catch (error) {
        console.error('Failed to get profile extensions:', error)
        throw new Error(`获取Profile扩展失败: ${error}`)
    }
}

/**
 * 为 Profile 启用扩展
 */
export async function enableExtensionForProfile(profileId: string, extensionId: string): Promise<void> {
    try {
        await invoke('enable_extension_for_profile', { profileId, extensionId })
    } catch (error) {
        console.error('Failed to enable extension for profile:', error)
        throw new Error(`启用扩展失败: ${error}`)
    }
}

/**
 * 为 Profile 禁用扩展
 */
export async function disableExtensionForProfile(profileId: string, extensionId: string): Promise<void> {
    try {
        await invoke('disable_extension_for_profile', { profileId, extensionId })
    } catch (error) {
        console.error('Failed to disable extension for profile:', error)
        throw new Error(`禁用扩展失败: ${error}`)
    }
}
