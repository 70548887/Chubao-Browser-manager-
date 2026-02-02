/**
 * @file profile.store.test.ts
 * @description Profile Store 单元测试
 * @author DeepAgent
 */

import { describe, it, expect, beforeEach, vi } from 'vitest'
import { setActivePinia, createPinia } from 'pinia'
import { useProfileStore } from '../profile.store'
import type { Profile } from '@/types/profile.types'
import { createProfile } from '@/api'

// Mock API
vi.mock('@/api', () => ({
    getProfiles: vi.fn(),
    getProfile: vi.fn(),
    createProfile: vi.fn(),
    updateProfile: vi.fn(),
    deleteProfile: vi.fn(),
    batchDeleteProfiles: vi.fn(),
    launchBrowser: vi.fn(),
    stopBrowser: vi.fn(),
    batchLaunchBrowsers: vi.fn(),
    batchStopBrowsers: vi.fn(),
}))

// Mock Tauri API
vi.mock('@tauri-apps/api/core', () => ({
    invoke: vi.fn(),
}))

describe('Profile Store', () => {
    beforeEach(() => {
        // 在每个测试前创建新的 Pinia 实例
        setActivePinia(createPinia())
    })

    describe('初始状态', () => {
        it('应该有空的 profiles 数组', () => {
            const store = useProfileStore()
            expect(store.profiles).toEqual([])
        })

        it('应该 isLoading 为 false', () => {
            const store = useProfileStore()
            expect(store.isLoading).toBe(false)
        })

        it('应该 error 为 null', () => {
            const store = useProfileStore()
            expect(store.error).toBe(null)
        })
    })

    describe('计算属性', () => {
        it('runningCount 应该返回运行中的环境数量', () => {
            const store = useProfileStore()

            // 手动添加测试数据
            store.profiles = [
                { id: '1', name: 'Test 1', status: 'running' } as Profile,
                { id: '2', name: 'Test 2', status: 'stopped' } as Profile,
                { id: '3', name: 'Test 3', status: 'running' } as Profile,
            ]

            expect(store.runningCount).toBe(2)
        })

        it('totalCount 应该返回总环境数量', () => {
            const store = useProfileStore()

            store.profiles = [
                { id: '1', name: 'Test 1' } as Profile,
                { id: '2', name: 'Test 2' } as Profile,
            ]

            expect(store.totalCount).toBe(2)
        })
    })

    describe('Actions', () => {
        it('createProfile 应该添加环境到列表', async () => {
            const store = useProfileStore()
            const newProfile: Profile = {
                id: '1',
                name: 'Test Profile',
                status: 'stopped',
            } as Profile

            // Mock createProfile API
            vi.mocked(createProfile).mockResolvedValue(newProfile)

            await store.createProfile({
                name: 'Test Profile',
            } as any)

            expect(store.profiles).toHaveLength(1)
            expect(store.profiles[0]).toEqual(newProfile)
        })

        it('deleteProfile 应该从列表中删除环境', async () => {
            const store = useProfileStore()
            store.profiles = [
                { id: '1', name: 'Test 1' } as Profile,
                { id: '2', name: 'Test 2' } as Profile,
            ]

            await store.deleteProfile('1')

            expect(store.profiles).toHaveLength(1)
            expect(store.profiles[0].id).toBe('2')
        })

        it('updateProfile 应该更新环境信息', async () => {
            const store = useProfileStore()
            store.profiles = [
                { id: '1', name: 'Old Name', status: 'stopped' } as Profile,
            ]

            // Mock getProfiles to return updated profile
            const { getProfiles } = await import('@/api')
            vi.mocked(getProfiles).mockResolvedValue([
                { id: '1', name: 'New Name', status: 'stopped' } as Profile
            ])

            await store.updateProfile('1', { name: 'New Name' })

            expect(store.profiles[0].name).toBe('New Name')
        })
    })
})
