/**
 * 消息通知 API
 * 
 * 所有请求通过 IPC 调用后端，需要登录后才能使用
 */

import { invoke } from '@tauri-apps/api/core'

// ==================== 类型定义 ====================

/** 通知类型 */
export type NotificationType = 'system' | 'update' | 'warning' | 'promo'

/** 通知消息 */
export interface Notification {
    id: string
    title: string
    content: string
    notification_type: NotificationType
    is_read: boolean
    created_at: string
    link?: string
}

/** 通知列表响应 */
export interface NotificationListResponse {
    notifications: Notification[]
    total: number
    unread_count: number
}

// ==================== API 接口 ====================

/**
 * 获取通知列表
 * @param page 页码（默认 1）
 * @param pageSize 每页数量（默认 20）
 */
export async function getNotifications(
    page: number = 1,
    pageSize: number = 20
): Promise<NotificationListResponse> {
    return await invoke<NotificationListResponse>('notification_list', {
        page,
        pageSize,
    })
}

/**
 * 获取未读通知数量
 */
export async function getUnreadCount(): Promise<number> {
    return await invoke<number>('notification_unread_count')
}

/**
 * 标记通知为已读
 * @param ids 通知 ID 列表
 */
export async function markAsRead(ids: string[]): Promise<void> {
    await invoke('notification_mark_read', { ids })
}

/**
 * 标记所有通知为已读
 */
export async function markAllAsRead(): Promise<void> {
    await invoke('notification_mark_all_read')
}

/**
 * 删除通知
 * @param id 通知 ID
 */
export async function deleteNotification(id: string): Promise<void> {
    await invoke('notification_delete', { id })
}

// ==================== 辅助函数 ====================

/**
 * 获取通知图标
 */
export function getNotificationIcon(type: NotificationType): string {
    const icons: Record<NotificationType, string> = {
        system: 'info',
        update: 'system_update',
        warning: 'warning',
        promo: 'campaign',
    }
    return icons[type] || 'notifications'
}

/**
 * 获取通知图标样式类
 */
export function getNotificationIconClass(type: NotificationType): string {
    const classes: Record<NotificationType, string> = {
        system: 'notification-icon--info',
        update: 'notification-icon--update',
        warning: 'notification-icon--warning',
        promo: 'notification-icon--promo',
    }
    return classes[type] || ''
}

/**
 * 格式化通知时间
 */
export function formatNotificationTime(dateStr: string): string {
    const date = new Date(dateStr)
    const now = new Date()
    const diffMs = now.getTime() - date.getTime()
    const diffMins = Math.floor(diffMs / 60000)
    const diffHours = Math.floor(diffMs / 3600000)
    const diffDays = Math.floor(diffMs / 86400000)

    if (diffMins < 1) return '刚刚'
    if (diffMins < 60) return `${diffMins}分钟前`
    if (diffHours < 24) return `${diffHours}小时前`
    if (diffDays < 7) return `${diffDays}天前`
    
    return date.toLocaleDateString('zh-CN')
}
