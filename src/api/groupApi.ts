import { invoke } from '@tauri-apps/api/core'

/**
 * Group 数据类型（后端返回）
 */
export interface GroupDto {
    id: string
    name: string
    sort: number
    profile_count: number
    permission: string
    remark?: string
    icon?: string
    created_at: string  // RFC3339
    updated_at: string  // RFC3339
}

/**
 * 创建 Group 的 DTO
 */
export interface CreateGroupDto {
    name: string
    sort: number
    permission: string
    remark?: string
    icon?: string
}

/**
 * 更新 Group 的 DTO
 */
export interface UpdateGroupDto {
    name?: string
    sort?: number
    permission?: string
    remark?: string
    icon?: string
}

/**
 * 前端使用的 Group 类型
 */
export interface Group {
    id: string
    name: string
    sort: number
    profileCount: number
    permission: string
    remark?: string
    icon?: string
    createdAt: number
    updatedAt: number
}

/**
 * 将后端 DTO 转换为前端 Group
 */
function dtoToGroup(dto: GroupDto): Group {
    return {
        id: dto.id,
        name: dto.name,
        sort: dto.sort,
        profileCount: dto.profile_count,
        permission: dto.permission,
        remark: dto.remark,
        icon: dto.icon,
        createdAt: new Date(dto.created_at).getTime(),
        updatedAt: new Date(dto.updated_at).getTime(),
    }
}

/**
 * 获取所有分组列表
 */
export async function getGroups(): Promise<Group[]> {
    try {
        const dtos = await invoke<GroupDto[]>('get_groups')
        return dtos.map(dtoToGroup)
    } catch (error) {
        console.error('Failed to get groups:', error)
        throw new Error(`获取分组列表失败: ${error}`)
    }
}

/**
 * 获取单个分组
 */
export async function getGroup(id: string): Promise<Group> {
    try {
        const dto = await invoke<GroupDto>('get_group', { id })
        return dtoToGroup(dto)
    } catch (error) {
        console.error('Failed to get group:', error)
        throw new Error(`获取分组失败: ${error}`)
    }
}

/**
 * 创建分组
 */
export async function createGroup(data: CreateGroupDto): Promise<Group> {
    try {
        const dto = await invoke<GroupDto>('create_group', { data })
        return dtoToGroup(dto)
    } catch (error) {
        console.error('Failed to create group:', error)
        throw new Error(`创建分组失败: ${error}`)
    }
}

/**
 * 更新分组
 */
export async function updateGroup(id: string, data: UpdateGroupDto): Promise<Group> {
    try {
        const dto = await invoke<GroupDto>('update_group', { id, data })
        return dtoToGroup(dto)
    } catch (error) {
        console.error('Failed to update group:', error)
        throw new Error(`更新分组失败: ${error}`)
    }
}

/**
 * 删除分组
 */
export async function deleteGroup(id: string): Promise<void> {
    try {
        await invoke('delete_group', { id })
    } catch (error) {
        console.error('Failed to delete group:', error)
        throw new Error(`删除分组失败: ${error}`)
    }
}
