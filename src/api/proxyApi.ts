import { invoke } from '@tauri-apps/api/core'

/**
 * @file proxyApi.ts
 * @description 代理模板管理 API 封装
 * @author DeepAgent
 */

// ==================== 类型定义 ====================

/**
 * Proxy 数据类型（后端返回）
 */
export interface ProxyDto {
    id: string
    name: string
    type: string  // "http" | "https" | "socks5" | "direct"
    source: string  // "custom" | "imported"
    tag: string
    host: string
    port: string
    username?: string
    password?: string
    ip_address?: string
    location?: string
    latency?: number
    used_count: number
    auto_check: boolean
    expire_at?: string
    bind_window?: string
    remark: string
    status: string  // "active" | "expired" | "error"
    created_at: string  // RFC3339
    updated_at: string  // RFC3339
}

/**
 * 创建 Proxy 的 DTO
 */
export interface CreateProxyDto {
    name: string
    type: string
    host: string
    port: string
    source?: string
    tag?: string
    username?: string
    password?: string
    remark?: string
    auto_check?: boolean
    expire_at?: string
}

/**
 * 更新 Proxy 的 DTO
 */
export interface UpdateProxyDto {
    name?: string
    type?: string
    host?: string
    port?: string
    source?: string
    tag?: string
    username?: string
    password?: string
    ip_address?: string
    location?: string
    remark?: string
    auto_check?: boolean
    expire_at?: string
    bind_window?: string
    status?: string
}

/**
 * 前端使用的 Proxy 类型
 */
export interface Proxy {
    id: string
    name: string
    type: 'http' | 'https' | 'socks5' | 'direct'
    source: string
    tag: string
    host: string
    port: string
    username?: string
    password?: string
    ipAddress?: string
    location?: string
    latency?: number
    usedCount: number
    autoCheck: boolean
    expireAt?: string
    bindWindow?: string
    remark: string
    status: 'active' | 'expired' | 'error'
    createdAt: number
    updatedAt: number
}

// ==================== DTO 转换 ====================

/**
 * 将后端 DTO 转换为前端 Proxy
 */
function dtoToProxy(dto: ProxyDto): Proxy {
    return {
        id: dto.id,
        name: dto.name,
        type: dto.type as Proxy['type'],
        source: dto.source,
        tag: dto.tag,
        host: dto.host,
        port: dto.port,
        username: dto.username,
        password: dto.password,
        ipAddress: dto.ip_address,
        location: dto.location,
        latency: dto.latency,
        usedCount: dto.used_count,
        autoCheck: dto.auto_check,
        expireAt: dto.expire_at,
        bindWindow: dto.bind_window,
        remark: dto.remark,
        status: dto.status as Proxy['status'],
        createdAt: new Date(dto.created_at).getTime(),
        updatedAt: new Date(dto.updated_at).getTime(),
    }
}

// ==================== API 函数 ====================

/**
 * 获取所有代理模板列表
 */
export async function getProxies(): Promise<Proxy[]> {
    try {
        const dtos = await invoke<ProxyDto[]>('get_proxies')
        return dtos.map(dtoToProxy)
    } catch (error) {
        console.error('Failed to get proxies:', error)
        throw new Error(`获取代理列表失败: ${error}`)
    }
}

/**
 * 创建代理模板
 */
export async function createProxy(data: CreateProxyDto): Promise<Proxy> {
    try {
        const dto = await invoke<ProxyDto>('create_proxy', { data })
        return dtoToProxy(dto)
    } catch (error) {
        console.error('Failed to create proxy:', error)
        throw new Error(`创建代理失败: ${error}`)
    }
}

/**
 * 更新代理模板
 */
export async function updateProxy(id: string, data: UpdateProxyDto): Promise<Proxy> {
    try {
        const dto = await invoke<ProxyDto>('update_proxy', { id, data })
        return dtoToProxy(dto)
    } catch (error) {
        console.error('Failed to update proxy:', error)
        throw new Error(`更新代理失败: ${error}`)
    }
}

/**
 * 删除代理模板
 */
export async function deleteProxy(id: string): Promise<void> {
    try {
        await invoke('delete_proxy', { id })
    } catch (error) {
        console.error('Failed to delete proxy:', error)
        throw new Error(`删除代理失败: ${error}`)
    }
}

/**
 * 代理检测结果（增强版）
 */
export interface ProxyTestResult {
    proxy_id: string
    success: boolean
    latency?: number
    ip?: string
    country?: string
    country_code?: string
    city?: string
    isp?: string
    location?: string
    error?: string
    checked_at: string
}

/**
 * 检测代理连通性（使用真实 HTTP/SOCKS5 协议检测）
 */
export async function testProxy(id: string): Promise<ProxyTestResult> {
    try {
        const result = await invoke<ProxyTestResult>('test_proxy', { id })
        return result
    } catch (error) {
        console.error('Failed to test proxy:', error)
        throw new Error(`检测代理失败: ${error}`)
    }
}

/**
 * 直接检测代理配置 (未保存前)
 */
export async function testProxyConfig(
    proxyType: string,
    host: string, 
    port: string,
    username?: string,
    password?: string
): Promise<ProxyTestResult> {
    try {
        const result = await invoke<ProxyTestResult>('test_proxy_config', { 
            proxyType, 
            host, 
            port,
            username,
            password
        })
        return result
    } catch (error) {
        console.error('Failed to test proxy config:', error)
        throw new Error(`检测代理配置失败: ${error}`)
    }
}

/**
 * 批量检测代理
 */
export async function batchTestProxies(ids: string[]): Promise<ProxyTestResult[]> {
    try {
        const results = await invoke<ProxyTestResult[]>('batch_test_proxies', { ids })
        return results
    } catch (error) {
        console.error('Failed to batch test proxies:', error)
        throw new Error(`批量检测代理失败: ${error}`)
    }
}

/**
 * 检测所有代理
 */
export async function testAllProxies(): Promise<ProxyTestResult[]> {
    try {
        const results = await invoke<ProxyTestResult[]>('test_all_proxies')
        return results
    } catch (error) {
        console.error('Failed to test all proxies:', error)
        throw new Error(`检测所有代理失败: ${error}`)
    }
}

/**
 * 设置代理自动检测状态
 */
export async function setProxyAutoCheck(id: string, enabled: boolean): Promise<void> {
    try {
        await invoke('set_proxy_auto_check', { id, enabled })
    } catch (error) {
        console.error('Failed to set proxy auto check:', error)
        throw new Error(`设置自动检测失败: ${error}`)
    }
}
