// Profile Store - 环境数据状态管理（统一 IPC 版本）
import { defineStore } from 'pinia'
import type { Profile, ProfileStatus, ProfileCreateDTO, ProfileUpdateDTO } from '@/types'
import {
    getProfiles,
    getProfile,
    createProfile,
    updateProfile,
    deleteProfile,
    batchDeleteProfiles,
    launchBrowser,
    stopBrowser,
    batchLaunchBrowsers,
    batchStopBrowsers,
} from '@/api'

interface ProfileState {
    /** 环境列表 */
    profiles: Profile[]
    /** 当前选中的环境 */
    currentProfile: Profile | null
    /** 加载状态 */
    isLoading: boolean
    /** 错误信息 */
    error: string | null
    /** 搜索关键词 */
    searchKeyword: string
    /** 筛选状态 */
    filterStatus: ProfileStatus | 'all'
}

export const useProfileStore = defineStore('profile', {
    state: (): ProfileState => ({
        profiles: [],
        currentProfile: null,
        isLoading: false,
        error: null,
        searchKeyword: '',
        filterStatus: 'all',
    }),

    getters: {
        /** 运行中的环境数量 */
        runningCount: (state) =>
            state.profiles.filter(p => p.status === 'running').length,

        /** 总环境数量 */
        totalCount: (state) => state.profiles.length,

        /** 过滤后的环境列表 */
        filteredProfiles: (state) => {
            let result = state.profiles

            // 状态筛选
            if (state.filterStatus !== 'all') {
                result = result.filter(p => p.status === state.filterStatus)
            }

            // 关键词搜索
            if (state.searchKeyword) {
                const keyword = state.searchKeyword.toLowerCase()
                result = result.filter(p =>
                    p.name.toLowerCase().includes(keyword) ||
                    p.id.toLowerCase().includes(keyword) ||
                    p.remark?.toLowerCase().includes(keyword)
                )
            }

            return result
        },
    },

    actions: {
        // ==================== 初始化与获取 ====================
        
        /** 初始化环境列表（从后端获取）*/
        async initProfiles() {
            await this.fetchProfiles()
        },

        /** 获取环境列表 */
        async fetchProfiles() {
            this.isLoading = true
            this.error = null
            try {
                this.profiles = await getProfiles()
            } catch (e) {
                this.error = (e as Error).message
                console.error('获取环境列表失败:', e)
            } finally {
                this.isLoading = false
            }
        },

        /** 根据 ID 获取单个环境 */
        async getProfileById(id: string): Promise<Profile | null> {
            try {
                return await getProfile(id)
            } catch (e) {
                console.error('获取环境详情失败:', e)
                return null
            }
        },

        // ==================== CRUD 操作 ====================
        
        /** 创建新环境 */
        async createProfile(data: ProfileCreateDTO): Promise<Profile | null> {
            this.isLoading = true
            this.error = null
            try {
                const newProfile = await createProfile(data as any)
                // 添加到本地列表
                this.profiles.unshift(newProfile)
                return newProfile
            } catch (e) {
                this.error = (e as Error).message
                console.error('创建环境失败:', e)
                throw e
            } finally {
                this.isLoading = false
            }
        },

        /** 更新环境 */
        async updateProfile(id: string, data: ProfileUpdateDTO): Promise<void> {
            this.isLoading = true
            this.error = null
            try {
                await updateProfile(id, data as any)
                // 刷新列表
                await this.fetchProfiles()
            } catch (e) {
                this.error = (e as Error).message
                console.error('更新环境失败:', e)
                throw e
            } finally {
                this.isLoading = false
            }
        },

        /** 删除单个环境 */
        async deleteProfile(id: string): Promise<void> {
            this.isLoading = true
            this.error = null
            try {
                await deleteProfile(id)
                // 从本地列表移除
                this.profiles = this.profiles.filter(p => p.id !== id)
            } catch (e) {
                this.error = (e as Error).message
                console.error('删除环境失败:', e)
                throw e
            } finally {
                this.isLoading = false
            }
        },

        /** 批量删除环境 */
        async deleteProfiles(ids: string[]): Promise<import('@/types').BatchResult> {
            this.isLoading = true
            this.error = null
            try {
                const result = await batchDeleteProfiles(ids)
                // 只移除成功删除的项
                const successIds = result.results.filter((r: import('@/types').BatchItemResult) => r.ok).map((r: import('@/types').BatchItemResult) => r.profileId)
                this.profiles = this.profiles.filter(p => !successIds.includes(p.id))
                return result
            } catch (e) {
                this.error = (e as Error).message
                console.error('批量删除环境失败:', e)
                throw e
            } finally {
                this.isLoading = false
            }
        },

        // ==================== 浏览器启停操作 ====================
        
        /** 启动单个环境 */
        async startProfile(id: string): Promise<void> {
            try {
                await launchBrowser(id)
                // 更新本地状态
                this.updateStatus(id, 'running')
            } catch (e) {
                console.error('启动环境失败:', e)
                this.updateStatus(id, 'error')
                throw e
            }
        },

        /** 停止单个环境 */
        async stopProfile(id: string): Promise<void> {
            try {
                await stopBrowser(id)
                // 更新本地状态
                this.updateStatus(id, 'stopped')
            } catch (e) {
                console.error('停止环境失败:', e)
                throw e
            }
        },

        /** 批量启动环境 */
        async startProfiles(ids: string[]): Promise<import('@/types').BatchResult> {
            try {
                const result = await batchLaunchBrowsers(ids)
                // 只更新成功启动的项
                result.results.filter((r: import('@/types').BatchItemResult) => r.ok).forEach((r: import('@/types').BatchItemResult) => this.updateStatus(r.profileId, 'running'))
                return result
            } catch (e) {
                console.error('批量启动环境失败:', e)
                throw e
            }
        },

        /** 批量停止环境 */
        async stopProfiles(ids: string[]): Promise<import('@/types').BatchResult> {
            try {
                const result = await batchStopBrowsers(ids)
                // 只更新成功停止的项
                result.results.filter((r: import('@/types').BatchItemResult) => r.ok).forEach((r: import('@/types').BatchItemResult) => this.updateStatus(r.profileId, 'stopped'))
                return result
            } catch (e) {
                console.error('批量停止环境失败:', e)
                throw e
            }
        },

        // ==================== 状态管理 ====================

        /** 更新单个环境状态 (来自 Tauri 事件或本地操作) */
        updateStatus(profileId: string, status: ProfileStatus) {
            const profile = this.profiles.find(p => p.id === profileId)
            if (profile) {
                profile.status = status
                profile.updatedAt = Date.now()
                if (status === 'running') {
                    profile.lastOpenTime = Date.now()
                }
            }
        },

        /** 设置搜索关键词 */
        setSearchKeyword(keyword: string) {
            this.searchKeyword = keyword
        },

        /** 设置状态筛选 */
        setFilterStatus(status: ProfileStatus | 'all') {
            this.filterStatus = status
        },

        /** 选中环境 */
        selectProfile(profile: Profile | null) {
            this.currentProfile = profile
        },

        // ==================== 便捷查询方法 ====================

        /** 按分组筛选 */
        getProfilesByGroup(groupId: string): Profile[] {
            return this.profiles.filter(p => p.group === groupId)
        },

        /** 按状态筛选 */
        getProfilesByStatus(status: ProfileStatus): Profile[] {
            return this.profiles.filter(p => p.status === status)
        },

        /** 搜索环境 */
        searchProfiles(keyword: string): Profile[] {
            const lowerKeyword = keyword.toLowerCase()
            return this.profiles.filter(p =>
                p.name.toLowerCase().includes(lowerKeyword) ||
                p.remark?.toLowerCase().includes(lowerKeyword) ||
                p.id.toLowerCase().includes(lowerKeyword)
            )
        },

        /** 批量移动到分组 */
        async moveToGroup(ids: string[], groupId: string): Promise<import('@/types').BatchResult> {
            this.isLoading = true
            this.error = null
            try {
                const { batchMoveToGroup } = await import('@/api/profileApi')
                const result = await batchMoveToGroup(ids, groupId)
                
                // 只更新成功的项
                result.results.filter((r: import('@/types').BatchItemResult) => r.ok).forEach((r: import('@/types').BatchItemResult) => {
                    const profile = this.profiles.find(p => p.id === r.profileId)
                    if (profile) {
                        profile.group = groupId
                        profile.updatedAt = Date.now()
                    }
                })
                
                return result
            } catch (e) {
                this.error = (e as Error).message
                console.error('批量移动到分组失败:', e)
                throw e
            } finally {
                this.isLoading = false
            }
        },

        /** 批量复制环境 */
        async duplicateProfiles(ids: string[]): Promise<import('@/types').BatchResult> {
            this.isLoading = true
            this.error = null
            try {
                const { batchDuplicateProfiles } = await import('@/api/profileApi')
                const result = await batchDuplicateProfiles(ids)
                
                // 刷新列表获取新创建的环境
                await this.fetchProfiles()
                
                return result
            } catch (e) {
                this.error = (e as Error).message
                console.error('批量复制环境失败:', e)
                throw e
            } finally {
                this.isLoading = false
            }
        },
    },

    persist: {
        // 只持久化部分状态
        pick: ['searchKeyword', 'filterStatus'],
    },
})
