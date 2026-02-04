import { invoke } from '@tauri-apps/api/core'

/**
 * @file tagApi.ts
 * @description 标签管理 API 封装
 * @author DeepAgent
 */

// ==================== 类型定义 ====================

/**
 * Tag 数据类型（后端返回）
 */
export interface TagDto {
    id: string
    name: string
    sort: number
    remark?: string | null
    window_count: number | null
    created_at: string  // RFC3339
    updated_at: string  // RFC3339
}

/**
 * 创建 Tag 的 DTO
 */
export interface CreateTagDto {
    name: string
    sort?: number
    remark?: string
}

/**
 * 更新 Tag 的 DTO
 */
export interface UpdateTagDto {
    name?: string
    sort?: number
    remark?: string
}

/**
 * 前端使用的 Tag 类型
 */
export interface Tag {
    id: string
    name: string
    sort: number
    remark?: string
    windowCount: number
    createdAt: number
    updatedAt: number
}

// ==================== DTO 转换 ====================

/**
 * 将后端 DTO 转换为前端 Tag
 */
function dtoToTag(dto: TagDto): Tag {
    return {
        id: dto.id,
        name: dto.name,
        sort: dto.sort,
        remark: dto.remark || undefined,
        windowCount: dto.window_count ?? 0,
        createdAt: new Date(dto.created_at).getTime(),
        updatedAt: new Date(dto.updated_at).getTime(),
    }
}

// ==================== API 函数 ====================

/**
 * 获取所有标签列表
 */
export async function getTags(): Promise<Tag[]> {
    try {
        const dtos = await invoke<TagDto[]>('get_tags')
        return dtos.map(dtoToTag)
    } catch (error) {
        console.error('Failed to get tags:', error)
        throw new Error(`获取标签列表失败: ${error}`)
    }
}

/**
 * 创建标签
 */
export async function createTag(data: CreateTagDto): Promise<Tag> {
    try {
        const dto = await invoke<TagDto>('create_tag', { data })
        return dtoToTag(dto)
    } catch (error) {
        console.error('Failed to create tag:', error)
        throw new Error(`创建标签失败: ${error}`)
    }
}

/**
 * 更新标签
 */
export async function updateTag(id: string, data: UpdateTagDto): Promise<Tag> {
    try {
        const dto = await invoke<TagDto>('update_tag', { id, data })
        return dtoToTag(dto)
    } catch (error) {
        console.error('Failed to update tag:', error)
        throw new Error(`更新标签失败: ${error}`)
    }
}

/**
 * 删除标签
 */
export async function deleteTag(id: string): Promise<void> {
    try {
        await invoke('delete_tag', { id })
    } catch (error) {
        console.error('Failed to delete tag:', error)
        throw new Error(`删除标签失败: ${error}`)
    }
}

/**
 * 获取窗口的标签列表
 */
export async function getProfileTags(profileId: string): Promise<Tag[]> {
    try {
        const dtos = await invoke<TagDto[]>('get_profile_tags', { profileId })
        return dtos.map(dtoToTag)
    } catch (error) {
        console.error('Failed to get profile tags:', error)
        throw new Error(`获取窗口标签失败: ${error}`)
    }
}

/**
 * 设置窗口的标签（替换模式）
 */
export async function setProfileTags(profileId: string, tagIds: string[]): Promise<void> {
    try {
        await invoke('set_profile_tags', { profileId, tagIds })
    } catch (error) {
        console.error('Failed to set profile tags:', error)
        throw new Error(`设置窗口标签失败: ${error}`)
    }
}
