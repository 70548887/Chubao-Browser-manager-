import { invoke } from '@tauri-apps/api/core'
import type { Profile, BatchResult } from '@/types'

/**
 * @file recycleBinApi.ts
 * @description 回收站管理 API 封装
 * @author DeepAgent
 */

// ==================== 类型定义 ====================

/**
 * 回收站中的 Profile（与 Profile 结构相同 + deletedAt）
 */
export interface RecycledProfile extends Profile {
  deletedAt: number  // ✅ V5 新增：删除时间戳
}

/**
 * Profile 数据类型（后端返回，与 profileApi.ts 保持一致）
 */
interface ProfileDto {
    id: string
    name: string
    group: string
    remark: string
    status: 'stopped' | 'running'
    fingerprint: Record<string, any>
    proxy: Record<string, any> | null
    created_at: string  // RFC3339
    updated_at: string  // RFC3339
}

/**
 * RecycledProfile 数据类型（后端返回）
 */
interface RecycledProfileDto extends ProfileDto {
    deleted_at: string  // RFC3339 - ✅ V5 新增
}

// ==================== DTO 转换 ====================

/**
 * 将后端 DTO 转换为前端 RecycledProfile
 */
function dtoToRecycledProfile(dto: RecycledProfileDto): RecycledProfile {
    return {
        id: dto.id,
        name: dto.name,
        group: dto.group || '',
        status: dto.status,
        remark: dto.remark,
        fingerprint: dto.fingerprint as any,
        proxy: dto.proxy as any,
        createdAt: new Date(dto.created_at).getTime(),
        updatedAt: new Date(dto.updated_at).getTime(),
        deletedAt: new Date(dto.deleted_at).getTime(),  // ✅ V5 新增：真实删除时间
    }
}

// ==================== API 函数 ====================

/**
 * 获取回收站列表
 */
export async function getRecycleBin(): Promise<RecycledProfile[]> {
    try {
        const dtos = await invoke<RecycledProfileDto[]>('get_recycle_bin')
        return dtos.map(dtoToRecycledProfile)
    } catch (error) {
        console.error('Failed to get recycle bin:', error)
        throw new Error(`获取回收站列表失败: ${error}`)
    }
}

/**
 * 恢复窗口（从回收站恢复）
 */
export async function restoreProfile(id: string): Promise<void> {
    try {
        await invoke('restore_profile', { id })
    } catch (error) {
        console.error('Failed to restore profile:', error)
        throw new Error(`恢复窗口失败: ${error}`)
    }
}

/**
 * 批量恢复窗口
 */
export async function batchRestoreProfiles(ids: string[]): Promise<BatchResult> {
    try {
        const result = await invoke<BatchResult>('batch_restore_profiles', { ids })
        return result
    } catch (error) {
        console.error('Failed to batch restore profiles:', error)
        throw new Error(`批量恢复窗口失败: ${error}`)
    }
}

/**
 * 永久删除窗口
 */
export async function permanentlyDeleteProfile(id: string): Promise<void> {
    try {
        await invoke('permanently_delete_profile', { id })
    } catch (error) {
        console.error('Failed to permanently delete profile:', error)
        throw new Error(`永久删除窗口失败: ${error}`)
    }
}

/**
 * 批量永久删除窗口
 */
export async function batchPermanentlyDeleteProfiles(ids: string[]): Promise<BatchResult> {
    try {
        const result = await invoke<BatchResult>('batch_permanently_delete_profiles', { ids })
        return result
    } catch (error) {
        console.error('Failed to batch permanently delete profiles:', error)
        throw new Error(`批量永久删除窗口失败: ${error}`)
    }
}

/**
 * 清空回收站
 */
export async function emptyRecycleBin(): Promise<number> {
    try {
        const count = await invoke<number>('empty_recycle_bin')
        return count
    } catch (error) {
        console.error('Failed to empty recycle bin:', error)
        throw new Error(`清空回收站失败: ${error}`)
    }
}
