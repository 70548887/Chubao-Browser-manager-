/**
 * 用户认证 API
 * 
 * 所有请求通过 IPC 调用后端，Token 安全存储在系统密钥链
 */

import { invoke } from '@tauri-apps/api/core'

// ==================== 类型定义 ====================

/** 用户信息 */
export interface UserInfo {
    id: string
    username: string
    email: string
    avatar?: string
    role: string
    created_at: string
}

/** 登录响应 */
export interface LoginResponse {
    user: UserInfo
    access_token: string
    refresh_token: string
    expires_in: number
}

/** 许可证信息 */
export interface LicenseInfo {
    license_key: string
    status: 'active' | 'expired' | 'revoked'
    plan: 'free' | 'pro' | 'enterprise'
    expires_at?: string
    max_profiles: number
    features: string[]
}

// ==================== 认证接口 ====================

/**
 * 用户登录
 * @param account 账号（用户名或邮箱）
 * @param password 密码
 * @param remember 是否记住登录状态
 */
export async function login(
    account: string,
    password: string,
    remember: boolean = false
): Promise<LoginResponse> {
    return await invoke<LoginResponse>('auth_login', {
        account,
        password,
        remember,
    })
}

/**
 * 用户注册
 * @param username 用户名
 * @param email 邮箱
 * @param password 密码
 * @param confirmPassword 确认密码
 * @param inviteCode 邀请码（可选）
 */
export async function register(
    username: string,
    email: string,
    password: string,
    confirmPassword: string,
    inviteCode?: string
): Promise<UserInfo> {
    return await invoke<UserInfo>('auth_register', {
        username,
        email,
        password,
        confirmPassword,
        inviteCode,
    })
}

/**
 * 用户登出
 */
export async function logout(): Promise<void> {
    await invoke('auth_logout')
}

/**
 * 检查登录状态
 * @returns 是否已登录
 */
export async function checkLogin(): Promise<boolean> {
    return await invoke<boolean>('auth_check_login')
}

/**
 * 获取当前用户信息
 */
export async function getUserInfo(): Promise<UserInfo> {
    return await invoke<UserInfo>('auth_get_user')
}

/**
 * 刷新 Token
 */
export async function refreshToken(): Promise<LoginResponse> {
    return await invoke<LoginResponse>('auth_refresh_token')
}

// ==================== 许可证接口 ====================

/**
 * 验证许可证
 */
export async function validateLicense(): Promise<LicenseInfo> {
    return await invoke<LicenseInfo>('license_validate')
}

/**
 * 激活许可证
 * @param licenseKey 许可证密钥
 */
export async function activateLicense(licenseKey: string): Promise<LicenseInfo> {
    return await invoke<LicenseInfo>('license_activate', { licenseKey })
}

// ==================== 辅助函数 ====================

/**
 * 检查是否为 Pro 用户
 */
export async function isPro(): Promise<boolean> {
    try {
        const license = await validateLicense()
        return license.plan === 'pro' || license.plan === 'enterprise'
    } catch {
        return false
    }
}

/**
 * 获取许可证剩余天数
 */
export async function getLicenseRemainingDays(): Promise<number | null> {
    try {
        const license = await validateLicense()
        if (!license.expires_at) return null
        
        const expiresAt = new Date(license.expires_at)
        const now = new Date()
        const diffTime = expiresAt.getTime() - now.getTime()
        const diffDays = Math.ceil(diffTime / (1000 * 60 * 60 * 24))
        
        return diffDays > 0 ? diffDays : 0
    } catch {
        return null
    }
}
