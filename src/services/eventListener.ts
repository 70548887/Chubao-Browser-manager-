// 事件监听服务 - 监听后端 Tauri 事件
import { listen } from '@tauri-apps/api/event'
import { useProfileStore } from '@/stores/profile.store'
import { useGroupStore } from '@/stores/groupStore'
import { Message } from '@/utils/message'
import type { Profile } from '@/types'

/**
 * Profile 状态变化事件
 */
interface ProfileStatusChangedEvent {
    profileId: string
    status: 'running' | 'stopped' | 'error'
}

/**
 * Profile 删除事件
 */
interface ProfileDeletedEvent {
    id: string
}

/**
 * 浏览器错误事件
 */
interface BrowserErrorEvent {
    profileId: string
    error: string
    timestamp: number
}

/**
 * 初始化事件监听
 */
export async function initEventListeners() {
    const profileStore = useProfileStore()
    const groupStore = useGroupStore()

    // 1. 监听 Profile 状态变化事件
    await listen<ProfileStatusChangedEvent>('profile:status_changed', (event) => {
        console.log('Profile status changed:', event.payload)
        const { profileId, status } = event.payload

        // 更新 store 中的状态
        profileStore.updateStatus(profileId, status)
    })

    // 2. 监听 Profile 创建事件
    await listen<Profile>('profile:created', (event) => {
        console.log('Profile created:', event.payload)
        const profile = event.payload
        
        // 将新创建的窗口添加到列表（如果当前不在列表中）
        const exists = profileStore.profiles.some(p => p.id === profile.id)
        if (!exists) {
            profileStore.profiles.unshift(profile)
            Message.success(`窗口 "${profile.name}" 创建成功`)
        }
        // 刷新分组数据（更新窗口数量）
        groupStore.initGroups()
    })

    // 3. 监听 Profile 更新事件
    await listen<Profile>('profile:updated', (event) => {
        console.log('Profile updated:', event.payload)
        const profile = event.payload
        
        // 更新列表中的窗口信息
        const index = profileStore.profiles.findIndex(p => p.id === profile.id)
        if (index !== -1) {
            profileStore.profiles[index] = profile
            Message.success(`窗口 "${profile.name}" 更新成功`)
        }
        // 刷新分组数据（分组可能改变）
        groupStore.initGroups()
    })

    // 4. 监听 Profile 删除事件
    await listen<ProfileDeletedEvent>('profile:deleted', (event) => {
        console.log('Profile deleted:', event.payload)
        const { id } = event.payload
        
        // 从列表中移除
        profileStore.profiles = profileStore.profiles.filter(p => p.id !== id)
        Message.success('窗口已删除')
        // 刷新分组数据（更新窗口数量）
        groupStore.initGroups()
    })

    // 5. 监听浏览器错误事件
    await listen<BrowserErrorEvent>('browser:error', (event) => {
        console.error('Browser error:', event.payload)
        const { profileId, error } = event.payload
        
        // 更新状态为错误
        profileStore.updateStatus(profileId, 'error')
        
        // 显示错误提示
        const profile = profileStore.profiles.find(p => p.id === profileId)
        const name = profile?.name || profileId
        Message.error(`窗口 "${name}" 启动失败：${error}`)
    })

    console.log('Event listeners initialized')
}

/**
 * 清理事件监听（可选，用于组件卸载时）
 */
export function cleanupEventListeners() {
    // Tauri 的 listen 返回 unlisten 函数，可以保存并在需要时调用
    // 这里简化处理，实际使用时可以保存 unlisten 函数
    console.log('Event listeners cleaned up')
}
